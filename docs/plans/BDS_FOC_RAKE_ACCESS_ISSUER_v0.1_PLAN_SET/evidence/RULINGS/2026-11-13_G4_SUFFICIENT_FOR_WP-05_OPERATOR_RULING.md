# Operator ruling: the executed G4 meets the WP-05 prerequisite — 2026-11-13

**Who:** Charles (Charlie) Boswell, BDS operator.
**How:** a choice in the agent session of 2026-11-13 (America/New_York). The operator answered "fix", the agent asked what it meant, and the operator chose one of the offered answers.
**Receipt:** this file holds the chosen answer verbatim. No signed operator-decision record exists for it.

## Verbatim text

> **Adopt my G4 recommendation**
>
> Record your ruling as recommended: the executed G4, plus the two 2026-11-13 database fixes, meets WP-05's G4 prerequisite. G4 stays not PASS. AC-08 starts with a fresh read-only re-check. Then merge #61, then forge #431.

The recommendation it adopts, as the agent gave it in the same session:

> Rule that the executed G4, plus the two database fixes of 2026-11-13, meets WP-05's G4 prerequisite. G4 stays recorded as not PASS, with no PASS added after the fact. As a guard, the AC-08 run would start with a fresh read-only re-check of the rows above. If any row fails, the run stops.

The two database fixes are DataForge migration `20261113_01` (Rake's tables in DataForgedb) and ForgeCustomer migration `0017` applied to the deployed ForgeCustomer database (`docs/KNOWN_ISSUES.md`, `KI-FOC-20261113-001`).

## Reading

| Item | Ruling |
|---|---|
| WP-05's G4 prerequisite | Met by the executed G4 plus the two 2026-11-13 database fixes. |
| G4 disposition | Unchanged: `EXECUTED_2026-11-12_BEFORE_G2_not_PASS`. |
| AC-08 | Starts with step 0, a read-only re-check of the G4 rows. A failed row stops the run. |
| Records | forgecustomer #61 may merge, then forge #431. |

## What this ruling does not authorize

WP-05 itself, the AC-08 run, the test accounts, a standby key rotation, or any change to a deployed service, secret or variable.
