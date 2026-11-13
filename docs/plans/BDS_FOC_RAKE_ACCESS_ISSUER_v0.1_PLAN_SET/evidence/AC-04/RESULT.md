# AC-04 — result

**Result: PASS, 2026-11-13.** The rows other than deletion passed on the WP-02 candidate and on `main` after its merge. The record is `../AC-02/RESULT.md`, which covers AC-02 to AC-04 in one run. The deletion row passed on `main` on 2026-11-13. It is recorded below.

## Deletion row

AC-04 expects: "A deletion run revokes the row."

- **Source:** forgecustomer `main` at merge commit `a49b73e7d8ef52e1f1d27bcdfc1158b34a2aab74` (forgecustomer #57, merged 2026-11-13T01:14:42Z). The commit is reachable from `main`.
- **Environment:** GitHub Actions, workflow `CI`, run `43343431931` (event `push`, branch `main`, conclusion `success`). Job `rake access HTTP smoke`, id `131617029828`, 01:14:55Z to 01:16:23Z, conclusion `success`.
- **Persistence path:** the job applies every migration in `supabase/migrations/` to PostgreSQL. It starts the ForgeCustomer API and drives it over HTTP. The deletion runs through `POST /v1/account/deletion-request`, `POST /v1/admin/deletion-requests/:id/advance` and `POST /v1/admin/deletion-requests/:id/execute`. The revoke is in `repositories::privacy::execute_deletion`. The test does not revoke the pseudonym with SQL.
- **Command:** `bash scripts/rake_access_http_smoke.sh`, as the CI job runs it.

| Step | Expected | Observed (job log) |
|---|---|---|
| Advance inside the cooling-off window | Refused | `PASS advance inside the cooling-off window: 409 CONFLICT` |
| Execute with an active subscription | Refused | `PASS execute with an active subscription: 409 CONFLICT` |
| After a refused deletion | Pseudonym still active | `PASS a refused deletion leaves the pseudonym active` |
| Executed deletion | Pseudonym row revoked | `PASS AC-04 deletion: the executed deletion revoked the customer's Rake pseudonym` |
| Exchange after deletion | Refused | `PASS exchange after deletion: 403 FORBIDDEN` |
| After deletion | No new rows | `PASS after deletion: no new pseudonym and no new mint row` |

The job has no `FAIL` line. Its last check line is `rake access HTTP smoke: PASS`.

**Negative control (before merge, local, at `800b286`):** the pseudonym revoke in `execute_deletion` was replaced with a no-op. On a fresh database the smoke failed at the revoke check with `AssertionError: 1|0|<tenant>`. The pseudonym stayed active. The mutant was never committed. The record is the forgecustomer #57 description.

## Limits

- **Tokens already issued stay valid.** The deletion stops new tokens. Rake does not learn of a revocation, and it keeps no revocation list. A token issued before the deletion is accepted until its `exp`, which is 300 s after issue. Rake allows 60 s of clock skew, so the outer limit is 360 s after issue (04, `exp` row). No test covers this window, because the design accepts it (DEC-4, DEC-5).
- **Two steps are set in SQL.** The test moves `cooling_off_until` into the past and sets the subscription to `canceled`. Each guard is checked before it is bypassed: the 409 rows above.
- **ForgeCustomer side only.** No deployed service was called. A deletion against deployed ForgeCustomer is not part of AC-04.
