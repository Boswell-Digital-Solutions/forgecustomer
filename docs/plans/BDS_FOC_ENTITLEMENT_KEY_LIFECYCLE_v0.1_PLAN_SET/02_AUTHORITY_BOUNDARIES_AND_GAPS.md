# 02 — Authority boundary and declared AUTHORITY_GAP conditions

This component allocates authority and lists the gaps that block gates.

## Authority allocation

- **ForgeCustomer:** holds the signing keys, the key registry and the published key list. It signs snapshots and leases.
- **forge-command:** discovers keys, shows fingerprints, records the operator's confirmation, and proposes and decides each rotation step (the fleet manager). It holds no key.
- **Operator:** sets the deployed secrets and configuration, and confirms each fingerprint and each rotation step.
- **Author-Forge release pipeline:** at build time, compiles the confirmed published keys into `src-tauri/entitlement_trust_set.json`, and refuses a 1.x build with an empty set.
- **Author-Forge desktop:** verifies against the compiled set. Unchanged (`forgecustomer_entitlement.rs`).

## Declared gaps

**AUTHORITY_GAP-01 No admin credential from forge-command.** ForgeCustomer verifies admin tokens against the predecessor's `ADMIN_JWT_PUBLIC_KEY`. forge-command's issuer is not activated (`BDS-FCO-P3-TOKEN-ISSUER-001`, G4–G7 held). The admin key routes (WP-02) can be built and tested in ForgeCustomer now. forge-command cannot call them until that gate. Blocks WP-03 live use only.

**AUTHORITY_GAP-02 No registry table in ForgeCustomer.** Inspected: migrations `0001`–`0018` (re-inspected at main `96ba3bc`, 2026-11-18) hold no key table. Issued snapshots record `key_id` (`entitlement_snapshots`), and leases record it (`license_leases`). A registry with status events is new schema. It ports DataForge's `verification_evidence_receipt_keys` design. Blocks WP-02.

**AUTHORITY_GAP-03 Trust-set statement on a release.** The forge-command guard needs to know which key ids a build trusts. ForgeCustomer release metadata has no such field (forge-command plan AUTHORITY_GAP-06). Blocks WP-04's guard input.

**AUTHORITY_GAP-04 The fleet pipeline.** The updater is inactive until the Fleet pipeline is proven (KI-AUF-20261101-006). A rotation reaches desktops only through installs until then. Blocks AC-07's delivered-fleet evidence.

**AUTHORITY_GAP-05 The deployed signing key was unread (closed).** Closed by WP-00 on 2026-11-06. WP-00 read `GET /v1/entitlements/keys`, a public route, under the operator's instruction (09 Decision 1): the live `key_id` is `forgecustomer-entitlement-prod-1` (`evidence/AC-00/SOURCE_LOCK.md` §3, read again in §3a). The id is kept so later references resolve.

**AUTHORITY_GAP-06 Snapshot status at a deactivated installation.** KI-FOC-20261105-002 is adjacent, not in scope. It is recorded so that WP-02 does not widen it.
