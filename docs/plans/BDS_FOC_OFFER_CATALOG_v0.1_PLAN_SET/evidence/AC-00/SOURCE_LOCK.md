# AC-00 — Source lock and evidence inventory

Read on 2026-11-08 (America/New_York). Quotes are verbatim.

**How each source was read:**

- **V** — re-read directly by the plan author in this change.
- **S** — read by a delegated read-only survey of Drive /Forge, and not re-read. The survey quoted it verbatim and reported no prompt injection.

## Repository sources

| Source | Pin | Read | What it says |
|---|---|---|---|
| forge `docs/plans/active/BDS_AF_ENTITLEMENT_LADDER_001/00_DECISION_PACKET.md` | `4c0fe91c` (#294) | V | R-2: "the software is a one-time purchase (Standard). Pro is a monthly subscription, paid through Stripe. Press_Forge is an add-on module with its own monthly fee." R-5: "Press_Forge is a separate module with a monthly fee." Grant table: "`pressforge` \| monthly add-on". Open: "does `pressforge` need `pro`, or only `license`? This packet assumes `license` only." Packet status: proposed; rulings R-1 to R-6 recorded. |
| forgecustomer `docs/plans/BDS_AUTHORFORGE_COMMERCE_LAUNCH_POLICY_v0.1_CP0_WP00.md` | `414f7e38c` | V | "Disposition: STOP — no CP1 implementation, paid pilot, or launch is authorized." Target model: "ForgeCustomer catalog identifies `one_time_major_version` or `subscription`". Tax candidate for Standard: "Standard grants the purchased major version rather than a subscription." Recovery: "commercial revocation never revokes manuscript recovery". Fee illustrations: Standard $99 → net $92.36; $149 → $139.16; Pro $19 → $17.35; $29 → $26.64. |
| forgecustomer `api/src/integrations/stripe.rs` | `414f7e38c` | V | Checkout sets `("mode", "subscription")` (line 83) and `allow_promotion_codes=true` (line 105). No `mode=payment` path. |
| forgecustomer `api/src/repositories/{licensing,entitlements}.rs`, migrations `0002`, `0004` | `414f7e38c` | V | Licenses are created from subscriptions (`sync_license_for_subscription`) or by an admin. Features: `authorforge.cloud.enabled`, `deep_analysis.enabled`, `premium.enabled`, `devices.max`; quota `cloud_tokens.monthly`. `licenses` has `expires_at`, `device_limit`; no update field. `uq_plan_versions_active` gives one active plan version per plan (`0002:66`). `deactivate_installation` exists. |
| Author-Forge `docs/plans/canebrake_press_pressforge_consolidation_plan_set_v1/02_decisions.md` (`BDS-AF-CANEBRAKE-001`, proposed, deferred) | `a5c1040d5` | V | D4: "PressForge merges into AuthorForge as a module gated by a dedicated feature entitlement key (working name `pressforge`), distinct from the `pro_*` subscription tier." Rejected: "Folding PressForge into `pro_active`". |
| Author-Forge `docs/plans/BDS_AF_PROTIER_v0.1_PLAN_SET/01_BINDING_RULINGS_AND_NON_GOALS.md` | `a5c1040d5` | V | R01 "Pro is cloud; Standard is local"; R02 the egress clause "in formal legal terms"; R05 "Beta testers become Pro through Stripe test mode". |
| Author-Forge `doc/system/37-pro-entitlement.md` | `a5c1040d5` | V | Line 163: "In local mode, entitlement enforcement is fully bypassed — no token required or checked." |
| Author-Forge PR #1011 (TIER-02) | open, `426d583ba` | V | The local Submission Ledger moves to Standard. |

## Drive sources — base product and pricing

| Source (Drive id) | Date | Read | Standing | What it says |
|---|---|---|---|---|
| Business Strategy Master Plan (`1WvD4wBYUozksbYdAIiJpuFpwFTCe2RRmke4PF2q2Xo4`) | 2026-07-30 | V | advisory | "Sell a complete premium local application through a one-time perpetual license." Pricing row: "AuthorForge \| $449 \| First 100: $299 \| $29/month; consider $290/year". "PressForge \| $299 add-on \| Test a bundle incentive \| Use AuthorForge Pro entitlements where possible; avoid a third confusing subscription". "A bounded founder benefit—locked cloud rate, priority onboarding, or named founder edition—is safer than indefinite support promises." "Define what "perpetual" covers: major version, maintenance period, devices, upgrades, and support." "Let cloud cancellation remove cloud service—not the customer's local product or local data." "These are working recommendations to validate with a founding cohort before public launch." |
| AuthorForge Market Analysis (`1uHmwKnjJK3Wg_eMS96qfZYqImeNEKwXLpX96ABi20Xo`) | 2026-07-29 | V (match on "Founding beta" and "capped at 250"); S (other quotes) | advisory | "Founding beta: $79 one-time, capped at 250 users, including 12 months of updates and a direct feedback channel." "General release core license: $149 one-time for the desktop product and the purchased major version." "Major-version upgrade: approximately $59-$79 every two to three years, with no forced upgrade." "Optional governed cloud deep-review packs: $10, $25, and $50 usage packs". "No subscription required to open, edit, export, or retain a manuscript." |
| BDS AuthorForge Direct Sales Research (`19sRN5nIckgqGHbcAAauHwikRRsDhSZF164xaPrlBcrU`) | 2026-08-30 | S | advisory | Standard "$99-$149 one-time for the purchased major version"; Pro "$19-$29 monthly; consider an annual option after retention evidence". "These are market-positioning hypotheses, not approved prices." |
| 03_COMMERCIAL_CATALOG_ALIGNMENT.md (`1sXazu4S2iKMERXT-rM-uZNDc2E_3xI3T`) | 2026-06-11 | S | superseded on this point by R05 (AUTHORITY_GAP-07) | "Every AuthorForge customer receives an included cloud allowance." "Do not hard-code plan names in product behavior". |
| BDS_RELEASE_WEEK_CHECKLIST (`1_qcZWpIR6cPWvhyleq6UKcAmp5sXs6ad_0dCUcZ6JQI`) | 2026-11-08 | S | — | No AuthorForge price. Infrastructure items only. |

## Drive sources — add-ons and Pro capabilities

| # | Candidate | Status | Lane | Commercial statement | Sources | Read |
|---|---|---|---|---|---|---|
| A.1 | **Press_Forge** (BDS-PFPE-v0.2; BDS_RESEARCH_PUBLICATION_PRESS_FORGE_v0.1) | Pre-Board brief 2026-09-12: "REWORK — HOLD NEW FUNCTIONAL SCOPE AND ALL LIVE-PROVIDER WORK"; code "landed, unaccepted candidate" | mixed: local sidecar, local SQLite for the v0.2 slice, DataForge for `/press` | Master Plan (above); v0.2 01: "whether PressForge is ultimately an AuthorForge tier or a separately sold product remain operator decisions"; superseded by R01, R02 (monthly add-on) | folders `1jvUS0DQDngalNvdVgu4VlukakIezljgw`, `1fAwfDnHcI0KMFP1648TyNsjoSTQbNRdb`; brief `1yddMlZkjP2KW2yYY-NLix8IUCcEzGg5hqrTqSkMRYDA` | S |
| A.2 | **World Engine** (BDS-AF-WORLD-ENGINE-NEXTGEN v0.1; review BDS-AF-WENG-REV-01) | "PRELIMINARY RESEARCH — NO IMPLEMENTATION AUTHORIZED" (2026-10-28); research-only on 2026-11-08 | local only: "100% offline local validation; zero-network baseline" | "Preserve World Engine as a possible premium add-on to AuthorForge"; "optional future authoring/export add-on; no runtime AI or cloud subscription needed for deterministic exports"; "Packaging/pricing and revenue forecasts remain separate research; no SKU or license approved." | folder `1z7-_1Ww5WR91MFSGJSuCRxbEy--VGrCb`; 01 `1y7Oz6o4OWB81X1O5DhYBCbb6V5nbcfgSYikrkwmRxVk`; 07 `1LfXw9OdSXA8MRKwhHlvE1Nrt-Q3CBFlRi6Ydyv2OQWQ`; 08 `1c1qr_5UUc5YuPu8LjsoENUSxesgK6OI8c9uUnyb8sks`; REV-01 `1GudqyYuwL_VCCrFm_tCoeoAWO2XfqRkZhZqyfGN_Z9Y` | S |
| A.3 | **Cover Forge marketplace / Panel Forge** | concept, February 2026 | hosted (own API, NeuroForge, R2, Stripe Connect) | "Cover Forge is **not** a tab inside AuthorForge. It is a standalone product"; "AuthorForge Core \| ~$99/year or one-time"; "AuthorForge Pro \| ~$249/year" (a graphic-novel tier, not today's Pro). Stale on tiers. Out of scope (01). | `1VsxBuKP3fx4hs_HssHtPAk2m_x51tToI` | S |
| A.4 | **Cloud narration (Story Reel, audiobook)** | draft v2, 2026-06-25; related audio work implemented | mixed | "`openai-tts` \| Pro cloud \| entitlement + quota required"; local voices ungated. A Pro capability, not an add-on. | folder `1V3e6v4XNM0gcfB_bLimIth9qoMUiJJnd`; `1utF8OymZrJcWLtfnxbqAHvVlNj-x4eYL` | S |
| A.5 | **Quench Submission Ledger** | implemented; moved to Standard by TIER-02 | local | Drive plan set titled "Pro" (2026-07-07); superseded on tier by R06 | `19ZwNuQ8FPKOqGdpbCTSbM0u4v_cCGAOa` (not decoded) | S |
| A.6 | **Hosted narrative craft diagnostics** | research essay, 2026-03-31 | hosted | "Pro users gain access to a baseline number of "Full Manuscript Scans" per month"; "Narrative Tokens" for one-off analysis. A Pro capability; supports credit packs. | `1M-qT12EDpApY3A1dVioizZ8_EqkCibGvWOMHEHoi2LA` | S |
| A.7 | **Google Drive storage** | lane implemented; profile layer "Implementation authorization: NONE" | hosted | "Pro-gated Drive routes in apps/api/src/routes/drive-storage.ts". A Pro capability. | `12puhiD2dHJTNbPHGSSPmjEbWhRwNoASCQikvcLV5ITg` | S |
| A.8 | **Native Scrivener import** | proposed, 2026-07-30 | local only | none; a Standard capability by D04 | `1Z7shURvjaUkqfqVKPar3_mX3VVNAg3PC` | S |

**Not add-ons:**

- The 2025-07 "Addons" folders hold UI-library code snapshots.
- Canebrake Press is the operator's own publishing imprint.
- Cartographer's Forge is listed as "Core" in the February 2026 module table.

## Conflicts and their disposition

| Conflict | Disposition |
|---|---|
| Press_Forge: "$299 add-on" (one-time, Master Plan, 2026-07-30) against "monthly add-on" (R01, R02, 2026-10-21) | The rulings are newer and bind. |
| Standard price: $449 / $99–$149 / $149 / ~$99 a year | DEC-9: deferred. |
| Pro price: $29 / $19–$29 / packs / ~$249 a year (different Pro) | DEC-9: deferred. |
| Included cloud allowance (2026-06-11) against Standard = local (R05) | AUTHORITY_GAP-07; DEC-0. |
| 2025-12 note: "subscription-based AI writing studio" | Stale; every 2026 source says a one-time core license. |

## Superseded or stale sources (do not cite as current)

- `authorforge_system_forge_plan_set_v2_SUPERSEDED.md`
- `authorforge_contextforge_plan_set_v1_SUPERSEDED.md`
- `authorforge_media_studio_plan_set_v2_SUPERSEDED.md`
- `AuthorForge_UX_UI_Refactor_v1_plan_set_SUPERSEDED.md`
- The `SUPERSEDED_…` research registers and ledgers
- The `QUARANTINED_DUPLICATE__…` documents
- "Application Packaging for Online Sales" (`1lBrQ5iI3eYbv3rbdYYE0QbJwPvn982Uv3oru1BTSv4E`). The Direct Sales Research marks it contaminated by an unrelated product.
- The February 2026 Cover Forge strategy and the 2025-12 marketing note, for tiers and prices.
- "15_CLOUD_ARCHITECTURE_AND_USAGE_TIERS.md" is about VibeForge, not AuthorForge.

## Not read (limits of this inventory)

- World Engine 02–06, and the "price points discussed previously" in its 07. None were found.
- The contents of these zips: the PressForge v0.2 and v0.1 plan sets, the Quench Pro plan set, the audio playback and VOICEFORGE closeouts.
- Story Reel `11_DECISIONS_TO_LOCK.md`.
- `COVER_FORGE_SPEC.md`.
- `CARTOGRAPHERS_FORGE_SPEC.md` and the map engine plan.
- The ForgeImages plan set and the Research Desk plan.
- Pages after the first of the `research/` folder listing.
- The full Plans folder (it was filtered by keywords).
- The 2026-10-21 ruling has no Drive copy. Its only record is the forge repository packet above.
