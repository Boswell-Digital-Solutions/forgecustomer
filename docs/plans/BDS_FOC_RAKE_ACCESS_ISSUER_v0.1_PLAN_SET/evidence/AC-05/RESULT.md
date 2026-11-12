# AC-05 and AC-06 — Verifier in Rake (WP-03)

**Result: AC-05 PASS. AC-06 PASS.** This record covers both ACs from one CI run. `../AC-06/RESULT.md` points here.

## Source

| Item | Value |
|---|---|
| Repository | `Boswell-Digital-Solutions/rake` |
| Pull request | rake #188, head `264b28ed205e17a593f11cb9d9fe5877d28da7fd` |
| Merge commit | `9b7af4786d1bf325dc7dd0b5cdaf3dc617e3b3ad` on `master`, 2026-11-11T13:38:07Z |
| Contract change | forge #424, merge `27efccb2ddc912ac42a732d97bb68674b7cc6297`: the shared service-contract schema accepts token type `application_access_jwt` |
| Test key | the 04 vector key: SHA-256 of `BDS rake-access-token-v1 test key - never deployed` (`tests/rake_access_tokens.py`). No deployed key is involved. |

## Environment and command

The run is the `master` push run of the merge commit: workflow `Rake V1 CI/CD`, run `43030910842`. Every job of the run passed. Deploy to Production skipped, as on every non-tag push.

| Job | Id | Command (rake `doc/system/50_operations/18-testing.md`) | Observed |
|---|---|---|---|
| Unit Tests | `130557440174` | `pytest tests/unit -v --timeout=60 --cov=. …` on Python 3.11.18 | `729 passed, 12 skipped, 2 subtests passed`. No verifier test is in the skip set. |
| Integration Tests | `130557440284` | `pytest tests/integration -m "integration and not requires_live_credentials" …` and the skip guard | success |
| Integration Qualification (credentialed) | `130557440270` | `pytest tests/integration -m requires_live_credentials` and the skip guard | success |

`UNIT_TEST_IDS.txt` holds the 101 result lines of the three verifier test files from the Unit Tests log: `test_jwt_security.py`, `test_rake_access_contract_vectors.py` and `test_rake_access_http.py`. Each line is `PASSED`. SHA-256 of the file: `d0674ae58eaf2bded48e2e82022c0545f8a39c2982f48d1c5f972e928dc2dd08`.

## AC-05 mapping

| Expected observation (07) | Test id | Result |
|---|---|---|
| A token from the 04 vectors is accepted on a read route | `test_rake_access_http.py::test_ac05_valid_token_accepted_on_read_route` (200, the mission is returned) | PASSED |
| … and on an execute route | `test_rake_access_http.py::test_ac05_valid_token_accepted_on_execute_route` (200, the mission is `CANCELLED`) | PASSED |
| The existing tenant tests pass unchanged | `test_ac05_tenant_binding_still_applies` (another tenant gets 404); `test_execution_access.py::test_cross_tenant_mission_is_hidden` | PASSED |
| The existing scope tests pass unchanged | `test_ac05_scope_rule_still_applies` (`rake:read` alone on cancel gets 403); `test_execution_access.py::test_creation_auth[…]` | PASSED |
| Every valid 04 vector is accepted | `test_rake_access_contract_vectors.py::test_valid_vector_is_accepted[…]` | PASSED |

## AC-06 mapping

Each case in `test_ac06_refused_with_reason` asserts status 401, the body `Invalid execution credential`, and that the logged refusal reason is exactly the named reason.

| Required refusal (07) | Test id | Asserted reason | Result |
|---|---|---|---|
| HS256 token signed with any secret | `test_ac06_refused_with_reason[hs256-any-secret]` | `ALG_NOT_ALLOWED` | PASSED |
| `alg` `none` | `[alg-none]` | `ALG_NOT_ALLOWED` | PASSED |
| wrong `iss` | `[wrong-iss]` | `ISSUER_MISMATCH` | PASSED |
| wrong `aud` | `[wrong-aud]` | `AUDIENCE_MISMATCH` | PASSED |
| unknown `kid` | `[unknown-kid]` | `KID_UNKNOWN` | PASSED |
| known `kid` with the wrong `iss` | `test_ac06_known_kid_with_wrong_issuer` | `ISSUER_MISMATCH` | PASSED |
| expired | `[expired]` | `EXPIRED` | PASSED |
| no `jti` | `[no-jti]` | `CLAIM_MISSING` | PASSED |
| `type` `refresh` | `[type-refresh]` | `TYPE_NOT_ACCESS` | PASSED |
| `typ` missing | `[typ-missing]` | `TYP_NOT_ALLOWED` | PASSED |
| lifetime over 900 s | `[lifetime-over-900]` | `LIFETIME_TOO_LONG` | PASSED |
| `rake:approve` mixed with another scope | `test_dec6_approve_mixed_with_another_scope_is_refused` (401; the mission stays in `STRATEGY_REVIEW`) | `APPROVE_SCOPE_NOT_ALONE` | PASSED |
| second approve call with the same token; the first call succeeded | `test_dec6_second_use_of_an_approve_token_is_refused` (first 200 `APPROVED` and one `consumed_approve_tokens` row; second 401) | `TOKEN_ALREADY_USED` | PASSED |
| 503 when `RAKE_ACCESS_TRUSTED_KEYS` is empty | `test_ac06_no_trusted_key_gives_503` (503, `JWT authority is not configured`) | — | PASSED |

Every invalid 04 vector is also refused for its named reason: `test_rake_access_contract_vectors.py::test_invalid_vector_is_refused[…]`, 15 cases, PASSED.

## Mutation checks (before merge, on the WP-03 branch)

The WP-03 agent session reported two mutants: one that skips the audience check, and one that skips the `consumed_approve_tokens` insert. Each made the tests fail, and the files were restored before the push. Their logs are not in this record, so this section is supporting only. The PASS above rests on the CI run.

## Limits

- The tests run in CI against an in-process app and a test database. No deployed Rake ran these checks. AC-08 covers the deployed services after G4.
- The approve route does not lock the mission row. Two different approve tokens that arrive at the same time can both approve one mission. This predates WP-03.
- No job removes expired rows from `consumed_approve_tokens`.
- rake `render.yaml`, `docker-compose` and `.env.example` still name `JWT_SECRET_KEY`. Rake does not read it after WP-03.
