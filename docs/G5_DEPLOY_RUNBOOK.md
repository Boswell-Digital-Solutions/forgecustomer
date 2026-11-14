# P3 gate G5 — deploy runbook for forgecustomer#63

**Read this before you merge forgecustomer#63.** Render deploys `forgecustomer-api` on every push
to `main` (`KI-FOC-20261002-001`), so **merging is the deploy**. Do steps 1 and 2 first. If the
merge lands before them, every operator call fails: it is refused without
`ADMIN_JWT_TRUSTED_KEYS`, and it returns 503 without migration 0018. That includes the five T-13
stop routes.

Source: forge-command `docs/plans/BDS_FCO_P3_TOKEN_ISSUER_v0.1/07_G5_AUTHORIZATION_REQUEST.md`
§10 (DG-1 to DG-4), and review 5904396183 (P1-1).

## Before you start

- CI is green on the PR head.
- Confirm the service URL in the Render dashboard ("your service is live at"). The repositories
  name `https://forge-customer.onrender.com`. Render can assign another subdomain.
- Keep `ADMIN_JWT_PUBLIC_KEY` and `ADMIN_JWT_ISSUER` set on Render until step 4 passes. They are
  the rollback.

## Step 1 — Apply migration 0018 to the deployed database

- Database: Supabase project `sffgcltrwewnjzvrzjak` (forgecustomer).
- File: `supabase/migrations/0018_operator_token_replay.sql` from the PR branch.
  SHA-256 `5fcbd7fe792576f02d69e0c93fdeefe8dca17d42fb3e8e7ad94a2dee0a84ceb7`.
- Apply it with the Supabase SQL editor or the Supabase migration tool. It only adds one table,
  and the current code does not use it, so it is safe before the merge.
- Check:

  ```sql
  select relrowsecurity, relforcerowsecurity
  from pg_class where relname = 'operator_token_replay';
  -- expect: t | t
  ```

## Step 2 — Set `ADMIN_JWT_TRUSTED_KEYS` on Render

The current code ignores this variable. Setting it before the merge changes nothing.

### DG-1 — Identify the deployed key

On the machine that runs Forge_Command, get the predecessor's public key:

```bash
curl -s http://127.0.0.1:8790/fc/public-key > fc_pub.pem
```

It must be the same key as `ADMIN_JWT_PUBLIC_KEY` on Render. Record its `kid` and fingerprint.
Never record the key itself.

```bash
python3 -c "import base64,hashlib,sys;der=base64.b64decode(''.join(l for l in open(sys.argv[1]) if not l.startswith('-----')));d=hashlib.sha256(der[12:]).digest();print('kid', base64.urlsafe_b64encode(d).decode()[:8]);print('fingerprint SHA256:'+d[:4].hex())" fc_pub.pem
```

### DG-2 — Build the value and check it

```bash
python3 -c "import base64,hashlib,json,sys;pem=open(sys.argv[1]).read();der=base64.b64decode(''.join(l for l in pem.splitlines() if not l.startswith('-----')));kid=base64.urlsafe_b64encode(hashlib.sha256(der[12:]).digest()).decode()[:8];print(json.dumps({kid:{'key':pem,'issuer':'forge_command_local'}}))" fc_pub.pem
```

- The output is one entry: the predecessor's key, issuer exactly `forge_command_local`.
- Its `kid` must match DG-1.
- Do **not** add the rebuild's key. That is gate G6.
- An entry whose issuer is `forge_command_local` is not route-bound (`KI-FOC-20261114-004`).
  Check each issuer.

### DG-3 — Write the rollback before you change anything

- Record the current deploy ID (Render dashboard, Events).
- Rollback: the Render dashboard **Rollback** to that deploy. The old code reads
  `ADMIN_JWT_PUBLIC_KEY` and `ADMIN_JWT_ISSUER`, which stay set. To keep `main` equal to what
  runs, also revert the merge commit.
- Migration 0018 can stay. The old code ignores it.

Then add `ADMIN_JWT_TRUSTED_KEYS` on Render with the DG-2 output.

## Step 3 — Merge

Merge forgecustomer#63. This needs the operator's own, specific word (review 5904396183, P1-2).
Wait until Render shows the deploy live and `GET /v1/ready` returns 200.

## Step 4 — DG-4: check it works

- From Forge_Command, run one ForgeCustomer admin read, such as the customer list. It must succeed.
- In Forge_Command, open the customer posture view. It must succeed.
- In Supabase: `select count(*) from operator_token_replay;` shows one row per operator call since
  the deploy.
- If either call fails, roll back at once (DG-3).
- After both pass, remove `ADMIN_JWT_PUBLIC_KEY` and `ADMIN_JWT_ISSUER` from Render.

## What this does not do

It is not G5 acceptance. That is a separate owner decision on the evidence (forge-command
`evidence/G5/ACCEPTANCE_MATRIX.md`). It does not make forge-command an issuer (G6). The predecessor
stays the only issuer.
