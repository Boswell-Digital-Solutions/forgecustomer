# AC-08 — procedure (prepared, not run)

**Status: prepared 2026-11-13. Not run. Not authorized.** This file says how AC-08 runs and what it records. Running it needs WP-05 authorization and the prerequisites below. AC-08 stays `NOT_RUN` until a receipt exists.

AC-08 is the 05 §6 path against the deployed services. Pass needs steps 2 and 3 to succeed, and steps 4, 5 and 6 to be refused with the named status (07).

## Prerequisites

Each is the operator's. The procedure does not start until all are met.

| # | Prerequisite | State on 2026-11-13 |
|---|---|---|
| P1 | WP-05 is authorized. | Not authorized |
| P2 | A ruling on G4. WP-05 lists G4 as a prerequisite. G4 is recorded as executed before G2, not PASS, and may not be made PASS after the fact. The ruling says whether the executed G4 meets the WP-05 prerequisite, or what must run first. | Open |
| P3 | Migration `0017` is applied to the deployed ForgeCustomer database, and a read-only check shows `app_clients` (with the `authorforge` row), `rake_tenant_pseudonyms` and `app_token_mints`. Without it, step 2 cannot pass (`KI-FOC-20261113-001`). | Not applied |
| P4 | Two synthetic test accounts exist (next section). No real customer account is used. | Not created |
| P5 | An operator admin token for the deployed ForgeCustomer admin routes, for the account setup only. | Operator holds |

## Test accounts

Both accounts belong to the operator. Each uses an address on a domain the operator controls. Neither holds customer data.

| Account | Setup | Purpose |
|---|---|---|
| A — entitled | `POST /v1/account/provision`; enroll TOTP MFA; `POST /v1/installations` with a device key made for this test; an admin license for product `authorforge` (`POST /v1/admin/licenses`); `POST /v1/installations/{id}/activate`; an admin override `authorforge.cloud.enabled` = `true` with `expires_at` 24 hours ahead (`POST /v1/admin/entitlements/override`) | Steps 1 to 5 |
| B — not entitled | The same setup, with the override `authorforge.cloud.enabled` = `false`, 24 hours | Step 6 |

Account B needs the full installation and activation. The exchange checks the client proof before the entitlement. Without an active installation, B is refused with `CLIENT_AUTH_FAILED`, and step 6 proves nothing.

The device keys and both Supabase sessions stay in the test runner's memory. They are not written to a file, a log or a record.

## Steps

The runner is a script on the operator's machine. It calls only `https://forge-customer.onrender.com` and `https://rake-zp35.onrender.com`.

| Step | Call | Expected |
|---|---|---|
| 1 | Account A signs in to Supabase with TOTP. | A session with `aal2` |
| 2 | `POST /v1/app-tokens/rake`, `app_id` `authorforge`, `scopes` `["rake:read"]`, a fresh `client_proof` | 200, `cache-control: no-store`, `expires_in` 300. Header `alg` `EdDSA`, `kid` `rake-access-1`. Claims `iss` `bds-forgecustomer`, `aud` `rake.application`, `type` `access`, `app_id` `authorforge`, `scopes` `["rake:read"]`, `exp` = `iat` + 300; no email, Supabase user id or `customer_id` (04 §2) |
| 3 | Rake `GET /api/v1/jobs` with the token | 200 |
| 4 | ForgeCustomer `GET /v1/account` with the same token | 401. The service accepts only a Supabase token there |
| 5 | Rake `GET /api/v1/jobs` with one changed byte in the token's claims segment | 401 |
| 6 | Account B: the same exchange as step 2 | 403 `ENTITLEMENT_DENIED`. No mint row for B |

A result other than the expected one is a FAIL. On FAIL, the runner records the step and the observed status, and stops (WP-05 stop conditions).

## Receipt

The runner writes `evidence/AC-08/receipt.json`, and the agent writes `evidence/AC-08/RESULT.md` from it.

- **Recorded:** the step, the HTTP status, the error `code`, the token `kid` and `jti`, `iat`, `exp`, the claim names present, the UTC time of each call, and the deployed commits of both services (from Render).
- **Never recorded:** a Rake token, a Supabase token or session, a device key, a password, a TOTP secret, an email address, or the values of `sub`, `user_id` and `tenant_id` (pseudonyms).
- **Before commit:** search the receipt for `eyJ` and for any 43-character base64 string. Expect zero matches. The repository secret scan runs on the PR.

The rake record goes to rake `docs/evidence/DEC-RAKE-JWT-01/` (WP-05).

## After the run

- Remove both overrides, or let them expire. Deactivate both installations (`POST /v1/installations/{id}/deactivate`).
- Account deletion of A and B is outside AC-08. If the operator wants it, it is a separate step. It also needs P3, because deletion revokes pseudonyms.
- A token from step 2 expires 300 s after issue, plus 60 s of skew in Rake. No other cleanup applies to it.

## Limits

- AC-08 proves the read path only. `rake:execute` and the single-use `rake:approve` path are covered by AC-05 and AC-06 on rake `master`, not on the deployed service.
- One run with two accounts. It is not a load or availability test.
- The procedure changes no deployed configuration. P3 and P5 are prerequisites that the operator meets. They are not part of the run.
