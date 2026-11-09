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
