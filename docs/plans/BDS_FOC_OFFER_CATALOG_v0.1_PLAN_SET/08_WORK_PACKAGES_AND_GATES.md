# 08 — Work packages and gates. WP-00 complete; WP-01 onward proposed.

This component gives each work package its nine-field brief, and the gates between them.

## Gates

| Gate | Passes when |
|---|---|
| OC-G0 | AC-00 PASS (the source lock). |
| OC-G1 | AC-01 PASS: DEC-0 to DEC-9 each have an outcome. |
| OC-G2 | The CP0 entry evidence of the commerce launch policy is attached (AUTHORITY_GAP-01 closed), and the operator authorizes the implementation work packages. |
| OC-G3 | AC-02 to AC-06 PASS on review branches. |
| OC-G4 | AC-07 and AC-08 PASS. A separate operator launch authorization is still required. |

**WP-00 Source lock and evidence inventory** — owner: this plan (read-only)

- **Outcome:** the inventory of 00 §Purpose item 1.
- **Starting evidence:** the pins in 00.
- **Prerequisites:** none.
- **Permitted scope:** this folder only.
- **Boundaries:** no change to any other repository, Drive file or service.
- **Deliverables:** `evidence/AC-00/SOURCE_LOCK.md`, `evidence/AC-00/RESULT.md`.
- **Acceptance criteria:** AC-00.
- **Verification:** 07 AC-00.
- **Stop conditions:** done on AC-00. **Status: complete 2026-11-08.**

**WP-01 Operator decision packet** — owner: the operator

- **Outcome:** DEC-0 to DEC-9 have outcomes (OC-G1).
- **Starting evidence:** 09 and the source lock.
- **Prerequisites:** OC-G0.
- **Permitted scope:** this folder (09, `plan_set.yaml`, the compiled file).
- **Boundaries:** a ruling authorizes no other work package.
- **Deliverables:** the rulings recorded.
- **Acceptance criteria:** AC-01.
- **Verification:** 07 AC-01.
- **Stop conditions:** done when every decision has an outcome; a deferral keeps its blocked work packages blocked.

**WP-02 Offer kinds, rights reducer and catalog seed** — owner: forgecustomer

- **Outcome:** 04 §A–C in the schema and the webhook reducer, as an extension of launch-policy CP1-WP01 to WP04.
- **Starting evidence:** forgecustomer `origin/main` at authorization (re-pin); migrations `0002`–`0005`; `repositories/{commerce,licensing,entitlements}.rs`.
- **Prerequisites:** OC-G1 (DEC-1, DEC-2, DEC-0); OC-G2; the launch policy CP1-WP01 design.
- **Permitted scope:** forgecustomer source, migrations, tests and `doc/system/` (04, 05) on a review branch.
- **Boundaries:** no live Stripe object; no production migration; no change to Pro's existing subscription behavior except as 04 §B states.
- **Deliverables:** migrations, reducer, tests, `doc/system` updates.
- **Acceptance criteria:** AC-02.
- **Verification:** `cd api && cargo fmt --all --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all`; `bash doc/system/BUILD.sh`.
- **Stop conditions:** done on AC-02; stop if the launch policy design and this plan disagree on a field (ask the operator).

**WP-03 Founders program** — owner: forgecustomer

- **Outcome:** 05 §D.
- **Starting evidence:** WP-02's schema.
- **Prerequisites:** WP-02; DEC-3 ruled.
- **Permitted scope:** as WP-02.
- **Boundaries:** the cap and benefits are data; no benefit outside D05.
- **Deliverables:** `founder_programs`, `founder_seats`, the reservation and confirmation path, tests.
- **Acceptance criteria:** AC-03.
- **Verification:** as WP-02, plus the concurrency test.
- **Stop conditions:** done on AC-03.

**WP-04 Update right on the license and the desktop** — owners: forgecustomer, Author-Forge

- **Outcome:** 05 §B for the DEC-2 choice, and the DEC-8 offline rule.
- **Starting evidence:** `licenses`; Author-Forge `connection_mode.rs` and `entitlementVerdict.ts`.
- **Prerequisites:** WP-02; DEC-2 and DEC-8 ruled.
- **Permitted scope:** both repositories' source, tests and `doc/system` on review branches.
- **Boundaries:** never stop an installed, allowed build; never block local work (REQ-06, REQ-09).
- **Deliverables:** snapshot fields, the shell install check, tests.
- **Acceptance criteria:** AC-04.
- **Verification:** forgecustomer gate as WP-02; Author-Forge `bun run api:test`, `bun run front:test`, the Tauri lanes; a live run of the packaged app.
- **Stop conditions:** done on AC-04.

**WP-05 Add-on grants** — owners: forgecustomer, Author-Forge

- **Outcome:** 05 §C for Press_Forge first.
- **Starting evidence:** `BDS-AF-CANEBRAKE-001` D4; the ladder packet's three grants.
- **Prerequisites:** WP-02; DEC-4 ruled.
- **Permitted scope:** both repositories on review branches.
- **Boundaries:** no Press_Forge feature work; the key gates nothing until Press_Forge tasks exist.
- **Deliverables:** feature keys, prerequisite checks, the shell display of each grant, tests.
- **Acceptance criteria:** AC-05.
- **Verification:** as WP-04.
- **Stop conditions:** done on AC-05.

**WP-06 Pro credits and annual price** — owners: forgecustomer, Author-Forge

- **Outcome:** 05 §E.
- **Starting evidence:** `plan_quotas`, the commercial policy layer in `repositories/entitlements.rs`.
- **Prerequisites:** WP-02; DEC-6 ruled.
- **Permitted scope:** both repositories on review branches.
- **Boundaries:** no unlimited allowance.
- **Deliverables:** `credit_pack` handling, quota increments, the desktop credit offer, tests.
- **Acceptance criteria:** AC-06.
- **Verification:** as WP-04.
- **Stop conditions:** done on AC-06.

**WP-07 Legal text, fact sheets and tax memo** — owners: the operator and counsel

- **Outcome:** AC-07 for each offer that will be sold.
- **Starting evidence:** launch policy §4 (tax candidates), §11 (disclosures); PROTIER checklist L-01 to L-07.
- **Prerequisites:** OC-G1.
- **Permitted scope:** documents only.
- **Boundaries:** agents draft for review only.
- **Deliverables:** the EULA, Pro terms, founders terms, add-on terms, beta agreement, fact sheets, the tax memo extended to the update plan and add-ons.
- **Acceptance criteria:** AC-07.
- **Verification:** operator and counsel sign-off.
- **Stop conditions:** done on AC-07. Other work packages continue meanwhile.

**WP-08 Test-mode rehearsal** — owner: cross-repository

- **Outcome:** AC-08.
- **Starting evidence:** WP-02 to WP-06 results.
- **Prerequisites:** OC-G3; the launch policy CP1-WP07 entry conditions.
- **Permitted scope:** Stripe test mode; review branches.
- **Boundaries:** no live object or transaction (R07, R08).
- **Deliverables:** fixtures and an evidence bundle.
- **Acceptance criteria:** AC-08.
- **Verification:** 07 AC-08.
- **Stop conditions:** done on AC-08; stop on any CP0 stop condition.
