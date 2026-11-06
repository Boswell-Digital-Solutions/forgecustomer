//! Shared application state injected into handlers.

use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::auth::{customer::CustomerJwtValidator, JwtValidator};
use crate::config::Config;
use crate::middleware::rate_limit::RateLimiter;
use crate::services::signing::{Signer25519, VerifyingKeyRing};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub pool: PgPool,
    pub signer: Arc<Signer25519>,
    pub key_ring: Arc<VerifyingKeyRing>,
    pub customer_validator: CustomerJwtValidator,
    pub admin_validator: JwtValidator,
    pub posture_validator: JwtValidator,
    pub http: reqwest::Client,
    pub rate_limiter: Arc<RateLimiter>,
}

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("signing key error: {0}")]
    Signing(#[from] crate::services::signing::SigningError),
    #[error("{var}: {0}", var = crate::config::VERIFY_PUBLIC_KEYS_VAR)]
    VerifyKeys(String),
    #[error("database pool error: {0}")]
    Pool(#[from] sqlx::Error),
    #[error("http client error: {0}")]
    Http(#[from] reqwest::Error),
}

/// Load retained and pending verification keys into the ring next to the active signer, so a
/// key change has an overlap window (`doc/system/11`). Each key is decoded by the ring's own
/// `add_base64`. An entry under the active signer's `key_id` must carry the active public key:
/// a different key under that id would make the published list contradict the signer.
fn add_verify_keys(
    key_ring: &mut VerifyingKeyRing,
    signer: &Signer25519,
    entries: &[(String, String)],
) -> Result<(), StateError> {
    let active_public_key = signer.public_key_base64();
    for (key_id, public_key) in entries {
        let mut probe = VerifyingKeyRing::new();
        probe
            .add_base64(key_id.clone(), public_key)
            .map_err(|e| StateError::VerifyKeys(format!("key_id {key_id}: {e}")))?;
        if key_id == signer.key_id() {
            let same = probe
                .published()
                .first()
                .is_some_and(|(_, published)| published == &active_public_key);
            if !same {
                return Err(StateError::VerifyKeys(format!(
                    "key_id {key_id} is the active signing key id with a different public key"
                )));
            }
            continue;
        }
        key_ring
            .add_base64(key_id.clone(), public_key)
            .map_err(|e| StateError::VerifyKeys(format!("key_id {key_id}: {e}")))?;
    }
    Ok(())
}

impl AppState {
    /// Build application state from configuration. The database pool connects lazily so the
    /// process can start (and serve `/v1/health`) before the database is reachable.
    pub fn build(config: Config) -> Result<Self, StateError> {
        let signer = Signer25519::from_base64_seed(
            &config.entitlement_signing_key_id,
            &config.entitlement_signing_private_key,
        )?;

        let mut key_ring = VerifyingKeyRing::new();
        key_ring.add_signer(&signer);
        add_verify_keys(
            &mut key_ring,
            &signer,
            &config.entitlement_verify_public_keys,
        )?;

        let customer_validator = CustomerJwtValidator::new(
            &config.supabase_jwt_secret,
            &config.supabase_jwt_issuer,
            &config.supabase_jwt_audience,
        )?;
        let admin_validator = JwtValidator::eddsa(
            &config.admin_jwt_public_key,
            &config.admin_jwt_issuer,
            &config.admin_jwt_audience,
        );

        let posture_validator = JwtValidator::eddsa(
            &config.admin_jwt_public_key,
            &config.admin_jwt_issuer,
            "forgecustomer-posture",
        );

        // `DATABASE_URL` points at Supabase's pooled connection string (RUNBOOK.md), which
        // may be the transaction-mode pooler (PgBouncer): connections are handed out per
        // transaction, not held for a session, so a server-side prepared statement created
        // on one backend can vanish before the next query that expects to reuse it lands on
        // a different backend ("prepared statement \"...\" does not exist"). SQLx's default
        // per-connection statement cache assumes a stable session and isn't safe under that
        // pooling mode. Disabling it falls back to unnamed prepare-per-query (safe under
        // both session and transaction pooling), at the cost of re-parsing each query text
        // every time instead of reusing a cached plan.
        let connect_options = config
            .database_url
            .parse::<sqlx::postgres::PgConnectOptions>()?
            .statement_cache_capacity(0);
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(config.database_acquire_timeout)
            .connect_lazy_with(connect_options);

        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self {
            config: Arc::new(config),
            pool,
            signer: Arc::new(signer),
            key_ring: Arc::new(key_ring),
            customer_validator,
            admin_validator,
            posture_validator,
            http,
            rate_limiter: Arc::new(RateLimiter::default()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::snapshot::{EntitlementSnapshot, SCHEMA_VERSION};
    use base64::engine::general_purpose::STANDARD as B64;
    use base64::Engine;
    use std::collections::BTreeMap;

    fn signer(key_id: &str, seed_byte: u8) -> Signer25519 {
        Signer25519::from_base64_seed(key_id, &B64.encode([seed_byte; 32])).unwrap()
    }

    fn signed_by(signer: &Signer25519) -> EntitlementSnapshot {
        let mut snapshot = EntitlementSnapshot {
            schema_version: SCHEMA_VERSION.to_string(),
            customer_id: "cust_1".to_string(),
            installation_id: Some("inst_1".to_string()),
            product: "authorforge".to_string(),
            issued_at: "2026-11-06T00:00:00Z".to_string(),
            expires_at: "2026-11-07T00:00:00Z".to_string(),
            features: BTreeMap::new(),
            quotas: BTreeMap::new(),
            key_id: String::new(),
            signature: String::new(),
        };
        signer.sign_snapshot(&mut snapshot).unwrap();
        snapshot
    }

    fn ring_with(
        active: &Signer25519,
        entries: &[(String, String)],
    ) -> Result<VerifyingKeyRing, StateError> {
        let mut ring = VerifyingKeyRing::new();
        ring.add_signer(active);
        add_verify_keys(&mut ring, active, entries)?;
        Ok(ring)
    }

    #[test]
    fn no_entries_keep_only_the_active_key() {
        let active = signer("entitlement-key-1", 1);
        let ring = ring_with(&active, &[]).unwrap();
        let ids: Vec<String> = ring.published().into_iter().map(|(id, _)| id).collect();
        assert_eq!(ids, vec!["entitlement-key-1".to_string()]);
    }

    #[test]
    fn a_retained_key_is_published_and_verifies_beside_the_active_key() {
        let active = signer("entitlement-key-2", 2);
        let retained = signer("entitlement-key-1", 1);
        let entries = vec![(
            "entitlement-key-1".to_string(),
            retained.public_key_base64(),
        )];
        let ring = ring_with(&active, &entries).unwrap();

        let mut ids: Vec<String> = ring.published().into_iter().map(|(id, _)| id).collect();
        ids.sort();
        assert_eq!(
            ids,
            vec![
                "entitlement-key-1".to_string(),
                "entitlement-key-2".to_string()
            ]
        );
        ring.verify_snapshot(&signed_by(&retained))
            .expect("a snapshot from the retained key verifies");
        ring.verify_snapshot(&signed_by(&active))
            .expect("a snapshot from the active key verifies");
    }

    #[test]
    fn the_active_key_repeated_with_its_own_public_key_is_accepted() {
        let active = signer("entitlement-key-1", 1);
        let entries = vec![("entitlement-key-1".to_string(), active.public_key_base64())];
        let ring = ring_with(&active, &entries).unwrap();
        assert_eq!(ring.published().len(), 1);
    }

    #[test]
    fn the_active_key_id_with_a_different_key_fails() {
        let active = signer("entitlement-key-1", 1);
        let other = signer("entitlement-key-1", 9);
        let entries = vec![("entitlement-key-1".to_string(), other.public_key_base64())];
        let error = ring_with(&active, &entries).err().expect("must fail");
        let text = error.to_string();
        assert!(
            text.starts_with("ENTITLEMENT_VERIFY_PUBLIC_KEYS: "),
            "{text}"
        );
        assert!(text.contains("different public key"), "{text}");
    }

    #[test]
    fn a_key_that_is_not_32_base64_bytes_fails_with_the_variable_named() {
        let active = signer("entitlement-key-1", 1);
        for bad in ["not base64!", "AAAA", ""] {
            let entries = vec![("entitlement-key-0".to_string(), bad.to_string())];
            let error = ring_with(&active, &entries).err().expect("must fail");
            let text = error.to_string();
            assert!(
                text.starts_with("ENTITLEMENT_VERIFY_PUBLIC_KEYS: key_id entitlement-key-0"),
                "{text}"
            );
        }
    }
}
