# 08 — Work packages and gates. All PROPOSED.

**Shared rules for every WP:**
- Reuse first (R01).
- Pin the SHAs, and work in a worktree.
- Use each repository's own gates and update its `doc/system/` with `BUILD.sh`.
- Write findings to that repository's `docs/KNOWN_ISSUES.md`.
- Use no real key and no deployed-service write.
- Do not change deployed configuration (R08).
- Open a PR and hand off.
- Merging a ForgeCustomer PR redeploys Render (KI-FOC-20261002-001), so a merge needs the operator's word.

**WP-00 Source lock**
- **Outcome:** confirm every inspected fact in 00 and 03 at current SHAs. Read the live `key_id` from the public `GET /v1/entitlements/keys`, if Decision 1 allows.
- **Starting evidence:** the 00 table.
- **Prerequisites:** Decision 1.
- **Permitted scope:** read-only in the five repositories; writes to `evidence/AC-00/`.
- **Boundaries:** no authenticated call.
- **Deliverables:** `SOURCE_LOCK.md`.
- **Acceptance criteria:** AC-00.
- **Verification:** every row cites a file and SHA.
- **Stop conditions:** pause if the reuse matrix is wrong on any row.

**WP-01 Overlap wiring (ForgeCustomer)**
- **Outcome:** 05 §A (REQ-01).
- **Starting evidence:** `state.rs:45-46`, `signing.rs` `add_base64`, `config.rs` `optional`.
- **Prerequisites:** G0.
- **Permitted scope:** forgecustomer `api/src/{config,state}.rs`, tests, `doc/system/12` (new variable), `doc/system/11` (overlap now real), `docs/ENTITLEMENTS.md`, `docs/KNOWN_ISSUES.md`.
- **Boundaries:** no crypto change; no wire-format change (R06).
- **Deliverables:** code, tests, docs, KI-FOC-20261105-001 overlap half closed on merge.
- **Acceptance criteria:** AC-01, AC-02.
- **Verification:** ForgeCustomer CI and the Author-Forge fixture test.
- **Stop conditions:** done on AC-01 and AC-02.

**WP-02 Registry and statuses (ForgeCustomer)**
- **Outcome:** 05 §B (REQ-02, REQ-03).
- **Starting evidence:** DataForge `verification_evidence_router.py`, its alembic migration and `doc/system/19`.
- **Prerequisites:** G1; Decision 2.
- **Permitted scope:** forgecustomer migration `0017_*`, repository, routes, tests, `contracts/openapi.yaml`, `doc/system/`.
- **Boundaries:** additive migration only; additive response fields only (Decision 3).
- **Deliverables:** code, migration, tests, OpenAPI, docs.
- **Acceptance criteria:** AC-03, AC-04.
- **Verification:** ForgeCustomer `migrations` and `rust` CI jobs.
- **Stop conditions:** pause if the RLS matrix needs a policy change outside the new tables.

**WP-03 Operator confirmation (forge-command)**
- **Outcome:** 05 §C (REQ-04).
- **Starting evidence:** predecessor `operator_signing.rs` and `operator_signing/*`.
- **Prerequisites:** G2 for statuses; fleet-manager plan G1. The register step also needs AUTHORITY_GAP-01 closed.
- **Permitted scope:** forge-command, under `BDS-FCO-FLEET-MANAGER-v0.1`.
- **Boundaries:** no key material in any IPC type.
- **Deliverables:** discover and confirm commands, receipts, tests.
- **Acceptance criteria:** AC-05.
- **Verification:** ACL test, fingerprint vectors.
- **Stop conditions:** register is BLOCKED until AUTHORITY_GAP-01 closes; discover and confirm continue.

**WP-04 Release-time trust set (Author-Forge)**
- **Outcome:** 05 §D (REQ-05).
- **Starting evidence:** `build-bundle.yml`, `entitlement_trust_set.json`, the RELACC 1.x-gate obligation.
- **Prerequisites:** G1; a confirmation source (WP-03 receipts, or the interim of Decision 5).
- **Permitted scope:** Author-Forge workflow, a small script, tests, `doc/system/` §15 and §54.
- **Boundaries:** the workflow holds no secret.
- **Deliverables:** step, gate, tests, docs.
- **Acceptance criteria:** AC-06.
- **Verification:** a workflow dry run on a branch, plus script tests.
- **Stop conditions:** pause if ForgeCustomer cannot store the release statement (AUTHORITY_GAP-03), and record the gap.

**WP-05 Rotation rehearsal**
- **Outcome:** 05 §E (REQ-06).
- **Prerequisites:** G2, G3, G4.
- **Permitted scope:** local services only.
- **Boundaries:** no real key; no deployed service.
- **Deliverables:** the rehearsal record.
- **Acceptance criteria:** AC-07.
- **Verification:** two Author-Forge builds.
- **Stop conditions:** done on AC-07. A real rotation is a separate operator act.
