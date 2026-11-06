# AC-01 — Overlap wired (WP-01)

- **Date:** 2026-11-06.
- **Candidate:** forgecustomer PR #43, branch `feat/entitlement-verify-keys`, commit `c2b30f5` on `main` `53d255a`.
- **Status:** candidate evidence. It does not satisfy acceptance until #43 merges and the result is rechecked on `main`.
- **Command:** the repository gate, `cd api && cargo fmt --all --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all`.
- **Observed:** exit 0. 146 unit tests and 36 integration tests pass; 1 ignored (`serve_isolated_local_commerce`, a manual server that predates this change). Log: `~/Forge/reports/foc_wp01_gate.log`, sha256 `789ddb7acc5b3d8be9d24c358cdb9abd6923b41da80392545a3bcee948b3244f`.

## AC-01 items

| Item | Test | Result |
|---|---|---|
| A second test key in the variable is listed on `GET /v1/entitlements/keys` | `entitlement_keys_publish_a_retained_key_beside_the_active_key` (route, through the router) | PASS |
| A snapshot signed by the second key verifies through `verify_snapshot` | `state::tests::a_retained_key_is_published_and_verifies_beside_the_active_key` | PASS |
| Unset means today's behavior (one key) | `verify_keys_unset_or_blank_means_none`; `no_entries_keep_only_the_active_key` | PASS |
| A malformed value fails start-up with a named error | `verify_keys_malformed_value_fails_with_the_variable_named`; `a_key_that_is_not_32_base64_bytes_fails_with_the_variable_named`; `a_malformed_verify_key_fails_state_build` | PASS |
| A duplicate `key_id` fails start-up | `verify_keys_repeated_key_id_fails`; `the_active_key_id_with_a_different_key_fails` | PASS |
| `rotation_overlap_both_keys_verify` still passes | existing test | PASS |

## Negative-check proof (hand mutants, committed first, each reverted)

| Mutant | Tests that fail |
|---|---|
| The wiring call in `AppState::build` removed | `entitlement_keys_publish_a_retained_key_beside_the_active_key`, `a_malformed_verify_key_fails_state_build` |
| The duplicate-`key_id` check disabled | `verify_keys_repeated_key_id_fails` |
| The active-`key_id` conflict check disabled | `the_active_key_id_with_a_different_key_fails` |

## Result

AC-01: **PASS on the candidate.**
