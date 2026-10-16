//! Customer JWT verification across Supabase's legacy HS256 and ES256 signing keys.
//! The JWKS location comes only from the configured issuer, never token headers.

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, EllipticCurve, JwkSet, KeyAlgorithm, KeyOperations, PublicKeyUse},
    Algorithm, DecodingKey, Validation,
};
use tokio::sync::Mutex;

use super::{map_jwt_error, Claims, JwtValidator};
use crate::error::{AppError, ErrorCode};

const CACHE_TTL: Duration = Duration::from_secs(600);
const REFRESH_INTERVAL: Duration = Duration::from_secs(60);
const MAX_JWKS_BYTES: usize = 64 * 1024;
const MAX_KEYS: usize = 32;
const MAX_TOKEN_BYTES: usize = 16 * 1024;

#[cfg(test)]
mod local_commerce;

#[derive(Clone)]
struct VerificationKey {
    kid: String,
    key: DecodingKey,
}

#[derive(Default)]
struct KeyCache {
    keys: Vec<VerificationKey>,
    fetched_at: Option<Instant>,
    attempted_at: Option<Instant>,
}

/// Admin and posture tokens continue to use their separate validators and keys.
#[derive(Clone)]
pub struct CustomerJwtValidator {
    legacy: JwtValidator,
    validation: Validation,
    jwks_url: Option<reqwest::Url>,
    http: reqwest::Client,
    cache: Arc<Mutex<KeyCache>>,
}

impl CustomerJwtValidator {
    pub fn new(secret: &str, issuer: &str, audience: &str) -> Result<Self, reqwest::Error> {
        let mut validation = Validation::new(Algorithm::ES256);
        validation.set_issuer(&[issuer]);
        validation.set_audience(&[audience]);
        validation.set_required_spec_claims(&["sub", "exp", "aud", "iss"]);
        validation.leeway = 0;
        Ok(Self {
            legacy: JwtValidator::hs256(secret, issuer, audience),
            validation,
            jwks_url: jwks_url(issuer),
            http: reqwest::Client::builder()
                .https_only(true)
                .redirect(reqwest::redirect::Policy::none())
                .connect_timeout(Duration::from_secs(3))
                .timeout(Duration::from_secs(5))
                .build()?,
            cache: Arc::new(Mutex::new(KeyCache::default())),
        })
    }

    pub async fn validate(&self, token: &str) -> Result<Claims, AppError> {
        if token.len() > MAX_TOKEN_BYTES {
            return Err(invalid_token());
        }
        let header = decode_header(token).map_err(|_| invalid_token())?;
        if header.alg == Algorithm::HS256 {
            // Explicit legacy secret only; public JWK bytes never become an HMAC key.
            return self.legacy.validate(token);
        }
        if header.alg != Algorithm::ES256 {
            return Err(invalid_token());
        }
        let kid = header
            .kid
            .filter(|kid| !kid.is_empty() && kid.len() <= 128)
            .ok_or_else(invalid_token)?;
        let mut cache = self.cache.lock().await;
        let fresh = cache.fetched_at.is_some_and(|at| at.elapsed() < CACHE_TTL);
        if fresh {
            if let Some(key) = cache.keys.iter().find(|key| key.kid == kid) {
                return self.verify(token, &key.key);
            }
        }
        // Single-flight fetch with a global cooldown, including unknown-kid and outage
        // requests. An attacker cannot turn random kid values into unbounded egress.
        if cache
            .attempted_at
            .is_some_and(|at| at.elapsed() < REFRESH_INTERVAL)
        {
            return Err(if fresh {
                invalid_token()
            } else {
                unavailable()
            });
        }
        cache.attempted_at = Some(Instant::now());
        let keys = self.fetch_keys().await?;
        cache.keys = keys;
        cache.fetched_at = Some(Instant::now());
        let key = cache
            .keys
            .iter()
            .find(|key| key.kid == kid)
            .ok_or_else(invalid_token)?;
        self.verify(token, &key.key)
    }

    fn verify(&self, token: &str, key: &DecodingKey) -> Result<Claims, AppError> {
        decode::<Claims>(token, key, &self.validation)
            .map(|data| data.claims)
            .map_err(|error| map_jwt_error(&error))
    }

    async fn fetch_keys(&self) -> Result<Vec<VerificationKey>, AppError> {
        let url = self.jwks_url.clone().ok_or_else(unavailable)?;
        let mut response = self.http.get(url).send().await.map_err(|_| unavailable())?;
        if !response.status().is_success()
            || response
                .content_length()
                .is_some_and(|size| size > MAX_JWKS_BYTES as u64)
        {
            return Err(unavailable());
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| unavailable())? {
            if bytes.len() + chunk.len() > MAX_JWKS_BYTES {
                return Err(unavailable());
            }
            bytes.extend_from_slice(&chunk);
        }
        parse_keys(&bytes)
    }
}

fn jwks_url(issuer: &str) -> Option<reqwest::Url> {
    let mut url = reqwest::Url::parse(issuer).ok()?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return None;
    }
    url.set_path(&format!(
        "{}/.well-known/jwks.json",
        url.path().trim_end_matches('/')
    ));
    Some(url)
}

fn parse_keys(bytes: &[u8]) -> Result<Vec<VerificationKey>, AppError> {
    let set: JwkSet = serde_json::from_slice(bytes).map_err(|_| unavailable())?;
    if set.keys.len() > MAX_KEYS {
        return Err(unavailable());
    }
    let mut seen = std::collections::HashSet::new();
    let mut keys = Vec::new();
    for jwk in set.keys {
        let Some(kid) = jwk
            .common
            .key_id
            .as_ref()
            .filter(|kid| !kid.is_empty() && kid.len() <= 128)
        else {
            continue;
        };
        if !seen.insert(kid.clone()) {
            return Err(unavailable());
        }
        if jwk.common.key_algorithm != Some(KeyAlgorithm::ES256)
            || jwk
                .common
                .public_key_use
                .as_ref()
                .is_some_and(|usage| *usage != PublicKeyUse::Signature)
            || jwk
                .common
                .key_operations
                .as_ref()
                .is_some_and(|ops| !ops.contains(&KeyOperations::Verify))
            || !matches!(&jwk.algorithm, AlgorithmParameters::EllipticCurve(ec) if ec.curve == EllipticCurve::P256)
        {
            continue;
        }
        let key = DecodingKey::from_jwk(&jwk).map_err(|_| unavailable())?;
        keys.push(VerificationKey {
            kid: kid.clone(),
            key,
        });
    }
    Ok(keys)
}

fn invalid_token() -> AppError {
    AppError::new(ErrorCode::InvalidToken, "The access token is invalid.")
}

fn unavailable() -> AppError {
    // Infrastructure/key-fetch failures must not look like a bad password or expire a
    // valid Supabase session. No JWT, key bytes, URL or upstream body is logged.
    AppError::new(
        ErrorCode::ServiceUnavailable,
        "Customer identity verification is temporarily unavailable.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        extract::State,
        http::{Response, StatusCode},
        routing::get,
        Router,
    };
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use ring::{
        rand::SystemRandom,
        signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING},
    };
    use serde_json::{json, Value};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::sync::RwLock;

    const ISS: &str = "https://project.supabase.co/auth/v1";
    const AUD: &str = "authenticated";

    struct Fixture {
        private: Vec<u8>,
        jwk: Value,
    }
    impl Fixture {
        fn new(kid: &str) -> Self {
            // Ephemeral test-only key: no production credential or static private fixture.
            let rng = SystemRandom::new();
            let private =
                EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
            let pair =
                EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, private.as_ref(), &rng)
                    .unwrap();
            let point = pair.public_key().as_ref();
            Self {
                private: private.as_ref().to_vec(),
                jwk: json!({"kid":kid,"kty":"EC","crv":"P-256","alg":"ES256","use":"sig","key_ops":["verify"],"x":URL_SAFE_NO_PAD.encode(&point[1..33]),"y":URL_SAFE_NO_PAD.encode(&point[33..65])}),
            }
        }
        fn token(&self, claims: Value) -> String {
            let mut header = Header::new(Algorithm::ES256);
            header.kid = Some(self.jwk["kid"].as_str().unwrap().into());
            // A token-supplied URL must never influence verification-key lookup.
            header.jku = Some("https://attacker.invalid/jwks.json".into());
            encode(&header, &claims, &EncodingKey::from_ec_der(&self.private)).unwrap()
        }
    }
    fn claims() -> Value {
        json!({"sub":"11111111-1111-1111-1111-111111111111","iss":ISS,"aud":AUD,"exp":(Utc::now().timestamp()+3600),"email":"test@example.test","aal":"aal1"})
    }
    use chrono::Utc;

    #[derive(Clone)]
    struct ServerState {
        body: Arc<RwLock<(StatusCode, Vec<u8>)>>,
        hits: Arc<AtomicUsize>,
    }
    struct Server {
        url: reqwest::Url,
        state: ServerState,
        task: tokio::task::JoinHandle<()>,
    }
    impl Drop for Server {
        fn drop(&mut self) {
            self.task.abort();
        }
    }
    impl Server {
        async fn new(body: Value) -> Self {
            let state = ServerState {
                body: Arc::new(RwLock::new((
                    StatusCode::OK,
                    serde_json::to_vec(&body).unwrap(),
                ))),
                hits: Arc::new(AtomicUsize::new(0)),
            };
            async fn serve(State(state): State<ServerState>) -> Response<Body> {
                state.hits.fetch_add(1, Ordering::SeqCst);
                let (status, body) = state.body.read().await.clone();
                Response::builder()
                    .status(status)
                    .header("content-type", "application/json")
                    .header("location", "https://attacker.invalid/")
                    .body(Body::from(body))
                    .unwrap()
            }
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let url = format!("http://{}/jwks", listener.local_addr().unwrap())
                .parse()
                .unwrap();
            let router = Router::new()
                .route("/jwks", get(serve))
                .with_state(state.clone());
            let task = tokio::spawn(async move {
                axum::serve(listener, router).await.unwrap();
            });
            Self { url, state, task }
        }
        fn validator(&self) -> CustomerJwtValidator {
            let mut validator = CustomerJwtValidator::new("legacy-test-secret", ISS, AUD).unwrap();
            // Only the unit-test harness uses loopback HTTP; production is HTTPS-only.
            validator.jwks_url = Some(self.url.clone());
            validator.http = reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .timeout(Duration::from_secs(2))
                .build()
                .unwrap();
            validator
        }
        async fn body(&self, status: StatusCode, body: Vec<u8>) {
            *self.state.body.write().await = (status, body);
        }
        fn hits(&self) -> usize {
            self.state.hits.load(Ordering::SeqCst)
        }
    }

    #[test]
    fn jwks_location_is_derived_from_configured_https_issuer() {
        assert_eq!(
            jwks_url(ISS).unwrap().as_str(),
            "https://project.supabase.co/auth/v1/.well-known/jwks.json"
        );
        for bad in [
            "http://project.test/auth/v1",
            "https://user:password@project.test/auth/v1",
            "https://project.test/auth/v1?next=other",
            "https://project.test/auth/v1#other",
            "not a URL",
        ] {
            assert!(jwks_url(bad).is_none());
        }
    }

    #[tokio::test]
    async fn accepts_es256_through_http_and_reuses_single_flight_cache() {
        let fixture = Fixture::new("current");
        let server = Server::new(json!({"keys":[fixture.jwk]})).await;
        let validator = server.validator();
        let token = fixture.token(claims());
        let (a, b) = tokio::join!(validator.validate(&token), validator.validate(&token));
        assert_eq!(a.unwrap().email.as_deref(), Some("test@example.test"));
        assert_eq!(b.unwrap().aal.as_deref(), Some("aal1"));
        assert_eq!(server.hits(), 1);
        assert!(
            JwtValidator::hs256("legacy-test-secret", ISS, AUD)
                .validate(&token)
                .is_err(),
            "regression: old customer validator rejects this valid ES256 token"
        );
    }

    #[tokio::test]
    async fn rejects_wrong_signature_issuer_audience_expiry_and_missing_subject() {
        let fixture = Fixture::new("current");
        let server = Server::new(json!({"keys":[fixture.jwk]})).await;
        let validator = server.validator();
        assert!(validator.validate(&fixture.token(claims())).await.is_ok());
        let attacker = Fixture::new("current");
        assert_eq!(
            validator
                .validate(&attacker.token(claims()))
                .await
                .unwrap_err()
                .code,
            ErrorCode::InvalidToken
        );
        for (field, value, code) in [
            (
                "iss",
                json!("https://other.supabase.co/auth/v1"),
                ErrorCode::InvalidToken,
            ),
            (
                "aud",
                json!("forgecustomer-admin"),
                ErrorCode::WrongAudience,
            ),
            (
                "exp",
                json!(Utc::now().timestamp() - 1),
                ErrorCode::TokenExpired,
            ),
        ] {
            let mut invalid = claims();
            invalid[field] = value;
            assert_eq!(
                validator
                    .validate(&fixture.token(invalid))
                    .await
                    .unwrap_err()
                    .code,
                code
            );
        }
        let mut missing = claims();
        missing.as_object_mut().unwrap().remove("sub");
        assert!(validator.validate(&fixture.token(missing)).await.is_err());
        assert_eq!(server.hits(), 1);
    }

    #[tokio::test]
    async fn legacy_hmac_stays_explicit_and_cannot_use_a_public_key_as_secret() {
        let fixture = Fixture::new("current");
        let server = Server::new(json!({"keys":[fixture.jwk]})).await;
        let validator = server.validator();
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims(),
            &EncodingKey::from_secret(b"legacy-test-secret"),
        )
        .unwrap();
        assert!(validator.validate(&token).await.is_ok());
        let confusion = encode(
            &Header::new(Algorithm::HS256),
            &claims(),
            &EncodingKey::from_secret(fixture.jwk["x"].as_str().unwrap().as_bytes()),
        )
        .unwrap();
        assert!(validator.validate(&confusion).await.is_err());
        let unsupported = encode(
            &Header::new(Algorithm::HS384),
            &claims(),
            &EncodingKey::from_secret(b"legacy-test-secret"),
        )
        .unwrap();
        assert!(validator.validate(&unsupported).await.is_err());
        let mut header = Header::new(Algorithm::ES256);
        header.kid = None;
        let missing_kid = encode(
            &header,
            &claims(),
            &EncodingKey::from_ec_der(&fixture.private),
        )
        .unwrap();
        assert!(validator.validate(&missing_kid).await.is_err());
        assert_eq!(server.hits(), 0);
        let no_secret = CustomerJwtValidator::new("", ISS, AUD).unwrap();
        assert!(no_secret.validate(&token).await.is_err());
    }

    #[tokio::test]
    async fn rotates_keys_with_bounded_unknown_kid_refresh() {
        let old = Fixture::new("old");
        let new = Fixture::new("new");
        let server = Server::new(json!({"keys":[old.jwk]})).await;
        let validator = server.validator();
        assert!(validator.validate(&old.token(claims())).await.is_ok());
        assert!(validator.validate(&new.token(claims())).await.is_err());
        assert_eq!(server.hits(), 1);
        server
            .body(
                StatusCode::OK,
                serde_json::to_vec(&json!({"keys":[new.jwk]})).unwrap(),
            )
            .await;
        validator.cache.lock().await.attempted_at = Some(Instant::now() - REFRESH_INTERVAL);
        assert!(validator.validate(&new.token(claims())).await.is_ok());
        assert!(validator.validate(&old.token(claims())).await.is_err());
        assert_eq!(server.hits(), 2);
    }

    #[tokio::test]
    async fn outage_does_not_accept_expired_cache_or_flood_the_upstream() {
        let fixture = Fixture::new("current");
        let server = Server::new(json!({"keys":[fixture.jwk]})).await;
        let validator = server.validator();
        let token = fixture.token(claims());
        assert!(validator.validate(&token).await.is_ok());
        server
            .body(StatusCode::SERVICE_UNAVAILABLE, b"unavailable".to_vec())
            .await;
        assert!(
            validator.validate(&token).await.is_ok(),
            "fresh trusted keys still work"
        );
        {
            let mut cache = validator.cache.lock().await;
            cache.fetched_at = Some(Instant::now() - CACHE_TTL);
            cache.attempted_at = Some(Instant::now() - REFRESH_INTERVAL);
        }
        assert_eq!(
            validator.validate(&token).await.unwrap_err().code,
            ErrorCode::ServiceUnavailable
        );
        assert_eq!(
            validator.validate(&token).await.unwrap_err().code,
            ErrorCode::ServiceUnavailable
        );
        assert_eq!(server.hits(), 2);
    }

    #[tokio::test]
    async fn rejects_redirect_oversized_and_malformed_jwks() {
        let fixture = Fixture::new("current");
        let token = fixture.token(claims());
        for (status, body) in [
            (StatusCode::FOUND, b"redirect".to_vec()),
            (StatusCode::OK, vec![b' '; MAX_JWKS_BYTES + 1]),
            (StatusCode::OK, b"not JSON".to_vec()),
        ] {
            let server = Server::new(json!({"keys":[]})).await;
            server.body(status, body).await;
            assert_eq!(
                server.validator().validate(&token).await.unwrap_err().code,
                ErrorCode::ServiceUnavailable
            );
            assert_eq!(server.hits(), 1);
        }
    }

    #[test]
    fn rejects_ambiguous_or_inappropriate_keys() {
        let fixture = Fixture::new("same");
        assert!(parse_keys(
            &serde_json::to_vec(&json!({"keys":[fixture.jwk,fixture.jwk]})).unwrap()
        )
        .is_err());
        assert!(parse_keys(
            &serde_json::to_vec(&json!({"keys":vec![fixture.jwk.clone();MAX_KEYS+1]})).unwrap()
        )
        .is_err());
        for (field, value) in [
            ("alg", json!("HS256")),
            ("use", json!("enc")),
            ("key_ops", json!(["sign"])),
            ("crv", json!("P-384")),
        ] {
            let mut wrong = fixture.jwk.clone();
            wrong[field] = value;
            assert!(
                parse_keys(&serde_json::to_vec(&json!({"keys":[wrong]})).unwrap())
                    .unwrap()
                    .is_empty()
            );
        }
    }
}
