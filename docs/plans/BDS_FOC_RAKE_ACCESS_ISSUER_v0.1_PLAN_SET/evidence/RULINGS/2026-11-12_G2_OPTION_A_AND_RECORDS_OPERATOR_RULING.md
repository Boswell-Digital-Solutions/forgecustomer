# Operator ruling G2 = A and the G4 records — 2026-11-12

**Who:** Charles (Charlie) Boswell, BDS operator.
**How:** a reply in the agent session of 2026-11-12 (America/New_York).
**Receipt:** this file holds the reply verbatim. No signed operator-decision record exists for it.

## Verbatim text

> **Decision: G2 = Option A. Records PR = Approved to open, not approved to merge.**
>
> ### **1. G2 — Write the deletion test**
>
> Do not waive G2 simply because the other evidence looks satisfactory.
>
> - Implement the missing deletion test.
> - Run it against the actual relevant persistence path.
> - Capture the expected versus observed behavior and test results.
> - Preserve existing tests and acceptance criteria.
> - Keep G2 **open** until the evidence establishes that its requirements are met.
>
> A missing verification row should not become an administrative exception when it can be tested.
>
> ### **2. G4 — Accept the cleanup evidence, with one qualification**
>
> The reported plaintext cleanup is satisfactory for the inspected working directory.
>
> However, the standby seed `rake-access-0` remains unverified in the secret manager.
>
> **Do not initiate standby key rotation until recoverability is established.** If the private key was previously committed or otherwise exposed, separately assess whether revocation and replacement are necessary. A clean working directory does not prove Git history or external artifacts are clean.
>
> ### **3. Records PR — Proceed**
>
> Authorize opening the docs-only PR with these requirements:
>
> 1. Record the G4 evidence and its verification boundaries.
> 2. Explicitly document that G4 ran before G2. Do not rewrite the chronology.
> 3. File the Rake known issue for the table created outside its migration system.
> 4. Record the standby seed verification as unresolved.
> 5. Keep implementation changes outside this PR.
> 6. Require green checks and review before merging.
>
> **No automatic merge authorization is granted.**
>
> ### **4. Current gate disposition**
>
> | Item                       | Status             |
> | -------------------------- | ------------------ |
> | Plaintext backup cleanup   | Reported complete  |
> | G4 working-folder key scan | Reported clean     |
> | Standby seed custody       | Unverified         |
> | G2 deletion test           | Required, pending  |
> | Rake migration KI          | To be filed        |
> | Records PR                 | Authorized to open |
> | Records PR merge           | Not authorized     |
>
> **Governing principle:** Evidence can establish what occurred, but cannot retroactively establish that an earlier prerequisite was satisfied.
>
> Proceed with Option A and open the records PR. Preserve the remaining gates without shortcuts.

## Reading

| Item | Ruling |
|---|---|
| G2 | A. A forgecustomer test runs a customer deletion and checks the Rake pseudonym. G2 is not waived. G2 stays open until that evidence passes. |
| G4 records | A docs-only records PR may be opened. It may not merge without the operator's word and a review. |
| Standby key | No standby rotation until the standby seed is known to be recoverable. |

## What this ruling does not authorize

A merge of the records PR or of the G2 test PR. A key rotation. A change to a deployed service, a secret or a variable.
