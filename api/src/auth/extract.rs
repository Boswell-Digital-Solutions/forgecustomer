//! Axum extractors that enforce authentication and resolve request context.
//!
//! `CustomerContext` validates a Supabase JWT and resolves the business customer.
//! `AdminContext` validates an operator JWT against the *separate* admin issuer/audience —
//! a customer token can never satisfy it.

use axum::async_trait;
use axum::extract::{FromRequestParts, MatchedPath, RawPathParams};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use uuid::Uuid;

use crate::auth::{bearer_token, AdminContext, AuthUserContext, CustomerContext};
use crate::error::{AppError, ErrorCode};
use crate::middleware::CorrelationId;
use crate::state::AppState;

fn correlation(parts: &Parts) -> Option<String> {
    parts.extensions.get::<CorrelationId>().map(|c| c.0.clone())
}

fn auth_header(parts: &Parts) -> Option<&str> {
    parts
        .headers
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
}

#[async_trait]
impl FromRequestParts<AppState> for CustomerContext {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let corr = correlation(parts);
        let attach = |e: AppError| match &corr {
            Some(id) => e.with_correlation(id.clone()),
            None => e,
        };

        let token = bearer_token(auth_header(parts)).map_err(attach)?;
        let claims = state
            .customer_validator
            .validate(token)
            .await
            .map_err(attach)?;

        let auth_user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            attach(AppError::new(
                ErrorCode::InvalidToken,
                "Token subject is not a valid user id.",
            ))
        })?;

        // Resolve the business customer. Missing profile => forbidden (fail closed).
        let row = crate::repositories::customers::find_by_auth_user_id(&state.pool, auth_user_id)
            .await
            .map_err(|e| attach(AppError::from(e)))?;

        let (customer_id, status, mfa_required, mfa_grace_period_ends_at) = match row {
            Some(r) => (
                Some(r.id),
                Some(r.status),
                r.mfa_required,
                r.mfa_grace_period_ends_at,
            ),
            None => (None, None, false, None),
        };

        Ok(CustomerContext {
            auth_user_id: claims.sub,
            customer_id,
            status,
            aal: claims.aal,
            mfa_required,
            mfa_grace_period_ends_at,
        })
    }
}

impl CustomerContext {
    /// Require a fully provisioned, non-suspended, non-terminated customer for
    /// privileged product actions. Anonymized/closed accounts fail closed.
    pub fn require_active(&self) -> Result<Uuid, AppError> {
        let id = self.customer_id.ok_or_else(|| {
            AppError::new(ErrorCode::Forbidden, "No customer profile is provisioned.")
        })?;
        if self.is_suspended() {
            return Err(AppError::new(
                ErrorCode::CustomerSuspended,
                "This customer account is suspended.",
            ));
        }
        if matches!(self.status.as_deref(), Some("anonymized") | Some("closed")) {
            return Err(AppError::new(
                ErrorCode::Forbidden,
                "This customer account is closed.",
            ));
        }
        if self.mfa_required && !self.mfa_grace_active() {
            self.require_aal2()?;
        }
        Ok(id)
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUserContext {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let corr = correlation(parts);
        let attach = |e: AppError| match &corr {
            Some(id) => e.with_correlation(id.clone()),
            None => e,
        };

        let token = bearer_token(auth_header(parts)).map_err(attach)?;
        let claims = state
            .customer_validator
            .validate(token)
            .await
            .map_err(attach)?;

        let auth_user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            attach(AppError::new(
                ErrorCode::InvalidToken,
                "Token subject is not a valid user id.",
            ))
        })?;

        Ok(AuthUserContext {
            auth_user_id,
            email: claims.email,
        })
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AdminContext {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let corr = correlation(parts);
        let attach = |e: AppError| match &corr {
            Some(id) => e.with_correlation(id.clone()),
            None => e,
        };

        let token = bearer_token(auth_header(parts)).map_err(attach)?;
        // Step 1 and 2 (P3 G5): signature, registered claims, issuer, `sub` and audience,
        // against the trusted-key set for the ADMIN audience only.
        let verified = state.admin_verifier.verify(token).map_err(attach)?;

        let roles = if verified.bound {
            // Step 2, SC-1: the token names this route's action and target. Only then does it
            // carry the admin role, so a role can never bypass the route check.
            let matched = parts
                .extensions
                .get::<MatchedPath>()
                .map(|m| m.as_str().to_string());
            let params = path_params(parts, state).await;
            crate::auth::binding::check_bound(
                &parts.method,
                matched.as_deref(),
                &params,
                &verified.claims,
            )
            .map_err(attach)?;
            vec!["admin".to_string()]
        } else {
            // The predecessor's key: unchanged behaviour until P3 G7 (DEC-G5-2 (b)).
            verified.claims.operator_roles()
        };

        // Step 3, SC-2: claim the replay identity. A failure claims nothing.
        claim_once(state, &verified).await.map_err(attach)?;

        Ok(AdminContext {
            operator_id: verified.claims.sub,
            roles,
        })
    }
}

/// The request's path parameters as the router matched them, in no particular order.
async fn path_params(parts: &mut Parts, state: &AppState) -> Vec<(String, String)> {
    match RawPathParams::from_request_parts(parts, state).await {
        Ok(raw) => raw
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// SC-2: accept an operator token once. Runs after every other check, so a token that
/// fails a check claims no identity (SC-2O). An unavailable store refuses the token.
pub(crate) async fn claim_once(
    state: &AppState,
    verified: &crate::auth::operator::VerifiedOperator,
) -> Result<(), AppError> {
    match state
        .replay
        .claim(&verified.issuer, &verified.jti, verified.claims.exp as i64)
        .await
    {
        Ok(true) => Ok(()),
        Ok(false) => Err(AppError::new(
            ErrorCode::InvalidToken,
            "The access token was already used.",
        )),
        Err(_) => Err(AppError::new(
            ErrorCode::ServiceUnavailable,
            "Operator token checks are unavailable. Try again.",
        )),
    }
}
