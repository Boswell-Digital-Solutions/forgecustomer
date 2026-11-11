# 04 — Contract and provenance

This component defines the token, the exchange, the key set and the records. Items marked **(DEC-n)** take the recommended value of that decision in 09. A different ruling changes the item.

## 1. Token header

| Field | Value |
|---|---|
| `alg` | `EdDSA` (DEC-4) |
| `typ` | `rake-access+jwt` |
| `kid` | The issuer key id, for example `rake-access-1` |

Rake refuses a token with any other `alg` or `typ`.

## 2. Token claims

| Claim | Type | Rule |
|---|---|---|
| `iss` | string | The configured issuer id, `RAKE_ACCESS_ISSUER` (proposed `bds-forgecustomer`). Exact match. |
| `aud` | string | `rake.application`. Exact match. One value, not a list. |
| `sub` | string (UUID) | The user pseudonym. Equal to `user_id`. |
| `tenant_id` | string (UUID) | The tenant pseudonym (DEC-3). Rake binds the request and its rows to it. |
| `user_id` | string (UUID) | The user pseudonym (DEC-3). Rake binds `approved_by` to it. |
| `app_id` | string | The registered application, for example `authorforge`. |
| `scopes` | array of strings | A subset of the application's allowed scopes (DEC-4, DEC-5). Example: `["rake:read", "rake:execute"]`. |
| `type` | string | `access`. Rake already refuses any other value. |
| `iat` | integer | Seconds since the epoch. |
| `exp` | integer | `iat` + 300 (DEC-4). Rake allows 60 seconds of clock skew. |
| `jti` | string (UUID v4) | Unique per mint (DEC-5). |

The token carries no email, no Supabase user id, no ForgeCustomer `customer_id`, no project id and no path.

### 2.3 `jti` rule (DEC-5, DEC-6)

The issuer writes each `jti` to the mint audit record. Rake logs the `jti` of each accepted request.

- **Read and execute tokens (DEC-5):** neither side stores `jti` for replay refusal. The 300-second lifetime and backend-only custody bound the replay window.
- **Approve tokens (DEC-6):** a token that carries `rake:approve` must carry no other scope. It is single-use. On the first approve call, Rake inserts its `jti` into a `consumed_approve_tokens` table (`jti` primary key, `exp`) in the same transaction as the approval. A second use of the same `jti` gets 401 `TOKEN_ALREADY_USED`. Rows past `exp` plus the 60-second skew may be deleted.
- The issuer refuses a request that mixes `rake:approve` with another scope (403 `SCOPE_NOT_ALLOWED`).

## 3. Key set

- The issuer signs with `RAKE_ACCESS_SIGNING_PRIVATE_KEY` (base64 Ed25519 seed) and `RAKE_ACCESS_SIGNING_KEY_ID`. Start-up fails if this key equals the entitlement key.
- `RAKE_ACCESS_VERIFY_PUBLIC_KEYS` holds retained and standby public keys, in the `ENTITLEMENT_VERIFY_PUBLIC_KEYS` shape.
- `GET /v1/app-tokens/keys` publishes `{"keys": [{"kid", "alg": "EdDSA", "public_key", "status"}]}`. Public keys only.
- Rake reads `RAKE_ACCESS_TRUSTED_KEYS`: `{"<kid>": {"issuer": "<iss>", "public_key": "<base64>"}}`. Rake accepts a token only when its `kid` is in the map and its `iss` equals that entry's `issuer`. Rake never fetches keys at run time.

## 4. Exchange route

`POST /v1/app-tokens/rake`

- **Caller authentication:** `Authorization: Bearer <Supabase access token>`, then the client authentication of DEC-2: `client_proof`, an Ed25519 signature by the activated installation's registered device key.
- **Request body:** `{"app_id": "authorforge", "scopes": ["rake:read", "rake:execute"], "client_proof": {"installation_id", "iat", "nonce", "signature"}}`. `scopes` is optional. If omitted, the issuer grants every allowed scope except `rake:approve` (DEC-6).
- **Proof message (WP-02):** the lines `forgecustomer.rake-access-exchange.v1`, `app_id`, `installation_id`, the scopes as sent joined with `,` (empty if omitted), `iat` and `nonce`, joined with `\n`. `iat` must be within 60 seconds of server time. A `nonce` is single-use. The installation must be active, with an active device and an active, unexpired license activation, and no revocation (the offline-lease rules).
- **Success, 200:** `{"access_token": "...", "token_type": "Bearer", "expires_in": 300, "scopes": [...]}`. The response has `cache-control: no-store`.

| Status | Code | Cause |
|---|---|---|
| 401 | `UNAUTHORIZED` | Missing or invalid Supabase token |
| 401 | `CLIENT_AUTH_FAILED` | DEC-2 client authentication failed. `details.reason`: `installation`, `device_key`, `activation`, `stale`, `malformed`, `signature`, `nonce_replayed` |
| 403 | `MFA_REQUIRED` | The existing `require_active` rule |
| 403 | `APP_NOT_ALLOWED` | Unknown or disabled `app_id` |
| 403 | `SCOPE_NOT_ALLOWED` | A requested scope is outside the application's list |
| 403 | `ENTITLEMENT_DENIED` | The application's required feature is false or unknown for the customer |
| 503 | `SERVICE_UNAVAILABLE` | Signing key not configured, or the entitlement state cannot be read |

## 5. Records (ForgeCustomer migrations)

| Table | Columns | Rules |
|---|---|---|
| `app_clients` | `app_id` (pk), `audience`, `product_key`, `allowed_scopes` (text[]), `required_feature_key`, `client_auth_method`, `status`, `created_at` | Seeded by migration. Seed row: `authorforge`, `rake.application`, `{rake:read,rake:execute,rake:approve}` (DEC-6), `authorforge.cloud.enabled`, the DEC-2 method, `active`. |
| `rake_tenant_pseudonyms` | `customer_id` (fk), `audience`, `tenant_pseudonym` (uuid, random), `user_pseudonym` (uuid, random), `created_at`, `revoked_at` | One active row per (customer, audience). Created on first mint. Revocation creates a new row. Never sent to DataForge. |
| `app_token_mints` | `jti` (pk), `customer_id`, `app_id`, `audience`, `kid`, `scopes`, `installation_id`, `client_auth_method`, `client_nonce` (unique), `issued_at`, `expires_at` | No token value. No request body. Written in the same transaction as the decision. |

A deletion request (forgecustomer `doc/system/03` deletion surface) must revoke the pseudonym row and keep `app_token_mints` rows only with the anonymized `customer_id`. WP-02 extends the existing deletion transaction.

## 6. Contract files

WP-01 writes these files in forgecustomer `contracts/`:

- `rake-access-token-v1.schema.json` — the header and claims of §1 and §2.
- `vectors/rake-access-token-v1/vectors.json` and its generator `generate.py` — 3 positive and 15 negative vectors. The test key is derived from a published label, so no private key is stored.

**Revision 2026-11-11 (WP-01):** the `contracts/openapi.yaml` entries for §3 and §4 move to WP-02. `openapi.yaml` lists only built routes, so an entry before the route exists would describe a route that does not exist.

## Provenance

The contract version is `v1`. A change to a claim name, a type or a rule creates `v2`. No version runs beside another, because nothing is in production (R09). Rake pins the contract version in its tests.
