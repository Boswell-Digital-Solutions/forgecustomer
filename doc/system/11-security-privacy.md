# §11 — Security & Privacy

ForgeCustomer is a fail-closed commercial authority. Security decisions must be explicit,
testable, and boring.

### Token validation

Customer tokens:

- Supabase-issued JWTs.
- ES256 signatures use the public JWKS at the configured HTTPS issuer plus
  `/.well-known/jwks.json`. Legacy HS256 tokens require an explicitly configured
  `SUPABASE_JWT_SECRET`. Both paths validate issuer, audience, and expiry.
- JWKS selection uses the token's bounded `kid` and an explicit ES256/P-256 signing-key
  policy. Token-supplied key URLs are ignored; redirects are rejected. Requests have a
  five-second timeout, 64 KiB response cap, and 32-key cap. Keys are cached for at most
  ten minutes, with single-flight refresh and a one-minute cooldown on misses/failures.
  Expired caches never authorize requests after a fetch failure (`503 SERVICE_UNAVAILABLE`).
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

Admin tokens (operator tokens; P3 gate G5 of `BDS-FCO-P3-TOKEN-ISSUER-001`):

- Separate operator audiences: `forgecustomer-admin` for admin routes, `forgecustomer-posture`
  for the posture read. A token for one audience fails on the other.
- Ed25519 public-key verification against a trusted-key set, `ADMIN_JWT_TRUSTED_KEYS`
  (`api/src/auth/operator.rs`). No shared admin secret, and no private material in the set.
- **Key selection.** The header `kid` selects exactly one entry. An absent or unknown `kid` is
  refused and never defaults. Each entry's `kid` must be a published derivation of its key.
  The algorithm must be `EdDSA`; a `crit` header member or a duplicate header member is refused.
- **SC-3 and DEC-G5-4.** `iss` must equal the selected entry's issuer, and so must `sub`.
- **K-1.** One invalid entry, a duplicate `kid`, or one key under two `kid`s refuses the whole
  set. An unset or refused set refuses every operator token. The log names a fixed reason and the
  `kid`s, never key material.
- **SC-1, for bound keys only.** Every key except the predecessor's (issuer
  `forge_command_local`) is route-bound: the token's `scope` must be exactly the route's action,
  its `target` must equal the route's path parameters, and a `roles` claim is refused. The
  admin role is granted only after this check, so a role can never bypass it. The `target` binds
  path parameters only: on the six mutating routes with no path parameter (licenses, entitlement
  override, usage adjust, commercial-policy publish, release create, campaign create) a bound
  token is bound to its action, not to the body's target (`KI-FOC-20261114-001`). **SC-1 is not met
  for the predecessor's key.** That exemption is DEC-G5-2 (b), a named, open state that ends at
  P3 gate G7, when the predecessor's key and `UNBOUND_PREDECESSOR_ISSUER` are removed.
- **SC-2, for every key.** Each operator token must carry a `jti` and is accepted once. The
  identity is the issuer and the `jti` together. The claim is one atomic insert into
  `public.operator_token_replay` (migration 0018), after every other check and before the
  handler runs, so a refused token claims nothing. A second use gets `401`. An unavailable store
  refuses the token with `503`. The predecessor's key is covered because no predecessor command
  sends more than one request per token (P-1, measured at `Forge_Command@e8382cc`).
- A customer token cannot authorize an admin route.

Rake access tokens (issued by `POST /v1/app-tokens/rake`):

- ForgeCustomer is the ruled issuer of short-lived Rake access tokens for applications
  (forge authority table; plan `BDS-FOC-RAKE-ACCESS-ISSUER-v0.1`).
- The contract is `contracts/rake-access-token-v1.schema.json`: EdDSA, `typ`
  `rake-access+jwt`, audience `rake.application`, pseudonymous `tenant_id` and `user_id`,
  a `scopes` list, at most 900 seconds of lifetime (300 issued).
- The signing key is `RAKE_ACCESS_SIGNING_PRIVATE_KEY`, separate from the entitlement key.
  Start-up fails if the two are equal, or if a Rake verify key is the entitlement key.
  Without the key, the exchange returns 503 and the key list is empty.
- Client authentication is an Ed25519 proof by the installation's registered device key.
  The installation must be active and activated on an active license, with no revocation.
  A proof is valid for 60 seconds and its nonce once (`app_token_mints.client_nonce`).
- The token carries random pseudonyms from `rake_tenant_pseudonyms`, never `customer_id`,
  the Supabase user id or an email. Account deletion revokes the pseudonyms.
- `rake:approve` is issued only alone (DEC-6). Rake enforces its single use.

An absent legacy JWT secret rejects HS256 tokens; it does not disable ES256 verification.
Unavailable or invalid JWKS data fails closed. No customer token, private key, or auth
response body is logged. Admin and posture validators do not use the customer JWKS.

### Secrets

Server-side only:

- `SUPABASE_SERVICE_ROLE_KEY`
- `SUPABASE_JWT_SECRET`
- `STRIPE_SECRET_KEY`
- `STRIPE_WEBHOOK_SECRET`
- `ENTITLEMENT_SIGNING_PRIVATE_KEY`
- `RAKE_ACCESS_SIGNING_PRIVATE_KEY`
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
`ENTITLEMENT_VERIFY_PUBLIC_KEYS` provides that window: each public key in it joins the
verifying ring next to the active key, is published, and verifies, but never signs. Before
this variable existed, the ring held only the active key. After a key change, the keys route
published only the new key, so a client that trusts only the old key refused every newly
signed snapshot, and a client that refetched the list dropped the old key. The operator sets
the variable; no agent sets it.

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
