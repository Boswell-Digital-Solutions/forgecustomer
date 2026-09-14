//! Tower middleware: correlation IDs, security headers, and per-client rate limiting.
//! JWT validation and customer/admin context are implemented as extractors in
//! `auth::extract`.

use axum::body::{to_bytes, Body};
use axum::extract::Request;
use axum::http::{HeaderName, HeaderValue};
use axum::middleware::Next;
use axum::response::Response;
use uuid::Uuid;

pub mod rate_limit;

/// Correlation id stored in request extensions and echoed on the response.
#[derive(Debug, Clone)]
pub struct CorrelationId(pub String);

const HEADER: &str = "x-correlation-id";

/// Client-supplied correlation ids are persisted into audit rows and logs, so only
/// short, log-safe values are honored; anything else is replaced with a generated id.
const MAX_CORRELATION_ID_LEN: usize = 128;

fn acceptable_correlation_id(value: &str) -> bool {
    (1..=MAX_CORRELATION_ID_LEN).contains(&value.len())
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}

/// Assign or propagate a correlation id for every request.
pub async fn correlation_id(mut req: Request, next: Next) -> Response {
    let incoming = req
        .headers()
        .get(HEADER)
        .and_then(|v| v.to_str().ok())
        .filter(|s| acceptable_correlation_id(s))
        .map(|s| s.to_string());

    let id = incoming.unwrap_or_else(|| format!("corr_{}", Uuid::new_v4().simple()));
    req.extensions_mut().insert(CorrelationId(id.clone()));

    let mut res = next.run(req).await;
    if let Ok(value) = HeaderValue::from_str(&id) {
        res.headers_mut()
            .insert(HeaderName::from_static(HEADER), value);
    }
    backfill_body_correlation_id(res, &id).await
}

/// Error responses carry `correlation_id` in the JSON body (`error.rs`'s `AppError`), but
/// only the auth extractors (`AuthUserContext`/`AdminContext`) attach it at construction
/// time -- an ordinary handler-body error (e.g. any `?`-converted `sqlx::Error`, which never
/// sees the request) leaves it `null`, even though the same id is already sitting right here
/// and is always echoed on the `x-correlation-id` header. Threading the id down into every
/// error-construction site (many of them in repository functions with no request access)
/// would be a much larger, invasive change than backfilling the still-null field here, once,
/// for every error response, so the body always matches the header it ships next to.
async fn backfill_body_correlation_id(res: Response, id: &str) -> Response {
    if !res.status().is_client_error() && !res.status().is_server_error() {
        return res;
    }
    let is_json = res
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|ct| ct.starts_with("application/json"));
    if !is_json {
        return res;
    }

    let (mut parts, body) = res.into_parts();
    // Error bodies are small, fixed-shape JSON; 1 MiB is far more than any of them need and
    // only guards against ever buffering something unbounded here.
    let Ok(bytes) = to_bytes(body, 1024 * 1024).await else {
        return Response::from_parts(parts, Body::empty());
    };

    let Ok(mut value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return Response::from_parts(parts, Body::from(bytes));
    };

    let needs_backfill = value
        .get("error")
        .and_then(|e| e.get("correlation_id"))
        .is_none_or(|v| v.is_null());
    if needs_backfill {
        if let Some(error) = value.get_mut("error").and_then(|e| e.as_object_mut()) {
            error.insert(
                "correlation_id".to_string(),
                serde_json::Value::String(id.to_string()),
            );
        }
    }

    let encoded = serde_json::to_vec(&value).unwrap_or_else(|_| bytes.to_vec());
    // The re-encoded body rarely matches the original byte-for-byte (a filled-in field
    // changes the length); drop the stale Content-Length so hyper recomputes it, rather
    // than ship a body whose length lies to the client.
    parts.headers.remove(axum::http::header::CONTENT_LENGTH);
    Response::from_parts(parts, Body::from(encoded))
}

/// Apply conservative security headers to every response.
pub async fn security_headers(req: Request, next: Next) -> Response {
    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    headers.insert(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    headers.insert(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("no-referrer"),
    );
    headers.insert(
        HeaderName::from_static("strict-transport-security"),
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );
    res
}

#[cfg(test)]
mod tests {
    use super::{acceptable_correlation_id, correlation_id, HEADER};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::response::IntoResponse;
    use axum::routing::get;
    use axum::{Json, Router};
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[test]
    fn accepts_short_log_safe_correlation_ids() {
        assert!(acceptable_correlation_id("corr_0af1"));
        assert!(acceptable_correlation_id("trace-7.segment_2"));
        assert!(acceptable_correlation_id(&"a".repeat(128)));
    }

    #[test]
    fn rejects_oversized_or_hostile_correlation_ids() {
        assert!(!acceptable_correlation_id(""));
        assert!(!acceptable_correlation_id(&"a".repeat(129)));
        assert!(!acceptable_correlation_id("corr id with spaces"));
        assert!(!acceptable_correlation_id("corr\"quote"));
        assert!(!acceptable_correlation_id("corr{json}"));
    }

    fn app_returning(body: Value, status: StatusCode) -> Router {
        Router::new()
            .route(
                "/probe",
                get(move || {
                    let body = body.clone();
                    async move { (status, Json(body)).into_response() }
                }),
            )
            .layer(axum::middleware::from_fn(correlation_id))
    }

    async fn body_json(res: axum::response::Response) -> Value {
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    /// The real bug: a handler-body error (e.g. `AppError::internal` via `?`, which never
    /// sees the request) serializes `correlation_id: null` -- the middleware must backfill
    /// it from the same id it stamps on the response header, so the body isn't misleading.
    #[tokio::test]
    async fn backfills_null_correlation_id_on_error_bodies() {
        let app = app_returning(
            json!({"error": {"code": "INTERNAL", "message": "A database error occurred.", "correlation_id": null, "details": {}}}),
            StatusCode::INTERNAL_SERVER_ERROR,
        );

        let res = app
            .oneshot(
                Request::builder()
                    .uri("/probe")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let header_id = res
            .headers()
            .get(HEADER)
            .and_then(|v| v.to_str().ok())
            .unwrap()
            .to_string();
        assert!(acceptable_correlation_id(&header_id));

        let body = body_json(res).await;
        assert_eq!(body["error"]["correlation_id"], json!(header_id));
    }

    /// A correlation id already attached at construction time (the auth extractors'
    /// `.with_correlation(...)`) must survive untouched, not be overwritten by this
    /// middleware's own generated id.
    #[tokio::test]
    async fn does_not_clobber_an_already_populated_correlation_id() {
        let app = app_returning(
            json!({"error": {"code": "INVALID_TOKEN", "message": "The access token is invalid.", "correlation_id": "corr_from_extractor", "details": {}}}),
            StatusCode::UNAUTHORIZED,
        );

        let res = app
            .oneshot(
                Request::builder()
                    .uri("/probe")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = body_json(res).await;
        assert_eq!(
            body["error"]["correlation_id"],
            json!("corr_from_extractor")
        );
    }

    /// Success responses are left alone -- no reason to buffer/rewrite a 200 JSON body that
    /// was never in the `{"error": {...}}` shape to begin with.
    #[tokio::test]
    async fn leaves_success_bodies_untouched() {
        let app = app_returning(json!({"ok": true}), StatusCode::OK);

        let res = app
            .oneshot(
                Request::builder()
                    .uri("/probe")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = body_json(res).await;
        assert_eq!(body, json!({"ok": true}));
    }
}
