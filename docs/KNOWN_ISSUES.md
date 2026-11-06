# Known Issues

Findings tracked here per the Forge workspace-wide protocol (`/home/charlie/Forge/CLAUDE.md`):
a bug, a root cause, a gap, or an open tracking state goes here, checked first before a finding
is reported as new, and written in the same session it is found.

---

## KI-FOC-20261106-001 — the deployed service runs on Render's Free instance type, so the first call after idle waits for a cold start

**Date found:** 2026-11-06
**Status:** open. Not fixed. Found by Author-Forge plan `BDS-AF-PROTIER-v0.1` WP-00 (evidence/AC-11). This is an operator configuration item; no code is wrong.

**What is wrong:** a read-only Render `list_services` on 2026-11-06 shows the `Forge Customer` web service (`forge-customer`, region oregon) on instance type **free** (build plan starter). Every other deployed BDS service in the workspace is on starter. The first unauthenticated `GET /v1/version` took 11.39 s, and the next call took 0.29 s.

**Effect:** the Free instance type spins down when idle (inference: consistent with Render's documented Free behavior and the timing above). Every AuthorForge Pro desktop gets its entitlement snapshot from this service, and the Pro tier plan adds a short-lived caller token from it. So the first Pro call after an idle period waits for the service to start. A client with a short timeout can report ForgeCustomer as unreachable.

**Root cause:** the instance type is a Render setting. It is not in this repository; `render.yaml` does not pin it.

**Fix:** none made. Changing the instance type is the operator's act, and it changes cost. Alternatives: move to an instance type that does not spin down, or make every client treat a cold start as a named waiting state with a long enough timeout. Author-Forge pre-release checklist item O-04 tracks it.

**Scope:** open until the operator rules on the instance type or the clients are proven to handle a cold start.

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
