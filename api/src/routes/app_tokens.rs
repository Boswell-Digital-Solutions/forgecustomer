//! Rake access-token exchange (BDS-FOC-RAKE-ACCESS-ISSUER-v0.1, 04 §3-§4 and 05 §2).
//!
//! `POST /v1/app-tokens/rake` turns a verified Supabase login, an active profile, an
//! installation device proof and the application's entitlement into a 300-second EdDSA
//! token for Rake. Each step fails closed, in the order of 05 §2. The token is never
//! logged and never stored; the audit row holds its `jti` only.
//!
//! `GET /v1/app-tokens/keys` publishes the public halves of the Rake access keys.

use axum::extract::State;
use axum::http::header::{HeaderValue, CACHE_CONTROL};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::auth::CustomerContext;
use crate::domain::entitlement::{evaluate, FeatureValue};
use crate::error::{AppError, AppResult, ErrorCode};
use crate::repositories::app_tokens::{self, InstallationAuthError, MintError, MintRecord};
use crate::repositories::entitlements;
use crate::services::rake_access::{
    proof_message, resolve_scopes, verify_proof, ProofError, RakeAccessClaims, ScopeError,
    TOKEN_LIFETIME_SECS,
};
use crate::state::AppState;

/// Publish the Rake access verification keys. Empty when no key is configured.
pub async fn keys(State(state): State<AppState>) -> Json<Value> {
    let keys = state
        .rake_access
        .as_ref()
        .map(|issuer| issuer.published())
        .unwrap_or_default();
    Json(json!({ "schema": "forge.app-tokens.keys.v1", "keys": keys }))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientProof {
    installation_id: String,
    iat: i64,
    nonce: String,
    signature: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExchangeRequest {
    app_id: String,
    scopes: Option<Vec<String>>,
    client_proof: ClientProof,
}

fn uuid_field(raw: &str, field: &'static str) -> AppResult<Uuid> {
    Uuid::parse_str(raw.trim())
        .map_err(|_| AppError::validation("must be a UUID").with_details(json!({ "field": field })))
}

fn clean_app_id(raw: &str) -> AppResult<&str> {
    let v = raw.trim();
    if v.is_empty()
        || v.len() > 64
        || !v
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
    {
        return Err(AppError::validation("app_id is not a valid application id")
            .with_details(json!({ "field": "app_id" })));
    }
    Ok(v)
}

fn clean_scopes(scopes: &Option<Vec<String>>) -> AppResult<()> {
    if let Some(list) = scopes {
        if list.len() > 8
            || list
                .iter()
                .any(|s| s.is_empty() || s.len() > 64 || s.contains([',', '\n', '\r']))
        {
            return Err(AppError::validation("scopes contains an invalid value")
                .with_details(json!({ "field": "scopes" })));
        }
    }
    Ok(())
}

fn client_auth_failed(reason: &'static str) -> AppError {
    AppError::new(ErrorCode::ClientAuthFailed, "Client authentication failed.")
        .with_details(json!({ "reason": reason }))
}

/// Exchange a Supabase login plus an installation device proof for a Rake access token.
pub async fn exchange_rake(
    ctx: CustomerContext,
    State(state): State<AppState>,
    Json(request): Json<ExchangeRequest>,
) -> AppResult<Response> {
    // 1-2. Supabase token (extractor) and the active-profile / MFA rules.
    let customer_id = ctx.require_active()?;

    let issuer = state.rake_access.clone().ok_or_else(|| {
        AppError::new(
            ErrorCode::ServiceUnavailable,
            "Rake access tokens are not configured.",
        )
    })?;

    let app_id = clean_app_id(&request.app_id)?;
    clean_scopes(&request.scopes)?;
    let installation_id = uuid_field(&request.client_proof.installation_id, "installation_id")?;
    let nonce = uuid_field(&request.client_proof.nonce, "nonce")?;

    // 4. The application must be registered and active.
    let client = app_tokens::find_app_client(&state.pool, app_id)
        .await?
        .filter(|c| c.status == "active")
        .ok_or_else(|| AppError::new(ErrorCode::AppNotAllowed, "Unknown or disabled app_id."))?;

    // 3. Client authentication (DEC-2 = A): the installation's registered device key.
    if client.client_auth_method != "installation_device_key" {
        return Err(client_auth_failed("unsupported_method"));
    }
    let device_key = match app_tokens::installation_device_key(
        &state.pool,
        customer_id,
        installation_id,
        &client.product_key,
    )
    .await?
    {
        Ok(key) => key,
        Err(InstallationAuthError::NotFound) => return Err(client_auth_failed("installation")),
        Err(InstallationAuthError::NoActiveDeviceKey) => {
            return Err(client_auth_failed("device_key"))
        }
        Err(InstallationAuthError::NotActivated) => return Err(client_auth_failed("activation")),
    };
    let now = chrono::Utc::now();
    let message = proof_message(
        &client.app_id,
        installation_id,
        request.scopes.as_deref(),
        request.client_proof.iat,
        nonce,
    );
    verify_proof(
        &device_key,
        &message,
        &request.client_proof.signature,
        request.client_proof.iat,
        now.timestamp(),
    )
    .map_err(|e| {
        client_auth_failed(match e {
            ProofError::Stale => "stale",
            ProofError::Malformed => "malformed",
            ProofError::BadSignature => "signature",
        })
    })?;

    // 5. Scopes inside the application's list; rake:approve alone (DEC-6).
    let scopes =
        resolve_scopes(request.scopes.as_deref(), &client.allowed_scopes).map_err(|e| {
            AppError::new(
                ErrorCode::ScopeNotAllowed,
                "Requested scopes are not allowed.",
            )
            .with_details(match e {
                ScopeError::NotAllowed(scope) => json!({ "scope": scope }),
                ScopeError::ApproveNotAlone => json!({ "reason": "rake:approve must be alone" }),
                ScopeError::Invalid => json!({ "reason": "invalid scope list" }),
            })
        })?;

    // 6. Entitlement: only an explicit true passes. Unknown product, error or any other
    // value denies.
    let loaded =
        entitlements::load_entitlement_inputs(&state.pool, customer_id, &client.product_key)
            .await
            .map_err(|_| {
                AppError::new(
                    ErrorCode::ServiceUnavailable,
                    "Entitlement state is unavailable.",
                )
            })?
            .ok_or_else(|| {
                AppError::new(
                    ErrorCode::EntitlementDenied,
                    "The application is not entitled.",
                )
            })?;
    let features = evaluate(&loaded.inputs).features;
    if !matches!(
        features.get(&client.required_feature_key),
        Some(FeatureValue::Bool(true))
    ) {
        return Err(AppError::new(
            ErrorCode::EntitlementDenied,
            "The application is not entitled.",
        )
        .with_details(json!({ "feature": client.required_feature_key })));
    }

    // 7-9. Pseudonyms and the audit row in one transaction, then sign.
    let jti = Uuid::new_v4();
    let iat = now.timestamp();
    let issued_at = chrono::DateTime::from_timestamp(iat, 0).unwrap_or(now);
    let expires_at = issued_at + chrono::Duration::seconds(TOKEN_LIFETIME_SECS);
    let (tenant, user) = app_tokens::record_mint(
        &state.pool,
        &MintRecord {
            jti,
            customer_id,
            app_id: &client.app_id,
            audience: &client.audience,
            kid: issuer.key_id(),
            scopes: &scopes,
            installation_id: Some(installation_id),
            client_auth_method: &client.client_auth_method,
            client_nonce: nonce,
            issued_at,
            expires_at,
        },
    )
    .await
    .map_err(|e| match e {
        MintError::NonceReplayed => client_auth_failed("nonce_replayed"),
        MintError::Db(error) => {
            tracing::error!(error = %error, "rake access mint audit write failed");
            AppError::new(
                ErrorCode::ServiceUnavailable,
                "The token could not be recorded.",
            )
        }
    })?;

    let claims = RakeAccessClaims::new(
        issuer.issuer(),
        &client.audience,
        tenant,
        user,
        &client.app_id,
        scopes.clone(),
        iat,
        jti,
    );
    let token = issuer.sign(&claims).map_err(|_| {
        tracing::error!(jti = %jti, "rake access token signing failed");
        AppError::internal("Failed to sign the token.")
    })?;
    // 10. Log ids only. Never the token.
    tracing::info!(jti = %jti, app_id = %client.app_id, kid = %issuer.key_id(), "rake access token issued");

    let mut response = Json(json!({
        "access_token": token,
        "token_type": "Bearer",
        "expires_in": TOKEN_LIFETIME_SECS,
        "scopes": scopes,
    }))
    .into_response();
    response
        .headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
    Ok(response)
}
