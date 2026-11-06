# 04 — Proposed contract and provenance model. NOT an admitted contract.

This component proposes the shapes. Each one names the existing shape it extends.

## A. `ENTITLEMENT_VERIFY_PUBLIC_KEYS` (configuration, R03)

A JSON object `{ "<key_id>": "<base64 32-byte Ed25519 public key>" }`. It holds retained and pending keys. The active key comes from the existing `ENTITLEMENT_SIGNING_PRIVATE_KEY` and `ENTITLEMENT_SIGNING_KEY_ID`, and is not repeated here. If an entry's `key_id` equals the active `key_id` with a different key, start-up fails. An unparseable value fails start-up; it never falls back to the active key alone.

## B. Published key list (extends `GET /v1/entitlements/keys`)

The response is today `{ "schema": "forge.entitlements.v1", "keys": [ { "key_id", "public_key", "alg" } ] }`.

**Proposed:** each entry adds `status` (`pending` | `active` | `retired`), `valid_from` and `retired_at`. These are the field names of DataForge `GET /receipt-keys`. Revoked keys are not listed. The existing three fields keep their meaning. Author-Forge's trust-set parser ignores nothing it needs, so this needs a decision (09 Decision 3).

## C. Key registry (ports DataForge `verification_evidence_receipt_keys`)

**`entitlement_keys`:** `key_id` (PK) · `public_key` · `alg` · `created_at`.

**`entitlement_key_events`**, insert-only: `key_id` · `status` (`pending`, `active`, `retired`, `revoked`) · `at` · `actor` · `reason`.

The current status is derived from the events, as in DataForge. A revoked key cannot change again.

## D. Admin routes (reuse the `/v1/admin/*` pattern)

- `POST /v1/admin/entitlement-keys/register-current`: records the active key (DataForge `register-current`).
- `POST /v1/admin/entitlement-keys/{key_id}/status`: body `{status, reason}`. It allows `pending → active` (the switch, which also needs the deployed signing key to match), `active → retired` and `* → revoked`.
- `GET /v1/admin/entitlement-keys`: the full history, including revoked keys.

Each route needs the `admin` role, a reason and operator-actor audit.

## E. Release trust-set statement (for AUTHORITY_GAP-03)

The release registration body gains `entitlement_trust_set_key_ids: [key_id]`. The release pipeline writes it.

## Provenance

- Every status change is an audited event.
- forge-command records the operator's fingerprint confirmation as an approval receipt (port of `set_fingerprint_confirmed`).
- The release record ties a build to the keys it trusts.
