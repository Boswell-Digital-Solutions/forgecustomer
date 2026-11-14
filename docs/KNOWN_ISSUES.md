# Known Issues

Findings tracked here per the Forge workspace-wide protocol (`/home/charlie/Forge/CLAUDE.md`):
a bug, a root cause, a gap, or an open tracking state goes here, checked first before a finding
is reported as new, and written in the same session it is found.

---

## KI-FOC-20261114-005 — the operator plane failed while readiness stayed green, and the deployed service had no health-check path

**Date found:** 2026-11-14
**Status:** OPEN for the dashboard setting only. The code part merged and deployed:
forgecustomer#63 merged as `3791b2f`, deployed on Render as `dep-dbs6ve2jnfac73bue4j0`, live
14:29:43 UTC. It stays open until the operator sets the Render health-check path.

**What is wrong:** During G5 preparation, the trusted-key JSON was saved into
`ADMIN_JWT_PUBLIC_KEY` instead of into `ADMIN_JWT_TRUSTED_KEYS`. The code on `main` reads
`ADMIN_JWT_PUBLIC_KEY` as a PEM key. It could not parse the value, so every operator route
returned 500 "token verification is not configured". That includes the five emergency stop
routes. `/v1/ready` still returned 200. The operator corrected the value and Render restarted the
service at 13:25 UTC. At 13:35 UTC the operator routes still returned 500. The operator then
started a manual deploy at 13:38 UTC with no environment change (Render event
`dep-dbs68gbl550s73fikbe0`, `envUpdated: false`). After it, a dummy token returned 401 on
`/v1/admin/customers` and `/v1/posture/customers`. So the corrected value was stored before
13:38, but the running process started before it was saved. A value saved with "Save only" does
not reach a running service. The redacted export of the corrected environment
(`render_env_redact.py` in the forge workspace) showed a well-formed key, which agrees.

The Render service also has no health-check path. `render.yaml` names `/v1/ready`, but the
service was created by hand and does not read the Blueprint. Render read the service settings
through its API on 2026-11-14: `healthCheckPath` is empty. So no deploy was ever gated on
readiness.

**Root cause:**
1. Readiness checked only the database. A missing or unparseable operator key was a logged
   state, not a readiness state.
2. No step after a Render change checked the operator plane. The fault was found only because
   the operator shared an environment export.
3. The dashboard setting drifted from `render.yaml`.

**Fix:**
1. forgecustomer#63: `/v1/ready` also checks that the operator trusted-key set loaded and that
   the table `operator_token_replay` exists. A failure returns 503 and names the check only.
   Tests: `ready1`..`ready4` in `api/tests/operator_tokens.rs` and
   `readiness_passes_with_migration_0018_and_fails_without_a_database` in
   `api/tests/operator_replay_pg.rs`.
2. `docs/operations/render_probe.json` holds a credential-free probe. Run it after every Render
   change. The forge workspace holds a runner for every Render service.
3. The operator sets **Settings → Health Check Path** to `/v1/ready` in the Render dashboard.
   Only the operator changes Render settings.

**Update, 2026-11-14 (post-deploy):** forgecustomer#63 merged (`3791b2f`) and deployed
(`dep-dbs6ve2jnfac73bue4j0`, live 14:29:43 UTC). Post-deploy probes: `/v1/ready` returns 200; a
dummy token on `/v1/admin/customers` returns 401; the Render log shows "operator trusted-key set
loaded" with `kid` `98Ue0PgZ`. The runbook's DG-4 check then passed from Forge Command against the
deployed service: the Commerce screen's customer list loaded, and the Business workspace's
Customer details → Load customers loaded a snapshot — neither read failed. This confirms the
operator plane itself works end to end on the new deploy. It does not confirm the health-check
path; that field is set by the operator only, in the Render dashboard, and is not yet done.

**Scope:** open until the dashboard shows `/v1/ready` as the health-check path. The deployed
operator routes return 401 to a dummy token since 13:38 UTC (measured 13:47 UTC), and again after
the #63 deploy at 14:29:43 UTC.

---

## KI-FOC-20261114-003 — the API reaches the database as `postgres`, which bypasses RLS, so RLS does not protect the server-only tables

**Date found:** 2026-11-14
**Status:** OPEN. Raised in review of forgecustomer#63 (review 5904396183, P2-1), carried over from forgecustomer#62. Source reading only.

**What is wrong:** server-only tables enable and force RLS with no policy (for example `0017_rake_access_issuer.sql` and `0018_operator_token_replay.sql`). The API can insert and delete in them only because it connects through the pooler as `postgres`, which bypasses RLS (`KI-FOC-20261113-001`, "The pooler role `postgres` bypasses RLS"). The CI step "Assert RLS enabled on customer-facing tables" checks that RLS is enabled. It does not check which role the API uses.

**Effect:** RLS protects these tables from a customer JWT through PostgREST. It does not limit the API itself. A defect or injection in the API reaches every table with full rights.

**Root cause:** no restricted database role exists for the API.

**Fix:** none in this change. A fix is a restricted API role with explicit grants and policies per table, and a CI check that the API's role is not a superuser and does not have `BYPASSRLS`. That changes the deployed database credential, so it needs an operator decision.

**Scope:** open.

---

## KI-FOC-20261114-004 — G5 review residuals: no lifetime limit, multi-audience tokens accepted, the exemption keyed on an issuer string

**Date found:** 2026-11-14
**Status:** OPEN. Raised in review of forgecustomer#63 (review 5904396183, P3-1, P3-2 and P3-4). Recorded, not changed: each is a security behaviour change, so it waits on the operator.

**What is open:**
- **P3-1, lifetime.** The operator verifier checks `exp` but sets no maximum for `exp` minus `iat`. A token with a 10-year expiry is accepted. The predecessor mints 1-hour tokens (Forge_Command `token_authority.rs`), and the execution-authority path 120 seconds. A per-issuer maximum lifetime would limit an unused stolen token.
- **P3-2, audience.** A token whose `aud` is an array that holds `forgecustomer-admin` verifies on the admin verifier. The exact scope match and the single-use rule (issuer and `jti`) keep the risk low. The verifier could require `aud` to be one string.
- **P3-4, exemption.** `bound` is false only when an entry's issuer is `forge_command_local`. Any key entered with that issuer is unbound until G7. Whoever builds `ADMIN_JWT_TRUSTED_KEYS` (deploy gate DG-2) must check each entry's issuer.

**Scope:** open until the operator rules on P3-1 and P3-2. P3-4 closes at G7.

---

## KI-FOC-20261114-001 — the action-scope names and the `target` claim are defined here; the rebuild's signer must emit them

**Date found:** 2026-11-14
**Status:** OPEN. Found while implementing P3 gate G5 (`BDS-FCO-P3-TOKEN-ISSUER-001`, request section 11).

**What is open:** DEC-G5-2 (b) rules "a scope per action, for example `customers:suspend`, and a target claim". It names neither the scope of each route nor the target claim. The G5 implementation defines both in `api/src/auth/binding.rs`, `ROUTES`: one `<resource>:<verb>` action per operator route (37 routes, the five T-13 stop routes among them), and a claim named `target` that holds the route's path parameters joined with `/`. The name `target` matches the field forge-command's approval and presence code already uses (`crates/fc-authority/src/approval.rs`). The rebuild issues no token yet, so nothing emits these names.

**Effect:** none today. At P3 gate G6, a token from the rebuild that does not use these exact names is refused on every route (fail closed).

**Fix:** none needed in this repository. The rebuild's signer must emit `scope` = the route's action from `ROUTES` and `target` as defined above. The operator confirms the names, or changes them in this table, before G6.

**Limit found in review (forgecustomer#63, review 5904396183, P2-2).** The `target` binds path parameters only. Six mutating routes have none, and carry their target in the request body: `POST /v1/admin/licenses`, `/v1/admin/entitlements/override`, `/v1/admin/usage/adjust`, `/v1/admin/products/authorforge/commercial-policy/versions`, `/v1/admin/releases` and `/v1/admin/update-campaigns`. On these routes a bound token is bound to its **action only**. A bound `entitlements:override` token authorizes an override for any customer and any value. SC-2 stops a replay, but a token taken before its first use can be redirected. The G6 signer contract must decide how to bind these: a digest of the request body, or the key body fields as claims.

**Scope:** open until the operator confirms the names and the body-target binding, or G6 lands with a signer test against this table.

---

## KI-FOC-20261114-002 — the Rake HTTP smoke reused one operator token for several admin calls

**Date found:** 2026-11-14
**Status:** CLOSED 2026-11-14, in the same change.

**What was wrong:** `scripts/rake_access_http_smoke.py` minted one operator token and sent it on five or more `POST /v1/admin/deletion-requests/...` calls. SC-2 (P3 G5) accepts an operator token once, so the second call would get `401`.

**Root cause:** the script predates SC-2. P-1 measured the predecessor's own commands, which mint one token per request; this test client was outside P-1.

**Fix:** the script mints a fresh token for each call, with the header `kid`, `sub` equal to the issuer, and a fresh `jti`. The full smoke passed on 2026-11-14 against a disposable PostgreSQL with every migration applied (`rake access HTTP smoke: PASS`).

**Scope:** closed.

---

## KI-FOC-20261113-001 — the deployed ForgeCustomer database lacked migration 0017, so the Rake exchange and account deletion could not work there

**Date found:** 2026-11-13
**Status:** CLOSED 2026-11-13. Found while preparing AC-08 of plan `BDS-FOC-RAKE-ACCESS-ISSUER-v0.1`. Source read at `origin/main` `9eb2561`.

**What is wrong:** Supabase project `sffgcltrwewnjzvrzjak` (forgecustomer) records migrations `0001` to `0016`. The repository has `0001` to `0017`. `0017_rake_access_issuer.sql` creates `app_clients` (with the `authorforge` row), `rake_tenant_pseudonyms` and `app_token_mints`. A read-only catalog query on 2026-11-13 found none of the three tables in any schema.

**Which database deployed ForgeCustomer uses:** a timing match, not proof. The project's pooler log shows `Connection authenticated` within seconds of each of the last three ForgeCustomer deploys: 01:14:45 → 01:14:54, 01:19:40 → 01:19:51 and 08:11:10 → 08:11:55 UTC on 2026-11-13. Deployed `/v1/ready` returns `ready`. No environment variable was read.

**Effect:**
- `POST /v1/app-tokens/rake` reads `app_clients` after the MFA check. On the deployed service that query fails, so no Rake token can be minted. AC-08 step 2 cannot pass.
- Account deletion: `repositories::privacy::execute_deletion` updates `rake_tenant_pseudonyms` inside the deletion transaction. On the deployed service the execute step fails and the transaction rolls back. It fails closed: no partial deletion. No deletion can complete. On 2026-11-13 `account_deletion_requests` held no rows, so no request is waiting.
- `GET /v1/app-tokens/keys` does not use the database, so it works. That is why the G4 HTTP check passed.

**Root cause:** no deploy step applies migrations. They are applied to the deployed project by hand. G4 (2026-11-12) set keys and variables and deployed the code. Its records do not name `0017`. The G4 database check covered DataForgedb only (`evidence/G4/g4-database-verification.json`).

**Fix:** operator authorization 2026-11-13 ("yes", a reply in the agent session; no signed record). The agent applied `0017_rake_access_issuer.sql` from `main` unchanged (SHA-256 `1496148aa60cfdcb8da3ea312ddff88e367edea8b6b9152c99fbf72b972dca4f`) through the Supabase migration tool.
- Before applying: `customer_profiles` and `installations` exist. The pooler role `postgres` bypasses RLS, and the existing tables already use forced RLS. So the new tables follow the posture the API already works with.
- The tool recorded the migration as version `20261113093444`, name `rake_access_issuer`. Migrations `0001` to `0016` are recorded under their file numbers. The content is the same; only the version label differs.

**Verification (read-only, after applying):** `app_clients`, `rake_tenant_pseudonyms` and `app_token_mints` exist. Each has RLS enabled and forced, with 0 policies. `app_clients` holds `authorforge`: `active`, `authorforge.cloud.enabled`, `rake:read,rake:execute,rake:approve`. The other two tables hold 0 rows. No exchange or deletion ran against the deployed service, so the effect on those routes is not yet observed. AC-08 covers the exchange.

**Scope:** closed 2026-11-13. The migration is recorded in the deployed project and the read-only check shows the three tables.

---

## KI-FOC-20261112-002 — the Rake issuer plan's `plan_set.yaml` gave stale acceptance statuses

**Date found:** 2026-11-12
**Status:** CLOSED in the same change as this entry. Found by the operator's review of forgecustomer #58. Source read at the #58 branch, based on `origin/main` `071000d`.

**What is wrong:** `docs/plans/BDS_FOC_RAKE_ACCESS_ISSUER_v0.1_PLAN_SET/plan_set.yaml` listed AC-05 and AC-06 as `NOT_RUN`, and AC-01 to AC-04 as `PASS_pre_merge`. Components 00 and 07 and the evidence records give AC-01 to AC-03, AC-05 and AC-06 as PASS, with after-merge evidence on `main`. They give AC-04 as PASS except its deletion row, which is UNVERIFIED. The same file gave G3 as PASS, which needs AC-05 and AC-06.

**Effect:** records only. A reader of the YAML alone saw an earlier plan state.

**Root cause:** forgecustomer #56 updated the work-package and gate lines of `plan_set.yaml`, and did not update the `acceptance_criteria` line.

**Fix:** the line now reads `AC-01: PASS`, `AC-02: PASS`, `AC-03: PASS`, `AC-04: PASS_except_deletion_row_unverified`, `AC-05: PASS` and `AC-06: PASS`. AC-07 to AC-09 stay `NOT_RUN`. AC-04 changes again only when the deletion test (forgecustomer #57) passes on `main`.

**Scope:** closed when the change that adds this entry merges.

---

## KI-FOC-20261112-001 — doc/system §13 did not list the Rake access HTTP smoke job

**Date found:** 2026-11-12
**Status:** CLOSED in the same change as this entry. Found while adding the account-deletion check to `scripts/rake_access_http_smoke.py` for plan `BDS-FOC-RAKE-ACCESS-ISSUER-v0.1` gate G2. Source read at `origin/main` `071000d`.

**What is wrong:** `.github/workflows/ci.yml` has the job `rake-access-http-smoke` since WP-02. `doc/system/13-verification-status.md` did not name it in the CI class table or in the list of DB-backed checks.

**Effect:** documentation only. A reader of §13 could not see that the Rake token exchange has a live-PostgreSQL check, or that a documentation-only change skips it.

**Root cause:** not established. WP-02 added the job and did not update §13.

**Fix:** §13 now names the job in the class table and describes what it checks, including the deletion run. `bash doc/system/BUILD.sh` rebuilt `doc/FOCSYSTEM.md`.

**Scope:** closed when the change that adds this entry merges.

---

## KI-FOC-20261111-001 — doc/system §4 listed migrations only through 0011

**Date found:** 2026-11-11
**Status:** CLOSED in the same change as this entry. Found while adding migration `0017` for plan `BDS-FOC-RAKE-ACCESS-ISSUER-v0.1` WP-02. Source read at the WP-02 branch base (forgecustomer #53 head `ebd1dfc`).

**What is wrong:** the "Migration domains" table in `doc/system/04-data-model.md` ended at `0011_fleet_release_update_domain.sql`. Migrations `0012` to `0016` exist in `supabase/migrations/` and were not listed.

**Effect:** documentation only. A reader of §4 could miss the MFA columns, `customer_mfa_history`, `commercial_policy_versions` and `update_outcome_tickets`.

**Root cause:** not established. The five migrations did not update §4.

**Fix:** §4 now lists `0012` to `0017`, each with its domain and tables, read from each migration's header. `bash doc/system/BUILD.sh` rebuilt `doc/FOCSYSTEM.md`.

**Scope:** closed when the change that adds this entry merges.

---

## KI-FOC-20261109-004 — doc/system §10 says admin routes use a shared secret; the code verifies EdDSA tokens with a public key

**Date found:** 2026-11-09
**Status:** CLOSED in the same change as this entry. Found by the Rake credential-authority research (forge `docs/architecture/rake_credential_authority_review.md` follow-up). Source read at `origin/main` `4bbe968`.

**What is wrong:** `doc/system/10-authority-boundaries.md` said "Admin routes use a separate issuer, audience, and secret." `api/src/state.rs:94-98` builds the admin validator with `JwtValidator::eddsa` from `ADMIN_JWT_PUBLIC_KEY`, `ADMIN_JWT_ISSUER` and `ADMIN_JWT_AUDIENCE` (`api/src/config.rs:197-199`). No admin secret exists. `doc/system/03-api-contract.md:141-145` already states this correctly.

**Effect:** documentation only. A reader of §10 could conclude that admin authentication uses a shared secret, and could plan a secret rotation that does not apply.

**Root cause:** not established. §10 was not updated when §3 was.

**Fix:** §10 now states the EdDSA public-key verification. `bash doc/system/BUILD.sh` rebuilt `doc/FOCSYSTEM.md`.

**Scope:** closed when the change that adds this entry merges.

---

## KI-FOC-20261109-003 — `cargo audit` fails on RUSTSEC-2026-0459 for `libsqlite3-sys`, a lockfile-only crate

**Date found:** 2026-11-09
**Status:** OPEN. Accepted residual risk once the waiver merges. Not resolved: the vulnerable version stays in `Cargo.lock`. Found when the dependency audit failed on documentation PR #47.

**What is wrong:** `cargo audit` reports RUSTSEC-2026-0459, `libsqlite3-sys` via C SQLite CVE-2026-119375, fixed in `>=0.39.0`. `Cargo.lock` records `libsqlite3-sys` 0.30.1. The scheduled dependency audit on `main` fails at `d8a97cf` with the same advisory, so the failure is not caused by any pull request.

**Effect:** none on the service, measured. `api/Cargo.toml:26` enables sqlx with `default-features = false` and the `postgres` feature, not `sqlite`. `cargo tree -i libsqlite3-sys --manifest-path api/Cargo.toml -e normal` prints nothing: no build compiles the crate. The lockfile records it because a lockfile covers every feature combination. Every pull request's dependency audit fails until this closes, so no pull request passes an all-green merge gate.

**Root cause:** sqlx 0.8 pins `libsqlite3-sys` below 0.39 for its optional `sqlite` feature.

**Fix:** none applied. Two options, both need the operator: (a) a waiver in `.cargo/audit.toml` with this justification, by the precedent of the RUSTSEC-2023-0071 `rsa` waiver; (b) an upgrade of sqlx to a release that allows `libsqlite3-sys >=0.39.0`.

**Re-check 2026-11-09, option (b):** the operator chose the upgrade. It is blocked upstream. The newest sqlx release is 0.9.0 (2026-05-21). Its `sqlx-sqlite` 0.9.0 requires `libsqlite3-sys >=0.30.1, <0.38.0` (crates.io dependency data). The advisory is fixed only in `>=0.39.0`. An upgrade to sqlx 0.9 leaves a vulnerable `libsqlite3-sys` in `Cargo.lock`, and a `[patch]` cannot replace a version outside that range. Fix: none applied. Option (b) becomes possible when a sqlx release allows `libsqlite3-sys >=0.39.0`.

**Waiver 2026-11-09, option (a):** the operator authorized a reviewed PR that adds RUSTSEC-2026-0459 to the ignore list of `.cargo/audit.toml`, and nothing else ("APPROVED: Prepare a narrowly scoped dependency-audit waiver for RUSTSEC-2026-0459 in ForgeCustomer"). The authorization does not include a merge. The waiver records the path, the evidence, the upstream block and three removal conditions; `docs/SECURITY.md` lists it as accepted residual risk. The RUSTSEC-2023-0071 waiver is unchanged. The waiver is accepted residual risk from the operator's merge; it does not fix the vulnerability.

**Enforcement 2026-11-09:** the operator authorized a separate PR that adds a fail-closed reachability check. The CI `audit` job runs `scripts/check-waived-crates.sh` and its tests before `cargo audit`. It fails when the crate is reachable, has left `Cargo.lock`, or when `cargo tree` or its positive control fails. Verified locally against real cargo: with the sqlx `sqlite` feature enabled the check fails, under `--locked` and with a regenerated lockfile alike.

**Scope:** open. The waiver does not close this entry. It closes when `libsqlite3-sys` leaves `Cargo.lock` or `Cargo.lock` holds `libsqlite3-sys >=0.39.0`, and the waiver is then removed. If `cargo tree -i libsqlite3-sys --workspace --all-features --target all -e all` prints anything first, the crate is reachable through the resolved dependency graph, the exclusion evidence is invalid, and the waiver must be removed at once, pending a security reassessment.

---

## KI-FOC-20261109-002 — an operator token's `roles` claim overrides its `scope`, so a scope cannot narrow an operator token

**Date found:** 2026-11-09
**Status:** OPEN. Found while the forge-command G5 authorization request was amended (`BDS-FCO-P3-TOKEN-ISSUER-001`, `07_G5_AUTHORIZATION_REQUEST.md`).

**What is wrong:** `Claims::operator_roles()` returns the token's `roles` claim when it is not empty, and reads `scope` only when `roles` is empty (`api/src/auth/mod.rs:59-62`, verified at `d8a97cf`). `roles` is read from the token with `#[serde(default)]` (`api/src/auth/mod.rs:35-36`). A token signed by a trusted operator key that carries `"roles": ["admin"]` therefore gets the `admin` role, whatever its `scope` says. Each other scope token becomes a role of the same name (`api/src/auth/mod.rs:63-67`), so a scope such as `customers:suspend` does not pass `require_role("admin")`.

**Effect today:** none observed. The only trusted operator key belongs to the predecessor Forge Command, and it sends `scope` and no `roles`. A token with `roles` needs that key to sign it.

**Effect on G5:** an action-bound scope (SC-1 of the forge-command plan) is not a limit while `roles` can override it. A token can carry a narrow `scope` and a wide `roles` claim together.

**Root cause:** the `roles` path exists for a token form that no current operator issuer sends. `scope` was added beside it as a bridge and does not replace it.

**Fix (2026-11-14, P3 G5 implementation, authorized by the operator 2026-11-14 "Authorize G5 only"):** for a route-bound key, a token that carries a `roles` claim at all is refused, on the admin and the posture validator, and the admin role is granted only after the token's `scope` matches the route's action and its `target` matches the route (`api/src/auth/binding.rs`, `check_bound`; tests `sc1r_scope_admin_fails_a_bound_route_and_any_roles_claim_fails_every_route` and `auth::binding::tests::any_roles_claim_fails`). `roles` becomes `Option<Vec<String>>` so that an empty array is still a claim that is present.

**Scope:** closed for route-bound keys. **Open for the predecessor's key** (issuer `forge_command_local`), which DEC-G5-2 (b) leaves unbound until P3 gate G7: for that key `roles` still overrides `scope`. It closes in full when G7 removes the predecessor's key.

---

## KI-FOC-20261109-001 — two admin commercial-policy route handlers call `expect()` in a production path

**Date found:** 2026-11-09
**Status:** OPEN. Recorded by forge BDS-FGITR-v0.1 (governance truth audit).

**What is wrong:** `CLAUDE.md:41` bans `unwrap()` and `expect()` in production paths. Two route handlers in `api/src/routes/mod.rs` call `serde_json::to_value(policy).expect("commercial policy is serializable")`, verified at `7cd8a13`. They are `admin_authorforge_commercial_policy` (`api/src/routes/mod.rs:1824`) and `admin_authorforge_commercial_policy_publish` (`api/src/routes/mod.rs:1866`). The router registers both handlers (`api/src/routes/mod.rs:151` and `:155`). No lint enforces the rule. No `clippy::unwrap_used` or `clippy::expect_used` setting exists in the repository. CI runs `cargo clippy --all-targets --all-features -- -D warnings` with the default lint set (`.github/workflows/ci.yml:60`).

**Effect:** if the serialization fails, the handler panics instead of returning an `AppError` response.

**Root cause:** Not established.

**Fix:** None applied.

**Scope:** Open.

---

## KI-FOC-20261107-001 — the Supabase organization is on the Free plan, so the forgecustomer database has no automatic backups

**Date found:** 2026-11-07
**Status:** open. Not fixed. Found by BDS Client Intake plan `BDS-CLIENT-INTAKE-v0.3` WP-00 (evidence D4). This is an operator plan item; no code is wrong.

**What is wrong:** a read-only Supabase `get_organization` on 2026-11-07 shows organization `qlikqkbebkhmaufcvnde` on the **Free** plan. It holds two active projects: `forgecustomer` (`sffgcltrwewnjzvrzjak`) and `DataForgedb` (`embvfponjxejbtrkryzs`). Both active-project slots are used. The Free plan includes no automatic backups.

**Effect:** customer, licensing, entitlement and installation records have no recovery copy. A bad migration, an accidental delete or a project loss cannot be undone from Supabase. The same applies to `DataForgedb`.

**Root cause:** the plan is a Supabase organization setting. It is not in this repository.

**Fix:** none made. Changing the plan is the operator's act, and it changes cost. The Pro plan adds daily backups. A free alternative is a scheduled `supabase db dump` to private storage; the operator did not choose it.

**Operator ruling (2026-11-07):** the organization moves to Pro in the week of the release ("We need to just plan on pro for week of release checklist"). It stays on Free until then.

**Scope:** open. It must be added to the pre-release checklist next to O-04. The entry closes when the organization is on Pro and a first backup of both projects is visible in the Supabase dashboard.

**Re-check 2026-11-09:** the pre-release checklist still has no Supabase item. Recorded by forge BDS-FGITR-v0.1 (governance truth audit). `Author-Forge@f3a39fa2:docs/plans/BDS_AF_PROTIER_v0.1_PLAN_SET/PRE_RELEASE_CHECKLIST.md` has O-04a (line 69), O-04b (line 70) and O-05 (line 72). None of them names Supabase or backups. The item that this scope requires is not added yet. Fix: none applied. Scope: open.

---

## KI-FOC-20261106-001 — the deployed service runs on Render's Free instance type, so the first call after idle waits for a cold start

**Date found:** 2026-11-06
**Status:** open. Not fixed. Found by Author-Forge plan `BDS-AF-PROTIER-v0.1` WP-00 (evidence/AC-11). This is an operator configuration item; no code is wrong.

**What is wrong:** a read-only Render `list_services` on 2026-11-06 shows the `Forge Customer` web service (`forge-customer`, region oregon) on instance type **free** (build plan starter). Every other deployed BDS service in the workspace is on starter. The first unauthenticated `GET /v1/version` took 11.39 s, and the next call took 0.29 s.

**Effect:** the Free instance type spins down when idle (inference: consistent with Render's documented Free behavior and the timing above). Every AuthorForge Pro desktop gets its entitlement snapshot from this service, and the Pro tier plan adds a short-lived caller token from it. So the first Pro call after an idle period waits for the service to start. A client with a short timeout can report ForgeCustomer as unreachable.

**Root cause:** the instance type is a Render setting. It is not in this repository; `render.yaml` does not pin it.

**Fix:** none made. Changing the instance type is the operator's act, and it changes cost. Alternatives: move to an instance type that does not spin down, or make every client treat a cold start as a named waiting state with a long enough timeout. Author-Forge pre-release checklist item O-04 tracks it.

**Operator ruling (2026-11-06):** the service stays on the Free instance type until the inaugural release, to save money ("it is free to save money until we are ready for inaugrial release"). At that release it moves to an instance type that does not spin down.

**Scope:** open. Before the release, clients must handle a cold start (Author-Forge checklist O-04a). The entry closes when the instance type changes at the inaugural release (checklist O-04b).

---

## KI-FOC-20261105-001 — the entitlement key ring holds only the active key, so the documented rotation overlap does not exist

**Date found:** 2026-11-05
**Status:** open. Not fixed. Found by a read-only source lock for Author-Forge plan `BDS-AF-RELACC-v0.1` (WP-00).

**What is wrong:** `doc/system/11` says that "Key rotation requires an overlap window where old and new public keys both verify existing snapshots". `docs/ENTITLEMENTS.md` and the doc comment on `routes/entitlements.rs::keys` say the same. The code builds no overlap:
- `AppState` creates the ring with `VerifyingKeyRing::new()` and then `add_signer(&signer)` only (`api/src/state.rs:45-46`).
- `VerifyingKeyRing::add_base64`, the only way to add a retained key, has no caller anywhere in `api/` (`grep -rn add_base64 api` finds only its definition, `signing.rs:90`).
- No configuration variable names a previous or retired verification key (`api/src/config.rs`, `doc/system/12`).

So `GET /v1/entitlements/keys` publishes exactly one key.

**Effect** (corrected 2026-11-06 after review; the first wording overstated it): after `ENTITLEMENT_SIGNING_PRIVATE_KEY` changes, ForgeCustomer signs new snapshots and leases with the new key and publishes only that key. The effect depends on each verifier's trust store:
- a client that trusts only the old key refuses every newly signed document. A client that pins keys at build time, which is the Author-Forge plan for its desktop shell, loses cloud mode at its next refresh until it gets a build with the new key;
- a verifier that refetches the published list drops the old key, so it refuses documents the old key signed, including unexpired 14-day offline leases;
- a document already signed stays valid wherever the old key is still trusted. ForgeCustomer verifies none of its own documents at run time.

So a key change does not invalidate every document at once. It breaks compatibility in both directions for any verifier that holds only one of the two keys.

**Root cause:** the ring type supports several keys, but no configuration path loads a retained key into the deployed ring.

**Fix:** none made. A candidate is an optional configuration value that lists retained `key_id` to public-key pairs, loaded with `add_base64` and published by `keys`, plus a test that a snapshot signed by the retired key still verifies. Changing deployed configuration is the operator's act.

**Scope:** open.

---

## KI-FOC-20261105-002 — a deactivated installation still receives an installation-bound entitlement snapshot

**Date found:** 2026-11-05
**Status:** open. Not fixed. Same source lock as KI-FOC-20261105-001.

**What is wrong:** `GET /v1/entitlements/current?installation_id=` binds the snapshot through `repositories::entitlements::find_owned_installation` (`api/src/repositories/entitlements.rs:499-513`). That query selects by installation id and customer only. It does not check `installations.status`. `docs/LICENSING.md` treats a deactivated installation as one that must register again before activation, and offline-lease issuance refuses it (`LeaseError::InstallationDeactivated`). Snapshot issuance does not.

**Effect:** a client of a deactivated installation can still obtain a signed snapshot that names that installation. Cloud features still follow the customer's subscription. The installation binding therefore does not reflect the deactivation.

**Root cause:** the snapshot path and the lease path check installation state differently.

**Fix:** none made. A candidate is to refuse a non-`active` installation in `find_owned_installation` for the snapshot route, matching the lease path, with a route test.

**Scope:** open. Severity is low: the customer is still authenticated, and the entitlements are the customer's own.

---

## KI-FOC-20261011-001 — Supabase advisors report RLS plan and search-path warnings on the forgecustomer project

**Date found:** 2026-10-11
**Status:** open. Not fixed. No migration drafted. The live state was not re-observed by an agent.

**What is wrong:** The operator's Supabase dashboard (Advisors, project `forgecustomer`, `main`, production) lists two warning groups. A screenshot shows the list. It is partial, so the full list is unknown.
- *Auth RLS Initialization Plan:* `public.customer_profiles`, `public.hud_thread` (three entries), `public.hud_message` (two entries).
- *Function Search Path Mutable:* `public.set_updated_at`, `public.reject_mutation`, `public.reject_commercial_policy_version_mutation`, `public.hud_current_thread`, `public.hud_post_message`.

**What the source shows (commit `5f2459b`):**
- The policy `p_profiles_read_own` on `customer_profiles` calls `auth.uid()` directly (`0009_rls.sql:53`). This matches the first warning.
- `set_updated_at` (`0001_customer_identity.sql:9`), `reject_mutation` (`0010_seed_constraints.sql:11`) and `reject_commercial_policy_version_mutation` (`0015_authorforge_commercial_policy.sql:55`) set no `search_path`. This matches the second warning.
- `current_customer_id()` already sets `search_path = public` (`0009_rls.sql:12`). A claim that every function is unpinned is wrong.
- No migration in this repository defines `hud_thread`, `hud_message`, `hud_current_thread` or `hud_post_message`. Another source created them. The owner is not yet identified. `bds_website` is a likely candidate, but this is unverified.

**Not known:** the severity and exploitability of each warning; the effective grants on the functions; the live migration state; the owner of the `hud_*` objects.

**Root cause:** Not established for the `hud_*` objects. For the `customer_profiles` policy and the three trigger functions, the migrations were written without the scalar-subquery form and without a pinned search path.

**Fix:** None made. A candidate fix must be a new forward migration. Do not rewrite `0001` to `0016`. Do not apply SQL in the dashboard without a migration. Replay the migration in an isolated fixture first. Run `supabase/tests/rls_customer_write_denial.sql` and the append-only checks. Prove that the allowed identities are the same before and after. Treat the `hud_*` objects as a separate item until their owner is known.

**Scope:** open. This entry records the finding only. The plan packet `BDS-FC-DB-VISIBILITY-v0.1` in `Forge_Command` holds the repair design (slices W2 and W3, both held).

---

## KI-FOC-20261002-001 — A documentation-only push still redeploys on Render

**Date found:** 2026-10-02
**Status: OBSERVED 2026-10-12: the mitigation in `render.yaml` did NOT prevent a deploy of the running service. Open. The fix is a Render dashboard action.**

**What is wrong:** The 2026-10-01 rule says a documentation-only change runs no code CI.
`ci.yml` now skips its code jobs for such a change. `render.yaml` sets no `autoDeploy` or
`buildFilter` key, so Render still builds and deploys on every push to the tracked branch.

**Root cause:** Render deploy settings live in `render.yaml` and the Render dashboard, not in
a workflow. The rule change touched only GitHub Actions.

**Fix:** Done before go-live. The `forgecustomer-api` service in `render.yaml` has a `buildFilter`
with `ignoredPaths` for root `*.md`, `doc/**` and `docs/**`. The Docker build copies only
`Cargo.toml`, `Cargo.lock` and `api/`, so documentation changes nothing in the image.
The filter uses `ignoredPaths`, not `paths`, so an unknown new path still builds. A service that
someone made by hand in the Render dashboard ignores `render.yaml`. Check that the Blueprint manages
this service. Do not call the fix proven until a real documentation-only push skips the build.

**Observed 2026-10-12.** Pull request #34 (merge commit `215619b2`) changed only `docs/KNOWN_ISSUES.md`. It was merged at 00:17:33Z. A Claude Chat session reading Render (read-only; this session did not read Render) reported that the running service "Forge Customer" started a deploy of commit `215619b` at 00:17:35Z, two seconds later, and that it went live at 00:20:15Z. The `buildFilter` in `render.yaml` ignores `docs/**`, so a correct filter would have skipped this build. It did not.

**Likely cause (an inference, not verified):** the running service is not managed by the blueprint. The operator states that the current services were created in a Render Project ("Boswell Digital Solutions") and that the older services outside it are suspended. A service made outside the blueprint ignores `render.yaml`. This entry already warned about that case.

**Effect:** Forge Customer was redeployed with unchanged application code. The Chat report says the API logged `starting` at 00:20:08Z with `env: development`. That value was not checked. Any documentation merge to `main` redeploys this service until the Render setting is fixed.

**Fix needed (operator, in the Render dashboard):** set a build filter on the running service that ignores `*.md`, `doc/**` and `docs/**`, or turn off auto-deploy for it. Then re-test with a documentation-only merge and confirm that no deploy starts. The `render.yaml` change alone is not enough.

**Scope:** open until a documentation-only merge is seen to skip the Render build on the running service. Merging a documentation change to `main` before then redeploys it.

---

## KI-FOC-20260924-001 — `cargo audit` fails on `main`: RUSTSEC-2026-0285 in `rustls` 0.23.40

**Date found:** 2026-09-24
**Status:** closed (fixed in the same change)

**What is wrong:** The CI `dependency audit` job fails on the `main` lockfile. `Cargo.lock`
locks `rustls` 0.23.40. RUSTSEC-2026-0285 applies to that version: "TLS 1.3 handshake messages
incorrectly accepted across encryption level boundaries" (dated 2026-09-14, severity 5.3,
medium). The advisory's solution is `rustls` 0.23.45 or later. forgecustomer#27 found the
failure. That PR does not change `Cargo.lock`.

**Root cause:** The advisory entered the RustSec database after the last CI run on `main`
(2026-09-14 07:29 UTC). That run passed with the same lockfile. No push to `main` has run CI
since then, so no check failed until the next pull request. KI-FOC-20260924-002 records this
gap.

**Fix:** `cargo update -p rustls` moves `rustls` to 0.23.45 and `rustls-webpki` to 0.103.15.
Both are semver-compatible patch releases. No other package changes. `cargo audit` passes, and
its four allowed warnings do not change.

**Scope:** closed. The change is `Cargo.lock` only. `.cargo/audit.toml` gets no new waiver.

---

## KI-FOC-20260924-002 — `cargo audit` runs only on push and pull request

**Date found:** 2026-09-24
**Status:** closed (fixed 2026-09-24 in a later change)

**What is wrong:** `.github/workflows/ci.yml` runs the `audit` job only on `push` and
`pull_request`. A new advisory against a locked dependency fails no check until the next push.
RUSTSEC-2026-0285 (KI-FOC-20260924-001) applied to `main` for ten days before a pull request
found it. The repository has no Dependabot configuration.

**Root cause:** The workflow has no `schedule` trigger. `cargo audit` reads the advisory
database at run time, so its result can change when the database changes, with no change to
the code.

**Fix:** `.github/workflows/dependency-audit.yml` runs `cargo audit` on `main` every day at
06:17 UTC. It also runs on manual dispatch and on a pull request that changes the workflow
file, so an edit to the workflow is tested before it merges. GitHub sends the notice of a
failed scheduled run to the user who last changed the cron schedule. The `audit` job in
`ci.yml` does not change.

**Scope:** closed. The change adds one workflow file and updates `docs/SECURITY.md`,
`docs/IMPLEMENTATION_STATUS.md`, and `doc/system` §13.
