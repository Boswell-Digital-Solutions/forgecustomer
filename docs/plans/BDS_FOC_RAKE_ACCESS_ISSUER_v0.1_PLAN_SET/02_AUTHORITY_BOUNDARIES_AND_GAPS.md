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
| AUTHORITY_GAP-02 | forgecustomer `doc/system/10` and `11` do not list application-token issuance. | The repository's canonical docs disagree with the forge authority row. | Nothing. **Closed by WP-02 (2026-11-11):** `doc/system/10` and `11` now list it. |
| AUTHORITY_GAP-03 | Author-Forge sends no `device_public_key` (`src-tauri/src/fleet_updates.rs:1628`). ForgeCustomer verifies no device signature today. | Desktop client authentication option A in DEC-2 needs new code on both sides. **2026-11-11:** the ForgeCustomer side is built in WP-02. The Author-Forge side (generate, register and keep the device key) stays open. | WP-04 |
| AUTHORITY_GAP-04 | Rake `service_contract.v1.json` `auth_requirements.token_types` names no application JWT (review §2.6). | The contract file disagrees with the code. | Nothing; WP-03 fixes it |
| AUTHORITY_GAP-05 | The Supabase project's signing-key type was not read (review §9). | It does not change this design, which verifies Supabase tokens with the existing ForgeCustomer verifier. | Nothing |
| AUTHORITY_GAP-06 | Rake's deployed `ALLOWED_ORIGINS` was not read (review §9). | R05 needs no public origin. A public origin there is a separate finding. | Nothing |
