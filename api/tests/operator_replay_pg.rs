//! SC-2 against a real PostgreSQL: the production replay store and migration 0018.
//!
//! Ignored by default: it needs a disposable database with every migration applied, named by
//! `OPERATOR_REPLAY_TEST_DATABASE_URL`. Run it with `cargo test --test operator_replay_pg --
//! --ignored`. It never passes silently: without the variable it fails.

use std::sync::Arc;

use forgecustomer_api::services::operator_replay::{PgReplayStore, ReplayStore};
use sqlx::postgres::PgPoolOptions;

fn url() -> String {
    std::env::var("OPERATOR_REPLAY_TEST_DATABASE_URL")
        .expect("set OPERATOR_REPLAY_TEST_DATABASE_URL to a disposable, migrated database")
}

async fn store() -> PgReplayStore {
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&url())
        .await
        .expect("connect");
    PgReplayStore::new(pool)
}

fn exp_in(secs: i64) -> i64 {
    chrono::Utc::now().timestamp() + secs
}

#[tokio::test]
#[ignore = "needs a disposable migrated PostgreSQL (OPERATOR_REPLAY_TEST_DATABASE_URL)"]
async fn a_claim_holds_across_two_instances_and_restarts() {
    let jti = uuid::Uuid::new_v4().to_string();
    // Two pools model two application instances over one database.
    let first = store().await;
    let second = store().await;
    assert!(first.claim("fcn_local", &jti, exp_in(120)).await.unwrap());
    assert!(!second.claim("fcn_local", &jti, exp_in(120)).await.unwrap());
    // A new pool after the others are gone (a restart) still sees the claim.
    drop(first);
    drop(second);
    assert!(!store()
        .await
        .claim("fcn_local", &jti, exp_in(120))
        .await
        .unwrap());
}

#[tokio::test]
#[ignore = "needs a disposable migrated PostgreSQL (OPERATOR_REPLAY_TEST_DATABASE_URL)"]
async fn concurrent_claims_of_one_identity_resolve_to_exactly_one() {
    let jti = uuid::Uuid::new_v4().to_string();
    let s = Arc::new(store().await);
    let mut tasks = Vec::new();
    for _ in 0..16 {
        let s = s.clone();
        let jti = jti.clone();
        tasks.push(tokio::spawn(async move {
            s.claim("forge_command_local", &jti, exp_in(120))
                .await
                .unwrap()
        }));
    }
    let mut wins = 0;
    for t in tasks {
        if t.await.unwrap() {
            wins += 1;
        }
    }
    assert_eq!(wins, 1);
}

#[tokio::test]
#[ignore = "needs a disposable migrated PostgreSQL (OPERATOR_REPLAY_TEST_DATABASE_URL)"]
async fn one_jti_under_two_issuers_is_two_identities() {
    let s = store().await;
    let jti = uuid::Uuid::new_v4().to_string();
    assert!(s
        .claim("forge_command_local", &jti, exp_in(120))
        .await
        .unwrap());
    assert!(s.claim("fcn_local", &jti, exp_in(120)).await.unwrap());
}

#[tokio::test]
#[ignore = "needs a disposable migrated PostgreSQL (OPERATOR_REPLAY_TEST_DATABASE_URL)"]
async fn rows_well_past_expiry_are_purged_and_live_rows_are_kept() {
    let s = store().await;
    let old = uuid::Uuid::new_v4().to_string();
    let live = uuid::Uuid::new_v4().to_string();
    assert!(s.claim("fcn_local", &old, exp_in(-600)).await.unwrap());
    assert!(s.claim("fcn_local", &live, exp_in(120)).await.unwrap());
    // The next claim purges the row more than five minutes past expiry.
    assert!(s
        .claim("fcn_local", &uuid::Uuid::new_v4().to_string(), exp_in(120))
        .await
        .unwrap());
    let pool = PgPoolOptions::new().connect(&url()).await.unwrap();
    let count = |jti: String| {
        let pool = pool.clone();
        async move {
            sqlx::query_scalar::<_, i64>(
                "select count(*) from public.operator_token_replay where jti = $1",
            )
            .bind(jti)
            .fetch_one(&pool)
            .await
            .unwrap()
        }
    };
    assert_eq!(count(old).await, 0);
    assert_eq!(count(live.clone()).await, 1);
    assert!(!s.claim("fcn_local", &live, exp_in(120)).await.unwrap());
}

#[tokio::test]
#[ignore = "needs a disposable migrated PostgreSQL (OPERATOR_REPLAY_TEST_DATABASE_URL)"]
async fn an_unreachable_database_is_undecided_never_a_pass() {
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(1))
        .connect_lazy("postgres://user@127.0.0.1:1/none")
        .unwrap();
    let s = PgReplayStore::new(pool);
    assert!(s.claim("fcn_local", "x", exp_in(120)).await.is_err());
}
