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
