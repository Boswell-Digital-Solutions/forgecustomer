# WP-00 — Source lock and reuse-matrix check

This record is the WP-00 deliverable. It pins every source, checks each row of the 03 reuse matrix, and records the live public key list.

- **Date:** 2026-11-06 America/New_York.
- **Authority:** DEC-1 (operator, 2026-11-06, "Proceed - in order authorized"), including one unauthenticated read of the public key list.
- **Method:** read-only inspection after `git fetch`. One public `GET`, below. No authenticated call. No `.env` or launcher file was read.

## 1. Pins

| Repository | Commit | Note |
|---|---|---|
| Boswell-Digital-Solutions/forgecustomer | `53d255ada3735233c1ad7fd6d8c9a7397c8c219e` | Equals the remote `main`. Unchanged since the plan was written. |
| Boswell-Digital-Solutions/DataForge | `6f56f3b1975ac2445e5d46d1fc312d07ab80f941` | `origin/master`. Unchanged. |
| Boswell-Digital-Solutions/dataforge-Local | `ccffbc527c35fb3f6987f10a55eb72b14cbfb323` | `origin/master`, read from `ecosystem/local-systems/dataforge-Local`. |
| Boswell-Digital-Solutions/Forge_Command | `8f780036984e95d1bbd176d3d9f077cc027863dd` | Reference only. |
| Boswell-Digital-Solutions/forge-command | `e414f4d6e3ec080f082836c8ad9ab7dc1cbad1ec` | Unchanged. |
| Boswell-Digital-Solutions/Author-Forge | `d0eacbf9332903c69af70bc35385b8019489fe33` | Moved from `9f3a5d1a5`. The entitlement module and trust set are unchanged between the two; the trust set is still empty. |

## 2. Reuse matrix (03), row by row

| Row | Result | Evidence (inspected) |
|---|---|---|
| `VerifyingKeyRing` holds several keys | **Confirmed.** `add_base64` (l. 90), `add_signer` (109), `published` (116), `verify` (124), `verify_snapshot` (142). | forgecustomer `api/src/services/signing.rs` |
| `add_base64` has no caller | **Confirmed.** `AppState::build` calls `add_signer` only. No other file under `api/src` calls `add_base64`. | `api/src/state.rs:40-46`; search of `api/src` |
| Overlap unit test | **Confirmed.** `rotation_overlap_both_keys_verify` (l. 220). | `signing.rs` |
| Keys route publishes the ring | **Confirmed.** | `api/src/routes/entitlements.rs:27` |
| Configuration helper for an optional value | **Confirmed.** `fn optional` (l. 71). | `api/src/config.rs` |
| DataForge key registry design | **Confirmed.** `guard_receipt_signer` (l. 329), `register_current_receipt_key` (l. 1337), `POST /receipt-keys/{key_id}/status` (l. 1346), table `verification_evidence_receipt_keys` (migration `20261024_01`). The same pattern is in `revocation_custody_router.py` and `memory_transition_router.py`. | DataForge `app/api/verification_evidence_router.py`; `alembic/versions/` |
| DataForge RFC 8785 bytes | **Confirmed.** `rfc8785_jcs_bytes`, imported from `forge_contract_core.model_certification`. | `verification_evidence_router.py:64, 596` |
| `{kid: {key, issuer}}` shape | **Confirmed.** | dataforge-Local `doc/system/17-configuration.md:17, 45` |
| Predecessor discover → confirm → register | **Confirmed.** `receipt_keys_discover` (l. 211), `receipt_keys_confirm` (218), `receipt_keys_register_current` (247); `fingerprint` = `sha256:` + SHA-256 of the raw 32-byte key, computed locally and never taken from a route answer; `set_fingerprint_confirmed`. | Forge_Command `src-tauri/src/commands/operator_signing.rs`; `operator_signing/mod.rs:168`; `operator_signing/keystore.rs:492` |
| forge-command RFC 8785 | **Confirmed.** `serde_jcs` in `fc-intent` and `fc-topology`. | forge-command `crates/*/Cargo.toml` |
| Author-Forge multi-key trust set | **Confirmed.** `TrustSet` keyed by `key_id`. `TrustSetFile` and `TrustSetKey` do **not** deny unknown fields, so the DEC-3 status fields cannot break a built desktop. The snapshot struct does deny unknown fields, as the signed bytes require. | Author-Forge `src-tauri/src/infrastructure/forgecustomer_entitlement.rs:41-80` |

No row is wrong, so the stop condition does not apply.

## 3. Live public key list

- **Call:** `GET https://forge-customer.onrender.com/v1/entitlements/keys`, unauthenticated, 2026-11-06T06:00Z. The base URL comes from Author-Forge `FORGECUSTOMER_API_BASE` (`src-tauri/src/fleet_updates.rs:67`).
- **Answer:** HTTP 200, `schema` `forge.entitlements.v1`, **one** key:
  - `key_id` `forgecustomer-entitlement-prod-1`, `alg` `Ed25519`;
  - `public_key` `9VzzfidvG7IAu1ZHdXaccO1IXvIWo472hBWErh8tz6Q=` (32 bytes);
  - fingerprint `sha256:c474f4f00337d0b1f5657393df593df27ccdbfaf78fa837e00a1f1e821118b20`, computed locally in the predecessor's format.
- **What it confirms:** the deployed ring publishes only its active key, as KI-FOC-20261105-001 states. No standby key exists yet (DEC-2).
- **What it does not do:** the key is not copied into any trust set. Under DIR-1 and DEC-5 it reaches Author-Forge only through the release step, after you confirm the fingerprint.

## 4. Result

- WP-00 deliverable complete. AC-00 is recorded in `RESULT.md`.
- **Limits:**
  - One public read is a point-in-time observation.
  - The fingerprint above comes from the route's answer. You still confirm it against the key you hold, which DEC-5 requires.
