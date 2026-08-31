# BDS AuthorForge Commerce Launch Policy v0.1

## CP0/WP00 evidence package and bounded CP1 authorization candidate

**Evidence date:** 2026-08-31  
**Operator authorization:** `AUTHORIZE BDS_AUTHORFORGE_COMMERCE_LAUNCH_POLICY_v0.1`  
**Authorized phase:** BR1 architecture rulings and CP0/WP00 source-truth, test-mode, and implementation-plan preparation only

### Pinned sources

| Source | Pin |
| --- | --- |
| ForgeCustomer | [`e953d7b1c9926ed73d9b19af8e6d690552e4c337`](https://github.com/Boswell-Digital-Solutions/forgecustomer/commit/e953d7b1c9926ed73d9b19af8e6d690552e4c337) |
| bds_website | [`842b63aee2f7b3f31cf88d88e98052f4b334e239`](https://github.com/Boswell-Digital-Solutions/bds_website/commit/842b63aee2f7b3f31cf88d88e98052f4b334e239) |
| AuthorForge | [`ff0d60e6688076918605182428d984f7bc42ab76`](https://github.com/Boswell-Digital-Solutions/Author-Forge/commit/ff0d60e6688076918605182428d984f7bc42ab76) |
| Research | [BDS AuthorForge Direct Sales Research — 2026-08-30](https://docs.google.com/document/d/19sRN5nIckgqGHbcAAauHwikRRsDhSZF164xaPrlBcrU/edit) |
| Stripe | Managed Payments, eligibility, integration, tax-code, event, and US pricing documentation read on 2026-08-31 |

## 1. CP0 disposition

**Disposition: STOP — no CP1 implementation, paid pilot, or launch is authorized.** The ruling is coherent, but three entry gates remain unproved: account/product eligibility, complete account-effective fees, and Stripe-sandbox webhook behavior. The source review also found an unapproved second Stripe authority inside the pinned AuthorForge application.

| Required evidence | Result | Consequence |
| --- | --- | --- |
| Managed Payments account eligibility | **Not demonstrated.** The BDS Stripe Dashboard exposes a sandbox Managed Payments page with a `Get started` enrollment action. No enrollment or legal terms were accepted. | Eligibility stop condition remains active. |
| AuthorForge product eligibility | **Not demonstrated account-specifically.** Both accessible test and live product-catalog routes resolved to a sandbox catalog with zero active products. | Standard and Pro cannot yet be approved for Managed Payments. |
| Effective fees | **Public fee envelope established; account-effective total incomplete.** Dashboard confirms a 3.5% add-on, but the account’s final Payments/Billing plan and method/currency mix were not evidenced. | Fee stop condition remains active. |
| Candidate tax classifications | **Defensible candidates identified; operator/tax review still required.** | Classification remains a CP1 entry gate, not an approved product setting. |
| Current-versus-target model | **Complete.** | Ready for implementation planning. |
| Order/refund/dispute/license transitions | **Complete as a design.** | Must be proven by deterministic tests before use. |
| Webhook matrix | **Stripe-documentation verified; sandbox-delivery proof not performed or claimed.** | Test-mode evidence gate remains open. |
| Recovery/export contract | **Specified; current implementation not proved.** | AuthorForge work is mandatory before paid launch. |
| Direct fallback gate | **Closed by default; initial allowlist is empty.** | No Direct Stripe sale is permitted. |
| Customer disclosure reconciliation | **Complete as target copy rules.** | Website changes and truth-label proof are mandatory. |

## 2. Managed Payments eligibility evidence

Stripe’s public eligibility rules support US direct-integration businesses and digital products such as SaaS, downloadable software, and digital media, but Stripe separately reviews each account and requires each product, customer, and transaction to be eligible. See [Managed Payments eligibility](https://docs.stripe.com/payments/managed-payments/eligibility).

Account-specific read-only observation on 2026-08-31:

- The BDS Dashboard was authenticated to **Boswell Digital Solutions LLC sandbox**.
- The Managed Payments settings page displayed the public feature description, `3.5% add-on fee for each transaction`, and a **Get started** action.
- The action was not selected because it begins enrollment and can lead to operator-only legal terms.
- The sandbox product catalog contained zero products.
- An attempted read-only switch to the live product catalog resolved back to the sandbox catalog; no live AuthorForge product or eligible tax code was observable.

Result: the account is geographically plausible and the product concepts fit supported categories, but **Stripe has not confirmed BDS or either AuthorForge offer as eligible**. The operator must personally complete the provider’s enrollment/terms step and obtain an account-level eligibility result. No agent may do so.

## 3. Effective-fee evidence

Stripe’s US public pricing lists:

- Managed Payments: **3.5% per successful transaction in addition to Payments fees**.
- Domestic cards: **2.9% + $0.30** per successful transaction.
- International cards: **+1.5%**.
- Currency conversion when required: **+1%**.
- Billing pay-as-you-go: **0.7% of Billing volume**, excluding one-off invoices.
- Dispute received: **$15**; manual dispute countering: **$15**, returned for won disputes and retained for lost disputes.

Sources: [Stripe US pricing](https://stripe.com/us/pricing) and [Managed Payments operations](https://docs.stripe.com/payments/managed-payments/how-it-works).

Default US domestic-card illustrations, not pricing approval:

| Offer illustration | Public components | Effective fee | Net before refunds, disputes, and other costs |
| --- | --- | ---: | ---: |
| Standard at $99 one time | 3.5% Managed + 2.9% Payments + $0.30 | $6.64 | $92.36 |
| Standard at $149 one time | 3.5% Managed + 2.9% Payments + $0.30 | $9.84 | $139.16 |
| Pro at $19 monthly | 3.5% Managed + 2.9% Payments + 0.7% Billing + $0.30 | $1.65 | $17.35 |
| Pro at $29 monthly | 3.5% Managed + 2.9% Payments + 0.7% Billing + $0.30 | $2.36 | $26.64 |

The public baseline is therefore `6.4% + $0.30` for a domestic-card Standard sale and, if BDS is on Billing pay-as-you-go, `7.1% + $0.30` for a Pro subscription payment. International-card and currency-conversion add-ons can raise those percentages to `7.9% + $0.30` or `8.9% + $0.30` for Standard, and to `8.6% + $0.30` or `9.6% + $0.30` for Pro.

The total cannot yet be called complete because the following remain account- or transaction-dependent:

1. BDS’s actual Payments and Billing pricing plan after Managed Payments enrollment.
2. The payment methods Managed Payments dynamically exposes and their fees.
3. Customer-country, international-card, and currency-conversion mix.
4. Refund economics, including jurisdictions where Stripe retains/remits original tax after a refund.
5. Dispute and dispute-prevention usage.
6. Any negotiated/custom pricing or provider amendments shown only after enrollment.

Required closure evidence: operator-visible Dashboard pricing/plan evidence, a written fee schedule for Standard one-time and Pro subscription paths, and sandbox balance-transaction reconciliation for one successful, refunded, disputed/won, and disputed/lost fixture. The fixtures must be test mode only.

## 4. Candidate tax classification

These are **candidates, not tax or legal approval**. Stripe requires an eligible product tax code for every Managed Payments product.

| Offer | Leading candidate | Why it fits | Unresolved questions |
| --- | --- | --- | --- |
| AuthorForge Standard | `txcd_10202001` — Downloadable Software, non-recreational, personal use | AuthorForge is prewritten downloaded writing/publishing software; Standard grants the purchased major version rather than a subscription. | Is the intended customer legally “personal use,” or is professional/commercial author use expected? If business use is material, evaluate `txcd_10202003`. Confirm that bundled support, updates, and recovery do not change the principal product. Confirm the exact duration/major-version rights. |
| AuthorForge Pro | `txcd_10105003` — AIaaS, cloud based and downloaded, personal use | Pro combines a downloaded desktop application with cloud AI capability. | Confirm cloud AI is the principal recurring service rather than incidental. If customers use it commercially, evaluate `txcd_10105004`. If the principal service is general SaaS rather than AI, compare `txcd_10103100`/`txcd_10103101`. Confirm no disqualifying human-delivered service is bundled. |

All listed candidate families appear in Stripe’s current eligible-tax-code list. See [Managed Payments eligibility](https://docs.stripe.com/payments/managed-payments/eligibility) and the [Stripe product tax-code catalog](https://docs.stripe.com/tax/tax-codes).

Classification closure requires a one-page offer fact sheet for each edition covering customer use, delivered components, cloud dependency, update rights, support, refund terms, and the purchased major-version definition. The operator and qualified tax adviser must approve the selected code before product configuration.

## 5. Source-truth reconciliation

### Current model at the pins

| Surface | Current truth | Gap against the ruling |
| --- | --- | --- |
| ForgeCustomer catalog | `products`, `plans`, and `plan_versions` resolve Stripe recurring prices. | No first-class one-time offer/SKU semantics. |
| ForgeCustomer checkout | `POST /v1/checkout` resolves a plan and creates Checkout with `mode=subscription`. | Standard `mode=payment` path is absent. |
| ForgeCustomer commerce projection | Billing accounts, Stripe customers, subscriptions/items, billing periods, checkout sessions, invoice references, deduplicated webhook events. | No normalized orders, order items, payment settlement, refunds, or disputes. |
| ForgeCustomer licensing | Licenses and signed entitlements are present and synchronized from subscription state. | License provenance does not support an order lane or merchant-model discriminator. |
| bds_website | Pro checkout through the ForgeCustomer BFF; success polls subscriptions. Standard uses request/contact coordination. | No Standard purchase flow, order recovery, merchant label, refund/dispute state, or typed success page. |
| AuthorForge | Local-first manuscripts and export capabilities exist, but the pinned system documentation also describes app-owned Stripe checkout, webhook handling, local entitlement rows, and email-keyed signed JWTs. Testing posture opens Pro gates. | This is a second commercial authority and violates the ruling that AuthorForge receives only signed ForgeCustomer decisions. No proved recovery-only mode exists. |
| Stripe | Raw payment, invoice, receipt, refund, dispute, payment-method, and event truth. | No observable BDS AuthorForge products or Managed Payments enrollment result. |

Authoritative pinned implementation references include [ForgeCustomer Stripe integration](https://github.com/Boswell-Digital-Solutions/forgecustomer/blob/e953d7b1c9926ed73d9b19af8e6d690552e4c337/docs/STRIPE.md), [ForgeCustomer API](https://github.com/Boswell-Digital-Solutions/forgecustomer/blob/e953d7b1c9926ed73d9b19af8e6d690552e4c337/docs/API.md), [website commerce boundary](https://github.com/Boswell-Digital-Solutions/bds_website/blob/842b63aee2f7b3f31cf88d88e98052f4b334e239/doc/system/07-security-commerce.md), [website ForgeCustomer integration](https://github.com/Boswell-Digital-Solutions/bds_website/blob/842b63aee2f7b3f31cf88d88e98052f4b334e239/doc/system/09-forgecustomer-integration.md), and [AuthorForge Pro entitlement](https://github.com/Boswell-Digital-Solutions/Author-Forge/blob/ff0d60e6688076918605182428d984f7bc42ab76/doc/system/37-pro-entitlement.md).

### Target model

| Concern | Target authority and rule |
| --- | --- |
| Offer | ForgeCustomer catalog identifies `one_time_major_version` or `subscription`; one-time offers cannot reference subscription lifecycle state. |
| Purchase intent | ForgeCustomer creates an idempotent purchase intent tied to customer, offer/version, currency, amount snapshot, tax-code snapshot, and merchant model. |
| Checkout | ForgeCustomer seals one merchant model into each Checkout Session. Standard uses `mode=payment`; Pro keeps `mode=subscription`. No automatic model switch. |
| Order | Standard produces a normalized order and immutable order items. Pro continues to produce subscription state; both may share payment/refund/dispute primitives without sharing lifecycle semantics. |
| Payment | Stripe remains raw truth. ForgeCustomer stores only normalized settlement facts and Stripe object references. |
| License | A license records its source type and source ID (`order` or `subscription`) plus merchant model. |
| Commercial entitlement | ForgeCustomer signs capability decisions. A redirect, Stripe object, or AuthorForge-local table cannot create a grant. |
| Recovery authorization | A separate, narrowly scoped signed recovery certificate permits opening, backing up, and exporting existing local work. It is not a paid entitlement. |
| AuthorForge | Removes or hard-disables app-owned Stripe checkout/webhook/customer lookup. It verifies ForgeCustomer-signed activation, release, update, commercial-entitlement, and recovery decisions only. |
| Website | Renders catalog, merchant disclosure, checkout initiation, pending success, account order/subscription status, and recovery entry points from ForgeCustomer. |

## 6. Merchant-model discriminator

Conceptual names are non-binding, but these invariants are mandatory:

- Allowed values are exactly `stripe_managed_payments` and `bds_direct_stripe`.
- The model is selected before Checkout creation by an operator-controlled policy evaluation.
- The selected value, policy version, jurisdiction decision, and selected product tax code are copied into the purchase intent and sent as sealed Stripe metadata where supported.
- Checkout Session, order, order item, payment projection, refund, dispute, license provenance, entitlement decision, and audit receipt each retain the model explicitly; no join-only inference is sufficient for audit evidence.
- A Stripe event whose metadata/model conflicts with the stored purchase intent is quarantined, acknowledged safely, and cannot grant or restore rights.
- A missing/unknown model, missing tax-code snapshot, or ambiguous jurisdiction fails closed.
- The direct fallback allowlist starts empty and is versioned, signed by an operator, product-specific, state-specific, and time-bounded.

## 7. State model

Commercial state must be modeled on separate axes so payment, refund, dispute, license, and recovery facts cannot overwrite one another.

### Checkout and settlement

| Axis | States | Allowed progression |
| --- | --- | --- |
| Checkout | `created`, `open`, `completed`, `expired` | `created → open → completed`; `created/open → expired`. Completion is not settlement proof. |
| Payment | `unpaid`, `processing`, `paid`, `failed` | `unpaid → processing → paid`; `unpaid/processing → failed`; a late failure cannot demote `paid`. |
| Refund aggregate | `none`, `partial`, `full` | `none → partial → full`. Only provider-confirmed successful refund amounts count. |
| Dispute | `none`, `open`, `won`, `lost` | `none → open → won/lost`. Provider warning states map to `open` until final. |

### Rights projection

| Facts | Commercial license | Signed commercial entitlement | Recovery certificate |
| --- | --- | --- | --- |
| Unpaid/processing/failed/expired | Not issued | None | None unless tied to a previously paid order |
| Paid, no full refund, no open/lost dispute | Active | Active purchased capabilities | Issued and cached for local recovery |
| Open dispute | Suspended | No new cloud use, activation, normal updates, or installer access | Remains valid for existing local work |
| Dispute won | Restore previous rights idempotently, unless another revocation fact exists | Reissue active decision | Unchanged |
| Full refund | Revoked-final (`refund`) | Revoked; no commerce capabilities | Remains/reissued for existing local work |
| Dispute lost/accepted/finalized against transaction | Revoked-final (`dispute_lost`) | Revoked; no commerce capabilities | Remains/reissued for existing local work |

Partial refunds require item-level allocation. A partial refund never silently revokes the entire order. If the refunded amount fully reverses a license-bearing item, revoke that item’s derived rights. Ambiguous allocation enters manual review and must not create new rights.

Monotonicity rules:

1. An older `paid` or Checkout event cannot reactivate a final refund/dispute revocation.
2. A won dispute restores only rights that remain valid under all other facts.
3. State is derived from provider facts plus operator overrides; handlers do not blindly set a single mutable status.
4. Every transition records prior state, new state, event ID, Stripe object ID, event creation time, processing time, merchant model, and policy version.
5. Operator correction uses a new auditable compensating decision; history is never rewritten.

## 8. Webhook matrix and deterministic replay plan

The event names below are verified against Stripe’s current event catalog, including [Checkout delayed-payment events and refunds](https://docs.stripe.com/api/events/types). Actual BDS delivery, object shape, API-version behavior, and Managed Payments metadata are **not yet sandbox-proved**.

| Event | Normalized effect | Rights effect |
| --- | --- | --- |
| `checkout.session.completed` | Close Checkout; project customer and references; set `paid` only when the webhook object proves paid settlement, otherwise `processing`. | May grant only when settlement is proved in the verified webhook. Redirect never grants. |
| `checkout.session.async_payment_succeeded` | Set delayed payment `paid`. | Issue order-derived license/entitlement once. |
| `checkout.session.async_payment_failed` | Set delayed payment `failed` unless already paid. | No grant. |
| `checkout.session.expired` | Set Checkout `expired`. | No grant. |
| `payment_intent.processing` | Set payment `processing`. | No grant. |
| `payment_intent.succeeded` | Reconcile payment as `paid` after matching sealed purchase intent/model. | Issue once if all gates match. |
| `payment_intent.payment_failed` | Set failure unless a newer/final paid fact controls. | No grant or demotion of a proved later payment. |
| `refund.created`, `refund.updated`, `refund.failed` | Store refund object/status/amount and reconcile aggregate. | Rights change only on successful provider-confirmed amounts. |
| `charge.refunded` | Reconcile charge-level refunded total, including partial refunds. | Full/item-complete refund causes final commercial revocation. |
| `charge.dispute.created` | Set dispute `open`. | Suspend commerce rights; preserve recovery. |
| `charge.dispute.updated` | Refresh reason/status/evidence deadlines. | Keep suspended while non-final. |
| `charge.dispute.closed` | Map provider final status to `won` or `lost`. | Won restores idempotently; lost causes final commercial revocation. |
| `charge.dispute.funds_withdrawn`, `charge.dispute.funds_reinstated` | Financial/audit reconciliation only. | Never independently grant or revoke. |
| `customer.subscription.created/updated/deleted` | Preserve the existing Pro subscription projection. | Derive Pro rights, never Standard order rights. |
| `invoice.paid`, `invoice.payment_failed` | Reconcile Pro renewal settlement/degradation. | Update Pro only; no Standard effect. |
| Any unsupported event | Store event as acknowledged/unsupported with no projection. | No grant, suspension, revocation, or restoration. |

Required replay suite:

1. Capture version-pinned Stripe sandbox fixtures for every supported event and every merchant model actually tested.
2. Deliver each fixture once, twice, and 100 times; require one webhook-event row, one semantic transition, and at most one outbox/license issuance.
3. Permute delivery order, including success before completion, refund before an older success event, dispute close before an older update, and duplicate final events.
4. Delay events across worker restarts and inject a crash between raw-event persistence, projection, and outbox dispatch.
5. Test synchronous cards and at least one delayed payment method exposed by Managed Payments.
6. Test partial refund, full refund, failed refund, dispute open, dispute won, dispute lost, and a refund during an open dispute.
7. Test invalid signatures, unknown event types, missing metadata, wrong customer/order, wrong amount/currency, and merchant-model mismatch.
8. Assert browser success polling remains `pending` until ForgeCustomer’s webhook projection is committed.
9. Assert no event reaches AuthorForge directly and no app-local Stripe state can grant a capability.
10. Compare replayed final projections and audit digests across every permutation; they must be byte-for-byte deterministic aside from processing timestamps.

## 9. Recovery/export capability contract

The recovery path is a separate safety capability, not a continuing Standard or Pro license.

### Recovery certificate

- Signed by ForgeCustomer and bound to the customer, original license/order, install or recovery device, purchased major version, and recovery policy version.
- Issued at initial activation so an existing installation can recover offline; refreshable from the authenticated website account after refund or dispute.
- Carries only `open_existing`, `backup_raw`, and `export_existing` capabilities.
- Cannot authorize new projects, new imports, editing that creates new commercial content, cloud AI, cloud storage/consumption, new normal activations, normal installer downloads, releases, or updates.
- Does not expire in a way that can strand already-local manuscript data. Compromise is handled by replacing the certificate or recovery build without deleting or encrypting manuscripts.

### Client behavior

- Existing manuscript files remain in the local workspace and are never deleted, encrypted, uploaded, renamed destructively, or made unreadable because of payment state.
- Recovery mode opens existing work read-only, creates an atomic raw backup, and exports to at least one documented open format. Additional advertised export formats require release-gate proof.
- If the normal app is unavailable, the account surface may supply a signed, version-pinned recovery-only artifact after platform qualification. It is not normal installer access.
- Recovery failure is explicit and non-destructive; the original workspace remains untouched.
- Telemetry may record recovery action metadata but never manuscript content.

### Current evidence and gap

The pinned app documents local workspace authority and an export pipeline, but the export chapter is only keyword-parity verified and no refund/dispute recovery-only mode is proved. The pinned app also contains a legacy local entitlement/Stripe lane and a testing posture that opens Pro gates. Therefore recovery is a mandatory implementation and release gate, not an existing launch capability.

Acceptance test: a paid Standard fixture creates a workspace; the account is fully refunded; the commercial entitlement is revoked; the machine is disconnected from the network; the existing workspace still opens in recovery mode, raw backup and open-format export succeed, and every cloud/new-activation/update/new-project attempt fails closed.

## 10. Direct Stripe fallback gate

**Initial jurisdiction allowlist: empty.** No state, including Kentucky, is approved by this package.

The fallback policy must be an operator-signed record containing state, products, effective period, evidence links, seller identity, tax registration ID, approved tax codes, Stripe Tax configuration version, refund/dispute owner, disclosure version, and revocation time. A transaction is eligible only when all fields are present and current. A missing state or expired authorization returns no Checkout Session.

Per-state readiness checklist:

1. Written product classification for Standard and Pro, including bundled and mixed transactions.
2. Seller identity and customer-facing address/support details.
3. Physical and economic nexus analysis; registration/permit evidence where required.
4. Current state/local rate and sourcing rules, exemption handling, and filing cadence.
5. Stripe Tax registration and calculation configuration proved in test mode.
6. Refund, bad-debt, dispute, and tax-credit treatment.
7. Terms, privacy, refund, invoice/receipt, and merchant disclosures reviewed for BDS as seller/MoR.
8. Test-mode expected-versus-actual tax evidence for representative addresses.
9. Operator signature and explicit state/product enablement; default remains disabled.

Kentucky requires special review. Current KRS 139.200 taxes digital property whether rights are permanent, temporary, or payment-conditioned, and KRS 139.010 treats prewritten software as tangible personal property and separately defines prewritten-software access services. See [KRS 139.200](https://apps.legislature.ky.gov/law/statutes/statute.aspx?id=58186) and [KRS 139.010](https://apps.legislature.ky.gov/law/statutes/statute.aspx?id=58185). Kentucky’s general DOR page still displays the historical `$100,000 or 200 transactions` remote-seller rule, while its Summer 2026 newsletter states the 200-transaction limb was removed. That inconsistency, plus BDS’s own Kentucky nexus, must be resolved with current DOR/adviser evidence before Kentucky is added.

## 11. Customer-facing disclosure matrix

| Surface | Managed Payments | Direct Stripe fallback |
| --- | --- | --- |
| Store/edition comparison | “Standard is a one-time license for the purchased major version” or Pro subscription terms; availability only after release proof. | Same product truth; jurisdiction-limited availability. |
| Before Checkout | “Sold through Link/Stripe as merchant of record. BDS provides AuthorForge product support and fulfillment.” | “Sold by Boswell Digital Solutions LLC. Taxes and refund terms are shown at checkout.” |
| Checkout | Stripe/Link merchant identity, tax, payment methods, receipt behavior, and terms must control. | BDS seller identity and Stripe-hosted payment UI must be clear. |
| Success redirect | “Payment received/pending verification. Access appears after payment confirmation.” Poll ForgeCustomer order/subscription projection. | Same; no rights from redirect. |
| Account | Show order or subscription type, merchant model, settlement/refund/dispute state, license state, recovery action, and provider receipt link/reference. | Same, with BDS as seller. |
| Receipt | Link/Stripe sends the Managed Payments receipt/invoice/refund notice. BDS sends no duplicate receipt. | Stripe sends the payment receipt on BDS’s behalf; BDS sends no duplicate payment receipt. |
| Statement | Explain `LINK.COM* [descriptor]` where Stripe’s actual Managed Payments behavior uses it. | Show the BDS descriptor proved in test mode. |
| Refund | Link support may handle eligible requests and Stripe may issue refunds within 60 days in certain cases; BDS terms cannot contradict that authority. | BDS owns the refund decision/process, subject to law and stated terms. |
| Dispute | Stripe manages Managed Payments disputes; BDS still suspends/restores/revokes its own rights from verified events. | BDS owns dispute response and commercial consequences. |
| Support | Link/Stripe: transaction/payment/subscription support. BDS: product, fulfillment, licensing, defects, recovery. Monitored BDS address with <48-hour escalation response. | BDS owns product and transaction support, with Stripe payment tooling. |

Website reconciliation requirements:

- Keep the current Standard purchase CTA non-transactional until all gates pass.
- Replace the subscription-only success page with a typed pending order/subscription view.
- Add orders, refunds/disputes, merchant model, and recovery to the account surface.
- Supersede the stale `docs/store_security_architecture_v_1.md` direct/FastAPI/ForgeCommand model with ForgeCustomer authority.
- Do not claim immediate Standard availability or any platform until a signed release is qualified for that platform.

## 12. CP1 work packages

All work begins from the pins at the top of this package. Re-pin and repeat source-truth review if any repository head changes before authorization.

| WP | Repository | Scope | Acceptance gate |
| --- | --- | --- | --- |
| CP1-WP01 | ForgeCustomer | Add conceptual offer kind, purchase intent, order/order item, settlement, refund, dispute, merchant-model, jurisdiction/tax snapshot, and audit provenance. Additive migrations only. | Schema tests prove Standard never enters subscription tables; every listed record has merchant model; rollback disables writers without data loss. |
| CP1-WP02 | ForgeCustomer | Split checkout construction into typed one-time and subscription paths; Standard `mode=payment`; Pro preserved. Implement sealed model selection and idempotency. | 100 duplicate requests create one purchase intent/session; no automatic fallback; missing model/jurisdiction fails closed. |
| CP1-WP03 | ForgeCustomer | Expand verified webhook projection, deterministic reducer, refund/dispute handling, outbox/audit, reconciliation. | Full replay suite passes; invalid/unknown events grant nothing; redirect-only test grants nothing. |
| CP1-WP04 | ForgeCustomer | Order- or subscription-sourced license provenance, signed commercial decisions, signed recovery-certificate contract. | Paid Standard issues once; full refund revokes commerce; dispute suspension/win/loss behave idempotently; recovery remains separate. |
| CP1-WP05 | bds_website | Standard checkout initiation, typed pending success, order account/recovery UI, merchant/refund/support disclosures, truth labels. | Copy matrix passes; no duplicate receipt; CTA remains behind release/commercial feature flag. |
| CP1-WP06 | AuthorForge | Remove/hard-disable legacy app Stripe checkout/webhook/email authority; consume ForgeCustomer-signed decisions only; implement offline-safe recovery-only mode. | Repository search and tests prove zero app-owned Stripe grant paths; refunded workspace recovery test passes; local manuscript bytes remain unchanged. |
| CP1-WP07 | Stripe sandbox + ForgeCustomer tests | After operator eligibility/terms: create test-only Standard/Pro products with approved codes, prove Managed flags/API version, capture event/fee fixtures. | Account/product eligibility evidence, exact fee schedule, webhook matrix, and balance reconciliation complete. No live objects or transactions. |
| CP1-WP08 | Operations/tax | Managed support escalation runbook; Direct fallback registry and per-state checklist; Kentucky memo. | Monitored support works inside 48 hours; allowlist remains empty until separate operator signatures. |
| CP1-WP09 | Cross-repository gate | Run contract, migration, security, webhook, recovery, disclosure, and platform-release suites with exact evidence manifest. | One signed evidence bundle; rollback and stop drills pass; separate operator launch authorization required. |

### Sequencing

1. Close eligibility, product-classification, and fee entry gates.
2. Implement WP01–WP04 in ForgeCustomer behind disabled flags.
3. Implement WP06 recovery and remove the second commercial authority.
4. Implement WP05 website surfaces with CTA still disabled.
5. Produce WP07 Stripe sandbox fixtures and replay evidence.
6. Close WP08 operations/tax gates.
7. Run WP09. Do not activate live commerce or publish purchase claims.

### Rollback conditions

- Any merchant-model mismatch, non-deterministic replay, duplicate grant, rights from redirect, or recovery regression disables all checkout flags.
- A Managed Payments eligibility or tax-coverage change disables affected model/jurisdiction/product combinations; it does not auto-route to Direct Stripe.
- New additive commerce data is retained for audit; rollback stops writers/projectors and restores the last compatible code path. No destructive down migration is required for launch safety.
- Existing local manuscripts and recovery certificates remain usable through any commerce rollback.

## 13. Bounded CP1 authorization candidate

The following is a candidate for later operator signature. It is **not active** and cannot be authorized until the entry evidence below is attached.

> **AUTHORIZE BDS_AUTHORFORGE_COMMERCE_CP1_TEST_MODE_IMPLEMENTATION_v0.1**
>
> Authorize CP1-WP01 through CP1-WP09 from the repository pins recorded in the CP0/WP00 evidence package, or from replacement pins explicitly listed at signature time.
>
> **Entry conditions:** attach Stripe confirmation that the BDS account and both AuthorForge product classifications are eligible for Managed Payments; operator personally accepts any required Managed Payments legal terms; attach the complete account-effective Payments, Managed Payments, Billing, refund, dispute, and ancillary fee schedule; approve Standard and Pro product fact sheets and tax codes; keep the Direct Stripe state allowlist empty unless separately signed.
>
> **Allow:** application and documentation changes on review branches; additive local/test database migrations; test-only Stripe products, prices, Checkout Sessions, refunds, and disputes; deterministic fixture capture; cross-repository tests; draft pull requests; test-mode support/disclosure validation.
>
> **Require:** ForgeCustomer remains sole BDS commercial/license/entitlement authority; Standard is a first-class one-time order; Pro remains a subscription; merchant model is sealed and recorded end to end; rights arise only from verified deduplicated webhooks; refund/dispute recovery preserves existing manuscript open/backup/export; AuthorForge’s legacy Stripe authority is removed or hard-disabled; all public purchase controls remain disabled.
>
> **Do not authorize:** agent acceptance of legal terms; live Stripe product, price, tax, receipt, email, Checkout, webhook, or transaction changes; production migrations; tax registration/filing; pricing approval; public CTA or availability claims; merge; paid pilot; or public launch.
>
> **Stop immediately if:** any CP0 stop condition recurs; Stripe cannot confirm eligibility; the account-effective fee schedule changes materially; either product lacks a defensible eligible tax code; any path makes Stripe or AuthorForge authoritative for BDS licenses; Standard enters subscription state; a refund/dispute can strand manuscripts; Direct fallback can run without an operator-signed state record; a redirect can grant rights; or website claims outrun release-qualified behavior.

Required attachments before signature:

1. Operator-captured Managed Payments enrollment/eligibility result and terms acceptance date.
2. Approved Standard and Pro fact sheets and tax-code memorandum.
3. Account-effective fee schedule.
4. Re-pinned repository source-truth manifest.
5. Direct fallback allowlist version, expected to be empty.

## 14. Final ruling interpretation

The authorized posture is **Managed-first, never automatic; Direct Stripe is a separately enabled US fallback; Standard one-time is a launch prerequisite; commercial revocation never revokes manuscript recovery**. The architecture can support that posture, but it does not support it today. CP0 therefore returns a bounded implementation candidate with active stop conditions rather than a launch recommendation.

