# §7 — Scope

**Truth class:** canonical doctrine

This `doc/system/` tree is the modular source of the **ForgeCustomer compiled
system reference**, assembled into the designation-bound artifact
`doc/FOCSYSTEM.md` (designation `FOC`) via `bash doc/system/BUILD.sh`. This chapter
defines ForgeCustomer's service authority and where it ends. ForgeCustomer is an
internal Forge ecosystem service — single-operator, governed — not a public
product and not externally release-certified.

## ForgeCustomer Service Authority

ForgeCustomer is the ecosystem's **customer / commercial authority**: it owns
customer identity, licensing, subscriptions, installations, devices, usage
metering, billing, and customer privacy — backed by its **own Supabase database**
(Rust/Axum service). This is durable commercial truth: no other Forge service owns
or writes it except through ForgeCustomer's API. Full authority detail is in §10.

## What ForgeCustomer Owns

- **Customer identity** and account state.
- **Licensing & entitlements** — including the permanent one-time-purchase license
  model and cloud subscriptions (Stripe-webhook driven); revocation is manual,
  admin-only.
- **Installations, devices, and usage metering / billing.**
- **Customer privacy** of the above.
- The `/v1/admin/*` admin API surface (role-gated, reason-required, audited) and
  the customer-facing surfaces.

## What ForgeCustomer Does Not Own

- **CSSA / cloud-service entitlements + policy bundles.** Which ecosystem
  principal/app may call which backend service remains Forge_Command's authority;
  Forge_Command is the *operator admin client* of ForgeCustomer's `/v1/admin/*`
  routes, not the owner of customer/commercial truth.
- **Operator/control-plane authority.** ForgeCommand orchestrates; it consumes
  ForgeCustomer through the admin API (mutations there are role-gated, audited).
- **Durable systems memory / analytics truth** beyond its own commercial domain —
  that is DataForge's.

## Mutation Discipline

Customer/commercial mutations occur only through the ForgeCustomer API and are
role-gated, reason-required, and audited. Forge_Command and other clients must not
own or write this truth except through that API.

## Release / Readiness Language Restrictions

This documentation describes an internal service under governed development. It
must be described as a verification-current internal service, not as externally
release-certified, and must not claim public-release/SaaS readiness or present
coverage percentages as guarantees unless a later governed slice proves the claim.

## Documentation truth classes

- **Canonical facts** define ForgeCustomer's customer/commercial authority,
  licensing/entitlement model, mutation discipline, and ecosystem boundaries. They
  change only through deliberate change control (§9).
- **Snapshot facts** are audit-derived counts (routes, tables, migrations, tests)
  labelled with a measurement date and corrected by re-measurement, not change
  control.

Ownership, designation doctrine, and the authority hierarchy that govern this tree
are defined in §8. Detailed authority boundaries are in §10; security & privacy in
§11.
