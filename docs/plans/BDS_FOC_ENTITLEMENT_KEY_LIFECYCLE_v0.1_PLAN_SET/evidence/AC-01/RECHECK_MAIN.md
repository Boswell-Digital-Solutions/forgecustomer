# AC-01 — recheck on `main` (WP-01)

- **Date:** 2026-11-18.
- **Subject:** forgecustomer `main` `ae62638`. WP-01 merged as PR #43 on 2026-11-06 (merge commit
  `7bce7f889`, an ancestor of `ae62638`).
- **Why:** `RESULT.md` recorded candidate evidence on `c2b30f5` and required a recheck on `main`
  after #43 merged. This is that recheck.
- **Environment:** the operator's Linux workstation, rustc 1.97.1, a fresh worktree at `ae62638`.
  No deployed service and no real key. The tests use keys they generate.
- **Command:** the repository gate, `cd api && cargo fmt --all --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all`.
- **Observed:** exit 0. 170 unit tests and 73 integration tests pass. Seven are ignored: the
  manual server `serve_isolated_local_commerce`, and six `operator_replay_pg` tests that need a
  disposable migrated PostgreSQL. None of the seven is an AC-01 item. Log:
  `~/Forge/reports/foc_wp01_gate_main_ae62638.log`, sha256
  `b9fd68c39f5cb7ed508a39de71327b13c05d82cd731320c111b7101c97b98b4d`.

## AC-01 items on `main`

| Item | Test | Result |
|---|---|---|
| A second test key in the variable is listed on `GET /v1/entitlements/keys` | `entitlement_keys_publish_a_retained_key_beside_the_active_key` | PASS |
| A snapshot signed by the second key verifies | `a_retained_key_is_published_and_verifies_beside_the_active_key` | PASS |
| Unset means one key | `verify_keys_unset_or_blank_means_none`; `no_entries_keep_only_the_active_key` | PASS |
| A malformed value fails start-up with a named error | `verify_keys_malformed_value_fails_with_the_variable_named`; `a_key_that_is_not_32_base64_bytes_fails_with_the_variable_named`; `a_malformed_verify_key_fails_state_build` | PASS |
| A duplicate `key_id` fails start-up | `verify_keys_repeated_key_id_fails`; `the_active_key_id_with_a_different_key_fails` | PASS |
| `rotation_overlap_both_keys_verify` still passes | existing test | PASS |

The hand mutants of `RESULT.md` were not rerun.

## Result

AC-01: **PASS on `main` `ae62638`.**
