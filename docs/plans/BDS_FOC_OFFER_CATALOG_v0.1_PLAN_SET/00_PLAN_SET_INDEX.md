# 00 — Plan identity, index and read order

This component gives the plan identity, the purpose, the source state and the read order.

| Field | Value |
|---|---|
| Plan ID | BDS-FOC-OFFER-CATALOG-v0.1 |
| Title | AuthorForge offer catalog and licensing model — Standard, updates, Pro, add-ons and the founders program |
| Lifecycle | proposed |
| Authority | Planning only. The operator asked for this plan on 2026-11-08 ("yes … so we have all the evidence documented on our next steps"). It authorizes no implementation, migration, Stripe object, price, legal text, deployment or merge. It holds no exception to the CP0 STOP of the commerce launch policy. |
| Decision owner | Charles (Charlie) Boswell, BDS operator |
| Prepared | 2026-11-08 America/New_York |
| Parent / extension target | Extends ForgeCustomer `BDS_AUTHORFORGE_COMMERCE_LAUNCH_POLICY_v0.1` (CP0/WP00, 2026-08-31), its target model and its CP1 work packages. Takes the commercial rulings of forge `BDS-AF-ENTITLEMENT-LADDER-001` (R-2, R-5, 2026-10-21). Consumer plans: Author-Forge `BDS-AF-PROTIER-v0.1`, `BDS-AF-RELACC-v0.1`, `BDS-AF-CANEBRAKE-001` (D4); ForgeCustomer `BDS-FOC-ENTITLEMENT-KEY-LIFECYCLE-v0.1`. Tier ruling TIER-02 (2026-11-08, Author-Forge #1011). |
| Proposed canonical owner | Boswell-Digital-Solutions/forgecustomer (catalog, orders, licenses, entitlements). Author-Forge consumes signed decisions only. bds_website renders offers. Each work package pins its repository SHA. |
| Primary candidate consumer | forgecustomer `supabase/migrations/`, `api/src/repositories/{entitlements,licensing,commerce}.rs`, `api/src/integrations/stripe.rs`, `api/src/routes/`; Author-Forge `src-tauri/src/infrastructure/connection_mode.rs`, `apps/frontend/src/lib/entitlementVerdict.ts`; bds_website store and account surfaces. |
| Canonical Drive destination | /Forge/Plans/BDS_FOC_OFFER_CATALOG_v0.1_PLAN_SET (not yet uploaded; see 09 §Handoff). The repository copy in forgecustomer `docs/plans/` is the byte-exact one. |
| Compilation | Components 00–09, concatenated by hand into BDS-FOC-OFFER-CATALOG-v0.1_COMPILED_PLAN_SET.md. No build script. |

## Purpose

AuthorForge will sell more than one thing: an owned desktop product, a cloud subscription, and add-on modules. The evidence for what to sell, and how, is spread across rulings, plans and research documents that disagree in places. ForgeCustomer can model only part of it today.

This plan does three things:

1. It records every source in one inventory, with what each one says, its date and whether a later ruling replaces it (evidence/AC-00).
2. It proposes one offer catalog that ForgeCustomer can represent: the offer kinds, their rights, what happens when a payment stops, and the founders program.
3. It lists the operator decisions that the catalog needs, each in plain language with a recommendation (09).

## What this plan does not change

It does not choose prices. Every price in this plan is a sourced hypothesis or an illustration. It does not lift the CP0 STOP: Stripe Managed Payments eligibility, the account-effective fee schedule and the tax codes stay unproven, and every implementation work package here waits for them.

## Source state used to prepare this plan

| Source | Pin |
|---|---|
| forgecustomer | `origin/main` `414f7e38c` |
| Author-Forge | `origin/main` `a5c1040d5`; PR #1011 (TIER-02) open |
| forge | `origin/main` `50a208f25`; ladder packet at `4c0fe91c` (#294) |
| Google Drive /Forge | read 2026-11-08; file ids in evidence/AC-00/SOURCE_LOCK.md |

## Read order

1. 01 — rulings that bind, and non-goals
2. 02 — authority boundary and AUTHORITY_GAPs
3. 03 — governing protocols and requirement coverage
4. 04 — the offer and rights contract
5. 05 — the catalog, the add-ons and the founders mechanism
6. 06 — decision semantics
7. 07 — acceptance and evidence
8. 08 — work packages and gates
9. 09 — operator decisions and handoff
10. evidence/AC-00/SOURCE_LOCK.md — the source inventory
