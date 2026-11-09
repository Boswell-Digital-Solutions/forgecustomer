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
