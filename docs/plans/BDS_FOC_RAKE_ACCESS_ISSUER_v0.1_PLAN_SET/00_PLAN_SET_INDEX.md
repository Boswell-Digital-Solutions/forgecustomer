# 00 — Plan identity, index and read order

This component gives the plan identity, the purpose, the source state and the read order.

| Field | Value |
|---|---|
| Plan ID | BDS-FOC-RAKE-ACCESS-ISSUER-v0.1 |
| Title | ForgeCustomer Rake access-token issuer — token exchange for BDS applications |
| Lifecycle | authorized |
| Authority | Operator ruling 2026-11-11 ("do it"): DEC-1 to DEC-5 accepted as recommended; WP-00 and WP-01 authorized. Operator 2026-11-11 ("check and yes"): WP-02 and WP-03 authorized; DEC-6 ruled A. WP-01, WP-02 and WP-03 merged on the operator's word. No key, configuration or deployment is authorized; that is G4, the operator's. Before that: the D-JWTI rulings fixed the design and authorized no work. |
| Decision owner | Charles (Charlie) Boswell, BDS operator |
| Prepared | 2026-11-11 America/New_York |
| Parent / extension target | rake `docs/reviews/RAKE_JWT_ISSUER_ARCHITECTURE_REVIEW_2026-11-10.md` and `docs/evidence/D-JWTI/2026-11-11_OPERATOR_RULINGS.md` (rake `caac54c1a`). Findings: rake `RAKE-EXECUTION-JWT-NO-ISSUER-20261109`; Author-Forge `KI-AUF-20261106-003`. Authority row: forge `CLAUDE.md` "ForgeCustomer (application-token issuer; ruled 2026-11-11, not built)" (forge #411). Companion: forge `docs/architecture/rake_credential_authority_review.md`. |
| Proposed canonical owner | Boswell-Digital-Solutions/forgecustomer for the issuer, its key and the token contract. Boswell-Digital-Solutions/rake for the verifier. Boswell-Digital-Solutions/Author-Forge for the first application client. Each work package pins its repository SHA. |
| Primary candidate consumer | forgecustomer `api/src/routes/`, `api/src/services/signing.rs`, `api/src/config.rs`, `api/src/state.rs`, `supabase/migrations/`, `contracts/`; rake `auth/jwt_handler.py`, `auth/execution_access.py`, `auth/tenant_context.py`, `config.py`, `tests/conftest.py`, `service_contract.v1.json`; Author-Forge `apps/api/src/routes/rake.ts`. |
| Canonical Drive destination | /Forge/Plans/BDS_FOC_RAKE_ACCESS_ISSUER_v0.1_PLAN_SET (Drive folder `1S9fhDpmwCLD-6NYg1Wbd8MTfteFB4_6N`, uploaded 2026-11-11). It holds the compiled plan, the four siblings, and `BDS_FOC_RAKE_ACCESS_ISSUER_v0.1_PLAN_SET_3448007.tar.gz` (35,500 bytes, SHA-256 `9ff11a10b546b156719d8729a84562d3ac6bb1fc39c6965489f311533fda0feb`). The archive is the plan set at forgecustomer `3448007`, built with `git archive --mtime=<commit time>` and `gzip -n -9` (two builds byte-identical). The repository copy is the current one; the Drive copy is refreshed at each lifecycle change. |
| Compilation | Components 00–09, concatenated by hand into BDS-FOC-RAKE-ACCESS-ISSUER-v0.1_COMPILED_PLAN_SET.md. No build script. |

## Purpose

Rake accepts an access JWT at its execution boundary. No system issues that JWT. Rake verifies it with one shared HS256 secret and checks no `iss` and no `aud`. Every holder of the secret can mint a token, and Rake cannot tell an authorized issuer from any other holder.

The operator ruled the target on 2026-11-11:

- ForgeCustomer is the issuer (D-JWTI-1 option b, D-JWTI-2).
- Supabase project `forgecustomer` stays the login (D-JWTI-4).
- Browsers do not call Rake. An application reaches Rake only through its own backend (D-JWTI-5).
- The token is asymmetric, with pinned keys and required `iss`, `aud` and `exp` (D-JWTI-6).
- Only a token from ForgeCustomer counts for the DEC-RAKE-JWT-01 acceptance path (D-JWTI-7).

This plan builds that design from code that already works:

- ForgeCustomer's Supabase verifier (`api/src/auth/customer.rs`) and its `CustomerContext::require_active` gate (`api/src/auth/extract.rs:83`);
- ForgeCustomer's Ed25519 signer and verifying key ring (`api/src/services/signing.rs:30-142`) and its key-publication route (`GET /v1/entitlements/keys`);
- ForgeCustomer's entitlement assembly (`GET /v1/entitlements/current`, `POST /v1/entitlements/check`) and the feature key `authorforge.cloud.enabled` (`supabase/seed.sql:18`);
- Rake's scope and tenant binding (`auth/execution_access.py:24-88`), which stays as it is;
- Author-Forge's Rake proxy (`apps/api/src/routes/rake.ts`), which already keeps every Rake call in the backend.

It writes new code only where nothing exists: the exchange route, a dedicated signing key, a mint audit record, the pseudonymous tenant mapping, the pinned-key verifier in Rake and the client call in Author-Forge.

## Source state used to prepare this plan

| Repository | Branch | Commit | Read |
|---|---|---|---|
| forgecustomer | `main` | `3a2ce487e` | `doc/system/` 03, 10, 11, 12; `api/src/services/signing.rs`; `api/src/auth/extract.rs`; `api/src/domain/installation.rs` (`device_public_key`); `api/src/routes/mod.rs`; `api/Cargo.toml` (`jsonwebtoken` 9, `ed25519-dalek` 2); `supabase/seed.sql`; plan `BDS_FOC_ENTITLEMENT_KEY_LIFECYCLE_v0.1_PLAN_SET` (form and key custody precedent) |
| rake | `master` | `caac54c1a` | `doc/system/10_service-contract/05-api-layer.md`, `20_runtime/07-backend-services.md`, `50_operations/17-configuration.md`; the review and the D-JWTI rulings. The auth code (`auth/`, `config.py`, `tests/conftest.py`, `main.py`, `service_contract.v1.json`) is byte-identical to `c26b095`, the review's pin (`git diff --stat c26b095 origin/master` is empty). |
| Author-Forge | `main` | `1856b3b73` | `apps/api/src/routes/rake.ts` (lines 1-110); `docs/KNOWN_ISSUES.md` `KI-AUF-20261106-003`; `src-tauri/src/fleet_updates.rs:1628` (no `device_public_key` sent) |
| bds_website | `main` | `3b6a2e51d` | `git grep -i rake`: no match. bds_website does not call Rake at this commit. |
| forge | `main` | `88fda8241` | `CLAUDE.md` authority table; `docs/canonical/plan_registry_v1.json` |

"Inspected" marks a fact read in source at the pin. "Review" marks a fact taken from the rake review at its pin.

## Status

Authorized 2026-11-11 (receipts in `evidence/RULINGS/`). Records updated 2026-11-12 on the operator's authorization.

- Merged: WP-01 (forgecustomer #53, `6d21e62`), WP-02 (forgecustomer #55, `8e0cf28`), WP-03 (rake #188, `9b7af47`, with forge #424, `27efccb`).
- AC-00, AC-01, AC-02, AC-03, AC-05 and AC-06 PASS. AC-04 PASS except its deletion row, which is UNVERIFIED. AC-07 to AC-09 NOT_RUN.
- G0, G1 and G3 PASS. G2 is open on the AC-04 deletion row. G4 waits on G2 and on the operator.

## Read order and components

00 → 09, then the four siblings. 00 Index · 01 Rulings · 02 Authority and gaps · 03 Bindings, coverage and reuse · 04 Contract · 05 Mechanism · 06 Decisions · 07 Acceptance · 08 Work packages · 09 Operator review
