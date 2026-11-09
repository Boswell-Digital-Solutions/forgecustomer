# BDS-FOC-OFFER-CATALOG-v0.1 — Compiled plan set

| Field | Value |
|---|---|
| Plan ID | BDS-FOC-OFFER-CATALOG-v0.1 |
| Lifecycle | authorized (rulings only; the central registry reads `approved`) |
| Compilation | Components 00–09, concatenated by hand. No build script. An edit to a component is mirrored here in the same change. |

# 00 — Plan identity, index and read order

This component gives the plan identity, the purpose, the source state and the read order.

| Field | Value |
|---|---|
| Plan ID | BDS-FOC-OFFER-CATALOG-v0.1 |
| Title | AuthorForge offer catalog and licensing model — Standard, updates, Pro, add-ons and the founders program |
| Lifecycle | authorized (rulings only; the central registry reads `approved`) |
| Authority | DEC-0 to DEC-9 ruled by the operator on 2026-11-08 ("OK I concur with recommendations - proceed", confirmed as all ten). WP-00 and WP-01 are complete. It authorizes no implementation, migration, Stripe object, price, legal text, deployment or merge. WP-02 onward need the CP0 entry evidence and a separate authorization. It holds no exception to the CP0 STOP of the commerce launch policy. |
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

---

# 01 — Binding rulings and non-goals

This component lists the rulings that bind this plan and what the plan does not do.

## Rulings in force (already made by the operator)

| ID | Date | Ruling | Source |
|---|---|---|---|
| R01 | 2026-10-21 | The software is a one-time purchase (Standard). Pro is a monthly subscription, paid through Stripe. Press_Forge is an add-on module with its own monthly fee. | forge `BDS-AF-ENTITLEMENT-LADDER-001` R-2 |
| R02 | 2026-10-21 | Press_Forge is a separate module with a monthly fee. | same, R-5 |
| R03 | 2026-10-21 | When a Pro customer uses the whole monthly allowance, AuthorForge falls back to local with an honest "reduced quality" label, and offers to buy more credits. | same, R-4 |
| R04 | 2026-10-20 / 10-21 | Until the initial release, testers can turn Pro and Press_Forge on and off in Settings. The switches compile out at 1.0.0. | same, R-1 and R-6; Author-Forge #883 |
| R05 | 2026-10-21, restated 2026-11-06 | Pro is cloud; Standard is local. Beta testers become Pro through Stripe test mode. The Pro license states the content egress in formal legal terms. | Author-Forge `BDS-AF-PROTIER-v0.1` 01, R01, R02, R05 |
| R06 | 2026-11-08 | The local Submission Ledger is a Standard capability (TIER-02). | Author-Forge #1011 |
| R07 | 2026-08-31 | ForgeCustomer is the sole commercial, license and entitlement authority. Standard is a first-class one-time order. Pro stays a subscription. A commercial revocation never revokes manuscript recovery. Managed Payments first, never an automatic fallback. | forgecustomer `BDS_AUTHORFORGE_COMMERCE_LAUNCH_POLICY_v0.1` §1, §13, §14 |
| R08 | 2026-08-31 | CP0 disposition is STOP. No CP1 implementation, paid pilot or launch until Managed Payments eligibility, the account-effective fees and the tax codes are proven. | same, §1 |
| R09 | 2026-11-06 | Admin overrides cannot bypass the subscription cloud gate. | forgecustomer `repositories/entitlements.rs:307`, cited in the PROTIER plan |
| R10 | 2026-11-05 | "Simple but secure": a credential or commercial rule that a reader cannot hold in their head has an unexamined corner. | operator steer, P3 plan; carried here as a design rule |

## Binding design rules this plan adds (bind on admission)

| ID | Rule |
|---|---|
| D01 | Each grant is independent: the Standard license, the update plan, Pro and each add-on. Ending one never ends another. |
| D02 | An owned grant (a one-time purchase) never expires because a later payment stops. Only a refund or a lost dispute revokes it, by R07. |
| D03 | A recurring grant ends at the end of its paid period. Its local data stays. |
| D04 | A grant for work that runs on BDS-hosted services is recurring. A grant for work that runs on the author's machine may be owned. A local feature behind a recurring grant is an explicit, recorded exception (TIER review principle, R06). |
| D05 | No "lifetime" grant for hosted work. A founder benefit may lock a price, not remove a usage limit. |
| D06 | Every offer has a written cancellation, refund and dispute outcome before it is sold. |

## Non-goals

- Choosing prices. 09 records the sourced price hypotheses and asks for no price ruling.
- Lifting or working around the CP0 STOP (R08).
- Live Stripe objects, live prices, tax registration or filing.
- Legal text. Counsel writes the EULA, the Pro terms, the founders terms and the beta agreement. This plan names what each one must cover.
- Building any add-on's features. Press_Forge, World Engine and others keep their own plans and gates.
- The Cover Forge marketplace. Its February 2026 strategy is a separate, two-sided platform with its own revenue model (evidence/AC-00 §A.3). It is out of scope.
- Team, seat or enterprise licensing. No AuthorForge source proposes it for the first release.

---

# 02 — Authority boundaries and gaps

This component states who owns each decision and which gaps block the work.

## Authority

| Concern | Owner | Notes |
|---|---|---|
| Offer catalog, orders, licenses, entitlements, founder seats | ForgeCustomer | R07. No other system grants a commercial right. |
| Payment, invoice, refund and dispute truth | Stripe | ForgeCustomer stores normalized facts and object references only (launch policy §5). |
| Prices, offer terms, the founders cap | The operator | No agent sets or approves a price. |
| Legal text (EULA, Pro terms, founders terms, beta agreement, privacy) | The operator and counsel | Drafts by agents are for counsel review only. |
| Tax classification of each offer | The operator and a qualified tax adviser | Launch policy §4 gives candidates only. |
| Showing a grant and enforcing it on the desktop | Author-Forge shell | It verifies ForgeCustomer-signed decisions only (RELACC). |
| Offer display and checkout start | bds_website | It renders the ForgeCustomer catalog. No rights come from a redirect. |
| Each add-on's features | That add-on's own plan | This plan owns only the commercial grant. |

## AUTHORITY_GAPs

| ID | Gap | Blocks | Disposition |
|---|---|---|---|
| AUTHORITY_GAP-01 | The CP0 STOP is active: Managed Payments eligibility, the account-effective fees and the Standard and Pro tax codes are not proven (R08). | WP-02 to WP-08 | Inherited. Closes only with the launch policy's CP1 entry evidence. |
| AUTHORITY_GAP-02 | ForgeCustomer has no one-time purchase lane. Checkout runs only in `mode=subscription` (`api/src/integrations/stripe.rs:83`). Licenses come only from a subscription (`repositories/licensing.rs` `sync_license_for_subscription`) or an admin insert. | Standard, the update plan, one-time add-ons, the founders Standard offer | Designed as launch-policy CP1-WP01 to WP04. This plan adds offer kinds to that design (04). |
| AUTHORITY_GAP-03 | No module dimension in the entitlements. Features today are `authorforge.cloud.enabled`, `deep_analysis.enabled`, `premium.enabled`, `devices.max` and the `cloud_tokens` quota. No `pressforge` or other add-on key exists in `api/src`. | WP-05 | New feature keys and a prerequisite rule (05 §C). |
| AUTHORITY_GAP-04 | No update entitlement. `licenses` has `expires_at` and `device_limit` but no `updates_until` or major-version field (`supabase/migrations/0004_licensing.sql`). | WP-04, DEC-2 | New fields, after DEC-2. |
| AUTHORITY_GAP-05 | No founder seat counter or numbered founder record. Promotion codes are on (`stripe.rs:105`) but a coupon limit is not an atomic cap. | WP-03 | New mechanism (05 §D). |
| AUTHORITY_GAP-06 | No legal text exists for any offer. | Selling any offer | Operator and counsel (WP-07). |
| AUTHORITY_GAP-07 | Two catalog records disagree on whether the base product has cloud. The 2026-06-11 catalog alignment (Drive `1sXazu4S2iKMERXT-rM-uZNDc2E_3xI3T`) gives every customer "an included cloud allowance". RELACC opens cloud only for an `active` or `trialing` subscription, and R05 says Standard is local. | WP-02 seed data | **Resolved 2026-11-08 by DEC-0:** the `authorforge_included` plan carries no cloud allowance. |
| AUTHORITY_GAP-08 | Standard offline use is not settled. An offline lease lasts 14 days by default. For an owned license that is a check-in every 14 days, which conflicts with D02. The recovery certificate (launch policy §9) covers opening and exporting work, not normal use. | WP-04 | **Resolved 2026-11-08 by DEC-8:** local features never check in; the snapshot and lease serve hosted grants and updates only. |

## Boundary rules

- An add-on grant never widens another grant. Press_Forge does not turn on Pro cloud features, and Pro does not turn on Press_Forge.
- A founder benefit is a property of a grant (a price, a longer update window), not a separate grant with its own rights.
- A desktop build that has no key for a grant shows the grant as absent, never as active.

---

# 03 — Protocol bindings and requirement coverage

This component names the governing protocols and connects each requirement to its design, work package and acceptance record.

## Governing material

| Source | Standing | Applies to |
|---|---|---|
| BDS-PSS-001 (Drive `1yWQUGn4yaSPJcQrui3Y5LBcycvCnAHyxhxayk_RiKCY`, amended 2026-10-21) | binding | The form of this plan set |
| BDS-HFPC-001 | binding (through PSS-001 §3.7) | The operator decision briefs in 09 |
| forgecustomer `BDS_AUTHORFORGE_COMMERCE_LAUNCH_POLICY_v0.1` | binding (R07, R08) | The order, rights, refund, dispute and recovery model; the CP1 sequence |
| forge `BDS-AF-ENTITLEMENT-LADDER-001` | rulings binding (R01–R04); packet proposed | The three grants and the allowance behavior |
| Workspace `CLAUDE.md` deployment rule | binding | No backwards-compatibility layer; a cross-repository contract must agree when the second change merges |
| ASD-STE100 (`docs/protocols/BDS_ASD_STE100_v1.md`, forge) | binding for prose | This plan's text |
| Business Strategy Master Plan (Drive `1WvD4wBYUozksbYdAIiJpuFpwFTCe2RRmke4PF2q2Xo4`, 2026-07-30) | advisory | Pricing hypotheses, founder benefit guidance |
| Market Analysis (Drive `1uHmwKnjJK3Wg_eMS96qfZYqImeNEKwXLpX96ABi20Xo`, 2026-07-29) | advisory | Founding offer shape, upgrade cadence |
| Direct Sales Research (Drive `19sRN5nIckgqGHbcAAauHwikRRsDhSZF164xaPrlBcrU`, 2026-08-30) | advisory | Price ranges, the subscription-only gap |

Advisory material sets no price and creates no authority.

## Requirements

| ID | Requirement | Design | WP | AC |
|---|---|---|---|---|
| REQ-01 | A Standard license is owned. No later payment state ends it; only a refund or a lost dispute revokes it. | 04 §B, D02 | WP-02 | AC-02 |
| REQ-02 | A recurring grant ends at the end of its paid period and leaves local data intact. | 04 §B, D03 | WP-02, WP-05 | AC-02, AC-05 |
| REQ-03 | Each add-on has its own feature key and its own prerequisite rule. Ending an add-on ends only that add-on. | 05 §C | WP-05 | AC-05 |
| REQ-04 | The founders cap is exact under concurrent checkouts. An abandoned checkout frees its seat. | 05 §D | WP-03 | AC-03 |
| REQ-05 | A founder benefit locks a price or extends an update window. It never removes a usage limit. | D05, 05 §D | WP-03 | AC-03 |
| REQ-06 | An update entitlement controls which builds a license may install, never whether owned builds keep running. | 05 §B | WP-04 | AC-04 |
| REQ-07 | Pro allowance exhaustion follows R03: local fallback with a label, plus a credit offer. | 05 §E | WP-06 | AC-06 |
| REQ-08 | Every offer has written cancellation, refund and dispute outcomes and legal text before sale. | D06, 04 §B | WP-07 | AC-07 |
| REQ-09 | A recovery certificate stays valid for existing local work through any commercial revocation. | R07 | WP-02, WP-04 | AC-02, AC-04 |
| REQ-10 | Rights come only from verified, deduplicated webhooks, never from a redirect or a desktop toggle (after 1.0.0). | R07, R04 | WP-02, WP-08 | AC-02, AC-08 |

## Dispositions

- **Prices:** deferred to the operator. Effect: 05 uses placeholders. No gate in this plan needs a price except WP-08, which needs a test price.
- **Tax classification of the update plan and of add-ons:** unresolved. The launch policy covers Standard and Pro only. Effect: WP-07 must extend the tax memo before WP-02 seeds those offers.
- **Cover Forge marketplace, teams and seats:** excluded (01 non-goals). No binding requirement is set aside.

---

# 04 — The offer and rights contract

This component defines the offer kinds, the rights each grant carries, and the provenance each record keeps.

## A. Offer kinds

The launch policy target model has two offer kinds: `one_time_major_version` and `subscription`. This plan proposes six. Each one extends that model and keeps its rule that a one-time offer never references subscription state.

| Offer kind | Billing | Grant it creates | Ends when | Example |
|---|---|---|---|---|
| `one_time_major_version` | one payment, `mode=payment` | Standard license for one major version | refund or lost dispute only | AuthorForge Standard 1.x |
| `update_plan` | recurring, `mode=subscription` | Extends the license's update window (`updates_until`) | the paid period ends; the license and every build it already may install stay | Standard Updates and Support (only if DEC-2 = A+) |
| `subscription` | recurring | A hosted-service grant with an allowance | the paid period ends | AuthorForge Pro |
| `addon_subscription` | recurring | One add-on feature key | the paid period ends | Press_Forge (R01, R02) |
| `one_time_addon` | one payment | One add-on feature key for one major version | refund or lost dispute only | World Engine, if DEC-5 = one-time |
| `credit_pack` | one payment | A one-time quota increment that expires at a stated date | used or expired | Pro allowance top-up (R03) |

`credit_pack` and both one-time kinds are sold through the same order lane as Standard (launch policy CP1-WP01 to WP03).

## B. Rights matrix

| Event | Standard license | Update plan | Pro | Add-on (recurring) | Add-on (one-time) | Recovery certificate |
|---|---|---|---|---|---|---|
| Paid | active | `updates_until` extended | active | active | active | issued |
| Recurring payment fails (dunning) | unchanged | grace, then lapse | non-cloud terms only (current code) | suspended | unchanged | unchanged |
| Cancelled / period ends | unchanged | lapsed; owned builds stay | ended | ended | unchanged | unchanged |
| Full refund | revoked-final | revoked; window returns to the last paid date | revoked | revoked | revoked-final | stays valid for existing work |
| Dispute open | suspended | suspended | suspended | suspended | suspended | stays valid |
| Dispute lost | revoked-final | revoked | revoked | revoked | revoked-final | stays valid |

The Standard and Pro rows restate the launch policy §7. The other columns are this plan's proposal.

## C. Records and provenance

Each record keeps the launch policy's provenance: merchant model, policy version, tax-code snapshot, Stripe object reference and event ID. This plan adds:

| Record | New fields (proposed) | Why |
|---|---|---|
| `offers` (the launch policy's offer concept) | `offer_kind`, `requires` (a list of grant keys), `founder_eligible` | Prerequisites and the founders program read from the catalog, not from code |
| `licenses` | `major_version`, `updates_until` (nullable) | REQ-06; AUTHORITY_GAP-04 |
| `founder_seats` | `seat_number`, `program_id`, `customer_id`, `status` (`reserved`, `confirmed`, `released`), `reserved_until`, `checkout_session_id` | REQ-04 |
| `founder_programs` | `program_id`, `cap`, `opens_at`, `closes_at`, `benefits` (catalog references), `terms_version` | One row per program; the cap is data, not code |
| `entitlement_snapshot` features | one boolean key per add-on, for example `authorforge.pressforge.enabled`; `authorforge.updates_until` | The desktop reads grants from the signed snapshot only |

Names are working names. WP-02 fixes them against the launch policy CP1-WP01 schema. No field is added for an installed base: none exists (workspace deployment rule).

---

# 05 — Core mechanism specification

This component specifies the catalog, the update window, the add-on grants, the founders program and the Pro allowance.

## A. The proposed catalog

| Offer | Kind | Requires | Lane | Source for the offer itself |
|---|---|---|---|---|
| AuthorForge Standard | `one_time_major_version` | — | local | R01, R07 |
| Standard Updates and Support | `update_plan` | Standard | none (delivery only) | **not offered at the first release** (DEC-2 = E) |
| AuthorForge Pro | `subscription` | Standard (DEC-1) | hosted | R01, R05 |
| Pro credit pack | `credit_pack` | Pro | hosted | R03 |
| Press_Forge | `addon_subscription` | Standard only (DEC-4) | mixed (local sidecar, hosted NeuroForge and DataForge) | R01, R02; `BDS-AF-CANEBRAKE-001` D4 |
| World Engine | `one_time_addon`, a module inside AuthorForge (DEC-5) | Standard | local only | World Engine plan set (research only); price deferred |
| Founders program | a property of the offers above | — | — | DEC-3: 100 numbered seats |

Pro already covers these hosted capabilities, so none of them is a separate add-on: cloud narration voices, hosted deep analysis, Google Drive storage, Rake research missions (evidence/AC-00 §A.4, §A.6, §A.7). The local Submission Ledger is Standard (R06).

## B. The update window (DEC-2)

**Ruled 2026-11-08: E** for the first release. A+ stays specified below for the review after a year of sales data.

Two designs keep D02. Both need AUTHORITY_GAP-04 closed.

**E — paid major upgrades (the launch policy baseline).** A license covers one major version, with every update inside that version. A new major version is a new purchase, at an upgrade price. The Market Analysis proposes "approximately $59-$79 every two to three years, with no forced upgrade." `updates_until` is not needed; `major_version` is.

**A+ — included updates, then an optional renewal with a fallback.** A license includes updates for a fixed window (for example 12 months). An `update_plan` subscription extends the window. When the window ends, the author keeps every build released inside the window, forever.

Desktop rule for both: the shell reads `major_version` or `updates_until` from the signed snapshot. It refuses to install a build outside the right, and states why. It never stops a build that is already installed and allowed. A missing or unreadable snapshot never blocks local work (REQ-09).

Illustration only, with placeholder prices ($149 Standard, $59 a year renewal, 40 % renewal rate): E earns $149 in the first three years, plus any upgrade sales. A+ earns about $149 + 0.4 × $59 × 2 ≈ $196. The renewal rate is an assumption. No source measures it.

## C. Add-on grants

- Each add-on has one feature key in the signed snapshot, for example `authorforge.pressforge.enabled`. The `BDS-AF-CANEBRAKE-001` D4 decision already chose a dedicated key, separate from the `pro_*` states.
- An offer's `requires` list is checked when the grant is issued and on every snapshot. A missing prerequisite leaves the add-on off and gives a stated reason.
- Ending an add-on changes only its own key (REQ-03).
- The pre-release switch for an add-on (R04) ships only when that add-on has real features in AuthorForge.

## D. The founders program (DEC-3)

**Shape.** One `founder_programs` row: a cap, an open date and a close date, and the benefits as references to catalog offers. Founders get numbered seats.

**Seat reservation.**

1. When ForgeCustomer creates a founder checkout session, it reserves the lowest free seat number in one transaction, with `reserved_until` about 30 minutes ahead.
2. When no seat is free, ForgeCustomer creates no founder session, and it states that the program is full.
3. The verified `checkout.session.completed` (or `async_payment_succeeded`) webhook confirms the seat once. A replayed event changes nothing.
4. An expired reservation is released, and its number becomes free.
5. A full refund or a lost dispute releases the seat's benefits. The program decides whether the seat number returns to the pool (DEC-3).
6. A Stripe promotion code with `max_redemptions` may sit beside the counter as a second limit. The counter is the authority.

**Benefits allowed (D05).** A founder price on Standard. A longer update window (A+) or a free first upgrade (E). A Pro price locked while the subscription is continuous. A founder badge or number.

**Benefit not allowed.** Lifetime Pro, or any unlimited hosted use.

**Locked Pro price.** A founder Pro plan version with its own Stripe price. Only one plan version is active for new sign-ups, so existing founders keep their price when the public price changes (forgecustomer `uq_plan_versions_active`, `supabase/migrations/0002_product_catalog.sql:66`). A lapse longer than a stated grace period ends the founder rate.

## E. Pro allowance and credits

- One Pro tier at launch, with the monthly allowance (`cloud_tokens` quota) as today.
- A `credit_pack` adds a one-time quota increment with a stated expiry. When the allowance is used, R03 applies.
- An annual Pro price is a second Stripe price on the Pro plan version. The Direct Sales Research says to "consider an annual option after retention evidence". This plan follows that.
- A second Pro tier waits for usage evidence (DEC-6).

---

# 06 — Decision semantics

This component states how each operator decision and each system decision is made and recorded.

## Operator decisions (DEC-0 to DEC-9)

- Each decision in 09 takes one of: **accept** the recommendation, **choose** a named alternative, **revise** (the operator states the change), or **defer** (the decision stays open and the work packages it blocks stay blocked).
- A ruling is recorded in 09 with its date and the operator's words, and in `plan_set.yaml` `decisions`, in the same change.
- A reviewer recommendation, a pasted review or an agent summary is not a ruling.
- A ruling on a decision authorizes no work package. Authorization is a separate operator act, recorded in 09.

## System decisions (for the implementation work packages)

| Decision | Inputs | Outcomes | Fail-closed default |
|---|---|---|---|
| Issue a grant | a verified, deduplicated webhook fact; the offer's `requires` list | issue once; or refuse with a reason code | refuse |
| Reserve a founder seat | the program row; free seats | reserve the lowest free number; or "program full" | no founder session |
| Confirm a founder seat | a verified payment event for the reserving session | confirm once | the reservation expires |
| Show an add-on as active (desktop) | the signed snapshot's key | active; or absent with a reason | absent |
| Allow a build to install (desktop) | `major_version` or `updates_until` from the signed snapshot; the build's version and date | allow; or refuse with the reason | refuse the install; never stop the installed build |
| Open local work | none; the recovery certificate when the license is revoked | always opens | opens (REQ-09) |

A refusal never deletes, encrypts or hides manuscript data.

---

# 07 — Acceptance and evidence

This component gives each acceptance criterion, its method, its expected observation and where its evidence goes.

All evidence goes under `evidence/<AC-ID>/` in this plan set. A planned check is not evidence. A skipped or unavailable check does not satisfy its criterion.

| AC | WP | Criterion | Method | Expected observation | Baseline and environment | Limits |
|---|---|---|---|---|---|---|
| AC-00 | WP-00 | Every source this plan relies on is inventoried with its id, date, standing and what it says. Conflicts and superseded sources are named. | Read the sources; spot-check quoted text against the source. | `evidence/AC-00/SOURCE_LOCK.md` exists; each quote used in 01–05 resolves to a listed source. | forgecustomer `414f7e38c`, Author-Forge `a5c1040d5`, forge `50a208f25`, Drive read 2026-11-08 | Some Drive files were not read (listed in the source lock). |
| AC-01 | WP-01 | DEC-0 to DEC-9 each have a recorded outcome (a ruling or a deferral). | Read 09 and `plan_set.yaml`. | Every decision row shows a dated outcome in both places. | this plan set | — |
| AC-02 | WP-02 | Grants follow the 04 §B rights matrix for every offer kind. Standard never enters subscription tables. A one-time grant survives a cancelled subscription. A refund revokes a grant and keeps the recovery certificate. | forgecustomer `cargo test` with new reducer tests, replaying fixtures for each row of 04 §B; schema tests. | Every matrix cell has a passing test. A negative control (a cancelled Pro event against a Standard license) leaves the license `active`. | forgecustomer review branch; no live database | Test-mode fixtures only until WP-08. |
| AC-03 | WP-03 | The founders cap is exact. | A concurrency test: cap N, N + 20 parallel founder checkout requests; then expire some reservations; then replay each confirming webhook 100 times. | Exactly N confirmed seats; released numbers are reused; one confirmation per session. A cap of 0 creates no founder session. | forgecustomer test database fixture | Stripe test mode for the checkout leg (WP-08). |
| AC-04 | WP-04 | An update right controls installs only. | Author-Forge shell tests: a build outside the right is refused with a stated reason; an installed build keeps running with an expired window, with no snapshot and with a revoked license (recovery). | Install refused; local work opens in every case. | Author-Forge review branch | The packaged app needs a live run before release (Author-Forge rule). |
| AC-05 | WP-05 | Each add-on key is independent. | forgecustomer and Author-Forge tests: grant and end Press_Forge with Pro on and off; a missing prerequisite. | Only the add-on key changes. A missing prerequisite gives the stated reason. | review branches | — |
| AC-06 | WP-06 | Credits and the allowance follow R03. | forgecustomer quota tests; Author-Forge routing test at an exhausted allowance. | The task runs on the local rung with the label; a credit offer appears; a credit pack raises the quota once and expires on its date. | review branches | Real hosted calls are the PROTIER plan's evidence, not this plan's. |
| AC-07 | WP-07 | Each offer for sale has legal text, a fact sheet, a tax-code memo and written cancellation, refund and dispute outcomes. | Document review by the operator and counsel. | One signed record per offer. | — | Agents cannot satisfy this criterion. |
| AC-08 | WP-08 | The catalog works end to end in Stripe test mode. | The launch policy CP1-WP07 fixture capture plus one founder purchase, one add-on purchase and one credit pack. | Fixtures captured; grants match 04 §B; no live object exists. | Stripe sandbox, after the CP0 entry evidence | Blocked by AUTHORITY_GAP-01. |

---

# 08 — Work packages and gates. WP-00 and WP-01 complete; WP-02 onward proposed.

This component gives each work package its nine-field brief, and the gates between them.

## Gates

| Gate | Passes when |
|---|---|
| OC-G0 | AC-00 PASS (the source lock). |
| OC-G1 | AC-01 PASS: DEC-0 to DEC-9 each have an outcome. |
| OC-G2 | The CP0 entry evidence of the commerce launch policy is attached (AUTHORITY_GAP-01 closed), and the operator authorizes the implementation work packages. |
| OC-G3 | AC-02 to AC-06 PASS on review branches. |
| OC-G4 | AC-07 and AC-08 PASS. A separate operator launch authorization is still required. |

**WP-00 Source lock and evidence inventory** — owner: this plan (read-only)

- **Outcome:** the inventory of 00 §Purpose item 1.
- **Starting evidence:** the pins in 00.
- **Prerequisites:** none.
- **Permitted scope:** this folder only.
- **Boundaries:** no change to any other repository, Drive file or service.
- **Deliverables:** `evidence/AC-00/SOURCE_LOCK.md`, `evidence/AC-00/RESULT.md`.
- **Acceptance criteria:** AC-00.
- **Verification:** 07 AC-00.
- **Stop conditions:** done on AC-00. **Status: complete 2026-11-08.**

**WP-01 Operator decision packet** — owner: the operator

- **Outcome:** DEC-0 to DEC-9 have outcomes (OC-G1).
- **Starting evidence:** 09 and the source lock.
- **Prerequisites:** OC-G0.
- **Permitted scope:** this folder (09, `plan_set.yaml`, the compiled file).
- **Boundaries:** a ruling authorizes no other work package.
- **Deliverables:** the rulings recorded.
- **Acceptance criteria:** AC-01.
- **Verification:** 07 AC-01.
- **Stop conditions:** done when every decision has an outcome; a deferral keeps its blocked work packages blocked. **Status: complete 2026-11-08** (all ten ruled; DEC-9 deferred).

**WP-02 Offer kinds, rights reducer and catalog seed** — owner: forgecustomer

- **Outcome:** 04 §A–C in the schema and the webhook reducer, as an extension of launch-policy CP1-WP01 to WP04.
- **Starting evidence:** forgecustomer `origin/main` at authorization (re-pin); migrations `0002`–`0005`; `repositories/{commerce,licensing,entitlements}.rs`.
- **Prerequisites:** OC-G1 (DEC-1, DEC-2, DEC-0); OC-G2; the launch policy CP1-WP01 design.
- **Permitted scope:** forgecustomer source, migrations, tests and `doc/system/` (04, 05) on a review branch.
- **Boundaries:** no live Stripe object; no production migration; no change to Pro's existing subscription behavior except as 04 §B states.
- **Deliverables:** migrations, reducer, tests, `doc/system` updates.
- **Acceptance criteria:** AC-02.
- **Verification:** `cd api && cargo fmt --all --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all`; `bash doc/system/BUILD.sh`.
- **Stop conditions:** done on AC-02; stop if the launch policy design and this plan disagree on a field (ask the operator).

**WP-03 Founders program** — owner: forgecustomer

- **Outcome:** 05 §D.
- **Starting evidence:** WP-02's schema.
- **Prerequisites:** WP-02; DEC-3 ruled.
- **Permitted scope:** as WP-02.
- **Boundaries:** the cap and benefits are data; no benefit outside D05.
- **Deliverables:** `founder_programs`, `founder_seats`, the reservation and confirmation path, tests.
- **Acceptance criteria:** AC-03.
- **Verification:** as WP-02, plus the concurrency test.
- **Stop conditions:** done on AC-03.

**WP-04 Update right on the license and the desktop** — owners: forgecustomer, Author-Forge

- **Outcome:** 05 §B for the DEC-2 choice, and the DEC-8 offline rule.
- **Starting evidence:** `licenses`; Author-Forge `connection_mode.rs` and `entitlementVerdict.ts`.
- **Prerequisites:** WP-02; DEC-2 and DEC-8 ruled.
- **Permitted scope:** both repositories' source, tests and `doc/system` on review branches.
- **Boundaries:** never stop an installed, allowed build; never block local work (REQ-06, REQ-09).
- **Deliverables:** snapshot fields, the shell install check, tests.
- **Acceptance criteria:** AC-04.
- **Verification:** forgecustomer gate as WP-02; Author-Forge `bun run api:test`, `bun run front:test`, the Tauri lanes; a live run of the packaged app.
- **Stop conditions:** done on AC-04.

**WP-05 Add-on grants** — owners: forgecustomer, Author-Forge

- **Outcome:** 05 §C for Press_Forge first.
- **Starting evidence:** `BDS-AF-CANEBRAKE-001` D4; the ladder packet's three grants.
- **Prerequisites:** WP-02; DEC-4 ruled.
- **Permitted scope:** both repositories on review branches.
- **Boundaries:** no Press_Forge feature work; the key gates nothing until Press_Forge tasks exist.
- **Deliverables:** feature keys, prerequisite checks, the shell display of each grant, tests.
- **Acceptance criteria:** AC-05.
- **Verification:** as WP-04.
- **Stop conditions:** done on AC-05.

**WP-06 Pro credits and annual price** — owners: forgecustomer, Author-Forge

- **Outcome:** 05 §E.
- **Starting evidence:** `plan_quotas`, the commercial policy layer in `repositories/entitlements.rs`.
- **Prerequisites:** WP-02; DEC-6 ruled.
- **Permitted scope:** both repositories on review branches.
- **Boundaries:** no unlimited allowance.
- **Deliverables:** `credit_pack` handling, quota increments, the desktop credit offer, tests.
- **Acceptance criteria:** AC-06.
- **Verification:** as WP-04.
- **Stop conditions:** done on AC-06.

**WP-07 Legal text, fact sheets and tax memo** — owners: the operator and counsel

- **Outcome:** AC-07 for each offer that will be sold.
- **Starting evidence:** launch policy §4 (tax candidates), §11 (disclosures); PROTIER checklist L-01 to L-07.
- **Prerequisites:** OC-G1.
- **Permitted scope:** documents only.
- **Boundaries:** agents draft for review only.
- **Deliverables:** the EULA, Pro terms, founders terms, add-on terms, beta agreement, fact sheets, the tax memo extended to the update plan and add-ons.
- **Acceptance criteria:** AC-07.
- **Verification:** operator and counsel sign-off.
- **Stop conditions:** done on AC-07. Other work packages continue meanwhile.

**WP-08 Test-mode rehearsal** — owner: cross-repository

- **Outcome:** AC-08.
- **Starting evidence:** WP-02 to WP-06 results.
- **Prerequisites:** OC-G3; the launch policy CP1-WP07 entry conditions.
- **Permitted scope:** Stripe test mode; review branches.
- **Boundaries:** no live object or transaction (R07, R08).
- **Deliverables:** fixtures and an evidence bundle.
- **Acceptance criteria:** AC-08.
- **Verification:** 07 AC-08.
- **Stop conditions:** done on AC-08; stop on any CP0 stop condition.

---

# 09 — Operator review and handoff

This component records the current authority, the decisions the plan needs, and the next action.

## Current authority

- **Planning:** asked for by the operator on 2026-11-08 ("yes … so we have all the evidence documented on our next steps").
- **Rulings:** DEC-0 to DEC-9 were ruled on 2026-11-08 (operator, 2026-11-08: "OK I concur with recommendations - proceed", confirmed as "All DEC-0 to DEC-9"). Each ruling is recorded under its decision below. WP-01 is complete (AC-01 PASS, OC-G1 PASS).
- **Lifecycle:** `authorized`, for the rulings only. The central registry has no `authorized` value, so its row reads `approved` (PSS-001 §3.8 mapping: plan `authorized` = registry `approved`).
- **Implementation:** none. WP-02 to WP-08 need the CP0 entry evidence (AUTHORITY_GAP-01) and a separate operator authorization. The rulings authorize no work package by themselves (06).

## Decisions

Each decision has a plain-language question, a recommendation and the alternatives. Prices are hypotheses from the sources, not proposals.

### DEC-0 — Does Standard include any cloud allowance?

**Question.** A June 2026 catalog note gives every customer "an included cloud allowance". The later rulings say Standard is local and that cloud opens only with a Pro subscription. Which stands?

**Recommendation: Standard has no cloud allowance.** The `authorforge_included` plan keeps a zero cloud quota. This follows R05 and the RELACC cloud gate, which are newer.

**Alternative:** a small included allowance, as a taste of Pro. It costs BDS money for every Standard customer, and it puts content egress terms on Standard.

**Ruling (2026-11-08): accepted.** Standard has no cloud allowance. The `authorforge_included` plan keeps a zero cloud quota. AUTHORITY_GAP-07 is resolved.

### DEC-1 — Does Pro require Standard? (TIER-04)

**Question.** Is Pro an add-on that needs a Standard license, or can someone subscribe to Pro alone?

**Recommendation: Pro requires Standard.** Checkout can offer both together.

**Why.** Two grants, independent (D01). Cancelling Pro never costs the author the app. Both tables exist today.

**Alternatives:**

- Pro includes Standard while subscribed. The author loses the app on cancel, which breaks the Master Plan's promise: "Let cloud cancellation remove cloud service—not the customer's local product or local data."
- Pro earns a perpetual Standard license after a number of paid months. Generous, but it adds a third rule to explain, test and write into the terms (R10).

**Effect.** Sets `requires: [standard]` on the Pro offer (WP-02) and the license text (WP-07).

**Ruling (2026-11-08): accepted.** Pro requires Standard. TIER-04 is closed.

### DEC-2 — How do Standard updates work?

**Question.** Does a Standard purchase cover one major version (paid upgrades later), or a window of updates with an optional yearly renewal?

**Recommendation: E, paid major upgrades, for the first release.** Revisit A+ after a year of sales data.

**Why.**

- The commerce launch policy, its tax-code candidate ("Standard grants the purchased major version") and its disclosure copy already assume E.
- A+ adds a third recurring offer, and its tax classification is not researched.
- The Market Analysis proposes E: upgrades at "approximately $59-$79 every two to three years, with no forced upgrade".
- E is simpler to explain (R10).

**Alternative: A+**, included updates for 12 months, then an optional renewal, with a fallback to the last build in the window. It gives steadier revenue: about 30 % more per Standard customer over three years at a 40 % renewal rate (05 §B). That rate is an assumption.

**Evidence that could change this:** a tax memo that classifies a renewal cleanly, or data on how often authors renew.

**Correction.** In the 2026-11-08 conversation that led to this plan, A+ was recommended before the launch policy's major-version design was read. This recommendation replaces that one.

**Ruling (2026-11-08): accepted.** E, paid major upgrades, for the first release. No `update_plan` offer is sold at the first release. A+ is revisited after a year of sales data.

### DEC-3 — The founders program

**Question.** How many founder seats, and what do founders get?

**Sources.**

- The Master Plan: "First 100: $299" against a $449 license, and "A bounded founder benefit—locked cloud rate, priority onboarding, or named founder edition—is safer than indefinite support promises."
- The Market Analysis: "Founding beta: $79 one-time, capped at 250 users, including 12 months of updates and a direct feedback channel."

**Recommendation.**

- **100 numbered seats** in the first program. One operator supports every founder, and the Master Plan warns not to underprice support.
- **Benefits:**
  - a founder Standard price;
  - the first major upgrade free (under E);
  - a Pro price locked while the subscription stays continuous, ending after a lapse of more than 30 days;
  - a founder number.
- **No lifetime Pro** (D05).
- A refunded seat's number is not reused, which keeps the audit simple.

**Alternatives:** 250 seats (more early revenue, more support load), or no locked Pro rate (simpler, a weaker offer). The cap is data (05 §D), so a later program can open more seats.

**Ruling (2026-11-08): accepted.** 100 numbered founder seats. Benefits: a founder Standard price, the first major upgrade free, a Pro price locked while the subscription stays continuous (it ends after a lapse of more than 30 days), and a founder number. No lifetime Pro. A refunded seat number is not reused. The founder price itself waits for DEC-9.

### DEC-4 — Does Press_Forge need Pro?

**Question.** Press_Forge is a monthly add-on (R01, R02). Can a Standard customer buy it without Pro?

**Recommendation: Standard only.** The ladder packet already assumes "`license` only". Press_Forge carries its own hosted costs in its own fee.

**Alternative:** require Pro. That stacks two subscriptions on one customer, which the Master Plan warns against ("avoid a third confusing subscription").

**Conflict recorded.** The Master Plan (2026-07-30) priced Press_Forge as a "$299 add-on" in its one-time column. R01 and R02 (2026-10-21) are newer and say monthly. The rulings stand.

**Ruling (2026-11-08): accepted.** Press_Forge requires Standard only, not Pro. The ladder packet open question D-2 is closed.

### DEC-5 — World Engine's commercial form

**Question.** World Engine is research only. It runs locally ("100% offline local validation"). How would it be sold?

**Recommendation (first version): defer,** and record the principle that a local-only add-on is a `one_time_addon` (D04).

**Recommendation (revised 2026-11-08, after the operator asked "world engine defer?"):** rule the form now: a one-time add-on module inside AuthorForge that requires Standard. Defer only the price.

**Ruling (2026-11-08): accepted, as revised the same day.** World Engine is a **one-time add-on module inside AuthorForge** (`one_time_addon`) that requires Standard. Its price is deferred to DEC-9 and to World Engine's own build gate. This answers the commercial half of the World Engine plan's own open question D2. The plan set's D11 (price, packaging and source-material rights) stays open there.

### DEC-6 — The shape of Pro

**Question.** One Pro tier or several, and when to add credits and an annual price?

**Recommendation.** One Pro tier at launch, plus credit packs (R03). Add an annual price after retention evidence, as the Direct Sales Research advises. Add a second tier only after usage data.

**Alternative:** two tiers at launch. More choice, but no usage data to set the line between them.

**Ruling (2026-11-08): accepted.** One Pro tier at launch, plus credit packs. An annual price comes after retention evidence. A second tier comes only after usage data.

### DEC-7 — How many machines per Standard license?

**Question.** The current default is one device.

**Recommendation: 3 active installations.** A desktop, a laptop, and room for a replacement machine. The author can deactivate a machine (`deactivate_installation` exists).

**Alternative:** 2 installations. Stricter, with more support requests.

**Ruling (2026-11-08): accepted.** 3 active installations per Standard license.

### DEC-8 — Must Standard check in online?

**Question.** An offline lease lasts 14 days. An owned product that must reach a server every 14 days is not really owned.

**Recommendation.**

- Local features never need a check-in.
- Activation happens once, online.
- The signed snapshot and the lease are needed only for hosted grants (Pro, add-on cloud tasks) and for installing updates.
- This matches the desktop today: "In local mode, entitlement enforcement is fully bypassed" (Author-Forge `doc/system/37-pro-entitlement.md:163`).

**Alternative:** a long lease (for example one year). It is more control, and it is still a lockout risk.

**Ruling (2026-11-08): accepted.** Local features never need a check-in. Activation happens once, online. The signed snapshot and the lease are needed only for hosted grants and for installing updates. AUTHORITY_GAP-08 is resolved.

### DEC-9 — Prices

**Question.** What are the prices?

**Recommendation: defer.** Set them after the account-effective fee schedule and the tax codes (the CP0 entry evidence) are known.

**Hypotheses on record (not proposals):**

| Source | Standard | Founders | Pro |
|---|---|---|---|
| Master Plan, 2026-07-30 | $449 | first 100 at $299 | $29 a month, maybe $290 a year |
| Direct Sales Research, 2026-08-30 | $99–$149 | — | $19–$29 a month |
| Market Analysis, 2026-07-29 | $149; upgrade about $59–$79 | 250 at $79, with 12 months of updates | cloud packs $10, $25 and $50 |
| Launch policy fee table | net $92.36 at $99; $139.16 at $149 | — | net $17.35 at $19; $26.64 at $29 |

**Ruling (2026-11-08): deferred, as recommended.** Prices are set after the CP0 fee and tax evidence. No price is approved.

## Authorization requested

None now. WP-01 is complete. Implementation needs:

1. the CP0 entry evidence (AUTHORITY_GAP-01); and
2. a separate operator authorization for WP-02 onward.

## Criteria status

| AC | Status |
|---|---|
| AC-00 | **PASS** (evidence/AC-00/RESULT.md) |
| AC-01 | **PASS** (evidence/AC-01/RESULT.md) |
| AC-02 to AC-08 | not started |

## Handoff

- **Repository:** this plan set, in forgecustomer `docs/plans/`, on a review branch.
- **Drive:** /Forge/Plans/BDS_FOC_OFFER_CATALOG_v0.1_PLAN_SET is not yet uploaded.
- **Forge registry:** the central registry row in forge `docs/canonical/plan_registry_v1.json` is a separate change.
- **Author-Forge and forge:** DEC-1 closes TIER-04. DEC-4 closes the ladder packet's open question D-2 (does `pressforge` need `pro`). Neither record is updated in this change.
- **World Engine plan set (Drive):** DEC-5 answers the commercial half of its D2. The Drive document is not updated (Drive writes are blocked for agents).
