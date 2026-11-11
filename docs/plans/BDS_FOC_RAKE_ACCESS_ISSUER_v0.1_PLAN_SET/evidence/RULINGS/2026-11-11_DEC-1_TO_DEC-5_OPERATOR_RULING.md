# Operator ruling DEC-1 to DEC-5 — 2026-11-11

**Who:** Charles (Charlie) Boswell, BDS operator.
**How:** a reply in the agent session of 2026-11-11 (America/New_York).
**Receipt:** this file holds the reply verbatim. No signed operator-decision record exists for it.

## Verbatim text

> do it

## What the reply answered

The agent's message before the reply listed DEC-1 to DEC-5 of `09_OPERATOR_REVIEW_AND_HANDOFF.md`, each with a recommendation. It ended: "I have not uploaded a Drive copy of the plan."

The agent read the reply as acceptance of each recommendation. The agent told the operator that reading in the same session. The operator can correct it.

## Reading

| ID | Ruling |
|---|---|
| DEC-1 | Yes. WP-00 and WP-01 are authorized. WP-02 onward stay proposed until G1 and the operator's word. |
| DEC-2 | A. The desktop app authenticates with an installation device key, registered with ForgeCustomer. |
| DEC-3 | A. The token carries a random pseudonym per customer and audience, stored in ForgeCustomer and never sent to DataForge. |
| DEC-4 | EdDSA (Ed25519), a `scopes` JSON list, a 300-second lifetime, no refresh token. |
| DEC-5 | Log `jti` on both sides with no replay store. No `rake:approve` for applications in v1. WP-00 checks whether the Author-Forge mission flow needs approve. If it does, the plan returns to the operator. |

## What this ruling does not authorize

No route, migration, key, configuration value, deployment or merge of code. A merge of WP-01, which adds a test, needs the operator's word.
