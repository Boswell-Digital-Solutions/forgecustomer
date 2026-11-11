# BDS-FOC-RAKE-ACCESS-ISSUER-v0.1 — compiled plan set

ForgeCustomer Rake access-token issuer — token exchange for BDS applications. Lifecycle: authorized. Components 00–09 in order, maintained by hand. An edit to a component must be mirrored here in the same change.

---

# 00 — Plan identity, index and read order

This component gives the plan identity, the purpose, the source state and the read order.

| Field | Value |
|---|---|
| Plan ID | BDS-FOC-RAKE-ACCESS-ISSUER-v0.1 |
| Title | ForgeCustomer Rake access-token issuer — token exchange for BDS applications |
| Lifecycle | authorized |
| Authority | Operator ruling 2026-11-11 ("do it"): DEC-1 to DEC-5 accepted as recommended. WP-00 and WP-01 are authorized. WP-02 onward need G1 and the operator's word. No route, key, configuration, deployment or code merge is authorized. Before that: the D-JWTI rulings fixed the design and authorized no work. |
| Decision owner | Charles (Charlie) Boswell, BDS operator |
| Prepared | 2026-11-11 America/New_York |
| Parent / extension target | rake `docs/reviews/RAKE_JWT_ISSUER_ARCHITECTURE_REVIEW_2026-11-10.md` and `docs/evidence/D-JWTI/2026-11-11_OPERATOR_RULINGS.md` (rake `caac54c1a`). Findings: rake `RAKE-EXECUTION-JWT-NO-ISSUER-20261109`; Author-Forge `KI-AUF-20261106-003`. Authority row: forge `CLAUDE.md` "ForgeCustomer (application-token issuer; ruled 2026-11-11, not built)" (forge #411). Companion: forge `docs/architecture/rake_credential_authority_review.md`. |
| Proposed canonical owner | Boswell-Digital-Solutions/forgecustomer for the issuer, its key and the token contract. Boswell-Digital-Solutions/rake for the verifier. Boswell-Digital-Solutions/Author-Forge for the first application client. Each work package pins its repository SHA. |
| Primary candidate consumer | forgecustomer `api/src/routes/`, `api/src/services/signing.rs`, `api/src/config.rs`, `api/src/state.rs`, `supabase/migrations/`, `contracts/`; rake `auth/jwt_handler.py`, `auth/execution_access.py`, `auth/tenant_context.py`, `config.py`, `tests/conftest.py`, `service_contract.v1.json`; Author-Forge `apps/api/src/routes/rake.ts`. |
| Canonical Drive destination | /Forge/Plans/BDS_FOC_RAKE_ACCESS_ISSUER_v0.1_PLAN_SET (Drive folder `1S9fhDpmwCLD-6NYg1Wbd8MTfteFB4_6N`, uploaded 2026-11-11). It holds the compiled plan, the four siblings, and `BDS_FOC_RAKE_ACCESS_ISSUER_v0.1_PLAN_SET_3448007.tar.gz` (35,500 bytes, SHA-256 `9ff11a10b546b156719d8729a84562d3ac6bb1fc39c6965489f311533fda0feb`). The archive is the plan set at forgecustomer `3448007`, built with `git archive --mtime=<commit time>` and `gzip -n -9` (two builds byte-identical). The repository copy is the current one; the Drive copy is refreshed at each lifecycle change. |
| Compilation | Components 00–09, concatenated by hand into BDS-FOC-RAKE-ACCESS-ISSUER-v0.1_COMPILED_PLAN_SET.md. No build script. |

## Purpose

Rake accepts an access JWT at its execution boundary. No system issues that JWT. Rake verifies it with one shared HS256 secret and checks no `iss` and no `aud`. Every holder of the secret can mint a token, and Rake cannot tell an authorized issuer from any other holder.

The operator ruled the target on 2026-11-11:

- ForgeCustomer is the issuer (D-JWTI-1 option b, D-JWTI-2).
- Supabase project `forgecustomer` stays the login (D-JWTI-4).
- Browsers do not call Rake. An application reaches Rake only through its own backend (D-JWTI-5).
- The token is asymmetric, with pinned keys and required `iss`, `aud` and `exp` (D-JWTI-6).
- Only a token from ForgeCustomer counts for the DEC-RAKE-JWT-01 acceptance path (D-JWTI-7).

This plan builds that design from code that already works:

- ForgeCustomer's Supabase verifier (`api/src/auth/customer.rs`) and its `CustomerContext::require_active` gate (`api/src/auth/extract.rs:83`);
- ForgeCustomer's Ed25519 signer and verifying key ring (`api/src/services/signing.rs:30-142`) and its key-publication route (`GET /v1/entitlements/keys`);
- ForgeCustomer's entitlement assembly (`GET /v1/entitlements/current`, `POST /v1/entitlements/check`) and the feature key `authorforge.cloud.enabled` (`supabase/seed.sql:18`);
- Rake's scope and tenant binding (`auth/execution_access.py:24-88`), which stays as it is;
- Author-Forge's Rake proxy (`apps/api/src/routes/rake.ts`), which already keeps every Rake call in the backend.

It writes new code only where nothing exists: the exchange route, a dedicated signing key, a mint audit record, the pseudonymous tenant mapping, the pinned-key verifier in Rake and the client call in Author-Forge.

## Source state used to prepare this plan

| Repository | Branch | Commit | Read |
|---|---|---|---|
| forgecustomer | `main` | `3a2ce487e` | `doc/system/` 03, 10, 11, 12; `api/src/services/signing.rs`; `api/src/auth/extract.rs`; `api/src/domain/installation.rs` (`device_public_key`); `api/src/routes/mod.rs`; `api/Cargo.toml` (`jsonwebtoken` 9, `ed25519-dalek` 2); `supabase/seed.sql`; plan `BDS_FOC_ENTITLEMENT_KEY_LIFECYCLE_v0.1_PLAN_SET` (form and key custody precedent) |
| rake | `master` | `caac54c1a` | `doc/system/10_service-contract/05-api-layer.md`, `20_runtime/07-backend-services.md`, `50_operations/17-configuration.md`; the review and the D-JWTI rulings. The auth code (`auth/`, `config.py`, `tests/conftest.py`, `main.py`, `service_contract.v1.json`) is byte-identical to `c26b095`, the review's pin (`git diff --stat c26b095 origin/master` is empty). |
| Author-Forge | `main` | `1856b3b73` | `apps/api/src/routes/rake.ts` (lines 1-110); `docs/KNOWN_ISSUES.md` `KI-AUF-20261106-003`; `src-tauri/src/fleet_updates.rs:1628` (no `device_public_key` sent) |
| bds_website | `main` | `3b6a2e51d` | `git grep -i rake`: no match. bds_website does not call Rake at this commit. |
| forge | `main` | `88fda8241` | `CLAUDE.md` authority table; `docs/canonical/plan_registry_v1.json` |

"Inspected" marks a fact read in source at the pin. "Review" marks a fact taken from the rake review at its pin.

## Status

Authorized 2026-11-11 for WP-00 and WP-01 (receipt `evidence/RULINGS/2026-11-11_DEC-1_TO_DEC-5_OPERATOR_RULING.md`). AC-00 PASS. DEC-6 is open (09). The other acceptance criteria are NOT_RUN.

## Read order and components

00 → 09, then the four siblings. 00 Index · 01 Rulings · 02 Authority and gaps · 03 Bindings, coverage and reuse · 04 Contract · 05 Mechanism · 06 Decisions · 07 Acceptance · 08 Work packages · 09 Operator review

---

# 01 — Binding rulings and non-goals

This component lists the rulings that bind the plan, and the work the plan does not do.

## Rulings carried in

| ID | Ruling | Source |
|---|---|---|
| R01 | The application-domain issuer pattern is a BDS exchange issuer. It mints short-lived Rake access tokens after login and an entitlement check. | D-JWTI-1 (b), 2026-11-11 |
| R02 | ForgeCustomer owns the issuer. | D-JWTI-2, 2026-11-11 |
| R03 | The forge authority table has a row: ForgeCustomer issues short-lived Rake access tokens for applications, after a Supabase login and an entitlement check. It may not issue run tokens or lifecycle transitions, and may not serve tokens for direct browser calls to Rake. | D-JWTI-3, forge #411 |
| R04 | The Supabase project `forgecustomer` stays the BDS end-user login. | D-JWTI-4 |
| R05 | No direct browser-to-Rake access. A frontend reaches Rake only through its own backend. | D-JWTI-5 |
| R06 | The token is signed with an asymmetric key (EdDSA, or ES256). Rake pins public keys per issuer and `kid`. `iss`, `aud` and `exp` are required. The issuer binds the tenant and the scopes. Lifetimes are in minutes. `jti` has replay rules. This plan fixes the names and shapes. | D-JWTI-6 |
| R07 | Only a token from ForgeCustomer counts as "legitimately issued" for DEC-RAKE-JWT-01. A token minted with a shared secret does not count. | D-JWTI-7 |
| R08 | The work goes in a new cross-repository plan. This plan is that plan. | D-JWTI-8 |
| R09 | Nothing is in production. No compatibility code. Rake's HS256 path is removed when the verifier changes. There is no dual-algorithm period. | forge `CLAUDE.md` "Deployment status"; review §6 item 2 |
| R10 | The operator creates every signing key, outside any agent, as a deployment secret. The operator sets every deployed configuration value. No agent reads or prints a private key. | forge `CLAUDE.md`; forgecustomer `BDS-FOC-ENTITLEMENT-KEY-LIFECYCLE-v0.1` DEC-2 |
| R11 | Auth failure denies. A missing key, an unreachable dependency or an ambiguous entitlement fails closed. | forgecustomer `doc/system/11` "Failure doctrine" |
| R12 | Hosted Rake stays in Pro. A Rake call carries no local path and no raw project id. Rake does not persist author content. | Author-Forge `KI-AUF-20261106-003`, operator ruling 2026-11-06 |

## Non-goals

- **Service-domain credentials.** SMITH and ForgeAgents keep governed per-service API keys (review §6 item 3). This plan does not change Rake's API-key path or move its key store.
- **Governed-execution tokens.** The forge-command P3 run token is a separate trust domain. This plan does not amend `BDS-FCO-P3-TOKEN-ISSUER-001`.
- **A general identity provider.** The issuer mints one token type for one audience family. It does not issue ID tokens, refresh tokens or tokens for other services. A second audience needs a plan amendment.
- **The AuthorForge mission content rules.** Excerpt approval and Rake's no-persistence rule (R12) belong to Author-Forge and rake plans. This plan supplies only the credential and the pseudonymous tenant. See AUTHORITY_GAP-01.
- **bds_website.** It does not call Rake at `3b6a2e51d`. A server-side client is specified (05 §4.2) but not built.
- **Key rotation automation.** The plan reuses the entitlement-key overlap pattern. Automated rotation is out of scope.

---

# 02 — Authority boundaries and gaps

This component states who owns each part, and the gaps that block work.

## Boundaries

| System | Owns in this plan | Does not own |
|---|---|---|
| Supabase Auth (`forgecustomer` project) | Login, MFA (`aal`), sessions, refresh tokens | Rake scopes, tenants, entitlements |
| ForgeCustomer | The exchange route, the Rake access signing key, the pseudonymous tenant mapping, the mint audit record, the token contract | Rake's routes, Rake's data, author content |
| Rake | Verification, scope enforcement, tenant binding of each request and each persisted row | Token issuance (after WP-03 it holds no signing material) |
| Application backend (Author-Forge `apps/api`) | When to request a token, holding it in the backend, sending it to Rake | Minting, choosing its own tenant or scopes |
| Operator | Key creation, deployed configuration, deploys, merges, every decision in 09 | — |

ForgeCustomer must stay inside forgecustomer `doc/system/10`. The exchange stores no content, no prompt and no path. It stores pseudonymous ids and an audit row only.

## AUTHORITY_GAP conditions

| ID | Gap | Effect | Blocks |
|---|---|---|---|
| AUTHORITY_GAP-01 | Author-Forge has no ruled design for excerpt approval, path removal and the pseudonymous project reference. Rake has no no-persistence rule for AuthorForge content (`RAKE-AUTHORFORGE-CONTENT-TO-DATAFORGE-20261106`). | A working credential would let a desktop path, a project id and author content reach Rake and DataForge. | WP-04 activation in cloud mode (G6) |
| AUTHORITY_GAP-02 | forgecustomer `doc/system/10` and `11` do not list application-token issuance. | The repository's canonical docs disagree with the forge authority row. | Nothing; WP-02 fixes it in the same change |
| AUTHORITY_GAP-03 | Author-Forge sends no `device_public_key` (`src-tauri/src/fleet_updates.rs:1628`). ForgeCustomer verifies no device signature today. | Desktop client authentication option A in DEC-2 needs new code on both sides. | WP-04 if DEC-2 = A |
| AUTHORITY_GAP-04 | Rake `service_contract.v1.json` `auth_requirements.token_types` names no application JWT (review §2.6). | The contract file disagrees with the code. | Nothing; WP-03 fixes it |
| AUTHORITY_GAP-05 | The Supabase project's signing-key type was not read (review §9). | It does not change this design, which verifies Supabase tokens with the existing ForgeCustomer verifier. | Nothing |
| AUTHORITY_GAP-06 | Rake's deployed `ALLOWED_ORIGINS` was not read (review §9). | R05 needs no public origin. A public origin there is a separate finding. | Nothing |

---

# 03 — Protocol bindings, requirement coverage and reuse

This component lists the governing requirements, maps each one to design, work and evidence, and lists the code the plan reuses.

## Governing sources

| Source | Standing | Applies to |
|---|---|---|
| BDS-PSS-001 (Drive `1yWQUGn4yaSPJcQrui3Y5LBcycvCnAHyxhxayk_RiKCY`, read 2026-11-11) | binding | plan form |
| BDS-QA-001 | binding | 07 evidence rules, negative checks |
| BDS-DOC-001 and the forge documentation protocol (`doc/system/` canonical, `BUILD.sh`) | binding | each repository's `doc/system/` change |
| ASD-STE100 (forge `docs/protocols/BDS_ASD_STE100_v1.md`) | binding | documentation, commit messages, operator-visible strings |
| forge `CLAUDE.md` authority table, "Deployment status" and findings protocol | binding | R03, R09, R10, AC-09 |
| RFC 7519 (JWT) | binding | token format |
| RFC 8037 (EdDSA in JOSE) and RFC 8032 (Ed25519) | binding if DEC-4 = A | signature |
| RFC 8725 (JWT best current practice) | advisory, selected | algorithm pinning, `typ`, audience checks |
| RFC 8693 (token exchange) | advisory, selected as a pattern | exchange shape. The wire format is BDS-specific, not RFC 8693 conformant. |
| RFC 8252 (native apps) | advisory, selected | desktop client handling, DEC-2 |

## Requirements and coverage

| REQ | Requirement | Source | Design | WP | AC |
|---|---|---|---|---|---|
| REQ-01 | Asymmetric signature. Rake holds no signing material. | R06 | 04 §1 | WP-03 | AC-05, AC-06 |
| REQ-02 | Rake pins public keys per issuer and `kid`. No key comes from the token or the network at run time. | R06 | 04 §3 | WP-03 | AC-06 |
| REQ-03 | `iss`, `aud`, `exp`, `iat` and `jti` are required. | R06 | 04 §2 | WP-01, WP-03 | AC-01, AC-06 |
| REQ-04 | The issuer binds the tenant, the user and the scopes. The caller cannot choose them. | R06 | 05 §2 | WP-02 | AC-02, AC-04 |
| REQ-05 | Lifetime in minutes. No Rake refresh token. | R06 | 04 §2 | WP-02, WP-03 | AC-02, AC-06 |
| REQ-06 | `jti` rule as ruled in DEC-5. | R06 | 04 §2.3 | WP-02, WP-03 | AC-02, AC-06 |
| REQ-07 | A mint needs a valid Supabase login, an active profile, the MFA rule and the application's entitlement. | R01 | 05 §2 | WP-02 | AC-02 |
| REQ-08 | The Rake token stays in the application backend. A browser or WebView never receives it. | R05 | 05 §4 | WP-04 | AC-07 |
| REQ-09 | A dedicated signing key, separate from the entitlement key, created by the operator. | R10, review §6 | 04 §3 | WP-02 | AC-03, G4 |
| REQ-10 | Rake's HS256 path is removed. No dual path. | R09 | 05 §3 | WP-03 | AC-06 |
| REQ-11 | Every failure denies. | R11 | 05 §5 | WP-02, WP-03 | AC-02, AC-06 |
| REQ-12 | The credential path carries no local path, no raw project id and no identity beyond pseudonyms. The issuer stores no content. | R12, forgecustomer `doc/system/10` | 04 §2, 05 §2 | WP-02, WP-04 | AC-04, AC-07 |
| REQ-13 | The DEC-RAKE-JWT-01 positive test uses a token from the deployed issuer. | R07 | 05 §6 | WP-05 | AC-08 |
| REQ-14 | Each change updates its `doc/system/` part, its findings and the registry in the same change. | forge `CLAUDE.md` | 08 | every WP | AC-09 |

### Dispositions

- **Excerpt approval and Rake no-persistence (R12, second half):** deferred to Author-Forge and rake plans. Effect: WP-04 can be built, but G6 (activation in cloud mode) stays closed. This deferral does not satisfy R12.
- **bds_website client:** deferred. bds_website does not call Rake. 05 §4.2 specifies the server client so that a later amendment is small.

## Reuse matrix

| Need | Existing code | Use |
|---|---|---|
| Verify the Supabase token | forgecustomer `api/src/auth/customer.rs` (ES256 JWKS, pinned `iss` and `aud`, fail-closed cache) | as is |
| Active profile and MFA gate | forgecustomer `CustomerContext::require_active` (`api/src/auth/extract.rs:83`) | as is |
| Entitlement decision | forgecustomer entitlement assembly behind `GET /v1/entitlements/current` and `POST /v1/entitlements/check` | call the same service function; no HTTP hop |
| Ed25519 signing | forgecustomer `Signer25519` (`api/src/services/signing.rs:30-77`) | a second instance with its own key |
| Key overlap and publication | forgecustomer `VerifyingKeyRing` (`signing.rs:80-142`), `ENTITLEMENT_VERIFY_PUBLIC_KEYS` pattern, `GET /v1/entitlements/keys` | same pattern, own variables and route |
| JWT encoding | forgecustomer `jsonwebtoken` 9 (`api/Cargo.toml:31`) | encode EdDSA with the dedicated key |
| Pinned key map at the verifier | `{kid: {key, issuer}}` shape of dataforge-Local `XA_RUN_TOKEN_PUBLIC_KEYS`, as cited in `BDS-FOC-ENTITLEMENT-KEY-LIFECYCLE-v0.1` 00 | same shape in Rake |
| Scope and tenant binding at Rake | rake `auth/execution_access.py:24-88` | unchanged; it keeps reading `type`, `tenant_id`, `user_id`, `scopes` |
| PyJWT EdDSA | rake `PyJWT[crypto]` 2.15.1 (rake `doc/system/30_dependencies/12-tech-stack.md:58`) | `algorithms=["EdDSA"]` |
| Backend-only Rake calls | Author-Forge `apps/api/src/routes/rake.ts` ("Zero direct frontend-to-Rake calls") | add the bearer in `rakeRequest` |
| Per-IP rate limit | forgecustomer router `RATE_LIMIT_PER_MINUTE` | as is |

---

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

- **Caller authentication:** `Authorization: Bearer <Supabase access token>`, then the client authentication of DEC-2.
- **Request body:** `{"app_id": "authorforge", "scopes": ["rake:read", "rake:execute"]}`. `scopes` is optional. If omitted, the issuer grants every allowed scope.
- **Success, 200:** `{"access_token": "...", "token_type": "Bearer", "expires_in": 300, "scopes": [...]}`. The response has `cache-control: no-store`.

| Status | Code | Cause |
|---|---|---|
| 401 | `UNAUTHORIZED` | Missing or invalid Supabase token |
| 401 | `CLIENT_AUTH_FAILED` | DEC-2 client authentication failed |
| 403 | `MFA_REQUIRED` | The existing `require_active` rule |
| 403 | `APP_NOT_ALLOWED` | Unknown or disabled `app_id` |
| 403 | `SCOPE_NOT_ALLOWED` | A requested scope is outside the application's list |
| 403 | `ENTITLEMENT_DENIED` | The application's required feature is false or unknown for the customer |
| 503 | `SERVICE_UNAVAILABLE` | Signing key not configured, or the entitlement state cannot be read |

## 5. Records (ForgeCustomer migrations)

| Table | Columns | Rules |
|---|---|---|
| `app_clients` | `app_id` (pk), `audience`, `allowed_scopes` (text[]), `required_feature_key`, `client_auth_method`, `status`, `created_at` | Seeded by migration. Seed row: `authorforge`, `rake.application`, `{rake:read,rake:execute,rake:approve}` (DEC-6), `authorforge.cloud.enabled`, the DEC-2 method, `active`. |
| `rake_tenant_pseudonyms` | `customer_id` (fk), `audience`, `tenant_pseudonym` (uuid, random), `user_pseudonym` (uuid, random), `created_at`, `revoked_at` | One active row per (customer, audience). Created on first mint. Revocation creates a new row. Never sent to DataForge. |
| `app_token_mints` | `jti` (pk), `customer_id`, `app_id`, `audience`, `kid`, `scopes`, `issued_at`, `expires_at`, `client_auth_method` | No token value. No request body. Written in the same transaction as the decision. |

A deletion request (forgecustomer `doc/system/03` deletion surface) must revoke the pseudonym row and keep `app_token_mints` rows only with the anonymized `customer_id`. WP-02 extends the existing deletion transaction.

## 6. Contract files

WP-01 writes these files in forgecustomer `contracts/`:

- `rake-access-token-v1.schema.json` — the header and claims of §1 and §2.
- `vectors/rake-access-token-v1/vectors.json` and its generator `generate.py` — 3 positive and 15 negative vectors. The test key is derived from a published label, so no private key is stored.

**Revision 2026-11-11 (WP-01):** the `contracts/openapi.yaml` entries for §3 and §4 move to WP-02. `openapi.yaml` lists only built routes, so an entry before the route exists would describe a route that does not exist.

## Provenance

The contract version is `v1`. A change to a claim name, a type or a rule creates `v2`. No version runs beside another, because nothing is in production (R09). Rake pins the contract version in its tests.

---

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

---

# 06 — Decision semantics

This component states what accept, decline and revise mean for this plan.

| Answer | Effect |
|---|---|
| **Accept** a decision in 09 | The recommended value binds. 04 and 05 stay as written for that item. The answer authorizes only what the decision row names. |
| **Choose an alternative** | The plan is revised: 04, 05, 07 and 08 change for that item in one change, before any work package uses it. |
| **Decline** a decision | The affected work packages stay proposed. The plan records the reason. |
| **Defer** | The plan stays proposed for the affected items. Independent authorized work continues. |

A gate passes only with its evidence in 07. A ruling does not pass a gate.

The plan lifecycle changes only by an operator ruling: `proposed` → `authorized` (DEC-1) → `in_progress` (first WP starts) → `implemented_unverified` (WP-02 to WP-04 merged) → `complete` (AC-08 PASS and records closed). The registry maps `authorized` to `approved`.

Work-package status uses: `proposed`, `authorized`, `in_progress`, `delivered`, `blocked`. Acceptance results use: `NOT_RUN`, `PASS`, `FAIL`, `BLOCKED`, `UNVERIFIED`.

---

# 07 — Acceptance and evidence

This component defines each acceptance criterion, its check and its evidence. AC-00 is PASS (2026-11-11, `evidence/AC-00/`). AC-01 is PASS on the WP-01 candidate before merge (`evidence/AC-01/`). The others are NOT_RUN.

Evidence goes to `evidence/AC-nn/` in this plan set, unless a row names another place. Each record names the source commit, the environment, the exact command, the test ids that ran and the observed result. A skipped or unavailable check is recorded as such. It does not pass.

| AC | WP / REQ | Method | Expected observation | Baseline and environment | Limits |
|---|---|---|---|---|---|
| AC-00 | WP-00 | Re-read each pinned file at the current default branch. `git diff --stat <pin> <head> -- <paths>` per repository. | Each path is unchanged, or each change is read and recorded with its effect on 04 and 05. | The 00 pins; the operator's machine | Source only. No deployed service is called. |
| AC-01 | WP-01; REQ-03, REQ-05 | forgecustomer `cargo test -p forgecustomer-api --test rake_access_contract` validates every vector against `rake-access-token-v1.schema.json`, checks the two cross-claim rules, and verifies each token signature. | Every positive vector validates. Each negative vector fails for its named reason: no `iss`, no `aud`, no `exp`, no `jti`, `alg` `HS256`, `alg` `none`, `scopes` as a string, lifetime 901 s, `type` `refresh`. The test output lists each vector id. | forgecustomer WP-01 branch; Linux, stable Rust | Static vectors. It proves the schema, not the runtime. |
| AC-02 | WP-02; REQ-04, REQ-05, REQ-07, REQ-11 | forgecustomer integration tests for `POST /v1/app-tokens/rake` against a test database. | No token → 401. Bad Supabase token → 401. `aal1` with `mfa_required` → 403 `MFA_REQUIRED`. Unknown app → 403 `APP_NOT_ALLOWED`. Extra scope → 403 `SCOPE_NOT_ALLOWED`. `rake:approve` with another scope → 403 `SCOPE_NOT_ALLOWED`. Feature false → 403 `ENTITLEMENT_DENIED`. Key unset → 503. Audit write failure → 503 and no token. Success → 200, the token verifies with the published key, the claims match 04 §2, one `app_token_mints` row exists, `cache-control: no-store`. | forgecustomer WP-02 branch; the existing test harness | A test Supabase issuer, not the deployed project |
| AC-03 | WP-02; REQ-09 | Start-up tests and a keys-route test. | Start-up fails when the Rake key equals the entitlement key. `GET /v1/app-tokens/keys` returns public keys only, with the active and standby `kid`. `GET /v1/entitlements/keys` does not list the Rake key. | as AC-02 | — |
| AC-04 | WP-02; REQ-04, REQ-12 | Tests on the pseudonym mapping and the token body. | Two mints for one customer give the same `tenant_id`. Two customers give different values. A revoked row gives a new value. The decoded token holds no email, no Supabase `sub`, no `customer_id`. A deletion run revokes the row. | as AC-02 | — |
| AC-05 | WP-03; REQ-01 | rake `pytest tests/` with the test key pinned. | A token from the 04 vectors is accepted on a read route and on an execute route. The existing scope and tenant tests pass unchanged. | rake WP-03 branch; Python per rake CI | — |
| AC-06 | WP-03; REQ-02, REQ-03, REQ-05, REQ-06, REQ-10, REQ-11 | rake negative tests. Each test asserts the status and the refusal reason. | 401 for: an HS256 token signed with any secret; `alg` `none`; wrong `iss`; wrong `aud`; unknown `kid`; known `kid` with the wrong `iss`; expired; no `jti`; `type` `refresh`; `typ` missing; lifetime over 900 s; a token that mixes `rake:approve` with another scope; a second approve call with the same approve token (`TOKEN_ALREADY_USED`, and the first call succeeded). 503 when `RAKE_ACCESS_TRUSTED_KEYS` is empty. The runner output names each test. A generic failure or a skipped test does not count. | as AC-05 | — |
| AC-07 | WP-04; REQ-08, REQ-12 | Author-Forge `apps/api` route tests. | In cloud mode, `rakeRequest` sends `Authorization: Bearer` with the exchanged token. In local mode it calls no exchange and sends no bearer. No route response body holds the Rake token. A Rake 401 causes one new exchange, then an error. | Author-Forge WP-04 branch; its backend test harness | Fixture ForgeCustomer and Rake servers |
| AC-08 | WP-05; REQ-13 | 05 §6 against the deployed services, run or authorized by the operator. | Steps 2 and 3 succeed. Steps 4, 5 and 6 are refused with the named status. | Deployed ForgeCustomer and Rake after G4; a test account | Records statuses, `jti`, `kid` and times only. No token value. |
| AC-09 | every WP; REQ-14 | Read the diff of each WP. Run `bash doc/system/BUILD.sh` in each repository changed. Run forge `python3 scripts/check-plan-registry.py`. | Each changed route, setting and caller is in that repository's `doc/system/`. The findings in 00 carry a dated update. The registry row matches this plan's lifecycle. | Each WP's PR | Review-time check |

---

# 08 — Work packages and gates

This component defines each work package with its nine-field brief, and the gates between them.

## Gates

| Gate | Passes when | Unblocks |
|---|---|---|
| G0 | AC-00 PASS | WP-01 |
| G1 | WP-01 merged with AC-01 PASS, and DEC-2 to DEC-5 ruled | WP-02, WP-03, WP-04 |
| G2 | WP-02 merged with AC-02 to AC-04 PASS | G4 |
| G3 | WP-03 merged with AC-05 and AC-06 PASS | G4 |
| G4 | The operator creates the Rake access keys, sets the ForgeCustomer and Rake variables, and deploys both | WP-05 |
| G5 | AC-08 PASS | plan `implemented_unverified` → `complete` after AC-09 |
| G6 | AUTHORITY_GAP-01 closed by its own plans, and WP-04 merged with AC-07 PASS | Author-Forge hosted Rake in cloud mode |

Order: G0 → G1 → {G2, G3} → G4 → G5. G6 is independent of G5 but needs G2 and G3.

WP-03 removes Rake's HS256 path. No known holder of a Rake JWT exists (review §2.2), so the merge breaks no caller.

## WP-00 — Source lock

**Status:** delivered 2026-11-11. AC-00 PASS (`evidence/AC-00/`). G0 PASS.

| Field | Brief |
|---|---|
| Outcome | The 00 pins are confirmed or updated, so 04 and 05 rest on current code. |
| Starting evidence | 00 source table. |
| Prerequisites | DEC-1. |
| Permitted scope | Read-only in every repository. Writes only `evidence/AC-00/` in this plan set. |
| Boundaries | No deployed service call. No secret read. |
| Deliverables | `evidence/AC-00/SOURCE_LOCK.md`, `evidence/AC-00/RESULT.md`. |
| Acceptance criteria | AC-00. |
| Verification | 07 AC-00. |
| Stop conditions | Done when AC-00 is recorded. Pause if a pinned file changed in a way that breaks 04 or 05. Then record the change and revise the plan first. |

## WP-01 — Token contract

**Status:** delivered for review 2026-11-11. AC-01 PASS on the candidate (`evidence/AC-01/`). Its merge needs the operator's word, because it adds a test.

| Field | Brief |
|---|---|
| Outcome | 04 exists as machine-checkable files. |
| Starting evidence | 04; forgecustomer `contracts/openapi.yaml` at the WP-00 pin. |
| Prerequisites | G0; DEC-1. DEC-2 to DEC-5 should be ruled first. If they are not, the vectors use the recommended values and the WP stays open. |
| Permitted scope | forgecustomer `contracts/`, one test file under `api/tests/`, this plan set, `doc/system/03` and `11`. |
| Boundaries | No route, no migration, no key. Test keys only. |
| Deliverables | The files of 04 §6 and the AC-01 test. |
| Acceptance criteria | AC-01, AC-09. |
| Verification | 07 AC-01; forgecustomer CI. |
| Stop conditions | Done when merged with AC-01 PASS. Pause if a decision ruling changes a claim. Then revise 04 first. |

## WP-02 — Issuer in ForgeCustomer

| Field | Brief |
|---|---|
| Outcome | 05 §2 works in ForgeCustomer. |
| Starting evidence | The 03 reuse matrix rows for ForgeCustomer. |
| Prerequisites | G1. |
| Permitted scope | forgecustomer `api/src/`, `supabase/migrations/` (three tables of 04 §5), `supabase/seed.sql`, `.env.example` (names only), `contracts/`, `doc/system/03`, `04`, `10`, `11`, `12`, `docs/KNOWN_ISSUES.md`. |
| Boundaries | No deployed configuration, key or secret. No change to the entitlement key or its route. No change to customer or admin token rules. |
| Deliverables | The route, the keys route, the `contracts/openapi.yaml` entries for both (moved from WP-01), the migration, the deletion extension, the tests, the doc/system updates (closes AUTHORITY_GAP-02). |
| Acceptance criteria | AC-02, AC-03, AC-04, AC-09. |
| Verification | 07; forgecustomer CI including `cargo audit`. |
| Stop conditions | Done when merged on the operator's word with the ACs PASS. Pause if the entitlement service cannot answer the feature check without a new authority. |

## WP-03 — Verifier in Rake

| Field | Brief |
|---|---|
| Outcome | 05 §3 works in Rake, and the HS256 path is gone. |
| Starting evidence | rake `auth/jwt_handler.py`, `auth/execution_access.py:9-88`, `auth/tenant_context.py`, `auth/__init__.py`, `config.py:282-304`, `config.py:659-681`, `forge_keys.py:56-57`, `tests/conftest.py:35`, `:167`, `service_contract.v1.json`, `render.yaml:44-45`. |
| Prerequisites | G1. The contract vectors from WP-01. |
| Permitted scope | Those files, `api/mission_routes.py` (the approve handler, for the single-use check), one alembic migration for `consumed_approve_tokens` (DEC-6), rake tests, rake `doc/system/` parts that name the JWT, `docs/KNOWN_ISSUES.md`. `render.yaml` may drop `JWT_SECRET_KEY` and declare `RAKE_ACCESS_TRUSTED_KEYS` without a value. |
| Boundaries | No deployed configuration change. The API-key path is unchanged. Scope and tenant binding are unchanged. |
| Deliverables | The verifier, the single-use approve check and its migration (DEC-6), the tests, the contract update, the doc/system updates. |
| Acceptance criteria | AC-05, AC-06, AC-09. |
| Verification | 07; rake CI (all required jobs). |
| Stop conditions | Done when merged on the operator's word. Pause if a non-test caller of `create_access_token` appears. |

## WP-04 — Author-Forge client

| Field | Brief |
|---|---|
| Outcome | 05 §4.1 works in `apps/api`. |
| Starting evidence | Author-Forge `apps/api/src/routes/rake.ts`, `apps/api/tests/routes/rake.test.ts`, `KI-AUF-20261106-003`. |
| Prerequisites | G1. If DEC-2 = A, AUTHORITY_GAP-03 work in `src-tauri` and in WP-02. |
| Permitted scope | `apps/api/src/routes/rake.ts`, a new exchange client module under `apps/api/src/services/`, tests, Author-Forge `doc/system/` part for the Rake proxy, `docs/KNOWN_ISSUES.md`. Use the `afma01` worktree, never `git checkout -b` in the shared tree. |
| Boundaries | No change to the request body, path handling or project id. Those belong to AUTHORITY_GAP-01. No WebView change. |
| Deliverables | The client, the tests, the doc update. |
| Acceptance criteria | AC-07, AC-09. |
| Verification | 07; Author-Forge CI. |
| Stop conditions | Done when merged with AC-07 PASS. Activation in cloud mode waits for G6. |

## WP-05 — Acceptance on deployed services

| Field | Brief |
|---|---|
| Outcome | DEC-RAKE-JWT-01's positive path is proven with a token from the ruled issuer. |
| Starting evidence | G4 receipts from the operator. |
| Prerequisites | G2, G3, G4. A test account with and one without the Pro entitlement. |
| Permitted scope | Calls to deployed ForgeCustomer and Rake as in 05 §6. Writes `evidence/AC-08/` and rake `docs/evidence/DEC-RAKE-JWT-01/`. |
| Boundaries | No configuration change. No token value in any record. |
| Deliverables | The AC-08 receipt; the rake KI update; the lifecycle change proposal. |
| Acceptance criteria | AC-08, AC-09. |
| Verification | 07 AC-08. |
| Stop conditions | Done when AC-08 is PASS or FAIL with evidence. On FAIL, record the cause and stop. |

---

# 09 — Operator review and handoff

This component records the current authority, the decisions this plan needs, and the handoff.

## Current authority

- D-JWTI-1 to D-JWTI-8, ruled 2026-11-11. Receipt: rake `docs/evidence/D-JWTI/2026-11-11_OPERATOR_RULINGS.md` (rake #186, `caac54c1a`). They fix the target design and ask for this plan. They authorize no work.
- The operator asked for this draft on 2026-11-11 ("I concur proceed", after the offer to draft the issuer plan as a proposed plan).
- Planning is complete.
- **DEC-6 ruled 2026-11-11: A** ("yes"). Receipt: `evidence/RULINGS/2026-11-11_DEC-6_OPERATOR_RULING.md`.
- **DEC-1 to DEC-5 ruled 2026-11-11** ("do it"), each as recommended. Receipt: `evidence/RULINGS/2026-11-11_DEC-1_TO_DEC-5_OPERATOR_RULING.md`. WP-00 and WP-01 are authorized.
- WP-00 delivered: AC-00 PASS, G0 PASS.

## Decisions

### DEC-1 (ruled 2026-11-11: as recommended) — Start the plan (WP-00 and WP-01)

- **Question:** may an agent lock the sources and write the token contract as files in forgecustomer?
- **Recommendation: yes.** WP-00 is read-only. WP-01 adds a schema, test vectors and one test. Neither changes a route, a key or a deployment.
- **Effect:** the plan becomes `authorized` for WP-00 and WP-01 only. WP-02 onward stay proposed until G1 and your word.
- **Alternative:** defer. The plan stays proposed. Rake keeps its HS256 path with no issuer.

### DEC-2 (ruled 2026-11-11: as recommended) — How the desktop app proves it is AuthorForge

- **Question:** the AuthorForge desktop app cannot keep a secret, because anyone can read a file on their own machine. What must it show, besides the user's login, to get a Rake token?
- **Options:**
  - **A. The installation's device key (recommended).** The app makes an Ed25519 key per installation, keeps it in the OS keyring, and registers its public half with ForgeCustomer, which already accepts `device_public_key`. The app signs each exchange request. ForgeCustomer checks the signature against the registered, activated installation.
  - **B. The user's login only.** The app names itself (`app_id`). The login, the MFA rule and the Pro entitlement are the whole gate.
- **Why A:** your 2026-11-06 ruling says the hosted Rake path must be "as secure as possible". A ties each token to one activated installation, inside the device limit. You can revoke one machine without touching the account. A stolen login token alone cannot get a Rake token.
- **Cost of A:** new code in the desktop app and in ForgeCustomer (AUTHORITY_GAP-03). It depends on keyring storage, which Author-Forge marks as not yet established. It is the slower path.
- **Why someone picks B:** it is less work and ships sooner. A stolen login token already gives that user's ForgeCustomer access. B adds Rake access for that one user, for five minutes per token.
- **Reversibility:** B can move to A later with a new `client_auth_method` value. Nothing is in production, so there is no migration.
- **Uncertainty:** the keyring state in Author-Forge was not re-read for this plan.

### DEC-3 (ruled 2026-11-11: as recommended) — Which tenant id Rake sees

- **Question:** Rake needs a tenant id and a user id. Which values does the token carry?
- **Options:**
  - **A. A random pseudonym per customer, stored in ForgeCustomer (recommended).** It is created on the first mint and never sent to DataForge.
  - **B. The ForgeCustomer `customer_id`.**
  - **C. A pseudonym that ForgeCustomer computes with a new secret (HMAC).**
- **Why A:** Rake stores job data in DataForge. ForgeCustomer also sends `customer_id` to DataForge in its outbox events. With B, DataForge could join research jobs with commercial events. A prevents that join and needs no new secret. A deletion revokes the row.
- **Cost of A:** one table and one lookup per mint.
- **Not decided here:** the book-level separation inside a tenant. Today Author-Forge sets `tenant_id` to the project id. Under this plan the tenant is per customer. A book reference inside the tenant is part of AUTHORITY_GAP-01.

### DEC-4 (ruled 2026-11-11: as recommended) — Signature, scope format and lifetime

- **Question:** which algorithm, which scope claim and how long a token lives?
- **Recommendation:**
  - EdDSA (Ed25519). ForgeCustomer already signs with Ed25519, and Rake's PyJWT supports it.
  - A `scopes` JSON list, the claim Rake reads today, so Rake's scope code does not change.
  - 300 seconds, with no refresh token.
- **Alternative:** ES256, which the review also allows, and the OAuth `scope` string. Either one changes code in Rake with no security gain.
- **Tradeoff:** a short lifetime means one exchange call about every five minutes per active user. The backend caches the token.

### DEC-5 (ruled 2026-11-11: as recommended) — Replay rule and the approve scope

- **Question:** must Rake refuse a token it has already seen? Do application tokens get `rake:approve`?
- **Recommendation:**
  - Log `jti` on both sides, and keep no replay store. The token never leaves the backend and lives 300 seconds.
  - No `rake:approve` for applications in v1. WP-00 checks whether the Author-Forge mission flow needs it. If it does, the plan returns to you with a single-use `jti` rule for approve only.
- **Alternative:** a single-use store in Rake for every token. It needs a database table and a lookup on every request.

### DEC-6 (ruled 2026-11-11: A, single-use approve token) — Approve permission for AuthorForge (new, from WP-00)

- **Question:** Rake runs a mission only after someone calls its approve route. AuthorForge creates missions but never approves them (AC-00). Should AuthorForge tokens carry `rake:approve`?
- **Options:**
  - **A. Yes, with a single-use rule (recommended).** A token that carries `rake:approve` is usable once for an approve call. Rake stores the `jti` until the token expires and refuses a second use. AuthorForge requests an approve token only when the author approves a mission.
  - **B. Yes, with no replay store.** Same as the other scopes. Simpler, but a captured token can approve again within five minutes.
  - **C. No.** AuthorForge missions cannot run until a different approval path exists.
- **Why A:** an approval spends the author's money and starts work. One click must give one approval. The store is small: one row per approve token for five minutes.
- **Cost of A:** a `jti` table or cache in Rake, and a separate exchange call with `scopes: ["rake:approve"]`. WP-03 grows by one test group.
- **What waits on this:** only the AuthorForge approve step in WP-04 and the approve rows of the contract. WP-01 can merge without it.

## Handoff

- **Next action:** the merge of the WP-01 PR (the operator gave the word on 2026-11-11; the agent's merge was refused by the session's permission classifier, so the operator merges it). After the merge, G1 passes.
- **Your word needed:** WP-02 (issuer in ForgeCustomer) and WP-03 (verifier in Rake).
- **Your actions later, at G4:** create the active and standby Rake access keys, set the ForgeCustomer and Rake variables, deploy both. No agent does these.
- **Open outside this plan:** AUTHORITY_GAP-01 (excerpt approval, no local path, no raw project id, Rake no-persistence). It blocks AuthorForge activation (G6), not the issuer or the verifier.
- **Drive copy:** uploaded 2026-11-11 at the `3448007` state (see 00). Drive reported the same byte size for each file. No hash readback was done.
