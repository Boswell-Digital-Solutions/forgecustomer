# 05 — Core mechanism

This component describes how a token is minted, verified and used.

## 1. Flow

```
user ──login (Supabase, MFA)──▶ application
application backend ──Supabase token + client auth──▶ ForgeCustomer POST /v1/app-tokens/rake
ForgeCustomer ──verify, gate, map pseudonyms, sign, audit──▶ Rake access token (300 s)
application backend ──Bearer token──▶ Rake /api/v1/...
Rake ──pinned key, iss, aud, exp, typ, type, scopes, tenant binding──▶ accept or 401/403
```

The browser or WebView talks only to its own backend. It never holds the Rake token (R05).

## 2. Mint (ForgeCustomer, WP-02)

Each step fails closed. The order is fixed.

1. Verify the Supabase token with the existing customer verifier.
2. Resolve `CustomerContext` and call `require_active()`. This applies the existing profile, suspension, anonymization and `aal2` rules.
3. Run the DEC-2 client authentication for the `app_id`.
4. Load `app_clients` for `app_id`. Refuse if missing or not `active`.
5. Check that each requested scope is in `allowed_scopes`.
6. Evaluate `required_feature_key` with the entitlement service. Only `true` passes. Unknown, error or `false` denies.
7. Get or create the `rake_tenant_pseudonyms` row for (customer, audience).
8. Build the claims of 04 §2. Sign with the dedicated key.
9. Write the `app_token_mints` row. If the write fails, return 503 and no token.
10. Return the token with `cache-control: no-store`. Log the `jti`, `app_id` and `kid`. Never log the token.

## 3. Verify (Rake, WP-03)

`auth/jwt_handler.py` `verify_token` changes as follows:

- Read the unverified header. Refuse unless `alg` is `EdDSA`, `typ` is `rake-access+jwt` and `kid` is in `RAKE_ACCESS_TRUSTED_KEYS`.
- Decode with that entry's public key, `algorithms=["EdDSA"]`, `issuer=<entry issuer>`, `audience="rake.application"`, `leeway=60`, and `options={"require": ["exp", "iat", "iss", "aud", "jti", "sub"]}`.
- Keep the existing `type`, `tenant_id` and `user_id` checks.
- Refuse a token whose `exp - iat` is more than 900 seconds.

The rest of the change:

- Remove `JWT_SECRET_KEY`, `JWT_ALGORITHM`, `JWT_REFRESH_TOKEN_EXPIRE_DAYS`, `create_access_token`, `create_refresh_token` and the `forge_keys` `secret`/`jwt` mapping (R09). Rake mints nothing.
- Replace the development-default 503 check (`execution_access.py:14-19`) with: no trusted key configured → 503.
- `auth/tenant_context.py` uses the same verifier. Its three call sites need no other change. Its demonstration block (`:309-334`) and the one in `jwt_handler.py` (`:339-375`) call `create_access_token`. They move to the test fixture or are removed.
- `auth/__init__.py` stops exporting the two mint functions. Password hashing in `jwt_handler.py` is not in scope.
- `tests/conftest.py` `mint_bearer` signs with a test Ed25519 key that the test configuration pins.
- `service_contract.v1.json` `token_types` adds `application_access_jwt` (AUTHORITY_GAP-04).

## 4. Clients

### 4.1 Author-Forge desktop (WP-04)

- `apps/api` already receives the user's token for `hasCloudCapability` (`rake.ts:102-110`).
- In cloud mode only, `apps/api` calls the exchange, caches the Rake token in memory until 30 seconds before `exp`, and adds `Authorization: Bearer` in `rakeRequest`.
- A 401 from Rake clears the cache and retries the exchange once.
- In local mode, `apps/api` calls no exchange and sends no bearer.
- No route returns the Rake token to the WebView.
- Client authentication follows DEC-2.

### 4.2 Server application (specified, not built)

A server backend, such as bds_website, authenticates with `private_key_jwt`: an Ed25519 key that the operator registers on its `app_clients` row. It holds the Supabase token in an `HttpOnly` cookie session. It follows the 4.1 cache rules.

## 5. Failure behavior

| Condition | Result |
|---|---|
| ForgeCustomer unreachable | The application returns its existing "service unavailable" error. Author-Forge returns `RAKE_UNREACHABLE` 503. |
| Signing key unset | Exchange 503. |
| Rake has no trusted key | Rake 503 on every JWT route. |
| Unknown `kid` | Rake 401. |
| Entitlement lost | The next mint fails. Live tokens expire within 300 seconds. |
| Key compromise | The operator removes the `kid` from `RAKE_ACCESS_TRUSTED_KEYS` and redeploys Rake. Every token with that `kid` fails at once. |

## 6. Acceptance path (WP-05)

The DEC-RAKE-JWT-01 positive test uses the deployed services and a real test account with the Pro entitlement:

1. The test account signs in to Supabase with MFA.
2. A backend calls the deployed exchange and receives a token.
3. Deployed Rake accepts the token on a read route.
4. The same token is refused at a ForgeCustomer customer route (wrong audience).
5. A token with one changed claim byte is refused by Rake.
6. The account without the entitlement is refused at step 2.

The receipt records statuses, `jti`, `kid` and timestamps. It records no token value.
