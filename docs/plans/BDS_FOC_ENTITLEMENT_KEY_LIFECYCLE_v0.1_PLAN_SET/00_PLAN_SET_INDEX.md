# 00 — Plan identity, index and read order

This component gives the plan identity, the purpose, the source state and the read order.

| Field | Value |
|---|---|
| Plan ID | BDS-FOC-ENTITLEMENT-KEY-LIFECYCLE-v0.1 |
| Title | ForgeCustomer entitlement-key lifecycle — overlap, publication and operator-confirmed rotation, built from existing code |
| Lifecycle | proposed |
| Authority | Documentation creation only (operator direction 2026-11-05, below). No code, configuration, key operation, deployment or release is authorized. |
| Decision owner | Charles (Charlie) Boswell, BDS operator |
| Prepared | 2026-11-05 America/New_York |
| Parent / extension target | Operator direction 2026-11-05: "build the plan ensuring current business code that works is considered instead of reinvented"; "we build the plan in Drive /Forge using the protocols". Findings: KI-FOC-20261105-001 (forgecustomer #41). Consumer plans: forge-command `BDS-FCO-FLEET-MANAGER-v0.1` (its AUTHORITY_GAP-03 and -06), Author-Forge `BDS-AF-RELACC-v0.1` (DIR-1). Authority: ForgeCustomer `BDS_AUTHORFORGE_COMMERCE_LAUNCH_POLICY_v0.1`. |
| Proposed canonical owner | Boswell-Digital-Solutions/forgecustomer for the key registry and routes. forge-command for the operator workflow. Author-Forge for the release-time trust set. Each work package pins its repository SHA. |
| Primary candidate consumer | forgecustomer `api/src/state.rs`, `api/src/config.rs`, `api/src/services/signing.rs`, `api/src/routes/entitlements.rs`, `supabase/migrations/`; forge-command Business / Releases and fleet; Author-Forge `.github/workflows/build-bundle.yml`, `src-tauri/entitlement_trust_set.json`. |
| Canonical Drive destination | /Forge/Plans/BDS_FOC_ENTITLEMENT_KEY_LIFECYCLE_v0.1_PLAN_SET — **pending**: the Drive connector was unavailable on 2026-11-05. Held in forgecustomer `docs/plans/` by operator direction. |
| Compilation | Components 00–09, concatenated by hand into BDS-FOC-ENTITLEMENT-KEY-LIFECYCLE-v0.1_COMPILED_PLAN_SET.md. No build script. |

## Purpose

ForgeCustomer signs every AuthorForge entitlement snapshot and offline lease with one Ed25519 key. Its deployed key ring holds only that key (KI-FOC-20261105-001). A key change therefore invalidates every desktop at once, because desktops compile the trusted keys in (DIR-1).

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
| Author-Forge | `9f3a5d1a5` | `src-tauri/src/infrastructure/forgecustomer_entitlement.rs` (multi-key `TrustSet`, byte-compatibility fixture), `src-tauri/entitlement_trust_set.json` (empty), `.github/workflows/build-bundle.yml` |

## Status

Proposal. All acceptance criteria are NOT_RUN. "Inspected" marks a fact read in source at the pin.

## Read order and components

00 → 09, then the four siblings. 00 Index · 01 Rulings · 02 Authority and gaps · 03 Bindings, coverage and **reuse matrix** · 04 Contracts · 05 Mechanism · 06 Decisions · 07 Acceptance · 08 Work packages · 09 Operator review
