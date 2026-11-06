# 05 — Core mechanism specification

This component specifies the mechanisms in order. Each step names the code it reuses.

## A. Overlap in ForgeCustomer (WP-01)

In `AppState::build`, after `key_ring.add_signer(&signer)`, parse `ENTITLEMENT_VERIFY_PUBLIC_KEYS` and call the existing `key_ring.add_base64` for each entry. The existing `keys` route then publishes them, and the existing `verify_snapshot` accepts them.

This is the whole fix for KI-FOC-20261105-001's overlap half. It changes no crypto, so the existing test `rotation_overlap_both_keys_verify` remains the unit proof. A new configuration-level test proves the wiring.

## B. Registry and statuses (WP-02)

1. Port DataForge's receipt-key registry: the tables, derived status, and lifecycle ordering under row locks (`guard_receipt_signer`, steps 1–5 of DataForge §19).
2. Before ForgeCustomer signs a snapshot or lease, it checks the active key's derived status. A `retired` or `revoked` key gives 503 and signs nothing, as in DataForge.
3. The `keys` route adds the statuses (04 §B).

## C. Operator confirmation in forge-command (WP-03)

Port the predecessor flow into the fleet manager's approval window:
1. **Discover:** read `GET /v1/entitlements/keys` and compute each fingerprint (the predecessor's `fingerprint`).
2. **Confirm:** the operator compares the fingerprint with the one ForgeCustomer's owner reads from the deployment, and confirms it with presence. A receipt is written.
3. **Register:** `register-current` through the admin client.

Read-only discovery works now (a public route). Registering waits on AUTHORITY_GAP-01.

## D. Release-time trust set (WP-04)

A step in `build-bundle.yml` does the following:
1. reads ForgeCustomer's published keys;
2. keeps those with status `active` or `pending` that forge-command has confirmed;
3. writes `src-tauri/entitlement_trust_set.json`;
4. records the key ids in the release registration (04 §E).

A version 1.x or later with an empty set fails the build. A 0.x build may stay empty, because the pre-release switch covers it (Author-Forge R07). The workflow reads a public route and holds no secret.

## E. A rotation, end to end (WP-05)

1. The owner creates the next key outside any agent. ForgeCustomer gets its public half in `ENTITLEMENT_VERIFY_PUBLIC_KEYS`, and the operator registers it as `pending`.
2. The operator confirms its fingerprint in forge-command.
3. A release built under §D carries both keys and ships through the fleet manager.
4. The fleet manager shows the share of installations whose build trusts the pending key, from the release statements and ForgeCustomer installation `build_id`.
5. The operator decides the switch. The owner sets the new private key and id, the old public key moves into `ENTITLEMENT_VERIFY_PUBLIC_KEYS`, ForgeCustomer redeploys, and the status changes to `pending → active` and `active → retired`.
6. After 14 days, the lease lifetime, the operator removes the retired key from configuration.

No desktop is refused at any step, because every installed build verifies with whichever key signs.
