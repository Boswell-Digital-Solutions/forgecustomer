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
