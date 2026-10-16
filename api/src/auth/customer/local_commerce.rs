//! Manually started local integration server. Never compiled into the shipping API.

use super::*;
use crate::{config::Config, routes, state::AppState};
use std::net::SocketAddr;

fn loopback_url(value: &str, scheme: &str) -> Option<reqwest::Url> {
    let url = reqwest::Url::parse(value).ok()?;
    (url.scheme() == scheme
        && url.host_str() == Some("127.0.0.1")
        && url.port().is_some_and(|port| port != 0)
        && url.query().is_none()
        && url.fragment().is_none())
    .then_some(url)
}

fn local_jwks(issuer: &str) -> Option<reqwest::Url> {
    let mut url = loopback_url(issuer, "http")?;
    if !url.username().is_empty() || url.password().is_some() || url.path() != "/auth/v1" {
        return None;
    }
    url.set_path("/auth/v1/.well-known/jwks.json");
    Some(url)
}

fn sandbox_key(key: &str) -> bool {
    key.is_empty()
        || key.starts_with("sk_test_")
        || key.starts_with("rk_test_")
        || key.starts_with("rkcs_test_")
}

/// Run only against a separately initialized, disposable local Supabase stack.
/// This is a server for manual verification, not a completed automated flow test.
#[tokio::test]
#[ignore = "manual local integration server; requires isolated Supabase and explicit environment"]
async fn serve_isolated_local_commerce() {
    let config = Config::from_env().expect("local harness configuration required");
    assert_eq!(config.app_env, "local-stripe-sandbox");
    assert_eq!(config.host, "127.0.0.1");
    assert_ne!(config.port, 0);
    assert!(
        loopback_url(&config.database_url, "postgres").is_some()
            || loopback_url(&config.database_url, "postgresql").is_some()
    );
    assert!(sandbox_key(&config.stripe_secret_key));
    assert_eq!(config.stripe_api_base, "https://api.stripe.com");
    assert!(config.admin_jwt_public_key.is_empty());
    assert!(config.dataforge_api_url.is_empty());
    assert!(config.dataforge_service_token.is_empty());
    // Public fixture seed already used by the security suite; never use it outside tests.
    assert!(
        config.entitlement_signing_private_key == "AwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwM="
    );
    let jwks = local_jwks(&config.supabase_jwt_issuer).expect("loopback Auth issuer required");
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let mut state = AppState::build(config).expect("local harness state");
    // Only transport changes: ES256 signatures, issuer/audience/expiry checks, bounded
    // refresh and key selection still use the real customer validator. No token minting.
    state.customer_validator.jwks_url = Some(jwks);
    state.customer_validator.http = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(3))
        .timeout(Duration::from_secs(5))
        .build()
        .expect("loopback JWKS client");
    let app = routes::build_router(state);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("local harness listener");
    println!("Isolated commerce harness listening on {addr}");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("local harness server");
}

#[test]
fn harness_rejects_remote_issuers_and_live_stripe_keys() {
    assert!(local_jwks("http://127.0.0.1:55321/auth/v1").is_some());
    for issuer in [
        "https://project.supabase.co/auth/v1",
        "http://localhost:55321/auth/v1",
        "http://127.0.0.1/auth/v1",
        "http://127.0.0.1:0/auth/v1",
        "http://user@127.0.0.1:55321/auth/v1",
        "http://127.0.0.1:55321/other",
        "http://127.0.0.1:55321/auth/v1?redirect=elsewhere",
        "http://127.0.0.1:55321/auth/v1#fragment",
    ] {
        assert!(local_jwks(issuer).is_none());
    }
    assert!(sandbox_key(""));
    assert!(sandbox_key("sk_test_fixture"));
    assert!(sandbox_key("rk_test_fixture"));
    assert!(sandbox_key("rkcs_test_fixture"));
    assert!(!sandbox_key("sk_live_fixture"));
    assert!(!sandbox_key("rk_live_fixture"));
    assert!(!sandbox_key("rkcs_live_fixture"));
    assert!(loopback_url("postgres://user@db.example.test:5432/test", "postgres").is_none());
    assert!(loopback_url("postgresql://user@127.0.0.1:55322/test", "postgresql").is_some());
}
