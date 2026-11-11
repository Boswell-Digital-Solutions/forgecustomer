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
