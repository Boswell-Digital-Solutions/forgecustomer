//! SC-2: an operator token is accepted once (P3 gate G5, DEC-G5-3 option (a)).
//!
//! The replay identity is the issuer and the `jti` together, so one `jti` under two issuers is
//! two identities. The claim is one atomic operation. A store that cannot answer refuses the
//! token: an unavailable store never lets a token through.

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use sqlx::PgPool;

/// The store could not decide. The caller refuses the token.
#[derive(Debug, thiserror::Error)]
#[error("replay store unavailable")]
pub struct ReplayStoreUnavailable;

#[async_trait]
pub trait ReplayStore: Send + Sync {
    /// Claim the identity. `Ok(true)`: first use, the request may run. `Ok(false)`: already
    /// used, refuse. `Err`: undecided, refuse.
    async fn claim(
        &self,
        issuer: &str,
        jti: &str,
        expires_at_unix: i64,
    ) -> Result<bool, ReplayStoreUnavailable>;
}

/// The production store: `public.operator_token_replay` (migration 0018).
pub struct PgReplayStore {
    pool: PgPool,
}

impl PgReplayStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReplayStore for PgReplayStore {
    async fn claim(
        &self,
        issuer: &str,
        jti: &str,
        expires_at_unix: i64,
    ) -> Result<bool, ReplayStoreUnavailable> {
        // One statement: purge rows well past expiry, then insert or do nothing. The primary
        // key makes two concurrent claims of one identity resolve to exactly one row.
        let row: Option<(i32,)> = sqlx::query_as(
            "with purge as (
               delete from public.operator_token_replay
               where expires_at < now() - interval '5 minutes'
             )
             insert into public.operator_token_replay (issuer, jti, expires_at)
             values ($1, $2, to_timestamp($3))
             on conflict (issuer, jti) do nothing
             returning 1",
        )
        .bind(issuer)
        .bind(jti)
        .bind(expires_at_unix as f64)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "operator token replay store unavailable; token refused");
            ReplayStoreUnavailable
        })?;
        Ok(row.is_some())
    }
}

/// An in-process store with the same contract, for tests. Share one value between two
/// routers to model two application instances over one database.
#[derive(Default)]
pub struct MemoryReplayStore {
    claimed: Mutex<HashMap<(String, String), i64>>,
}

#[async_trait]
impl ReplayStore for MemoryReplayStore {
    async fn claim(
        &self,
        issuer: &str,
        jti: &str,
        expires_at_unix: i64,
    ) -> Result<bool, ReplayStoreUnavailable> {
        let mut claimed = self.claimed.lock().map_err(|_| ReplayStoreUnavailable)?;
        let key = (issuer.to_string(), jti.to_string());
        if claimed.contains_key(&key) {
            return Ok(false);
        }
        claimed.insert(key, expires_at_unix);
        Ok(true)
    }
}

impl MemoryReplayStore {
    /// Number of claimed identities. Tests use it to prove a refused token claimed nothing.
    pub fn len(&self) -> usize {
        self.claimed.lock().map(|c| c.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A store that never answers, for the fail-closed test.
pub struct UnavailableReplayStore;

#[async_trait]
impl ReplayStore for UnavailableReplayStore {
    async fn claim(&self, _: &str, _: &str, _: i64) -> Result<bool, ReplayStoreUnavailable> {
        Err(ReplayStoreUnavailable)
    }
}
