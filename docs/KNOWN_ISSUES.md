# Known Issues

Findings tracked here per the Forge workspace-wide protocol (`/home/charlie/Forge/CLAUDE.md`):
a bug, a root cause, a gap, or an open tracking state goes here, checked first before a finding
is reported as new, and written in the same session it is found.

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
**Status:** mitigated in `render.yaml` (2026-10-02), not yet observed. The Render dashboard setting is unchecked.

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

**Scope:** open until a documentation-only push is seen to skip the Render build and the dashboard setting is checked.

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
