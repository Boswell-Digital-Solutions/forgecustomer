# 06 — Decision semantics

This component states what accept, decline and revise mean for this plan.

| Answer | Effect |
|---|---|
| **Accept** a decision in 09 | The recommended value binds. 04 and 05 stay as written for that item. The answer authorizes only what the decision row names. |
| **Choose an alternative** | The plan is revised: 04, 05, 07 and 08 change for that item in one change, before any work package uses it. |
| **Decline** a decision | The affected work packages stay proposed. The plan records the reason. |
| **Defer** | The plan stays proposed for the affected items. Independent authorized work continues. |

A gate passes only with its evidence in 07. A ruling does not pass a gate.

The plan lifecycle changes only by an operator ruling: `proposed` → `authorized` (DEC-1) → `in_progress` (first WP starts) → `implemented_unverified` (WP-02 to WP-04 merged) → `complete` (AC-08 PASS and records closed). The registry maps `authorized` to `approved`.

Work-package status uses: `proposed`, `authorized`, `in_progress`, `delivered`, `blocked`. Acceptance results use: `NOT_RUN`, `PASS`, `FAIL`, `BLOCKED`, `UNVERIFIED`.
