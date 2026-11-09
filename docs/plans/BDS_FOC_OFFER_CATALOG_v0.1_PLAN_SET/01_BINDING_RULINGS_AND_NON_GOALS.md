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
