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
