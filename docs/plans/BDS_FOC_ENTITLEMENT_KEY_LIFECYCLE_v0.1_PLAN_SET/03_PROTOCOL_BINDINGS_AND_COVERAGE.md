# 03 — Bindings, coverage and the reuse matrix

This component names the governing material and maps every capability to the code that already provides it.

## Binding

- **BDS-PSS-001** (Drive `1yWQUGn4yaSPJcQrui3Y5LBcycvCnAHyxhxayk_RiKCY`).
- **Workspace `CLAUDE.md`:** authority table; no agent changes deployed configuration or secrets; no compatibility layer.
- **forgecustomer `doc/system/` 09 (change control) and 11 (entitlement signing):** "Key rotation requires an overlap window". A signing change is a change-controlled item.
- **forge-command `CLAUDE.md` and `doc/system/14`:** command table, approval window, presence.
- **Author-Forge `CLAUDE.md`:** repository gates.

## Reuse matrix (R01)

| Capability | Existing working code | Where | Use in this plan |
|---|---|---|---|
| Hold several verifying keys and verify by `key_id` | `VerifyingKeyRing` (`add_signer`, `add_base64`, `verify`, `published`) | forgecustomer `api/src/services/signing.rs` | **Wire.** Call `add_base64` for each entry of `ENTITLEMENT_VERIFY_PUBLIC_KEYS` in `AppState::build`. No crypto changes. |
| Old and new keys both verify | test `rotation_overlap_both_keys_verify` (passes) | same file | **Keep** as the unit proof. Add a config-level test. |
| Publish the key list | `routes/entitlements.rs::keys` publishes `key_ring.published()` | forgecustomer | **Keep.** It publishes wired keys with no change. Add `status` and `valid_from` from the registry (WP-02). |
| Sign snapshots and leases with the active key | `Signer25519::sign_snapshot`, lease signing | forgecustomer | **Keep unchanged** (R06). |
| Key registry with status events, `register-current`, revoke, lifecycle ordering under row locks | `verification_evidence_receipt_keys`, `register_current_receipt_key`, `guard_receipt_signer`, `POST /receipt-keys/{key_id}/status` | DataForge `app/api/verification_evidence_router.py`, `doc/system/19`; repeated in `revocation_custody_router.py`, `memory_transition_router.py` | **Port the design** to ForgeCustomer (Rust and SQL), field for field where it applies. Add one status, `pending` (R04). |
| Admin route auth, reason, audit | `/v1/admin/*` pattern: role gate, required reason, operator-actor audit, outbox | forgecustomer `routes/mod.rs`, `doc/system/05` "Admin operations" | **Reuse** for the new key routes. |
| `{kid: key}` configuration shape | `XA_RUN_TOKEN_PUBLIC_KEYS` | dataforge-Local `doc/system/17` §5.2 | **Copy the shape** (R03). |
| Operator discovers keys, checks the fingerprint, confirms, registers | `receipt_keys_discover`, `receipt_keys_confirm`, `receipt_keys_register_current`, `fingerprint`, `set_fingerprint_confirmed` | Forge_Command `src-tauri/src/commands/operator_signing.rs`, `operator_signing/{client,keystore,mod}.rs` | **Port the flow** to forge-command's approval window, using FIDO2 presence instead of TOTP (`doc/system/14` §14.5). |
| ForgeCustomer admin client with a short-lived operator token | `ForgeCustomerAdminClient`, `mint_forgecustomer_admin_token` | Forge_Command `commands/forgecustomer_admin.rs`, `doc/system/19` | **Port** in the fleet-manager plan (WP-03 there). This plan only calls it. |
| RFC 8785 bytes | `serde_jcs` (forge-command, 25 SMITH vectors); `rfc8785_jcs_bytes` (forge_contract_core, used by DataForge) | forge-command `crates/fc-intent`, `crates/fc-topology` | **Available** for a later wire version (R06, Decision 4). Not used now. |
| Desktop verifies against several keys | `TrustSet` keyed by `key_id`; byte-compatibility fixture from ForgeCustomer's own signer | Author-Forge `forgecustomer_entitlement.rs` | **Keep unchanged.** Fill its file at build time (WP-04). |
| Release build | `build-bundle.yml`, release registration routes, update eligibility | Author-Forge, forgecustomer | **Extend** with one step that writes the trust set, and one gate. |

**No existing code (new work is allowed):**
- the `ENTITLEMENT_VERIFY_PUBLIC_KEYS` parser;
- the ForgeCustomer key table and its migration (ported design, new schema);
- the `pending` state;
- the release-time trust-set step and the 1.x non-empty gate;
- the trust-set statement on a release record (AUTHORITY_GAP-03).

## Requirement coverage

| REQ | Requirement | WP | AC |
|---|---|---|---|
| REQ-01 | Retained and pending public keys verify and are published | WP-01 | AC-01, AC-02 |
| REQ-02 | Each key has an auditable status: pending, active, retired, revoked | WP-02 | AC-03 |
| REQ-03 | No snapshot is signed with a retired or revoked key | WP-02 | AC-04 |
| REQ-04 | The operator confirms each fingerprint before a key is trusted | WP-03 | AC-05 |
| REQ-05 | Release builds compile only confirmed published keys; a 1.x build with none fails | WP-04 | AC-06 |
| REQ-06 | A full rotation completes with no desktop refused | WP-05 | AC-07 |
| REQ-07 | Reuse first: no capability in the matrix is rewritten | all | AC-08 |
| REQ-08 | Plan conformance | all | AC-00 |
