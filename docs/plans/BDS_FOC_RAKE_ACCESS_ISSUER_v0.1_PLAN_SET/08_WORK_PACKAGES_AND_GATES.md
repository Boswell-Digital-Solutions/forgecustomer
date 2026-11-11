# 08 — Work packages and gates

This component defines each work package with its nine-field brief, and the gates between them.

## Gates

| Gate | Passes when | Unblocks |
|---|---|---|
| G0 | AC-00 PASS | WP-01 |
| G1 | WP-01 merged with AC-01 PASS, and DEC-2 to DEC-5 ruled | WP-02, WP-03, WP-04 |
| G2 | WP-02 merged with AC-02 to AC-04 PASS | G4 |
| G3 | WP-03 merged with AC-05 and AC-06 PASS | G4 |
| G4 | The operator creates the Rake access keys, sets the ForgeCustomer and Rake variables, and deploys both | WP-05 |
| G5 | AC-08 PASS | plan `implemented_unverified` → `complete` after AC-09 |
| G6 | AUTHORITY_GAP-01 closed by its own plans, and WP-04 merged with AC-07 PASS | Author-Forge hosted Rake in cloud mode |

Order: G0 → G1 → {G2, G3} → G4 → G5. G6 is independent of G5 but needs G2 and G3.

WP-03 removes Rake's HS256 path. No known holder of a Rake JWT exists (review §2.2), so the merge breaks no caller.

## WP-00 — Source lock

**Status:** delivered 2026-11-11. AC-00 PASS (`evidence/AC-00/`). G0 PASS.

| Field | Brief |
|---|---|
| Outcome | The 00 pins are confirmed or updated, so 04 and 05 rest on current code. |
| Starting evidence | 00 source table. |
| Prerequisites | DEC-1. |
| Permitted scope | Read-only in every repository. Writes only `evidence/AC-00/` in this plan set. |
| Boundaries | No deployed service call. No secret read. |
| Deliverables | `evidence/AC-00/SOURCE_LOCK.md`, `evidence/AC-00/RESULT.md`. |
| Acceptance criteria | AC-00. |
| Verification | 07 AC-00. |
| Stop conditions | Done when AC-00 is recorded. Pause if a pinned file changed in a way that breaks 04 or 05. Then record the change and revise the plan first. |

## WP-01 — Token contract

**Status:** delivered for review 2026-11-11. AC-01 PASS on the candidate (`evidence/AC-01/`). Its merge needs the operator's word, because it adds a test.

| Field | Brief |
|---|---|
| Outcome | 04 exists as machine-checkable files. |
| Starting evidence | 04; forgecustomer `contracts/openapi.yaml` at the WP-00 pin. |
| Prerequisites | G0; DEC-1. DEC-2 to DEC-5 should be ruled first. If they are not, the vectors use the recommended values and the WP stays open. |
| Permitted scope | forgecustomer `contracts/`, one test file under `api/tests/`, this plan set, `doc/system/03` and `11`. |
| Boundaries | No route, no migration, no key. Test keys only. |
| Deliverables | The files of 04 §6 and the AC-01 test. |
| Acceptance criteria | AC-01, AC-09. |
| Verification | 07 AC-01; forgecustomer CI. |
| Stop conditions | Done when merged with AC-01 PASS. Pause if a decision ruling changes a claim. Then revise 04 first. |

## WP-02 — Issuer in ForgeCustomer

| Field | Brief |
|---|---|
| Outcome | 05 §2 works in ForgeCustomer. |
| Starting evidence | The 03 reuse matrix rows for ForgeCustomer. |
| Prerequisites | G1. |
| Permitted scope | forgecustomer `api/src/`, `supabase/migrations/` (three tables of 04 §5), `supabase/seed.sql`, `.env.example` (names only), `contracts/`, `doc/system/03`, `04`, `10`, `11`, `12`, `docs/KNOWN_ISSUES.md`. |
| Boundaries | No deployed configuration, key or secret. No change to the entitlement key or its route. No change to customer or admin token rules. |
| Deliverables | The route, the keys route, the `contracts/openapi.yaml` entries for both (moved from WP-01), the migration, the deletion extension, the tests, the doc/system updates (closes AUTHORITY_GAP-02). |
| Acceptance criteria | AC-02, AC-03, AC-04, AC-09. |
| Verification | 07; forgecustomer CI including `cargo audit`. |
| Stop conditions | Done when merged on the operator's word with the ACs PASS. Pause if the entitlement service cannot answer the feature check without a new authority. |

## WP-03 — Verifier in Rake

| Field | Brief |
|---|---|
| Outcome | 05 §3 works in Rake, and the HS256 path is gone. |
| Starting evidence | rake `auth/jwt_handler.py`, `auth/execution_access.py:9-88`, `auth/tenant_context.py`, `auth/__init__.py`, `config.py:282-304`, `config.py:659-681`, `forge_keys.py:56-57`, `tests/conftest.py:35`, `:167`, `service_contract.v1.json`, `render.yaml:44-45`. |
| Prerequisites | G1. The contract vectors from WP-01. |
| Permitted scope | Those files, rake tests, rake `doc/system/` parts that name the JWT, `docs/KNOWN_ISSUES.md`. `render.yaml` may drop `JWT_SECRET_KEY` and declare `RAKE_ACCESS_TRUSTED_KEYS` without a value. |
| Boundaries | No deployed configuration change. The API-key path is unchanged. Scope and tenant binding are unchanged. |
| Deliverables | The verifier, the tests, the contract update, the doc/system updates. |
| Acceptance criteria | AC-05, AC-06, AC-09. |
| Verification | 07; rake CI (all required jobs). |
| Stop conditions | Done when merged on the operator's word. Pause if a non-test caller of `create_access_token` appears. |

## WP-04 — Author-Forge client

| Field | Brief |
|---|---|
| Outcome | 05 §4.1 works in `apps/api`. |
| Starting evidence | Author-Forge `apps/api/src/routes/rake.ts`, `apps/api/tests/routes/rake.test.ts`, `KI-AUF-20261106-003`. |
| Prerequisites | G1. If DEC-2 = A, AUTHORITY_GAP-03 work in `src-tauri` and in WP-02. |
| Permitted scope | `apps/api/src/routes/rake.ts`, a new exchange client module under `apps/api/src/services/`, tests, Author-Forge `doc/system/` part for the Rake proxy, `docs/KNOWN_ISSUES.md`. Use the `afma01` worktree, never `git checkout -b` in the shared tree. |
| Boundaries | No change to the request body, path handling or project id. Those belong to AUTHORITY_GAP-01. No WebView change. |
| Deliverables | The client, the tests, the doc update. |
| Acceptance criteria | AC-07, AC-09. |
| Verification | 07; Author-Forge CI. |
| Stop conditions | Done when merged with AC-07 PASS. Activation in cloud mode waits for G6. |

## WP-05 — Acceptance on deployed services

| Field | Brief |
|---|---|
| Outcome | DEC-RAKE-JWT-01's positive path is proven with a token from the ruled issuer. |
| Starting evidence | G4 receipts from the operator. |
| Prerequisites | G2, G3, G4. A test account with and one without the Pro entitlement. |
| Permitted scope | Calls to deployed ForgeCustomer and Rake as in 05 §6. Writes `evidence/AC-08/` and rake `docs/evidence/DEC-RAKE-JWT-01/`. |
| Boundaries | No configuration change. No token value in any record. |
| Deliverables | The AC-08 receipt; the rake KI update; the lifecycle change proposal. |
| Acceptance criteria | AC-08, AC-09. |
| Verification | 07 AC-08. |
| Stop conditions | Done when AC-08 is PASS or FAIL with evidence. On FAIL, record the cause and stop. |
