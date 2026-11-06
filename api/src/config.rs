//! Environment-driven configuration. Secrets are loaded here and never logged.

use std::time::Duration;

/// Application configuration assembled from environment variables.
#[derive(Clone)]
pub struct Config {
    pub app_env: String,
    pub host: String,
    pub port: u16,

    pub database_url: String,
    /// How long pool acquisition may retry connecting before failing closed.
    pub database_acquire_timeout: Duration,

    pub supabase_jwt_issuer: String,
    pub supabase_jwt_audience: String,
    /// Optional legacy HS256 secret. ES256 customer tokens use the issuer's public JWKS.
    pub supabase_jwt_secret: String,

    pub admin_jwt_issuer: String,
    pub admin_jwt_audience: String,
    /// PEM-encoded Ed25519 public key used to verify operator/admin JWTs minted by Forge
    /// Command's Token Authority. Empty = the admin surface fails closed.
    pub admin_jwt_public_key: String,

    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
    /// Stripe API base URL; overridable for mocked end-to-end tests.
    pub stripe_api_base: String,

    pub entitlement_signing_private_key: String,
    pub entitlement_signing_key_id: String,
    /// Retained and pending entitlement verification keys, `(key_id, base64 public key)`, from
    /// `ENTITLEMENT_VERIFY_PUBLIC_KEYS`. They verify and are published; they never sign. The
    /// active key comes from the signing key above and is not repeated here. Empty = none.
    pub entitlement_verify_public_keys: Vec<(String, String)>,

    pub dataforge_api_url: String,
    pub dataforge_service_token: String,

    /// Server-side secret used for deterministic update-campaign rollout buckets.
    pub update_rollout_secret: String,
    /// Optional base URL used when release artifact storage keys are relative paths.
    pub release_artifact_base_url: String,

    pub snapshot_ttl: Duration,
    pub offline_grace: Duration,
    /// How long a deletion request rests in cooling-off before it may be processed.
    pub deletion_cooling_off: Duration,
    /// How long a usage reservation holds quota before it auto-expires.
    pub usage_reservation_ttl: Duration,
    /// Usage thresholds (percent of limit) that emit `quota_threshold_reached`.
    pub usage_threshold_percents: Vec<u8>,

    pub request_timeout: Duration,
    pub max_body_bytes: usize,
    /// Per-client request budget per minute (`0` disables rate limiting).
    pub rate_limit_per_minute: u32,
}

/// Configuration errors. Missing required variables fail closed at startup.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing required environment variable: {0}")]
    Missing(&'static str),
    #[error("invalid value for {0}: {1}")]
    Invalid(&'static str, String),
}

fn require(key: &'static str) -> Result<String, ConfigError> {
    std::env::var(key).map_err(|_| ConfigError::Missing(key))
}

fn optional(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn parse_u64(key: &'static str, default: u64) -> Result<u64, ConfigError> {
    match std::env::var(key) {
        Ok(v) => v
            .parse::<u64>()
            .map_err(|e| ConfigError::Invalid(key, e.to_string())),
        Err(_) => Ok(default),
    }
}

/// Parse a comma-separated list of percentages (1–100), e.g. "80,100".
fn parse_percent_list(key: &'static str, default: &[u8]) -> Result<Vec<u8>, ConfigError> {
    let raw = match std::env::var(key) {
        Ok(value) => value,
        Err(_) => return Ok(default.to_vec()),
    };
    let mut out = Vec::new();
    for part in raw.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let pct = part
            .parse::<u8>()
            .ok()
            .filter(|pct| (1..=100).contains(pct))
            .ok_or(ConfigError::Invalid(
                key,
                "must be comma-separated percentages between 1 and 100".to_string(),
            ))?;
        out.push(pct);
    }
    out.sort_unstable();
    out.dedup();
    Ok(out)
}

/// The variable that carries retained and pending entitlement verification keys.
pub const VERIFY_PUBLIC_KEYS_VAR: &str = "ENTITLEMENT_VERIFY_PUBLIC_KEYS";

/// Parse `ENTITLEMENT_VERIFY_PUBLIC_KEYS`: a JSON object
/// `{"<key_id>": "<base64 32-byte Ed25519 public key>"}`, the shape DataForge Local uses for
/// `XA_RUN_TOKEN_PUBLIC_KEYS` without its issuer field. Unset or blank means no extra keys.
///
/// A malformed value fails start-up. It never falls back to the active key alone, because a
/// silent fallback would refuse every snapshot signed by a retained key. A repeated `key_id`
/// also fails: a plain JSON map would keep the last value and hide the conflict. The key bytes
/// themselves are checked where they are loaded (`AppState::build`).
pub fn parse_verify_public_keys(raw: Option<&str>) -> Result<Vec<(String, String)>, ConfigError> {
    let Some(raw) = raw.map(str::trim).filter(|raw| !raw.is_empty()) else {
        return Ok(Vec::new());
    };
    let entries: VerifyKeyEntries = serde_json::from_str(raw).map_err(|e| {
        ConfigError::Invalid(
            VERIFY_PUBLIC_KEYS_VAR,
            format!("must be a JSON object of key_id to base64 public key: {e}"),
        )
    })?;
    Ok(entries.0)
}

/// Object entries in document order, refusing an empty or repeated `key_id`.
struct VerifyKeyEntries(Vec<(String, String)>);

impl<'de> serde::Deserialize<'de> for VerifyKeyEntries {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct EntriesVisitor;

        impl<'de> serde::de::Visitor<'de> for EntriesVisitor {
            type Value = VerifyKeyEntries;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a JSON object of key_id to base64 public key")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut entries: Vec<(String, String)> = Vec::new();
                while let Some((key_id, public_key)) = map.next_entry::<String, String>()? {
                    if key_id.trim().is_empty() {
                        return Err(serde::de::Error::custom("empty key_id"));
                    }
                    if entries.iter().any(|(seen, _)| seen == &key_id) {
                        return Err(serde::de::Error::custom(format!(
                            "duplicate key_id {key_id}"
                        )));
                    }
                    entries.push((key_id, public_key));
                }
                Ok(VerifyKeyEntries(entries))
            }
        }

        deserializer.deserialize_map(EntriesVisitor)
    }
}

impl Config {
    /// Load configuration from the process environment.
    pub fn from_env() -> Result<Self, ConfigError> {
        // 8090 is the registered local port (forge PORT_REGISTRY.md), where Forge Command
        // looks for ForgeCustomer. The Docker image and the Render Blueprint set 8080.
        let port = optional("PORT", "8090")
            .parse::<u16>()
            .map_err(|e| ConfigError::Invalid("PORT", e.to_string()))?;

        let snapshot_ttl_hours = parse_u64("ENTITLEMENT_SNAPSHOT_TTL_HOURS", 24)?;
        let offline_grace_days = parse_u64("OFFLINE_GRACE_DAYS", 14)?;

        Ok(Self {
            app_env: optional("APP_ENV", "development"),
            host: optional("HOST", "0.0.0.0"),
            port,
            database_url: require("DATABASE_URL")?,
            database_acquire_timeout: Duration::from_secs(parse_u64(
                "DATABASE_ACQUIRE_TIMEOUT_SECS",
                30,
            )?),
            supabase_jwt_issuer: require("SUPABASE_JWT_ISSUER")?,
            supabase_jwt_audience: optional("SUPABASE_JWT_AUDIENCE", "authenticated"),
            supabase_jwt_secret: optional("SUPABASE_JWT_SECRET", ""),
            admin_jwt_issuer: require("ADMIN_JWT_ISSUER")?,
            admin_jwt_audience: require("ADMIN_JWT_AUDIENCE")?,
            admin_jwt_public_key: optional("ADMIN_JWT_PUBLIC_KEY", ""),
            stripe_secret_key: optional("STRIPE_SECRET_KEY", ""),
            stripe_webhook_secret: optional("STRIPE_WEBHOOK_SECRET", ""),
            stripe_api_base: optional("STRIPE_API_BASE", "https://api.stripe.com"),
            entitlement_signing_private_key: require("ENTITLEMENT_SIGNING_PRIVATE_KEY")?,
            entitlement_signing_key_id: optional("ENTITLEMENT_SIGNING_KEY_ID", "entitlement-key-1"),
            entitlement_verify_public_keys: parse_verify_public_keys(
                std::env::var(VERIFY_PUBLIC_KEYS_VAR).ok().as_deref(),
            )?,
            dataforge_api_url: optional("DATAFORGE_API_URL", ""),
            dataforge_service_token: optional("DATAFORGE_SERVICE_TOKEN", ""),
            update_rollout_secret: optional("UPDATE_ROLLOUT_SECRET", ""),
            release_artifact_base_url: optional("RELEASE_ARTIFACT_BASE_URL", ""),
            snapshot_ttl: Duration::from_secs(snapshot_ttl_hours * 3600),
            offline_grace: Duration::from_secs(offline_grace_days * 86400),
            deletion_cooling_off: Duration::from_secs(
                parse_u64("DELETION_COOLING_OFF_DAYS", 14)? * 86400,
            ),
            usage_reservation_ttl: Duration::from_secs(parse_u64(
                "USAGE_RESERVATION_TTL_SECS",
                900,
            )?),
            usage_threshold_percents: parse_percent_list("USAGE_THRESHOLD_PERCENTS", &[80, 100])?,
            request_timeout: Duration::from_secs(parse_u64("REQUEST_TIMEOUT_SECS", 30)?),
            max_body_bytes: parse_u64("MAX_BODY_BYTES", 1024 * 1024)? as usize,
            rate_limit_per_minute: u32::try_from(parse_u64("RATE_LIMIT_PER_MINUTE", 300)?)
                .map_err(|_| {
                    ConfigError::Invalid("RATE_LIMIT_PER_MINUTE", "value too large".to_string())
                })?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_keys_unset_or_blank_means_none() {
        assert!(parse_verify_public_keys(None).unwrap().is_empty());
        assert!(parse_verify_public_keys(Some("")).unwrap().is_empty());
        assert!(parse_verify_public_keys(Some("  \n")).unwrap().is_empty());
        assert!(parse_verify_public_keys(Some("{}")).unwrap().is_empty());
    }

    #[test]
    fn verify_keys_parse_in_document_order() {
        let keys = parse_verify_public_keys(Some(r#"{"key-b":"BBBB","key-a":"AAAA"}"#)).unwrap();
        assert_eq!(
            keys,
            vec![
                ("key-b".to_string(), "BBBB".to_string()),
                ("key-a".to_string(), "AAAA".to_string())
            ]
        );
    }

    fn invalid_reason(raw: &str) -> String {
        match parse_verify_public_keys(Some(raw)) {
            Err(ConfigError::Invalid(var, reason)) => {
                assert_eq!(var, VERIFY_PUBLIC_KEYS_VAR);
                reason
            }
            other => panic!("expected Invalid for {raw:?}, got {other:?}"),
        }
    }

    #[test]
    fn verify_keys_malformed_value_fails_with_the_variable_named() {
        for raw in [
            "not json",
            r#"["key-a","AAAA"]"#,
            r#""AAAA""#,
            r#"{"key-a":1}"#,
            r#"{"key-a":{"key":"AAAA"}}"#,
        ] {
            invalid_reason(raw);
        }
        let error = parse_verify_public_keys(Some("not json")).unwrap_err();
        assert!(
            error.to_string().contains(VERIFY_PUBLIC_KEYS_VAR),
            "{error}"
        );
    }

    #[test]
    fn verify_keys_repeated_key_id_fails() {
        let reason = invalid_reason(r#"{"key-a":"AAAA","key-a":"BBBB"}"#);
        assert!(reason.contains("duplicate key_id key-a"), "{reason}");
    }

    #[test]
    fn verify_keys_empty_key_id_fails() {
        let reason = invalid_reason(r#"{"":"AAAA"}"#);
        assert!(reason.contains("empty key_id"), "{reason}");
    }
}
