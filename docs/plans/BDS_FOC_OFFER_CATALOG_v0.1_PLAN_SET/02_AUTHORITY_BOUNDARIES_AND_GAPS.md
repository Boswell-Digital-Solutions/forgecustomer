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
