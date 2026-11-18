# AC-02 — recheck on `main` (WP-01)

- **Date:** 2026-11-18.
- **Subject:** forgecustomer `main` `ae62638`, and Author-Forge `main` `c539ac800`.
- **Why:** `RESULT.md` recorded candidate evidence on forgecustomer `c2b30f5` and Author-Forge
  `d0eacbf`, and required a recheck after #43 merged. This is that recheck.
- **Consumer unchanged:** Author-Forge `src-tauri/src/infrastructure/forgecustomer_entitlement.rs`
  at `c539ac800` has sha256 `bae11061116cc4f81a37b2ac5e3ea0a8b24c73a4b4685fa9b395f93bc2800312`,
  the same as at `d0eacbf`. `git diff --stat d0eacbf c539ac800` over that file and
  `src-tauri/tests/fixtures/forgecustomer_snapshots.json` is empty.
- **Method:** the harness in `harness/`, unchanged. Its two placeholders were filled in a scratch
  copy: the forgecustomer path pointed at a worktree of `ae62638`, and the Author-Forge path at
  the three files extracted from `c539ac800`. The keys are the harness's own test seeds, not real
  keys. No deployed service.
- **Command:** `CARGO_TARGET_DIR=<forgecustomer>/target cargo test --test ac02 -- --nocapture`.
- **Observed** (log `~/Forge/reports/foc_wp01_ac02_main_ae62638.log`, sha256
  `1d69abde8bc5e157b024b224e872ccaff28b3f59147f68385fcffdd97735fbfd`):
  - With the variable set: two keys are published (`entitlement-key-1`, `entitlement-key-2`).
    The active snapshot → `Ok`, cloud allowed. The standby snapshot → `Ok`, cloud allowed.
  - **Negative control,** with the variable blank: one key is published. The standby snapshot →
    `UnknownKeyId`.
  - `test result: ok. 1 passed`, exit 0.
- **Result:** AC-02 **PASS on `main`** (forgecustomer `ae62638`, Author-Forge `c539ac800`).
- **Limit, unchanged:** a desktop trusts a key only once it is compiled into a build (WP-04).
  This check proves the bytes and the published shape, not a shipped build. The harness is still
  plan evidence with placeholder paths; no CI runs it (`KI-FOC-20261118-002`).
