//! Liveness, readiness, and version endpoints.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::state::AppState;

/// Liveness: the process is up.
pub async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

/// Readiness: the service can do its whole job. Render's health check calls this route
/// (`render.yaml` `healthCheckPath`), so a failed check keeps the previous deploy live.
///
/// Checks, by name:
/// - `database`: `select 1` answers;
/// - `operator_token_replay`: the replay store accepts claims (migration 0018 applied);
/// - `ADMIN_JWT_TRUSTED_KEYS`: the operator trusted-key set loaded. Without it every operator
///   route, the five stop routes included, returns 500.
///
/// The body names failed checks only. It never carries a value, a key or an error text.
pub async fn ready(State(state): State<AppState>) -> impl IntoResponse {
    let mut failed: Vec<&'static str> = Vec::new();
    if let Err(e) = sqlx::query_scalar::<_, i32>("select 1")
        .fetch_one(&state.pool)
        .await
    {
        tracing::warn!(error = %e, "readiness check failed: database");
        failed.push("database");
    }
    if state.replay.ready().await.is_err() {
        tracing::warn!("readiness check failed: operator_token_replay");
        failed.push("operator_token_replay");
    }
    if !state.admin_verifier.is_configured() || !state.posture_verifier.is_configured() {
        tracing::warn!("readiness check failed: ADMIN_JWT_TRUSTED_KEYS");
        failed.push(crate::auth::operator::TRUSTED_KEYS_VAR);
    }
    if failed.is_empty() {
        (StatusCode::OK, Json(json!({ "status": "ready" })))
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "status": "not_ready", "failed": failed })),
        )
    }
}

/// Version / build information.
pub async fn version(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "service": "forgecustomer-api",
        "version": env!("CARGO_PKG_VERSION"),
        "git_sha": option_env!("GIT_SHA").unwrap_or("unknown"),
        "app_env": state.config.app_env,
    }))
}
