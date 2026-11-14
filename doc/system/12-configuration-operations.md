# §12 — Configuration & Operations

Configuration is loaded from environment variables in `api/src/config.rs`. Missing
required variables fail startup. Empty legacy verification secrets reject HS256 tokens;
ES256 customer validation uses the configured HTTPS issuer's public JWKS.

### Core environment variables

| Variable | Required for startup | Default | Purpose |
| --- | --- | --- | --- |
| `APP_ENV` | no | `development` | Environment label returned by `/v1/version`. |
| `HOST` | no | `0.0.0.0` | Bind host. |
| `PORT` | no | `8090` | Bind port. 8090 is the registered local port (forge `PORT_REGISTRY.md`), where Forge Command looks for ForgeCustomer. The Docker image and the Render Blueprint set `8080`. |
| `DATABASE_URL` | yes | none | Postgres/Supabase database URL. |
| `DATABASE_ACQUIRE_TIMEOUT_SECS` | no | `30` | Connection-pool acquire timeout in seconds. |
| `SUPABASE_JWT_ISSUER` | yes | none | Exact customer JWT issuer; HTTPS URL also determines the public JWKS location. |
| `SUPABASE_JWT_AUDIENCE` | no | `authenticated` | Customer JWT audience. |
| `SUPABASE_JWT_SECRET` | only for legacy HS256 tokens | empty | Optional legacy customer HS256 verification secret; ES256 needs no secret. |
| `ADMIN_JWT_AUDIENCE` | yes | none | Operator token audience for the admin routes (e.g. `forgecustomer-admin`). The posture audience is fixed: `forgecustomer-posture`. |
| `ADMIN_JWT_TRUSTED_KEYS` | required to accept operator tokens | empty | The operator trusted-key set (P3 gate G5, DEC-G5-1): a JSON object `{kid: {"key": <Ed25519 SPKI PEM>, "issuer": <issuer>}}`. **Public** material only; no shared secret. Each `kid` must be a published derivation of its key: `base64url(SHA-256(raw))[:8]` or the first 8 bytes of `SHA-256(raw)` in lower-case hex. One invalid entry, a duplicate `kid` or one key under two `kid`s refuses the whole set. An unset or refused set refuses every operator token; the service still starts. It replaced `ADMIN_JWT_PUBLIC_KEY` and `ADMIN_JWT_ISSUER`. |
| `STRIPE_SECRET_KEY` | required for checkout/webhook work | empty | Stripe API secret. |
| `STRIPE_WEBHOOK_SECRET` | required for webhook work | empty | Stripe webhook verification secret. |
| `STRIPE_API_BASE` | no | `https://api.stripe.com` | Stripe REST API base URL; override to target a mock or test gateway. |
| `ENTITLEMENT_SIGNING_PRIVATE_KEY` | yes | none | Base64 Ed25519 seed for snapshot signing. |
| `ENTITLEMENT_SIGNING_KEY_ID` | no | `entitlement-key-1` | Published signing key ID. |
| `ENTITLEMENT_VERIFY_PUBLIC_KEYS` | no | empty | JSON object `{"<key_id>": "<base64 32-byte Ed25519 public key>"}` of retained and standby verification keys. They verify and are published on `GET /v1/entitlements/keys`; only the active key signs. A malformed value, a repeated `key_id`, or the active `key_id` with a different key fails start-up. Public keys only, so not a secret. |
| `RAKE_ACCESS_SIGNING_PRIVATE_KEY` | no | empty | Base64 Ed25519 seed that signs Rake access tokens. Must differ from the entitlement key. Empty = `POST /v1/app-tokens/rake` returns 503. The operator creates it. |
| `RAKE_ACCESS_SIGNING_KEY_ID` | no | `rake-access-1` | Published `kid` of the Rake access key. |
| `RAKE_ACCESS_VERIFY_PUBLIC_KEYS` | no | empty | JSON `{"<kid>": "<base64 public key>"}` of retained and standby Rake access keys, published on `GET /v1/app-tokens/keys`. Set without the signing key, or holding the entitlement key, fails start-up. |
| `RAKE_ACCESS_ISSUER` | no | `bds-forgecustomer` | The `iss` of every Rake access token. Rake pins it per `kid`. |
| `DATAFORGE_API_URL` | no | empty | Enables outbox worker when set. |
| `DATAFORGE_SERVICE_TOKEN` | no | empty | DataForge service bearer token. |
| `UPDATE_ROLLOUT_SECRET` | required for update lookup | empty | Server-side HMAC secret for deterministic campaign rollout buckets. Empty returns `503` from the update endpoint. |
| `RELEASE_ARTIFACT_BASE_URL` | required for relative artifact keys | empty | Prefix used when `release_artifacts.storage_key` is not already an absolute URL. |
| `ENTITLEMENT_SNAPSHOT_TTL_HOURS` | no | `24` | Snapshot lifetime. |
| `OFFLINE_GRACE_DAYS` | no | `14` | Offline grace window. |
| `DELETION_COOLING_OFF_DAYS` | no | `14` | Cooling-off window in days before a verified deletion request may be processed. |
| `USAGE_RESERVATION_TTL_SECS` | no | `900` | Lifetime of an uncommitted usage reservation before it expires and is swept. |
| `USAGE_THRESHOLD_PERCENTS` | no | `80,100` | Comma-separated usage percentages that emit threshold outbox events. |
| `REQUEST_TIMEOUT_SECS` | no | `30` | Per-request deadline enforced by the router; expiry returns `503`. |
| `MAX_BODY_BYTES` | no | `1048576` | Request body cap enforced by the router; oversized bodies return `413`. |
| `RATE_LIMIT_PER_MINUTE` | no | `300` | Per-client (per-IP) request budget per minute; exceeding it returns `429 RATE_LIMITED` with `retry-after`. `0` disables. |

`.env.example` is a template only. Real values must come from a secret manager or the
deployment environment.

### Local build and run

```bash
cp .env.example .env
cargo build --all
cargo test --all
cargo run -p forgecustomer-api
```

For a local Supabase development database:

```bash
supabase db reset
```

For an existing target project:

```bash
supabase db push
```

### Deployment checklist

1. Select the correct Supabase project for dev, staging, or production.
2. Apply migrations and deterministic seed data.
3. Configure secrets for that environment only; never reuse production secrets in lower
   environments.
4. Build the API release binary.
5. Start the process with `HOST`, `PORT`, and required secrets.
6. Verify `/v1/health`, `/v1/ready`, and `/v1/version`.
7. Configure Stripe webhook endpoint only after `STRIPE_WEBHOOK_SECRET` is set.
8. Confirm DataForge outbox worker behavior when `DATAFORGE_API_URL` is configured.

### Operational probes

- `/v1/health` means the process is running.
- `/v1/ready` means the database answers `select 1`, the table `operator_token_replay` exists,
  and the operator trusted-key set loaded. A failure returns 503 with
  `{"status":"not_ready","failed":[...]}`. The list holds check names only: `database`,
  `operator_token_replay`, `ADMIN_JWT_TRUSTED_KEYS`.
- After every change to the Render service (environment, settings, deploy), run the
  credential-free probe in `docs/operations/render_probe.json`. A dummy bearer token on an
  operator route must return 401. A 500 that says "not configured" means the operator plane is
  down while readiness may still pass on the old code.
- `/v1/version` identifies service, crate version, git SHA if injected, and environment.

Load balancers should gate on readiness, not liveness.

### Runbook references

Detailed operational notes remain in `docs/RUNBOOK.md`. Domain-specific references remain
in `docs/STRIPE.md`, `docs/LICENSING.md`, `docs/ENTITLEMENTS.md`, `docs/USAGE.md`, and
`docs/PRIVACY.md`.
