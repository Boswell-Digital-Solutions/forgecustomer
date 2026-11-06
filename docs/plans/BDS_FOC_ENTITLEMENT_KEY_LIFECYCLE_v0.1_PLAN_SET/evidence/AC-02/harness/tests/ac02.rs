use std::collections::BTreeMap;

use ac02_harness::af::{verify_snapshot, ReasonCode, TrustSet};
use axum::body::Body;
use axum::http::Request;
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use forgecustomer_api::config::Config;
use forgecustomer_api::domain::entitlement::FeatureValue;
use forgecustomer_api::domain::snapshot::{EntitlementSnapshot, SCHEMA_VERSION};
use forgecustomer_api::routes::build_router;
use forgecustomer_api::services::signing::Signer25519;
use forgecustomer_api::state::AppState;
use http_body_util::BodyExt;
use tower::ServiceExt;

const INSTALLATION: &str = "inst_ac02";

fn config(verify_keys: &str) -> Config {
    for (k, v) in [
        ("DATABASE_URL", "postgres://user@127.0.0.1:1/none"),
        ("DATABASE_ACQUIRE_TIMEOUT_SECS", "1"),
        ("SUPABASE_JWT_ISSUER", "https://example.supabase.co/auth/v1"),
        ("ADMIN_JWT_ISSUER", "test_issuer"),
        ("ADMIN_JWT_AUDIENCE", "forgecustomer-admin"),
        ("ENTITLEMENT_SIGNING_KEY_ID", "entitlement-key-2"),
    ] {
        std::env::set_var(k, v);
    }
    std::env::set_var("ENTITLEMENT_SIGNING_PRIVATE_KEY", B64.encode([2u8; 32]));
    std::env::set_var("ENTITLEMENT_VERIFY_PUBLIC_KEYS", verify_keys);
    Config::from_env().expect("config")
}

async fn published_keys(state: AppState) -> String {
    let res = build_router(state)
        .oneshot(Request::builder().uri("/v1/entitlements/keys").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert!(res.status().is_success());
    String::from_utf8(res.into_body().collect().await.unwrap().to_bytes().to_vec()).unwrap()
}

fn snapshot_json(signer: &Signer25519, now: i64) -> String {
    let mut features = BTreeMap::new();
    features.insert("authorforge.cloud.enabled".to_string(), FeatureValue::Bool(true));
    let ts = |s: i64| chrono::DateTime::from_timestamp(s, 0).unwrap().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let mut snap = EntitlementSnapshot {
        schema_version: SCHEMA_VERSION.to_string(),
        customer_id: "cust_ac02".to_string(),
        installation_id: Some(INSTALLATION.to_string()),
        product: "authorforge".to_string(),
        issued_at: ts(now - 60),
        expires_at: ts(now + 86_400),
        features,
        quotas: BTreeMap::new(),
        key_id: String::new(),
        signature: String::new(),
    };
    signer.sign_snapshot(&mut snap).unwrap();
    serde_json::to_string(&snap).unwrap()
}

/// One sequential test: environment variables are process-global.
#[tokio::test]
async fn ac02_author_forge_trusts_both_keys_forgecustomer_publishes() {
    let now = chrono::Utc::now().timestamp();
    let active = Signer25519::from_base64_seed("entitlement-key-2", &B64.encode([2u8; 32])).unwrap();
    let standby = Signer25519::from_base64_seed("entitlement-key-1", &B64.encode([1u8; 32])).unwrap();
    let from_active = snapshot_json(&active, now);
    let from_standby = snapshot_json(&standby, now);

    // ForgeCustomer with the standby key in ENTITLEMENT_VERIFY_PUBLIC_KEYS.
    let with = config(&format!(r#"{{"entitlement-key-1":"{}"}}"#, standby.public_key_base64()));
    let body = published_keys(AppState::build(with).expect("state")).await;
    println!("published (with variable): {body}");
    let trust = TrustSet::from_json(&body).expect("Author-Forge parses the published list");
    for (name, raw) in [("active", &from_active), ("standby", &from_standby)] {
        let verdict = verify_snapshot(raw, &trust, Some(INSTALLATION), now);
        println!("{name}: {:?} cloud_allowed={}", verdict.reason_code, verdict.cloud_allowed);
        assert_eq!(verdict.reason_code, ReasonCode::Ok, "{name}");
        assert!(verdict.cloud_allowed, "{name}");
    }

    // Negative control: ForgeCustomer without the variable publishes only the active key.
    let without = config("");
    let body = published_keys(AppState::build(without).expect("state")).await;
    println!("published (without variable): {body}");
    let trust = TrustSet::from_json(&body).unwrap();
    let verdict = verify_snapshot(&from_standby, &trust, Some(INSTALLATION), now);
    println!("standby without variable: {:?}", verdict.reason_code);
    assert_eq!(verdict.reason_code, ReasonCode::UnknownKeyId);
    assert_eq!(verify_snapshot(&from_active, &trust, Some(INSTALLATION), now).reason_code, ReasonCode::Ok);
}
