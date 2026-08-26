# §11 — Security & Privacy

ForgeCustomer is a fail-closed commercial authority. Security decisions must be explicit,
testable, and boring.

### Token validation

Customer tokens:

- Supabase-issued JWTs.
- Validated for HS256 signature, issuer, audience, and expiry.
- `sub` must parse as a UUID.
- Missing or unprovisioned customer profiles fail closed.
- When the resolved profile has `mfa_required` set, the token's `aal` claim must equal
  `"aal2"` or the request fails closed with `MFA_REQUIRED` — a missing claim or `"aal1"`
  is never treated as sufficient, unless the profile is still inside its
  `mfa_grace_period_ends_at` window (see below). `mfa_required` is set by the customer
  themselves via `POST /v1/account/mfa-status` (which requires that same `aal2` check on
  the setting call, so it can't be turned off by anyone holding only a stolen password),
  by an operator via `POST /v1/admin/customers/{id}/mfa-required` (`admin` role, written
  reason) — Forge Command's incident-response action, audited separately in
  `customer_mfa_history` since the operator holds no aal2 proof about the target account —
  or by default: MFA is mandatory, so `mfa_required` defaults to `true` for every new
  account, and every account that predates that policy was migrated to `true` at rollout
  (`0014_mandatory_mfa_grace_period.sql`).
- Only that migration and new-account provisioning ever set `mfa_grace_period_ends_at` —
  a 30-day window from whichever of those two moments applies, during which `aal1` still
  passes for an unenrolled account so the mandatory-MFA rollout didn't lock out the
  existing customer base instantly. The self-service and operator-forced paths above
  never set it and always enforce immediately, since neither has a reason to wait: a
  self-service caller already has a verified factor by construction, and an
  operator-forced requirement is deliberate incident response.

Admin tokens:

- Separate operator issuer and audience.
- Ed25519 public-key verification against Forge Command's Token Authority key; no shared
  admin secret.
- A customer token cannot authorize an admin route.

If a JWT secret is absent, the validator is marked unconfigured and rejects tokens. That
prevents accidental local or production token acceptance.

### Secrets

Server-side only:

- `SUPABASE_SERVICE_ROLE_KEY`
- `SUPABASE_JWT_SECRET`
- `STRIPE_SECRET_KEY`
- `STRIPE_WEBHOOK_SECRET`
- `ENTITLEMENT_SIGNING_PRIVATE_KEY`
- `DATAFORGE_SERVICE_TOKEN`
- `UPDATE_ROLLOUT_SECRET`

Secrets must not appear in clients, logs, docs examples with real values, outbox payloads,
or repo history.

### Security headers

Every response receives:

- `x-content-type-options: nosniff`
- `x-frame-options: DENY`
- `referrer-policy: no-referrer`
- `strict-transport-security: max-age=31536000; includeSubDomains`

### Entitlement signing

Entitlement snapshots use Ed25519. The signing private key is loaded from
`ENTITLEMENT_SIGNING_PRIVATE_KEY`, and published keys are exposed by key ID. Key rotation
requires an overlap window where old and new public keys both verify existing snapshots.

Forged snapshots must fail verification. Private signing keys must never be logged.

### PII classification

| Class | Examples | Handling |
| --- | --- | --- |
| Direct PII | email, full name | RLS, redaction, no outbox payloads |
| Financial references | Stripe customer/payment identifiers | server-side only, no outbox payloads |
| Pseudonymous IDs | `customer_id`, `installation_id`, device public key | allowed in sanitized events |
| Secrets | API keys, JWT secrets, signing private key | secret manager/env only |
| Creative content | manuscripts, prompts, model output | never stored here |

### Failure doctrine

- Auth failure denies.
- Suspended customers receive no privileged product actions.
- Revoked devices receive no new activations/leases.
- Ambiguous entitlement state denies cloud/new lease access.
- Missing update rollout/artifact URL configuration is visible as `503`; normal
  ineligibility returns `204` without leaking internal campaign state.
- Local creative access must remain available despite cloud, billing, or DataForge
  outages.
- DataForge outage degrades to queued outbox delivery, not failed customer transactions.
