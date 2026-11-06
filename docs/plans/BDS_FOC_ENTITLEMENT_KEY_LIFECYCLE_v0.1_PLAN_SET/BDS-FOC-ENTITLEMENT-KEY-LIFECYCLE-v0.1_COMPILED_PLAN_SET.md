# BDS-FOC-ENTITLEMENT-KEY-LIFECYCLE-v0.1 — COMPILED PLAN SET

Plan ID: BDS-FOC-ENTITLEMENT-KEY-LIFECYCLE-v0.1 · Lifecycle: authorized · Authority: operator ruling 2026-11-06 (WP-00, WP-01; DEC-1 to DEC-5 ruled) · Decision owner: Charles (Charlie) Boswell · Prepared 2026-11-05 America/New_York

Compilation: hand-maintained concatenation of components 00–09 under BDS-PSS-001 §3.3. No build script exists. An edit to a component must be mirrored here in the same change.


===== 00_PLAN_SET_INDEX.md =====

# 00 — Plan identity, index and read order

This component gives the plan identity, the purpose, the source state and the read order.

| Field | Value |
|---|---|
| Plan ID | BDS-FOC-ENTITLEMENT-KEY-LIFECYCLE-v0.1 |
| Title | ForgeCustomer entitlement-key lifecycle — overlap, publication and operator-confirmed rotation, built from existing code |
| Lifecycle | authorized |
| Authority | Operator ruling 2026-11-06 ("Proceed - in order authorized"): DEC-1 to DEC-5 accepted as recommended in the 2026-11-06 research brief. WP-00 and WP-01 are authorized. WP-02 onward need their gates. No key operation, configuration change, deployment, merge or release is authorized. |
| Decision owner | Charles (Charlie) Boswell, BDS operator |
| Prepared | 2026-11-05 America/New_York |
| Parent / extension target | Operator direction 2026-11-05: "build the plan ensuring current business code that works is considered instead of reinvented"; "we build the plan in Drive /Forge using the protocols". Findings: KI-FOC-20261105-001 (forgecustomer #41). Consumer plans: forge-command `BDS-FCO-FLEET-MANAGER-v0.1` (its AUTHORITY_GAP-03 and -06), Author-Forge `BDS-AF-RELACC-v0.1` (DIR-1). Authority: ForgeCustomer `BDS_AUTHORFORGE_COMMERCE_LAUNCH_POLICY_v0.1`. |
| Proposed canonical owner | Boswell-Digital-Solutions/forgecustomer for the key registry and routes. forge-command for the operator workflow. Author-Forge for the release-time trust set. Each work package pins its repository SHA. |
| Primary candidate consumer | forgecustomer `api/src/state.rs`, `api/src/config.rs`, `api/src/services/signing.rs`, `api/src/routes/entitlements.rs`, `supabase/migrations/`; forge-command Business / Releases and fleet; Author-Forge `.github/workflows/build-bundle.yml`, `src-tauri/entitlement_trust_set.json`. |
| Canonical Drive destination | /Forge/Plans/BDS_FOC_ENTITLEMENT_KEY_LIFECYCLE_v0.1_PLAN_SET (Drive folder `1FyK4JLWIDzwdyeY_Mz2seNIbWjkfhC_z`, uploaded 2026-11-05). The repository copy in forgecustomer `docs/plans/` is the byte-exact one. |
| Compilation | Components 00–09, concatenated by hand into BDS-FOC-ENTITLEMENT-KEY-LIFECYCLE-v0.1_COMPILED_PLAN_SET.md. No build script. |

## Purpose

ForgeCustomer signs every AuthorForge entitlement snapshot and offline lease with one Ed25519 key. Its deployed key ring holds only that key (KI-FOC-20261105-001). Desktops compile the trusted keys in (DIR-1). After a key change, ForgeCustomer signs every new snapshot with the new key. A build that trusts only the old key refuses each new snapshot (`unknown_key_id`), so it loses cloud mode at its next refresh. Snapshots already signed with the old key stay valid wherever that key is still trusted; ForgeCustomer verifies none of its own documents at run time.

This plan adds a safe rotation. It does so by connecting code that already works in three places:
- ForgeCustomer's multi-key ring and its passing overlap test;
- DataForge's receipt-key registry, with its status, routes and lifecycle ordering;
- the predecessor Forge_Command's discover → confirm → register operator flow.

It writes new code only where nothing exists: the configuration wiring, a `pending` state for a pre-published next key, and the release-time trust-set step.

## Source state used to prepare this plan

| Repository | Commit | Read |
|---|---|---|
| forgecustomer | `53d255a` | `api/src/services/signing.rs` (incl. test `rotation_overlap_both_keys_verify`), `api/src/state.rs:34-50`, `api/src/config.rs`, `api/src/routes/entitlements.rs`, `api/src/routes/mod.rs` (admin routes); `doc/system/` 01, 05, 06, 07, 10, 11, 12, 13; `docs/ENTITLEMENTS.md`, `docs/LICENSING.md`; migrations through `0016` |
| DataForge (cloud) | `6f56f3b1` (`master`) | `doc/system/19-verification-evidence-custody.md` (receipt keys, lifecycle ordering, onboarding), `doc/system/14`; `app/api/verification_evidence_router.py` (`guard_receipt_signer`, `register_current_receipt_key`, `rfc8785_jcs_bytes`); the same pattern in `revocation_custody_router.py` and `memory_transition_router.py` |
| dataforge-Local | `ccffbc5` (`master`) | `doc/system/17` §5.2 (`XA_RUN_TOKEN_PUBLIC_KEYS` = `{kid: {key, issuer}}`), `doc/system/09` (fleet, releases and entitlements belong to ForgeCustomer) |
| Forge_Command (predecessor, reference) | `8f780036` | `src-tauri/src/commands/operator_signing.rs` (`receipt_keys_discover`, `receipt_keys_confirm`, `receipt_keys_register_current`), `src-tauri/src/operator_signing/{client,keystore,mod}.rs` (`fingerprint`, `set_fingerprint_confirmed`), `commands/forgecustomer_admin.rs` |
| forge-command | `e414f4d` | `crates/fc-intent` and `crates/fc-topology` (`serde_jcs`, RFC 8785, passes 25 SMITH vectors); `doc/system/14`, `15`, `16.7`; plan `BDS-FCO-FLEET-MANAGER-v0.1` (#49) |
| Drive `/Forge/Plans/AuthorForge_Fleet_Client_Update_Plan_Set_2026-06-11` (folder `1nsPFJVAGb_pWj6_s-yJPZOOl5suuL2T4`), architecture approved 2026-06-11 | — | 11 (key separation; staged rotation "Release N signed by key A embeds trust for key B"), 18 (D-14 to D-16); read 2026-11-06 |
| Author-Forge | `9f3a5d1a5` | `src-tauri/src/infrastructure/forgecustomer_entitlement.rs` (multi-key `TrustSet`, byte-compatibility fixture), `src-tauri/entitlement_trust_set.json` (empty), `.github/workflows/build-bundle.yml` |

## Status

Authorized 2026-11-06 for WP-00 and WP-01. All acceptance criteria are NOT_RUN. "Inspected" marks a fact read in source at the pin.

## Read order and components

00 → 09, then the four siblings. 00 Index · 01 Rulings · 02 Authority and gaps · 03 Bindings, coverage and **reuse matrix** · 04 Contracts · 05 Mechanism · 06 Decisions · 07 Acceptance · 08 Work packages · 09 Operator review


===== 01_BINDING_RULINGS_AND_NON_GOALS.md =====

# 01 — Binding rulings and explicit non-goals

This component lists the rulings that bind the plan once it is authorized, and the work it does not do.

## Rulings

**R01 Reuse first.** A work package may write new code only for a capability that the 03 reuse matrix marks "no existing code". Every other capability is built by wiring, porting or calling the code named in the matrix. A deviation needs a recorded reason and the operator's approval (operator direction 2026-11-05).

**R02 ForgeCustomer holds every entitlement key.** Private keys live only in ForgeCustomer's deployment secrets. No agent creates, reads or sets them. This copies DataForge's rule (§19 "Receipt signing key"). forge-command and Author-Forge hold public keys only.

**R03 One variable, one shape.** Retained and pending public keys reach ForgeCustomer through one configuration value, `ENTITLEMENT_VERIFY_PUBLIC_KEYS`, a JSON object `{key_id: base64_public_key}`. It mirrors the shape DataForge Local already uses (`XA_RUN_TOKEN_PUBLIC_KEYS`), without the issuer field ForgeCustomer does not need. It is the only new configuration value.

**R04 Publish before sign.** A new signing key is published as `pending` before ForgeCustomer signs with it. A desktop can then receive it in a release before the switch. DataForge publishes a key at its first use. That is the one deliberate difference from DataForge's model, because desktops compile keys in.

**R05 Retired keys keep verifying.** A retired key stays published until its last snapshot (24 h) and its last offline lease (14 days) have expired. A revoked key stops at once.

**R06 Keep the wire format that works.** `forge.entitlements.v1` and `forge.lease.v1`, with their struct-order byte rule, stay unchanged in this plan. Author-Forge already verifies them, with a byte-compatibility fixture. RFC 8785 bytes are a separate, later version (09 Decision 4).

**R07 The operator confirms each published key.** forge-command discovers the keys ForgeCustomer publishes, shows each fingerprint, and records the operator's confirmation. This ports the predecessor's `receipt_keys_*` flow. A key that has not been confirmed is not compiled into a release.

**R08 No configuration change by an agent.** Setting `ENTITLEMENT_SIGNING_PRIVATE_KEY`, `ENTITLEMENT_SIGNING_KEY_ID` or `ENTITLEMENT_VERIFY_PUBLIC_KEYS` on the deployed service is the operator's act (workspace `CLAUDE.md`).

## Explicit non-goals

- A new key store, KMS, HSM or certificate authority.
- A root-signed key set fetched by desktops at run time. That is a later option (forge-command plan 05 §E).
- Changes to Stripe, licensing or entitlement evaluation.
- The rest of the fleet manager: fleet reads, containment and release authorization belong to `BDS-FCO-FLEET-MANAGER-v0.1`.
- Removing Author-Forge's own HS256 commercial authority (ForgeCustomer CP1-WP06).


===== 02_AUTHORITY_BOUNDARIES_AND_GAPS.md =====

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

**AUTHORITY_GAP-02 No registry table in ForgeCustomer.** Inspected: migrations `0001`–`0016` hold no key table. Issued snapshots record `key_id` (`entitlement_snapshots`), and leases record it (`license_leases`). A registry with status events is new schema. It ports DataForge's `verification_evidence_receipt_keys` design. Blocks WP-02.

**AUTHORITY_GAP-03 Trust-set statement on a release.** The forge-command guard needs to know which key ids a build trusts. ForgeCustomer release metadata has no such field (forge-command plan AUTHORITY_GAP-06). Blocks WP-04's guard input.

**AUTHORITY_GAP-04 The fleet pipeline.** The updater is inactive until the Fleet pipeline is proven (KI-AUF-20261101-006). A rotation reaches desktops only through installs until then. Blocks AC-07's delivered-fleet evidence.

**AUTHORITY_GAP-05 The deployed signing key is unread.** The live `key_id` is unknown. The default is `entitlement-key-1`. WP-00 reads `GET /v1/entitlements/keys`, a public route, only under the operator's instruction (09 Decision 1).

**AUTHORITY_GAP-06 Snapshot status at a deactivated installation.** KI-FOC-20261105-002 is adjacent, not in scope. It is recorded so that WP-02 does not widen it.


===== 03_PROTOCOL_BINDINGS_AND_COVERAGE.md =====

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


===== 04_CONTRACTS_AND_PROVENANCE.md =====

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


===== 05_CORE_MECHANISM_SPECIFICATION.md =====

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
5. The operator decides the switch, and only when the fleet manager shows every active installation on a build that trusts the new key (ruling B, 09). The owner sets the new private key and id, the old public key moves into `ENTITLEMENT_VERIFY_PUBLIC_KEYS`, ForgeCustomer redeploys, and the status changes to `pending → active` and `active → retired`.
6. After 14 days, the lease lifetime, plus 5 minutes of clock skew (DEC-5), the operator removes the retired key from configuration.

No desktop is refused at any step **only if** every installed build trusts the new key before the switch. With the standby key compiled into the first shipped build (DEC-2), every build trusts it from day one. An installed build that lacks the new key refuses new snapshots after the switch and loses cloud mode until it updates. Local writing is unaffected (ruling B).


===== 06_DECISION_SEMANTICS.md =====

# 06 — Decision semantics

## Key statuses

| Status | Published | Signs | Verifies on ForgeCustomer |
|---|---|---|---|
| `pending` | yes | no | yes |
| `active` | yes | yes | yes |
| `retired` | yes, until removed from configuration | no | yes |
| `revoked` | no | no | no |

Allowed transitions: `pending → active` (only when the deployed signing key matches); `active → retired` (at the switch); `pending | active | retired → revoked` (terminal).

## Gate dispositions

PASS | FAIL | BLOCKED | NOT_RUN. A skip is NOT_RUN.

## Gates

| Gate | Needs | Covers |
|---|---|---|
| G0 | AC-00, Decision 1 ruled | Plan admitted; source lock |
| G1 | AC-01, AC-02 | Overlap wired (ForgeCustomer) |
| G2 | G1, AC-03, AC-04 | Registry and statuses |
| G3 | G2, AC-05; AUTHORITY_GAP-01 closed for the register step | Operator confirmation |
| G4 | G1, AC-06 | Release-time trust set |
| G5 | G2, G3, G4, AC-07 | First rotation rehearsal |

Order: G0 → G1 → {G2, G4} → G3 → G5. G1 alone closes KI-FOC-20261105-001's overlap half.


===== 07_ACCEPTANCE_AND_EVIDENCE.md =====

# 07 — QA, verification and acceptance-to-evidence map. ALL CHECKS NOT_RUN.

**Common evidence rule.** Every record states:
- the repository SHA;
- the environment;
- the command;
- the expected and observed results;
- the retained artifacts with sha256;
- the limits.

Tests use local ForgeCustomer and test keys generated in the suite, never the deployed service or a real key. Skips do not count. A negative check must reach its named refusal.

**Evidence destination:** this plan set's `evidence/<AC-ID>/`, with the repository records referenced.

**AC-00 Structure.** 15 plan files. The manifest equals the folder, evidence files included. The lifecycle matches in four places. The IDs agree.

**AC-01 Overlap wired.**
- With `ENTITLEMENT_VERIFY_PUBLIC_KEYS` holding a second test key, `GET /v1/entitlements/keys` lists both.
- A snapshot signed by the second key verifies through `VerifyingKeyRing::verify_snapshot`.
- With the variable unset, behavior equals today's (one key).
- A malformed value fails start-up with a named error.
- A duplicate `key_id` carrying a different key fails start-up.
- `rotation_overlap_both_keys_verify` still passes.
- `cargo fmt`, `clippy -D warnings` and `cargo test --all` pass (the ForgeCustomer `doc/system/13` gates).

**AC-02 Cross-repo verification.** An Author-Forge build whose trust set holds both test keys verifies a snapshot from each, using the existing byte-compatibility method (fixtures from ForgeCustomer's own signer).

**AC-03 Registry.**
- The migration applies, re-applies deterministically and passes the RLS write-denial matrix (ForgeCustomer CI `migrations` job).
- Status is derived from events.
- Each admin route needs `admin`, a reason, and writes audit.
- A revoked key cannot change.

**AC-04 Signing refuses a retired or revoked active key.**
- With the active key marked retired or revoked, `GET /v1/entitlements/current` and lease issuance give 503 and store nothing.
- The ported lifecycle-ordering test shows no snapshot is stored after a concurrent revocation commits. This mirrors DataForge's concurrency proof.

**AC-05 Operator confirmation.**
- forge-command discovery lists each published key with a fingerprint that equals the predecessor's `fingerprint` function output on the same key, by test vector.
- A confirmation needs presence and writes a receipt.
- An unconfirmed key is excluded by AC-06.

**AC-06 Release-time trust set.**
- The workflow step writes only confirmed `active` and `pending` keys.
- A 1.x version with none fails the build with a named message.
- A 0.x build with none passes.
- The release record carries the key ids.

**AC-07 Rotation rehearsal.** On a local ForgeCustomer and two Author-Forge builds (old: key A; new: keys A and B), across steps 1–6 of 05 §E:
- the old build verifies until the switch;
- the new build verifies before and after the switch;
- after the switch, the old build refuses with `unknown_key_id`, the expected result for an un-updated install;
- no step needs a user action.

Delivered-fleet evidence is blocked on AUTHORITY_GAP-04.

**AC-08 Reuse audit.** A review of each work package's diff against the 03 matrix. No listed capability is reimplemented, and each new-code item maps to the "no existing code" list.


===== 08_WORK_PACKAGES_AND_GATES.md =====

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


===== 09_OPERATOR_REVIEW_AND_HANDOFF.md =====

# 09 — Operator review packet and handoff

## Existing authority

On 2026-11-05 the operator directed: "well build the plan ensuring current business code that works is considered instead of reinvented. - we build the plan in Drive /Forge using the protocols." That authorizes creating this plan set in BDS-PSS-001 form. The Drive connector was unavailable, and the operator then directed: "just hold it in repo - docs/plans then". So this copy in forgecustomer `docs/plans/` is the held copy. It was uploaded to the Drive destination on 2026-11-05, once the connector worked again. On 2026-11-06 the operator ruled DEC-1 to DEC-5 ("Proceed - in order authorized"). Lifecycle: `authorized`.

## Status

Planning v0.1 drafted. Rulings DEC-1 to DEC-5 approved 2026-11-06. Execution authorization: WP-00 and WP-01. WP-00 complete 2026-11-06: AC-00 PASS, G0 PASS (evidence/AC-00). The live key list holds one key, `forgecustomer-entitlement-prod-1`. WP-01 is built as forgecustomer PR #43 (`c2b30f5`): AC-01 and AC-02 PASS on the candidate (evidence/AC-01, evidence/AC-02).

---

## Decision 1 — Authorize WP-00 and WP-01?

**Recommendation: yes.** WP-01 is the smallest real fix. It wires ForgeCustomer's existing multi-key ring to one new configuration value, and it closes the overlap half of KI-FOC-20261105-001. WP-00 includes one unauthenticated read of the public `GET /v1/entitlements/keys`, to learn the live `key_id`.

**Alternative:** WP-00 without the public read. Then the live `key_id` stays unknown until you supply it.

**Note:** merging the WP-01 PR redeploys ForgeCustomer on Render (KI-FOC-20261002-001). Setting the new variable is your act.

**Ruling (2026-11-06): approved.** WP-00 includes the one unauthenticated read of the public key list. WP-01 ends in a PR; its merge needs your word.

## Decision 2 — Who creates a new signing key?

**Recommendation:** you, outside any agent, set as a deployment secret. That is DataForge's rule, and it is ForgeCustomer's practice today.

**Alternative:** a ForgeCustomer admin route that generates the key. This adds private-key handling to the API, and the plan does not recommend it.

**Ruling (2026-11-06): approved, with one addition.** You create two keys, an active key and a standby key, outside any agent. Both public halves are published and compiled into every Author-Forge trust set; only the active key signs. A compromise then becomes a signer switch, not a rebuild (TUF and APK Signature Scheme v3 pre-trust the next key; the 2026-06-11 plan's staged rotation). WP-01's verify-key variable carries the standby public key.

## Decision 3 — Extend the public key list with status fields?

**Recommendation: yes, additively.** Add `status`, `valid_from` and `retired_at` (04 §B). Author-Forge's parser does not deny unknown fields (inspected at `9f3a5d1a5`), so built desktops are unaffected.

**Ruling (2026-11-06): approved.**

## Decision 4 — Keep the v1 wire format?

**Recommendation: keep `forge.entitlements.v1` with the struct-order byte rule now (R06).** It works and is verified across repositories. RFC 8785 (`serde_jcs`, already proven in forge-command) can be a `v2` later, behind its own plan.

**Alternative:** move to RFC 8785 now. That changes the bytes both sides sign and verify, and it needs a coordinated release.

**Ruling (2026-11-06): approved.** v1 keeps the struct-order rule. RFC 8785 becomes the v2 rule, triggered when a non-Rust verifier is first needed. RFC 8785 is an informational Independent Submission, so its use rests on the ecosystem precedent (forge-command's SMITH vectors), not on standard status.

## Decision 5 — Interim confirmation source for WP-04

**Question:** until forge-command's confirmation exists, what should the release step trust?

**Recommendation:** the release step accepts a key only if its fingerprint is listed in a committed file `src-tauri/entitlement_key_confirmations.json`, edited only by you. The forge-command receipts replace it when WP-03 lands.

**Alternative:** trust every `active` key ForgeCustomer publishes. This is simpler, but a compromised deployment could add a key.

**Ruling (2026-11-06): approved.** The committed, operator-edited file is the interim. Retire an old key no earlier than the longest-lived signed document plus clock skew: the 14-day lease plus 5 minutes.

---

## Ruling B — An installed build without the new key (2026-11-06)

**Question (review of #42, M1).** 05 §E promised that no desktop is refused, but AC-07 expects an old build to refuse after the switch.

**Ruling (2026-11-06, operator "yes do it", as recommended).** A build that lacks the new key loses cloud mode after a signer switch, until it updates; local writing is unaffected. The switch waits until the fleet manager shows every active installation on a build that trusts the new key. 05 §E now states the condition, and AC-07 keeps the old-build refusal as the expected result.

## Dependencies

- forge-command P3 (ForgeCustomer admin credential).
- `BDS-FCO-FLEET-MANAGER-v0.1` (forge-command #49) for WP-03.
- Fleet pipeline (KI-AUF-20261101-006) for delivered-fleet evidence.

## Handoff

**Next action:** your word to merge #43. The merge redeploys Render, and with the variable unset the deployed behavior does not change. Then recheck AC-01 and AC-02 on `main`. Then your acts: create the standby key and set `ENTITLEMENT_VERIFY_PUBLIC_KEYS` (DEC-2), and confirm the live fingerprint in `evidence/AC-00/SOURCE_LOCK.md` §3 (DEC-5).

Keep R01 (reuse first), R02 (no key outside ForgeCustomer), every AC NOT_RUN, and AUTHORITY_GAP-01 to -06.
