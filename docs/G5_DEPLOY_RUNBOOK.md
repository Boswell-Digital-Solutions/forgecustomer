# P3 gate G5 — deploy runbook for forgecustomer#63

**Read all of this before you merge forgecustomer#63.**

Render deploys `forgecustomer-api` on every push to `main` (`KI-FOC-20261002-001`). So **merging the
pull request is the deploy.** You must finish Part 1 and Part 2 before you merge. If you merge
first, every operator call fails: it is refused without `ADMIN_JWT_TRUSTED_KEYS`, and it returns
503 without migration 0018. That includes the five emergency stop routes.

Time: about 30 minutes. You need:

- the Supabase dashboard, signed in, with access to project `sffgcltrwewnjzvrzjak` (forgecustomer);
- the Render dashboard, signed in, with access to the service `forgecustomer-api`;
- the operator machine, with Forge_Command open and running;
- a terminal on the operator machine with `curl` and `python3`.

Source: forge-command `docs/plans/BDS_FCO_P3_TOKEN_ISSUER_v0.1/07_G5_AUTHORIZATION_REQUEST.md`
§10 (DG-1 to DG-4), and review 5904396183 (P1-1).

---

## Part 0 — Check before you start (2 minutes)

1. Open forgecustomer#63 on GitHub. At the bottom, every check must be green. If one is red or
   still running, stop and wait.
2. Open the Render dashboard. Click the service **forgecustomer-api**. Near the top it says "Your
   service is live at" a URL. Write that URL down. In this runbook it is `$FC_URL`. (The
   repositories name `https://forge-customer.onrender.com`, but Render can assign another
   subdomain, so use what the dashboard shows.)
3. In a terminal, check the service answers now:

   ```bash
   FC_URL=https://forge-customer.onrender.com   # replace with the URL from step 2
   curl -s -o /dev/null -w "%{http_code}\n" "$FC_URL/v1/ready"
   ```

   Expected: `200`. If not, stop: the service already has a problem, and this runbook does not
   fix it.

---

## Part 1 — Apply migration 0018 to the deployed database (5 minutes)

> **Done 2026-11-14**, on the operator's instruction ("Do that", in session). The agent applied the
> file unchanged (SHA-256 `5fcbd7fe…84ceb7`) through the Supabase migration tool. Supabase recorded
> it as version `20261114130359`, name `operator_token_replay` (like 0017, a timestamp version, not
> `0018`). Verified after: RLS enabled and forced, 0 policies, 0 rows, indexes
> `operator_token_replay_pkey` and `idx_operator_token_replay_expires`, and 0 public tables without
> RLS. **Skip to Part 2.**

This adds one empty table, `operator_token_replay`. The code that runs today never reads it, so
this is safe before the merge.

1. On GitHub, open the PR's file
   `supabase/migrations/0018_operator_token_replay.sql`
   (branch `claude/relaxed-johnson-iyd1jp`). Click **Raw**. Select all the text and copy it.
2. Open the Supabase dashboard. Select the project **forgecustomer** (`sffgcltrwewnjzvrzjak`).
3. In the left menu, click **SQL Editor**, then **New query**.
4. Paste the text. Click **Run**. Expected: "Success. No rows returned."
5. Check the result. Clear the editor, paste this, and click **Run**:

   ```sql
   select relrowsecurity, relforcerowsecurity
   from pg_class where relname = 'operator_token_replay';
   ```

   Expected: one row, `true | true`. If you get no row, the migration did not run. Repeat
   step 4 and read the error message.

Optional: to compare the file you ran with the reviewed file, its SHA-256 is
`5fcbd7fe792576f02d69e0c93fdeefe8dca17d42fb3e8e7ad94a2dee0a84ceb7`.

---

## Part 2 — Set `ADMIN_JWT_TRUSTED_KEYS` on Render (15 minutes)

The code that runs today ignores this variable. Setting it before the merge changes nothing.

### DG-1 — Identify the key that is deployed today

1. On the operator machine, with Forge_Command running, open a terminal and run:

   ```bash
   curl -s http://127.0.0.1:8790/fc/public-key > fc_pub.pem
   head -1 fc_pub.pem
   ```

   Expected: `-----BEGIN PUBLIC KEY-----`. If the file is empty, Forge_Command's local REST
   server is not running. Open Forge_Command and try again.
2. Print the key's `kid` and fingerprint:

   ```bash
   python3 -c "import base64,hashlib,sys;t=open(sys.argv[1]).read().replace(chr(92)+'n',chr(10));der=base64.b64decode(''.join(l for l in t.splitlines() if l.strip() and not l.startswith('-----')));d=hashlib.sha256(der[12:]).digest();print('kid', base64.urlsafe_b64encode(d).decode()[:8]);print('fingerprint SHA256:'+d[:4].hex())" fc_pub.pem
   ```

   Write both lines down. This is the record DG-1 asks for. Never write the key itself into a
   record.
3. Check that Render uses the same key. In Render, open **forgecustomer-api → Environment**. Find
   `ADMIN_JWT_PUBLIC_KEY` and click the eye icon to show its value. Copy the value into a new file
   `render_pub.pem`, then run the same command on it:

   ```bash
   python3 -c "import base64,hashlib,sys;t=open(sys.argv[1]).read().replace(chr(92)+'n',chr(10));der=base64.b64decode(''.join(l for l in t.splitlines() if l.strip() and not l.startswith('-----')));d=hashlib.sha256(der[12:]).digest();print('kid', base64.urlsafe_b64encode(d).decode()[:8]);print('fingerprint SHA256:'+d[:4].hex())" render_pub.pem
   ```

   The `kid` and fingerprint must be the same as in step 2. **If it is different, stop.** Forge_Command and Render
   use different keys, and you must resolve that first.
4. On the same Render page, check that `ADMIN_JWT_ISSUER` is `forge_command_local`. If it is
   something else, stop and ask before you continue.

### DG-2 — Build the new value and check it

1. Build the value:

   ```bash
   python3 -c "import base64,hashlib,json,sys;pem=open(sys.argv[1]).read();der=base64.b64decode(''.join(l for l in pem.splitlines() if not l.startswith('-----')));kid=base64.urlsafe_b64encode(hashlib.sha256(der[12:]).digest()).decode()[:8];print(json.dumps({kid:{'key':pem,'issuer':'forge_command_local'}}))" fc_pub.pem > trusted_keys.json
   cat trusted_keys.json
   ```

   Expected: one line that starts with `{"` followed by the `kid` from DG-1, then
   `{"key": "-----BEGIN PUBLIC KEY-----\n...`, and ends with `"issuer": "forge_command_local"}}`.
2. Check it:

   ```bash
   python3 -c "import json;v=json.load(open('trusted_keys.json'));assert len(v)==1;[(k,e)]=v.items();assert set(e)=={'key','issuer'} and e['issuer']=='forge_command_local';print('OK, kid', k)"
   ```

   Expected: `OK, kid` and the `kid` from DG-1.
3. Do **not** add any other key. In particular, do not add a forge-command key: that is gate G6.

### DG-3 — Write the rollback before you change anything

1. In Render, open **forgecustomer-api → Events**. The top "Deploy live" entry is the version
   running now. Write down its date and commit. That is your rollback target.
2. The rollback, if you need it in Part 4: open **Events**, find that deploy, and click
   **Rollback**. The old version reads `ADMIN_JWT_PUBLIC_KEY` and `ADMIN_JWT_ISSUER`. **Do not
   delete those two variables until Part 4 passes.** The table from Part 1 can stay; the old
   version ignores it.

### Set the variable

1. In Render, open **forgecustomer-api → Environment**.
2. Click **Add Environment Variable**.
3. Key: `ADMIN_JWT_TRUSTED_KEYS`. Value: the whole one line from `trusted_keys.json`. Paste it
   exactly; do not add line breaks.
4. Click **Save**. If Render offers **Save only** next to **Save and deploy**, choose **Save only**.
   If it restarts the service anyway, that is harmless: the running version ignores this
   variable.
5. Check: the variable list now shows `ADMIN_JWT_TRUSTED_KEYS`, and still shows
   `ADMIN_JWT_PUBLIC_KEY` and `ADMIN_JWT_ISSUER`.

---

## Part 3 — Merge (5 minutes, then wait for the deploy)

1. Merge forgecustomer#63. It is a draft, so first click **Ready for review**, then **Merge pull
   request**. (Or tell the agent, in your own words, to merge it.)
2. In Render, open **forgecustomer-api → Events**. A new deploy starts within a minute. Wait until
   it says **Deploy live**. This takes a few minutes. If it says **Deploy failed**, Render keeps
   the old version running; nothing is broken. Read the deploy log and stop.
3. Check readiness:

   ```bash
   curl -s -o /dev/null -w "%{http_code}\n" "$FC_URL/v1/ready"
   ```

   Expected: `200`.
4. In Render, open **Logs**. Search for `operator trusted-key set loaded`. It must be there, with
   the `kid` from DG-1. If you see `ADMIN_JWT_TRUSTED_KEYS refused` instead, roll back now
   (DG-3), and check the value you pasted.

---

## Part 4 — DG-4: check it works (5 minutes)

Do both checks in Forge_Command.

1. **An admin read.** Open the **Commerce** screen. The customer list must load. An error, or an
   empty list where you expect customers, is a failure.
2. **A posture read.** Open the **Business** workspace. In **Customer details**, click **Load
   customers**. The list must load. "Customer details unavailable" is a failure.
3. **The replay table.** In Supabase **SQL Editor**, run:

   ```sql
   select issuer, count(*) from public.operator_token_replay group by issuer;
   ```

   Expected: one row, `forge_command_local`, with a count of at least 2 (one per call above).
   This proves the new single-use check is live.

**If check 1 or 2 fails:** roll back at once (DG-3: Render **Events → Rollback**). The emergency
stop routes depend on the same checks. Then report what you saw.

**If all three pass:** G5 is deployed. You can now delete `ADMIN_JWT_PUBLIC_KEY` and
`ADMIN_JWT_ISSUER` from Render (**Environment**, the trash icon on each, then **Save**). You may
wait a day first and keep them as a rollback. Then delete the local files `fc_pub.pem`,
`render_pub.pem` and `trusted_keys.json`. They hold only public material, but you no longer need
them.

---

## What this does not do

- It is not G5 acceptance. That is a separate owner decision on the evidence (forge-command
  `docs/plans/BDS_FCO_P3_TOKEN_ISSUER_v0.1/evidence/G5/ACCEPTANCE_MATRIX.md`).
- It does not make forge-command an issuer (G6). Forge_Command stays the only issuer.
- After this deploy, every Forge_Command token is accepted once. Forge_Command already mints one
  token per call (measured, P-1), so nothing in its behaviour changes.
