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
