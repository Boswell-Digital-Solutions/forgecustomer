# G4 — result

**Result: EXECUTED 2026-11-12, before G2 passed. Not recorded as PASS.**

The operator created the Rake access keys, set the variables and deployed both services on 2026-11-12. The gate order in 08 is G0 → G1 → {G2, G3} → G4. At that time G2 was open on the AC-04 deletion row (records at forgecustomer `071000d`). This record does not change that order. Evidence can show what occurred. It cannot show that G2 was satisfied first.

## Chronology (UTC)

| Time | Event | Source |
|---|---|---|
| 2026-11-12 | G2 open on the AC-04 deletion row; G1 and G3 PASS | forgecustomer #56, merge `071000d` |
| 23:22:18 | `public.consumed_approve_tokens` created in Supabase project `embvfponjxejbtrkryzs` (DataForgedb), as migration `rake_consumed_approve_tokens` | Supabase migration list, read by the agent |
| 23:24 | ForgeCustomer deploy `dep-dbr4jugu01pc73a5ing0`, source `071000dc3be54ec06773afd94d95521942515a91`, reported live | operator receipt |
| 23:30 | Rake deploy `dep-dbr4lg56e07s73agofug`, source `78b64958d3affea06ffd5c6f6e515dda278b12a6`, reported live | operator receipt |
| 23:30:58 | Operator HTTP checks | `g4-http-verification.json` |
| 2026-11-13 00:19 | Agent checks (below) | this record |
| 2026-11-12 (session) | Operator ruling: G2 = A; records PR may open, not merge | `../RULINGS/2026-11-12_G2_OPTION_A_AND_RECORDS_OPERATOR_RULING.md` |

The operator's receipt gives the deploy times in EST (6:24 PM and 6:30 PM). The agent did not read the deploy records, because the Render connector had no workspace selected.

## What the operator reported

- Two independent Ed25519 key pairs. Active `rake-access-1`, standby `rake-access-0`, issuer `bds-forgecustomer`. Each pair passed a sign and verify check.
- ForgeCustomer: `RAKE_ACCESS_SIGNING_PRIVATE_KEY` (active seed only), `RAKE_ACCESS_SIGNING_KEY_ID`, `RAKE_ACCESS_VERIFY_PUBLIC_KEYS` and `RAKE_ACCESS_ISSUER` set.
- Rake: `RAKE_ACCESS_TRUSTED_KEYS` set, public keys only. The Rake runtime has no `RAKE_ACCESS_SIGNING_PRIVATE_KEY`.
- Rake's deployed verifier, imported in its shell, loads key fingerprints that match the generated public keys, and trusts only `bds-forgecustomer`.
- A read-only jobs request with an invalid token returned 503 before the Rake deploy and 401 after.
- The approve-token table rejects a duplicate `jti`. The test rows were rolled back.
- The table definition is from Rake revision `004_consumed_approve_tokens`. The shared Alembic version stayed `20261101_01`.

Files in this folder, copied from the operator's output folder. Each holds public values only:

| File | Content |
|---|---|
| `forgecustomer-published-keys.json` | The key endpoint response |
| `g4-http-verification.json` | The operator's HTTP checks |
| `g4-database-verification.json` | The operator's table checks |
| `rake-consumed-approve-tokens.sql` | The applied table definition and access restrictions |

The runtime screenshot, the receipt text and `rake-access-public-config.json` stay in the operator's folder. The config file maps key ids to public keys, and the secret scan's `generic-api-key` rule reads that shape as a secret. Its public keys are the same as in `forgecustomer-published-keys.json`.

## What the agent checked (2026-11-13 00:19 UTC)

| Check | Observed |
|---|---|
| `GET https://forge-customer.onrender.com/v1/app-tokens/keys` | 200. `rake-access-1` `active` and `rake-access-0` `verify_only`, both `EdDSA`. The public keys equal the values in the operator's `rake-access-public-config.json` and in `forgecustomer-published-keys.json`. |
| `GET https://rake-zp35.onrender.com/health` | 200 |
| `GET https://rake-zp35.onrender.com/api/v1/jobs` with an invalid EdDSA token | 401 `UNAUTHENTICATED`, "Invalid execution credential" |
| DataForgedb `public.consumed_approve_tokens` | Columns `jti` and `exp`, both NOT NULL. Row level security on. No `anon` or `authenticated` grant. Alembic version `20261101_01`. |
| DataForgedb tables named like `job`, `mission` or `alembic` | `alembic_version` and `cloud_image_jobs` only. No `jobs` and no `missions` table in any schema. |
| G4 output folder, after the operator removed the seed backup | `work/private/` is empty. Every 32-byte base64 value in the `.json`, `.sql`, `.md` and `.txt` files is one of the two public keys. `work/verify_g4.py` holds no key value. |

## Verification boundaries

- **Not exercised:** a customer sign-in, a token exchange, a Rake call with a valid token, and an approve call. These are AC-08 and need a test account.
  - **Update 2026-11-13:** G4 did not apply ForgeCustomer migration `0017` to the deployed ForgeCustomer database. So at G4 a token exchange could not have succeeded there. The HTTP check passed because the key route does not use the database. The gap was found and `0017` applied on 2026-11-13 on the operator's word (`docs/KNOWN_ISSUES.md`, `KI-FOC-20261113-001`, closed).
- **Not read by the agent:** the Render deploy records, the variables of either service, and Rake's shell. Those rows rest on the operator's receipt.
- **Approve-token table location — unresolved.** Rake's `render.yaml` says its `DATABASE_URL` is the DataForge database. DataForgedb has no `jobs` or `missions` table, which Rake owns. So it is not shown that deployed Rake reads DataForgedb. If it does not, the approve table is in a database Rake does not use. Rake tracks this in its `docs/KNOWN_ISSUES.md` (`RAKE-APPROVE-TOKEN-TABLE-OUTSIDE-MIGRATIONS-20261112`).
  - **Update 2026-11-13 — resolved.** A read-only probe through deployed Rake's own connection showed DataForgedb. DataForge migration `20261113_01` then created `jobs` and `missions` there. The rake entry is closed (rake #196).
- **Standby seed custody — unresolved.** The operator removed the local file that held both seeds. No record shows that the standby seed `rake-access-0` is in a secret manager. Until that is shown, no standby rotation may start (operator ruling, 2026-11-12). If the standby seed is lost, a new standby pair is needed before any rotation.
- **Exposure outside the output folder — not assessed.** The output folder is not a git repository. The agent scanned only that folder. Shell history, session transcripts and other copies were not checked. A clean working folder does not prove that no other copy exists. If a private seed was exposed, revocation and replacement need a separate assessment.

## Gate effect

- G4 is recorded as executed out of order. It is not PASS.
- G2 stays open until the deletion test (forgecustomer #57) merges and passes on `main`.
  - **Update 2026-11-13:** G2 PASS (`../G2/RESULT.md`). G4 stays executed out of order, not PASS.
- AC-08 (WP-05) needs G2, G3 and G4 and the open items above.
