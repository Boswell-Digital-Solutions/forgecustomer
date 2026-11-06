# AC-02 — Cross-repository verification (WP-01)

- **Date:** 2026-11-06.
- **Candidate:** forgecustomer `c2b30f5` (PR #43). Author-Forge `d0eacbf`, its verifier compiled unchanged.
- **Status:** candidate evidence. It is rechecked after #43 merges.
- **Method:** the harness in `harness/`. Its manifest is stored as `Cargo.toml.txt`, so repository tooling does not read it as a crate. Rename it to `Cargo.toml` to run it.
  1. It starts ForgeCustomer's `AppState` with active key `entitlement-key-2` and standby key `entitlement-key-1` in `ENTITLEMENT_VERIFY_PUBLIC_KEYS`. Both are test seeds, not real keys.
  2. It reads the real `GET /v1/entitlements/keys` answer through the router.
  3. It parses that answer with Author-Forge's `TrustSet::from_json`.
  4. It verifies one snapshot from each key with Author-Forge's `verify_snapshot`. ForgeCustomer's own `Signer25519::sign_snapshot` signed both, and their bytes come from ForgeCustomer's `EntitlementSnapshot`.
- **Command:** `CARGO_TARGET_DIR=<forgecustomer>/api/target cargo test --test ac02 -- --nocapture`.
- **Observed** (log `~/Forge/reports/foc_wp01_ac02_run.log`, sha256 `14c0ba129670eba772814d45fb6086ac7e8f8efae334432fca628ba541ce427d`):
  - With the variable set: two keys are published. The active snapshot → `Ok`, cloud allowed. The standby snapshot → `Ok`, cloud allowed.
  - **Negative control,** the same ForgeCustomer with the variable blank: one key is published. The standby snapshot → `UnknownKeyId`. The active snapshot → `Ok`.
- **Result:** AC-02 **PASS on the candidate.**
- **Limit:** a desktop trusts a key only once it is compiled into a build (WP-04). This check proves the bytes and the published shape, not a shipped build.
