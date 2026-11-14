//! P3 gate G5 acceptance tests: the operator trusted-key set, route binding (SC-1) and the
//! replay rule (SC-2), through the real router.
//!
//! Two test keys stand in for the two issuers. Each comes from a fixed seed in this file, so no
//! real `kid`, public key or issuer of the rebuild appears anywhere (G-3). Nothing issues a
//! real token. The test database is unreachable, so a request that passes every operator check
//! reaches its handler and then fails on the database: a status other than 401 or 403 proves
//! the checks passed.
//!
//! Case ids follow forge-command `docs/plans/BDS_FCO_P3_TOKEN_ISSUER_v0.1/07_G5_AUTHORIZATION_REQUEST.md` §7.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use ed25519_dalek::SigningKey;
use http_body_util::BodyExt;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tower::ServiceExt;

use forgecustomer_api::config::Config;
use forgecustomer_api::routes::build_router;
use forgecustomer_api::services::operator_replay::{
    MemoryReplayStore, ReplayStore, UnavailableReplayStore,
};
use forgecustomer_api::state::AppState;

const ADMIN_AUD: &str = "forgecustomer-admin";
const POSTURE_AUD: &str = "forgecustomer-posture";
const PRED_ISS: &str = "forge_command_local";
/// A test issuer for the bound key. Not a real deployed value (G-3).
const BOUND_ISS: &str = "fcn_local";
const ID: &str = "9f1c2d3e-4b5a-6789-0abc-def012345678";

/// A test key: a fixed seed, its `kid` in one of the two published derivations, its issuer.
#[derive(Clone, Copy)]
struct TestKey {
    seed: u8,
    rebuild_kid: bool,
    issuer: &'static str,
}

const PRED: TestKey = TestKey {
    seed: 11,
    rebuild_kid: false,
    issuer: PRED_ISS,
};
const BOUND: TestKey = TestKey {
    seed: 22,
    rebuild_kid: true,
    issuer: BOUND_ISS,
};

impl TestKey {
    fn raw_public(self) -> [u8; 32] {
        SigningKey::from_bytes(&[self.seed; 32])
            .verifying_key()
            .to_bytes()
    }

    fn kid(self) -> String {
        let digest = Sha256::digest(self.raw_public());
        if self.rebuild_kid {
            hex::encode(&digest[..8])
        } else {
            URL_SAFE_NO_PAD.encode(digest)[..8].to_string()
        }
    }

    fn spki_pem(self) -> String {
        let mut der = vec![
            0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
        ];
        der.extend_from_slice(&self.raw_public());
        format!(
            "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n",
            STANDARD.encode(der)
        )
    }

    fn encoding_key(self) -> EncodingKey {
        // PKCS#8 v1 for Ed25519: a fixed prefix, then the 32-byte seed.
        let mut der = vec![
            0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22,
            0x04, 0x20,
        ];
        der.extend_from_slice(&[self.seed; 32]);
        EncodingKey::from_ed_der(&der)
    }

    /// Sign `claims` with this key, under `kid` (or none).
    fn sign_with_kid(self, kid: Option<String>, claims: &Value) -> String {
        let mut header = Header::new(Algorithm::EdDSA);
        header.kid = kid;
        encode(&header, claims, &self.encoding_key()).unwrap()
    }

    fn sign(self, claims: &Value) -> String {
        self.sign_with_kid(Some(self.kid()), claims)
    }

    /// The standard claims for this key, with a fresh `jti`.
    fn claims(self, aud: &str, extra: Value) -> Value {
        let mut c = json!({
            "sub": self.issuer, "iss": self.issuer, "aud": aud,
            "exp": now() + 120, "jti": uuid::Uuid::new_v4().to_string(),
        });
        for (k, v) in extra.as_object().unwrap() {
            c[k] = v.clone();
        }
        c
    }
}

fn now() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

fn key_set(keys: &[TestKey]) -> String {
    let mut set = serde_json::Map::new();
    for k in keys {
        set.insert(k.kid(), json!({ "key": k.spki_pem(), "issuer": k.issuer }));
    }
    Value::Object(set).to_string()
}

fn config() -> Config {
    std::env::set_var("DATABASE_URL", "postgres://user@127.0.0.1:1/none");
    std::env::set_var("DATABASE_ACQUIRE_TIMEOUT_SECS", "1");
    std::env::set_var("SUPABASE_JWT_ISSUER", "https://proj.supabase.co/auth/v1");
    std::env::set_var("SUPABASE_JWT_AUDIENCE", "authenticated");
    std::env::set_var("SUPABASE_JWT_SECRET", "supabase-secret");
    std::env::set_var("ADMIN_JWT_AUDIENCE", ADMIN_AUD);
    std::env::set_var("ADMIN_JWT_TRUSTED_KEYS", key_set(&[PRED, BOUND]));
    std::env::set_var("STRIPE_WEBHOOK_SECRET", "whsec_test");
    std::env::set_var(
        "ENTITLEMENT_SIGNING_PRIVATE_KEY",
        STANDARD.encode([3u8; 32]),
    );
    std::env::set_var("ENTITLEMENT_SIGNING_KEY_ID", "entitlement-key-1");
    std::env::set_var("RATE_LIMIT_PER_MINUTE", "10000");
    std::env::set_var(
        "UPDATE_ROLLOUT_SECRET",
        "operator-token-test-rollout-secret",
    );
    std::env::set_var(
        "RELEASE_ARTIFACT_BASE_URL",
        "https://downloads.example.test/authorforge",
    );
    Config::from_env().expect("config")
}

/// State over a given key set and replay store.
fn state_with(keys: &str, store: Arc<dyn ReplayStore>) -> AppState {
    let mut cfg = config();
    cfg.admin_jwt_trusted_keys = keys.to_string();
    let mut state = AppState::build(cfg).expect("state");
    state.replay = store;
    state
}

fn state() -> AppState {
    state_with(
        &key_set(&[PRED, BOUND]),
        Arc::new(MemoryReplayStore::default()),
    )
}

fn request(method: &str, uri: &str, token: &str) -> Request<Body> {
    let builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"));
    if method == "GET" {
        builder.body(Body::empty()).unwrap()
    } else {
        builder
            .header("content-type", "application/json")
            .header("idempotency-key", uuid::Uuid::new_v4().to_string())
            .body(Body::from(
                json!({ "reason": "G5 acceptance test request" }).to_string(),
            ))
            .unwrap()
    }
}

async fn send(state: AppState, req: Request<Body>) -> (StatusCode, Value) {
    let res = build_router(state).oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

/// The operator checks passed: the request reached its handler.
fn passed(status: StatusCode) -> bool {
    status != StatusCode::UNAUTHORIZED && status != StatusCode::FORBIDDEN
}

fn admin_read(key: TestKey) -> Value {
    // The bound key needs this route's action; the predecessor sends scope `admin`.
    let scope = if key.issuer == PRED_ISS {
        "admin"
    } else {
        "customers:read"
    };
    key.claims(ADMIN_AUD, json!({ "scope": scope }))
}

fn posture_read(key: TestKey) -> Value {
    key.claims(POSTURE_AUD, json!({ "scope": "posture:read" }))
}

// --- C-1 to C-6, SC-3, DEC-G5-4 -------------------------------------------------------------

#[tokio::test]
async fn c1_each_key_passes_under_its_own_issuer_on_both_validators() {
    for key in [PRED, BOUND] {
        let (s, _) = send(
            state(),
            request("GET", "/v1/admin/customers", &key.sign(&admin_read(key))),
        )
        .await;
        assert!(passed(s), "admin read, issuer {}: {s}", key.issuer);
        let (s, _) = send(
            state(),
            request(
                "GET",
                "/v1/posture/customers",
                &key.sign(&posture_read(key)),
            ),
        )
        .await;
        assert!(passed(s), "posture read, issuer {}: {s}", key.issuer);
    }
}

#[tokio::test]
async fn c2_wrong_issuer_audience_expiry_and_signature_fail_for_both_keys() {
    for key in [PRED, BOUND] {
        let base = admin_read(key);
        let mut wrong_iss = base.clone();
        wrong_iss["iss"] = json!("https://not-the-issuer");
        let mut wrong_aud = base.clone();
        wrong_aud["aud"] = json!("someone-else");
        let mut expired = base.clone();
        expired["exp"] = json!(now() - 1);
        for (label, claims) in [
            ("issuer", wrong_iss),
            ("audience", wrong_aud),
            ("expiry", expired),
        ] {
            let (s, _) = send(
                state(),
                request("GET", "/v1/admin/customers", &key.sign(&claims)),
            )
            .await;
            assert_eq!(
                s,
                StatusCode::UNAUTHORIZED,
                "{label}, issuer {}",
                key.issuer
            );
        }
        // A bad signature: the right kid, signed by a key that is not in the set.
        let stranger = TestKey { seed: 99, ..key };
        let forged = stranger.sign_with_kid(Some(key.kid()), &base);
        let (s, _) = send(state(), request("GET", "/v1/admin/customers", &forged)).await;
        assert_eq!(
            s,
            StatusCode::UNAUTHORIZED,
            "signature, issuer {}",
            key.issuer
        );
    }
}

#[tokio::test]
async fn c3_kid_of_one_key_on_a_token_signed_by_the_other_fails() {
    let token = BOUND.sign_with_kid(Some(PRED.kid()), &admin_read(PRED));
    let (s, _) = send(state(), request("GET", "/v1/admin/customers", &token)).await;
    assert_eq!(s, StatusCode::UNAUTHORIZED);
    let token = PRED.sign_with_kid(Some(BOUND.kid()), &admin_read(BOUND));
    let (s, _) = send(state(), request("GET", "/v1/admin/customers", &token)).await;
    assert_eq!(s, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn c4_sc3_the_bound_key_carrying_the_predecessor_issuer_fails() {
    // A valid signature under the bound key's kid, claiming the predecessor's issuer, to get
    // the predecessor's unbound treatment. The selected entry's issuer is not `iss`.
    let mut claims = admin_read(PRED);
    claims["scope"] = json!("admin");
    let token = BOUND.sign(&claims);
    let (s, _) = send(state(), request("GET", "/v1/admin/customers", &token)).await;
    assert_eq!(s, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn sc3_iss_is_checked_against_the_selected_entry_on_its_own() {
    // `sub` is right for the selected entry, so only the issuer check can refuse it. Without
    // this case the DEC-G5-4 `sub` check would hide a missing SC-3 check.
    for (key, foreign) in [(BOUND, PRED_ISS), (PRED, BOUND_ISS)] {
        let mut claims = admin_read(key);
        claims["iss"] = json!(foreign);
        let (s, _) = send(
            state(),
            request("GET", "/v1/admin/customers", &key.sign(&claims)),
        )
        .await;
        assert_eq!(s, StatusCode::UNAUTHORIZED, "issuer {}", key.issuer);
    }
}

#[tokio::test]
async fn c5_an_issuer_right_for_one_kid_and_wrong_for_the_other_fails() {
    let mut claims = admin_read(BOUND);
    claims["iss"] = json!(BOUND_ISS);
    claims["sub"] = json!(BOUND_ISS);
    // Signed by the predecessor's key, under its own kid, with the bound key's issuer.
    let token = PRED.sign(&claims);
    let (s, _) = send(state(), request("GET", "/v1/admin/customers", &token)).await;
    assert_eq!(s, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn c6_an_unknown_or_absent_kid_fails_and_never_selects_a_default() {
    for key in [PRED, BOUND] {
        let claims = admin_read(key);
        for kid in [None, Some("AAAAAAAA".to_string()), Some(String::new())] {
            let (s, _) = send(
                state(),
                request(
                    "GET",
                    "/v1/admin/customers",
                    &key.sign_with_kid(kid.clone(), &claims),
                ),
            )
            .await;
            assert_eq!(
                s,
                StatusCode::UNAUTHORIZED,
                "kid {kid:?}, issuer {}",
                key.issuer
            );
        }
    }
}

#[tokio::test]
async fn dec_g5_4_sub_must_equal_the_selected_issuer() {
    for key in [PRED, BOUND] {
        for aud_read in [admin_read(key), posture_read(key)] {
            let mut claims = aud_read;
            claims["sub"] = json!("operator-7");
            let uri = if claims["aud"] == ADMIN_AUD {
                "/v1/admin/customers"
            } else {
                "/v1/posture/customers"
            };
            let (s, _) = send(state(), request("GET", uri, &key.sign(&claims))).await;
            assert_eq!(s, StatusCode::UNAUTHORIZED, "{uri}, issuer {}", key.issuer);
        }
    }
}

#[tokio::test]
async fn a_token_without_a_jti_fails() {
    for key in [PRED, BOUND] {
        let mut claims = admin_read(key);
        claims.as_object_mut().unwrap().remove("jti");
        let (s, _) = send(
            state(),
            request("GET", "/v1/admin/customers", &key.sign(&claims)),
        )
        .await;
        assert_eq!(s, StatusCode::UNAUTHORIZED, "issuer {}", key.issuer);
    }
}

#[tokio::test]
async fn a_crit_header_fails() {
    let claims = admin_read(PRED);
    let header = URL_SAFE_NO_PAD.encode(
        json!({ "alg": "EdDSA", "typ": "JWT", "kid": PRED.kid(), "crit": ["x"], "x": 1 })
            .to_string(),
    );
    let payload = URL_SAFE_NO_PAD.encode(claims.to_string());
    let signing = SigningKey::from_bytes(&[PRED.seed; 32]);
    let signature = ed25519_dalek::Signer::sign(&signing, format!("{header}.{payload}").as_bytes());
    let token = format!(
        "{header}.{payload}.{}",
        URL_SAFE_NO_PAD.encode(signature.to_bytes())
    );
    // A positive control: the same claims, without `crit`, pass.
    let (s, _) = send(
        state(),
        request("GET", "/v1/admin/customers", &PRED.sign(&admin_read(PRED))),
    )
    .await;
    assert!(passed(s));
    let (s, _) = send(state(), request("GET", "/v1/admin/customers", &token)).await;
    assert_eq!(s, StatusCode::UNAUTHORIZED);
}

// --- R-1, R-2, K-1 ----------------------------------------------------------------------------

#[tokio::test]
async fn r1_r2_removing_and_readding_the_bound_key() {
    let store = || -> Arc<dyn ReplayStore> { Arc::new(MemoryReplayStore::default()) };
    // R-1: without the bound key, its tokens fail and the predecessor's pass.
    let without = key_set(&[PRED]);
    let (s, _) = send(
        state_with(&without, store()),
        request(
            "GET",
            "/v1/admin/customers",
            &BOUND.sign(&admin_read(BOUND)),
        ),
    )
    .await;
    assert_eq!(s, StatusCode::UNAUTHORIZED);
    let (s, _) = send(
        state_with(&without, store()),
        request("GET", "/v1/admin/customers", &PRED.sign(&admin_read(PRED))),
    )
    .await;
    assert!(passed(s));
    // R-2: re-adding it restores C-1.
    let with = key_set(&[PRED, BOUND]);
    for key in [PRED, BOUND] {
        let (s, _) = send(
            state_with(&with, store()),
            request("GET", "/v1/admin/customers", &key.sign(&admin_read(key))),
        )
        .await;
        assert!(passed(s), "issuer {}", key.issuer);
    }
}

#[tokio::test]
async fn k1_an_invalid_or_unset_set_refuses_every_operator_token() {
    let good = key_set(&[PRED, BOUND]);
    let one_bad_entry = {
        let mut v: Value = serde_json::from_str(&good).unwrap();
        v[BOUND.kid()]["issuer"] = json!("");
        v.to_string()
    };
    let no_issuer = {
        let mut v: Value = serde_json::from_str(&good).unwrap();
        v[PRED.kid()].as_object_mut().unwrap().remove("issuer");
        v.to_string()
    };
    let one_key_two_kids = {
        let mut v: Value = serde_json::from_str(&good).unwrap();
        // The bound key under its other published derivation as a second entry.
        let other = URL_SAFE_NO_PAD.encode(Sha256::digest(BOUND.raw_public()))[..8].to_string();
        v[other] = json!({ "key": BOUND.spki_pem(), "issuer": "x" });
        v.to_string()
    };
    let kid_not_derived = {
        let mut v: Value = serde_json::from_str(&good).unwrap();
        let entry = v[PRED.kid()].clone();
        v.as_object_mut().unwrap().remove(&PRED.kid());
        v["AAAAAAAA"] = entry;
        v.to_string()
    };
    for (label, set) in [
        ("unset", String::new()),
        ("kid not derived from its key", kid_not_derived),
        ("one bad entry", one_bad_entry),
        ("no issuer", no_issuer),
        ("one key under two kids", one_key_two_kids),
        ("not json", "{".to_string()),
    ] {
        for key in [PRED, BOUND] {
            let state = state_with(&set, Arc::new(MemoryReplayStore::default()));
            let (s, body) = send(
                state.clone(),
                request("GET", "/v1/admin/customers", &key.sign(&admin_read(key))),
            )
            .await;
            assert_eq!(
                body["error"]["message"], "token verification is not configured",
                "{label}: {s}"
            );
            let (_, body) = send(
                state,
                request(
                    "GET",
                    "/v1/posture/customers",
                    &key.sign(&posture_read(key)),
                ),
            )
            .await;
            assert_eq!(
                body["error"]["message"], "token verification is not configured",
                "{label} posture"
            );
        }
    }
}

// --- SC-1, SC-1R ------------------------------------------------------------------------------

#[tokio::test]
async fn sc1_a_bound_token_fails_on_another_action_and_another_target() {
    let pause = |target: &str| {
        BOUND.claims(
            ADMIN_AUD,
            json!({ "scope": "update_campaigns:pause", "target": target }),
        )
    };
    // Positive control.
    let (s, _) = send(
        state(),
        request(
            "POST",
            &format!("/v1/admin/update-campaigns/{ID}/pause"),
            &BOUND.sign(&pause(ID)),
        ),
    )
    .await;
    assert!(passed(s), "{s}");
    // The same token shape on another action.
    let (s, _) = send(
        state(),
        request(
            "POST",
            &format!("/v1/admin/update-campaigns/{ID}/resume"),
            &BOUND.sign(&pause(ID)),
        ),
    )
    .await;
    assert_eq!(s, StatusCode::FORBIDDEN);
    // The same action, another target.
    let (s, _) = send(
        state(),
        request(
            "POST",
            &format!("/v1/admin/update-campaigns/{ID}/pause"),
            &BOUND.sign(&pause("00000000-0000-0000-0000-000000000000")),
        ),
    )
    .await;
    assert_eq!(s, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn sc1_each_stop_route_of_the_t13_amendment_binds_action_and_target() {
    let other = "00000000-0000-0000-0000-000000000000";
    for (label, path, action) in [
        (
            "S-1 pause",
            format!("/v1/admin/update-campaigns/{ID}/pause"),
            "update_campaigns:pause",
        ),
        (
            "S-2 revoke",
            format!("/v1/admin/update-campaigns/{ID}/revoke"),
            "update_campaigns:revoke",
        ),
        (
            "S-3 add hold",
            format!("/v1/admin/update-campaigns/{ID}/holds"),
            "update_campaigns:add_hold",
        ),
        (
            "S-4 block",
            format!("/v1/admin/releases/{ID}/block"),
            "releases:block",
        ),
        (
            "S-5 quarantine",
            format!("/v1/admin/release-artifacts/{ID}/quarantine"),
            "release_artifacts:quarantine",
        ),
    ] {
        let ok = BOUND.claims(ADMIN_AUD, json!({ "scope": action, "target": ID }));
        let (s, _) = send(state(), request("POST", &path, &BOUND.sign(&ok))).await;
        assert!(passed(s), "{label} positive control: {s}");
        let wrong_target = BOUND.claims(ADMIN_AUD, json!({ "scope": action, "target": other }));
        let (s, _) = send(state(), request("POST", &path, &BOUND.sign(&wrong_target))).await;
        assert_eq!(s, StatusCode::FORBIDDEN, "{label} wrong target");
        let admin = BOUND.claims(ADMIN_AUD, json!({ "scope": "admin", "target": ID }));
        let (s, _) = send(state(), request("POST", &path, &BOUND.sign(&admin))).await;
        assert_eq!(s, StatusCode::FORBIDDEN, "{label} scope admin");
        // Another stop route's action on this route.
        let foreign = if action == "releases:block" {
            "update_campaigns:pause"
        } else {
            "releases:block"
        };
        let wrong_action = BOUND.claims(ADMIN_AUD, json!({ "scope": foreign, "target": ID }));
        let (s, _) = send(state(), request("POST", &path, &BOUND.sign(&wrong_action))).await;
        assert_eq!(s, StatusCode::FORBIDDEN, "{label} wrong action");
    }
}

#[tokio::test]
async fn sc1r_scope_admin_fails_a_bound_route_and_any_roles_claim_fails_every_route() {
    // Scope `admin` from the bound key fails reads and mutations alike.
    let admin = BOUND.claims(ADMIN_AUD, json!({ "scope": "admin" }));
    let (s, _) = send(
        state(),
        request("GET", "/v1/admin/customers", &BOUND.sign(&admin)),
    )
    .await;
    assert_eq!(s, StatusCode::FORBIDDEN);
    // A roles claim fails every route, even with the right action and target.
    for (method, path, action, target) in [
        (
            "GET",
            "/v1/admin/customers".to_string(),
            "customers:read",
            None,
        ),
        (
            "POST",
            format!("/v1/admin/customers/{ID}/suspend"),
            "customers:suspend",
            Some(ID),
        ),
        (
            "POST",
            format!("/v1/admin/releases/{ID}/block"),
            "releases:block",
            Some(ID),
        ),
    ] {
        for roles in [json!(["admin"]), json!([])] {
            let mut claims = BOUND.claims(ADMIN_AUD, json!({ "scope": action, "roles": roles }));
            if let Some(t) = target {
                claims["target"] = json!(t);
            }
            let (s, _) = send(state(), request(method, &path, &BOUND.sign(&claims))).await;
            assert_eq!(s, StatusCode::FORBIDDEN, "{method} {path} roles {roles}");
        }
    }
    let posture_roles = BOUND.claims(
        POSTURE_AUD,
        json!({ "scope": "posture:read", "roles": ["admin"] }),
    );
    let (s, _) = send(
        state(),
        request("GET", "/v1/posture/customers", &BOUND.sign(&posture_roles)),
    )
    .await;
    assert_eq!(s, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn sc1_is_not_met_for_the_predecessor_key_until_g7() {
    // DEC-G5-2 (b): a named, open exemption. The predecessor's scope-`admin` token still
    // passes a stop route with no target claim. This test records the gap; it is not a
    // guarantee. G7 removes the predecessor's key and turns this test around.
    let claims = PRED.claims(ADMIN_AUD, json!({ "scope": "admin" }));
    let (s, _) = send(
        state(),
        request(
            "POST",
            &format!("/v1/admin/releases/{ID}/block"),
            &PRED.sign(&claims),
        ),
    )
    .await;
    assert!(passed(s), "{s}");
}

// --- X-1 ----------------------------------------------------------------------------------------

#[tokio::test]
async fn x1_admin_and_posture_audiences_do_not_cross() {
    for key in [PRED, BOUND] {
        let admin_token = key.sign(&admin_read(key));
        let posture_token = key.sign(&posture_read(key));
        let (s, _) = send(
            state(),
            request("GET", "/v1/posture/customers", &admin_token),
        )
        .await;
        assert_eq!(
            s,
            StatusCode::UNAUTHORIZED,
            "admin audience on posture, issuer {}",
            key.issuer
        );
        let (s, _) = send(
            state(),
            request("GET", "/v1/admin/customers", &posture_token),
        )
        .await;
        assert_eq!(
            s,
            StatusCode::UNAUTHORIZED,
            "posture audience on admin, issuer {}",
            key.issuer
        );
        // Positive controls, each with a fresh token.
        let (s, _) = send(
            state(),
            request(
                "GET",
                "/v1/posture/customers",
                &key.sign(&posture_read(key)),
            ),
        )
        .await;
        assert!(passed(s));
        let (s, _) = send(
            state(),
            request("GET", "/v1/admin/customers", &key.sign(&admin_read(key))),
        )
        .await;
        assert!(passed(s));
    }
}

// --- SC-2, SC-2A, SC-2O -------------------------------------------------------------------------

#[tokio::test]
async fn sc2_a_used_token_is_refused_for_each_key() {
    for key in [PRED, BOUND] {
        let state = state();
        let token = key.sign(&admin_read(key));
        let (s, _) = send(state.clone(), request("GET", "/v1/admin/customers", &token)).await;
        assert!(passed(s), "first use, issuer {}", key.issuer);
        let (s, body) = send(state, request("GET", "/v1/admin/customers", &token)).await;
        assert_eq!(
            s,
            StatusCode::UNAUTHORIZED,
            "second use, issuer {}",
            key.issuer
        );
        assert_eq!(
            body["error"]["message"],
            "The access token was already used."
        );
    }
}

#[tokio::test]
async fn sc2_the_refusal_holds_across_two_instances_that_share_the_store() {
    let shared: Arc<dyn ReplayStore> = Arc::new(MemoryReplayStore::default());
    let set = key_set(&[PRED, BOUND]);
    for key in [PRED, BOUND] {
        let token = key.sign(&posture_read(key));
        let first = state_with(&set, shared.clone());
        let second = state_with(&set, shared.clone());
        let (s, _) = send(first, request("GET", "/v1/posture/customers", &token)).await;
        assert!(passed(s));
        let (s, _) = send(second, request("GET", "/v1/posture/customers", &token)).await;
        assert_eq!(s, StatusCode::UNAUTHORIZED, "issuer {}", key.issuer);
    }
}

#[tokio::test]
async fn sc2_an_unavailable_store_refuses_the_token() {
    for key in [PRED, BOUND] {
        let state = state_with(&key_set(&[PRED, BOUND]), Arc::new(UnavailableReplayStore));
        let (s, body) = send(
            state.clone(),
            request("GET", "/v1/admin/customers", &key.sign(&admin_read(key))),
        )
        .await;
        assert_eq!(s, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            body["error"]["message"],
            "Operator token checks are unavailable. Try again."
        );
        let (_, body) = send(
            state,
            request(
                "GET",
                "/v1/posture/customers",
                &key.sign(&posture_read(key)),
            ),
        )
        .await;
        assert_eq!(
            body["error"]["message"],
            "Operator token checks are unavailable. Try again."
        );
    }
}

#[tokio::test]
async fn sc2a_two_concurrent_requests_with_one_token_exactly_one_passes() {
    for key in [PRED, BOUND] {
        let state = state();
        let token = key.sign(&admin_read(key));
        let (a, b) = tokio::join!(
            send(state.clone(), request("GET", "/v1/admin/customers", &token)),
            send(state.clone(), request("GET", "/v1/admin/customers", &token)),
        );
        let passes = [a.0, b.0].into_iter().filter(|s| passed(*s)).count();
        assert_eq!(passes, 1, "issuer {}: {} and {}", key.issuer, a.0, b.0);
    }
}

#[tokio::test]
async fn sc2a_the_same_jti_under_two_issuers_is_two_identities() {
    let state = state();
    let jti = uuid::Uuid::new_v4().to_string();
    for key in [PRED, BOUND] {
        let mut claims = admin_read(key);
        claims["jti"] = json!(jti);
        let (s, _) = send(
            state.clone(),
            request("GET", "/v1/admin/customers", &key.sign(&claims)),
        )
        .await;
        assert!(passed(s), "issuer {}", key.issuer);
    }
}

#[tokio::test]
async fn sc2o_a_token_that_fails_a_check_claims_nothing() {
    let store = Arc::new(MemoryReplayStore::default());
    let state = state_with(&key_set(&[PRED, BOUND]), store.clone());
    // Step 2 failures: a wrong target, then a wrong audience, then a bad sub.
    let jti = uuid::Uuid::new_v4().to_string();
    let mut wrong_target = BOUND.claims(
        ADMIN_AUD,
        json!({ "scope": "customers:suspend", "target": "other" }),
    );
    wrong_target["jti"] = json!(jti);
    let (s, _) = send(
        state.clone(),
        request(
            "POST",
            &format!("/v1/admin/customers/{ID}/suspend"),
            &BOUND.sign(&wrong_target),
        ),
    )
    .await;
    assert_eq!(s, StatusCode::FORBIDDEN);
    let mut bad_sub = admin_read(PRED);
    bad_sub["sub"] = json!("operator-7");
    let (s, _) = send(
        state.clone(),
        request("GET", "/v1/admin/customers", &PRED.sign(&bad_sub)),
    )
    .await;
    assert_eq!(s, StatusCode::UNAUTHORIZED);
    assert!(store.is_empty(), "a refused token claimed a replay row");
    // The same jti in a later valid token is unaffected: nothing was claimed.
    let mut ok = BOUND.claims(
        ADMIN_AUD,
        json!({ "scope": "customers:suspend", "target": ID }),
    );
    ok["jti"] = json!(jti);
    let (s, _) = send(
        state,
        request(
            "POST",
            &format!("/v1/admin/customers/{ID}/suspend"),
            &BOUND.sign(&ok),
        ),
    )
    .await;
    assert!(passed(s), "{s}");
    assert_eq!(store.len(), 1);
}

// --- S-1, N-1 -----------------------------------------------------------------------------------

#[tokio::test]
async fn s1_a_customer_token_reaches_no_operator_route() {
    let customer = encode(
        &Header::new(Algorithm::HS256),
        &json!({ "sub": "11111111-1111-1111-1111-111111111111",
                 "iss": "https://proj.supabase.co/auth/v1", "aud": "authenticated",
                 "exp": now() + 60 }),
        &EncodingKey::from_secret(b"supabase-secret"),
    )
    .unwrap();
    for uri in ["/v1/admin/customers", "/v1/posture/customers"] {
        let (s, _) = send(state(), request("GET", uri, &customer)).await;
        assert_eq!(s, StatusCode::UNAUTHORIZED, "{uri}");
    }
}

#[test]
fn n1_the_key_set_is_public_material_and_no_secret_variable_is_added() {
    // The trusted-key set holds public keys and issuers only.
    let set: Value = serde_json::from_str(&key_set(&[PRED, BOUND])).unwrap();
    for (_, entry) in set.as_object().unwrap() {
        let mut fields: Vec<_> = entry.as_object().unwrap().keys().cloned().collect();
        fields.sort();
        assert_eq!(fields, vec!["issuer".to_string(), "key".to_string()]);
        assert!(entry["key"]
            .as_str()
            .unwrap()
            .starts_with("-----BEGIN PUBLIC KEY-----"));
    }
    // The config's operator fields: the audience and the public key set. No private key, no
    // new connection credential (the replay store uses the existing database pool).
    let source = include_str!("../src/config.rs");
    assert!(source.contains("pub admin_jwt_trusted_keys: String"));
    assert!(!source.contains("admin_jwt_private"));
    assert!(!source.contains("REPLAY_DATABASE"));
}

// ---------------------------------------------------------------------------------------
// Readiness (READY-1..READY-4): `/v1/ready` gates the Render deploy. It must fail when the operator
// plane cannot work, and name the failed check without any value. The test database is
// unreachable, so `database` always fails here; the other checks are what these tests prove.
// ---------------------------------------------------------------------------------------

async fn ready_body(state: AppState) -> (StatusCode, Value, String) {
    let req = Request::builder()
        .uri("/v1/ready")
        .body(Body::empty())
        .unwrap();
    let res = build_router(state).oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let text = String::from_utf8(bytes.to_vec()).unwrap();
    (status, serde_json::from_str(&text).unwrap(), text)
}

fn failed(body: &Value) -> Vec<String> {
    body["failed"]
        .as_array()
        .map(|a| a.iter().map(|v| v.as_str().unwrap().to_string()).collect())
        .unwrap_or_default()
}

#[tokio::test]
async fn ready1_a_loaded_key_set_and_store_are_not_reported() {
    let (status, body, _) = ready_body(state()).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(failed(&body), vec!["database".to_string()]);
}

#[tokio::test]
async fn ready2_an_unset_or_refused_key_set_fails_readiness() {
    for keys in [
        "",
        "not json",
        r#"{"98Ue0PgZ": {"key": "-----BEGIN PUBLIC KEY-----\nAAAA\n-----END PUBLIC KEY-----\n", "issuer": "forge_command_local"}}"#,
    ] {
        let (status, body, _) =
            ready_body(state_with(keys, Arc::new(MemoryReplayStore::default()))).await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "keys {keys:?}");
        assert!(
            failed(&body).contains(&"ADMIN_JWT_TRUSTED_KEYS".to_string()),
            "keys {keys:?}: {body}"
        );
    }
}

#[tokio::test]
async fn ready3_an_unavailable_replay_store_fails_readiness() {
    let (status, body, _) = ready_body(state_with(
        &key_set(&[PRED, BOUND]),
        Arc::new(UnavailableReplayStore),
    ))
    .await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert!(failed(&body).contains(&"operator_token_replay".to_string()));
    assert!(!failed(&body).contains(&"ADMIN_JWT_TRUSTED_KEYS".to_string()));
}

#[tokio::test]
async fn ready4_the_body_names_checks_and_never_carries_key_material() {
    let (_, body, text) = ready_body(state_with(
        "{\"x\": \"MCowBQYDK2VwAyEA-secret-looking\"}",
        Arc::new(UnavailableReplayStore),
    ))
    .await;
    assert_eq!(body["status"], "not_ready");
    for f in failed(&body) {
        assert!(
            [
                "database",
                "operator_token_replay",
                "ADMIN_JWT_TRUSTED_KEYS"
            ]
            .contains(&f.as_str()),
            "unexpected check name {f}"
        );
    }
    assert!(
        !text.contains("MCowBQ"),
        "readiness body leaked key material"
    );
    assert!(!text.contains("BEGIN PUBLIC KEY"));
}
