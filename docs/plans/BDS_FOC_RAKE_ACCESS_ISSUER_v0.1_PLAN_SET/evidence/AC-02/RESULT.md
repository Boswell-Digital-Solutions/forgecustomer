# AC-02 to AC-04 — result

**Result: PASS on the WP-02 candidate, before merge.** Base: the WP-01 branch head `ebd1dfcf9` (forgecustomer #53) plus the uncommitted WP-02 files. Environment: the operator's machine, Linux, cargo 1.97.1, a throwaway PostgreSQL 16.15 cluster on 127.0.0.1:55432 (the operator's own database server was not used), OpenSSL 3.5.5. 2026-11-11.

AC-03 and AC-04 point to this file.

## Database set-up (as in CI)

Created database `fc` with the CI auth shims. Applied every migration twice, plus `supabase/seed.sql`, with no error. Tables without RLS: 0. `supabase/tests/update_eligibility_matrix.sql` and `supabase/tests/rls_customer_write_denial.sql`: exit 0.

## HTTP smoke — `bash scripts/rake_access_http_smoke.sh`

It runs the real API twice: once with a Rake key, once without. Exit 0. Output:

```
PASS AC-03 keys route: public halves only; entitlement route lists none of them
PASS no Supabase token: 401 UNAUTHENTICATED
PASS bad Supabase token: 401 INVALID_TOKEN
PASS aal1 with mfa_required: 403 MFA_REQUIRED
PASS unknown app: 403 APP_NOT_ALLOWED
PASS scope outside the list: 403 SCOPE_NOT_ALLOWED
PASS rake:approve mixed with another scope: 403 SCOPE_NOT_ALLOWED rake:approve must be alone
PASS feature not entitled: 403 ENTITLEMENT_DENIED
PASS revoked device: 401 CLIENT_AUTH_FAILED device_key
PASS proof signed by another key: 401 CLIENT_AUTH_FAILED signature
PASS stale proof: 401 CLIENT_AUTH_FAILED stale
PASS proof for other scopes: 401 CLIENT_AUTH_FAILED signature
PASS no audit row for any refused request
PASS AC-02 success: 200, no-store, contract header and claims, verifies with the published key, one audit row
PASS AC-04 token carries no customer_id, Supabase user id or email
PASS replayed proof (same nonce): 401 CLIENT_AUTH_FAILED nonce_replayed
PASS AC-04 pseudonyms: stable across mints, not the customer id, replaced after revocation
PASS DEC-6 approve-only token issued with scopes [rake:approve]
PASS audit write failure: 503 SERVICE_UNAVAILABLE
PASS audit write failure leaves no token and no row
PASS no key configured: the key list is empty
PASS no key configured: 503 SERVICE_UNAVAILABLE
rake access HTTP smoke: PASS
```

A second run on the same database also exits 0, with 22 PASS lines, so the fixture resets cleanly.

The fixture's entitled customer holds an active Pro subscription. A first attempt gave it only an admin override and was refused `ENTITLEMENT_DENIED`: cloud features need an entitling subscription, and an override alone cannot grant one (`domain::entitlement`). That is the intended rule, so the fixture now uses the real path.

## Mutation check

Making the entitlement check always pass (`if false && !matches!(...)`) made the smoke fail at `FAIL feature not entitled: got 200`. The file was restored and compared with `cmp`.

## Rust tests

| Command (from `api/`) | Observed |
|---|---|
| `cargo test --lib rake_access` | 9 passed: key separation, empty issuer, published keys, scope rules (DEC-6), token claims and signature, device proof valid / changed field / stale / future / malformed |
| `cargo test --test rake_access_routes` | 7 passed: no token 401, bad token 401, keys route lists Rake keys only, empty key list without a key, start-up fails on equal keys, on verify keys without a signing key, and on a malformed key |
| `cargo fmt --all --check`; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo test --all` | clean; exit 0; 155 passed and 1 ignored (lib), 1 contract, 7 routes, 36 security |
| `bash scripts/update_campaign_http_smoke.sh` (existing) | passed |
| `npx @redocly/cli lint contracts/openapi.yaml` | valid. 93 warnings against 89 on the base file: the two new routes have no `operationId`, as no other route does, and the keys route lists no error response, like `/v1/entitlements/keys`. |

## Mapping to 07

| AC | Rows | Where |
|---|---|---|
| AC-02 | no token, bad token, `aal1` + `mfa_required`, unknown app, extra scope, approve mixed, feature false, key unset (503), audit write failure (503, no token, no row), success (200, verifies with the published key, contract claims, one audit row, `no-store`) | smoke |
| AC-03 | start-up fails when the Rake key equals the entitlement key; keys route public halves with active and standby `kid`; entitlement route does not list them | routes test, smoke |
| AC-04 | same tenant across mints; differs from `customer_id`; new value after revocation; no email, Supabase user id or `customer_id` in the token | smoke |

## Limits

- **Deletion revokes the pseudonyms** (04 §5): the code is in `repositories::privacy::execute_deletion`, but no test runs a deletion. The deletion flow needs an operator token and a cooling-off period. This row of AC-04 is UNVERIFIED.
- The Supabase token in the smoke is the legacy HS256 test path. The ES256 JWKS path is the existing verifier and is not changed.
- Two customers is not a broad "different customers get different tenants" proof. The values come from `gen_random_uuid()` with a unique constraint.

## After merge (2026-11-12)

- forgecustomer #55 merged on head `532fa35dc87cae1e48fa273394945273b5d230f4`. That head merges `main` (after WP-01) into the candidate `e4586104c92e88141f40f71ffdc7f290fe9a758b`. `git diff --stat e458610 532fa35` is empty, so the merged files are the files tested above. Merge commit `8e0cf284736f1dd1b9fc0e195dce172cf79135fa`, 2026-11-11T13:37:50Z.
- The `main` push run `43030867714` on the merge commit passed every job:
  - `rake access HTTP smoke` (`130557325593`): every check line is `PASS`, no line is `FAIL`, and the last line is `rake access HTTP smoke: PASS`.
  - `fmt + clippy + test` (`130557325546`): `tests/rake_access_routes.rs` 7 passed; `tests/rake_access_contract.rs` 1 passed; lib 155 passed, 1 ignored.
- AC-02 and AC-03 stay PASS. AC-04 stays PASS except the deletion row, which is still UNVERIFIED: no test runs a deletion. G2 waits on that row.
