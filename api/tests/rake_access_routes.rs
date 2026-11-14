//! BDS-FOC-RAKE-ACCESS-ISSUER-v0.1 WP-02: route and start-up checks that need no database
//! (AC-02 auth rows, AC-03). The database-backed checks are in
//! `scripts/rake_access_http_smoke.sh`.
//!
//! Environment variables are process-global, so every test builds one base `Config` from the
//! same variables and changes fields on its own copy.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use forgecustomer_api::config::Config;
use forgecustomer_api::routes::build_router;
use forgecustomer_api::state::AppState;

const ENTITLEMENT_SEED: [u8; 32] = [3u8; 32];
const RAKE_SEED: [u8; 32] = [5u8; 32];
const STANDBY_SEED: [u8; 32] = [6u8; 32];

fn base_config() -> Config {
    std::env::set_var("DATABASE_URL", "postgres://user@127.0.0.1:1/none");
    std::env::set_var("DATABASE_ACQUIRE_TIMEOUT_SECS", "1");
    std::env::set_var("SUPABASE_JWT_ISSUER", "https://proj.supabase.co/auth/v1");
    std::env::set_var("SUPABASE_JWT_SECRET", "supabase-secret");
    std::env::set_var("ADMIN_JWT_AUDIENCE", "forgecustomer-admin");
    std::env::set_var(
        "ENTITLEMENT_SIGNING_PRIVATE_KEY",
        B64.encode(ENTITLEMENT_SEED),
    );
    std::env::set_var("RATE_LIMIT_PER_MINUTE", "0");
    let mut config = Config::from_env().expect("config");
    config.rake_access_signing_private_key = String::new();
    config.rake_access_verify_public_keys = Vec::new();
    config
}

fn with_rake_key(mut config: Config) -> Config {
    config.rake_access_signing_private_key = B64.encode(RAKE_SEED);
    config.rake_access_signing_key_id = "rake-access-1".into();
    config
}

fn public_of(seed: [u8; 32]) -> String {
    B64.encode(
        ed25519_dalek::SigningKey::from_bytes(&seed)
            .verifying_key()
            .to_bytes(),
    )
}

async fn call(state: AppState, req: Request<Body>) -> (StatusCode, Value) {
    let res = build_router(state).oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

fn exchange(auth: Option<&str>) -> Request<Body> {
    let mut b = Request::builder()
        .method("POST")
        .uri("/v1/app-tokens/rake")
        .header("content-type", "application/json");
    if let Some(a) = auth {
        b = b.header("authorization", a);
    }
    b.body(Body::from(
        r#"{"app_id":"authorforge","client_proof":{"installation_id":"00000000-0000-4000-8000-000000000001","iat":0,"nonce":"00000000-0000-4000-8000-000000000002","signature":"x"}}"#,
    ))
    .unwrap()
}

#[tokio::test]
async fn exchange_without_a_token_is_401() {
    let state = AppState::build(with_rake_key(base_config())).unwrap();
    let (status, body) = call(state, exchange(None)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "{body}");
}

#[tokio::test]
async fn exchange_with_a_bad_token_is_401() {
    let state = AppState::build(with_rake_key(base_config())).unwrap();
    let (status, body) = call(state, exchange(Some("Bearer not.a.jwt"))).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "{body}");
}

#[tokio::test]
async fn keys_route_publishes_the_rake_keys_only() {
    let mut config = with_rake_key(base_config());
    config.rake_access_verify_public_keys = vec![("rake-access-2".into(), public_of(STANDBY_SEED))];
    let state = AppState::build(config).unwrap();

    let get = |uri: &str| Request::builder().uri(uri).body(Body::empty()).unwrap();
    let (status, body) = call(state.clone(), get("/v1/app-tokens/keys")).await;
    assert_eq!(status, StatusCode::OK);
    let keys = body["keys"].as_array().unwrap();
    let ids: Vec<(&str, &str)> = keys
        .iter()
        .map(|k| (k["kid"].as_str().unwrap(), k["status"].as_str().unwrap()))
        .collect();
    assert_eq!(
        ids,
        vec![
            ("rake-access-1", "active"),
            ("rake-access-2", "verify_only")
        ]
    );
    assert_eq!(keys[0]["public_key"], public_of(RAKE_SEED));
    assert!(keys.iter().all(|k| k["alg"] == "EdDSA"));
    assert!(
        keys.iter().all(|k| k.as_object().unwrap().len() == 4),
        "public fields only: {body}"
    );

    // The entitlement key route does not list the Rake keys (one key, one purpose).
    let (_, ent) = call(state, get("/v1/entitlements/keys")).await;
    let ent_keys: Vec<&str> = ent["keys"]
        .as_array()
        .unwrap()
        .iter()
        .map(|k| k["public_key"].as_str().unwrap())
        .collect();
    assert!(!ent_keys.contains(&public_of(RAKE_SEED).as_str()));
    assert!(!ent_keys.contains(&public_of(STANDBY_SEED).as_str()));
}

#[tokio::test]
async fn without_a_rake_key_the_keys_list_is_empty() {
    let state = AppState::build(base_config()).unwrap();
    let req = Request::builder()
        .uri("/v1/app-tokens/keys")
        .body(Body::empty())
        .unwrap();
    let (status, body) = call(state, req).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["keys"], serde_json::json!([]));
}

#[test]
fn startup_fails_when_the_rake_key_is_the_entitlement_key() {
    let mut config = base_config();
    config.rake_access_signing_private_key = B64.encode(ENTITLEMENT_SEED);
    let err = AppState::build(config).err().expect("must fail");
    assert!(
        err.to_string().contains("must differ from the entitlement"),
        "{err}"
    );
}

#[test]
fn startup_fails_when_verify_keys_are_set_without_a_signing_key() {
    let mut config = base_config();
    config.rake_access_verify_public_keys = vec![("rake-access-2".into(), public_of(STANDBY_SEED))];
    let err = AppState::build(config).err().expect("must fail");
    assert!(
        err.to_string().contains("RAKE_ACCESS_VERIFY_PUBLIC_KEYS"),
        "{err}"
    );
}

#[test]
fn startup_fails_on_a_malformed_rake_key() {
    let mut config = base_config();
    config.rake_access_signing_private_key = "not-base64!".into();
    let err = AppState::build(config).err().expect("must fail");
    assert!(
        err.to_string().contains("RAKE_ACCESS_SIGNING_PRIVATE_KEY"),
        "{err}"
    );
}
