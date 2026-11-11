# Operator ruling DEC-6 — 2026-11-11

**Who:** Charles (Charlie) Boswell, BDS operator.
**How:** a reply in the agent session of 2026-11-11 (America/New_York).
**Receipt:** this file holds the reply verbatim. No signed operator-decision record exists for it.

## Verbatim text

> yes

## What the reply answered

The agent's message before the reply did two things:
- It said that forgecustomer #53 (WP-01) "needs your word to merge".
- It asked DEC-6, "should AuthorForge get Rake's approve permission?", with A recommended: "yes, with a single-use token".

The agent read the reply as both answers: the word to merge #53, and DEC-6 = A. It told the operator that reading in the same session. The reply does not authorize WP-02 or WP-03. The agent asked about them separately.

## Reading

| ID | Ruling |
|---|---|
| DEC-6 | A. The `authorforge` application may receive `rake:approve`, only in a token whose `scopes` is exactly `["rake:approve"]`. Such a token is single-use: Rake records its `jti` on the first approve call and refuses any later use until `exp`. |

## What this ruling does not authorize

No route, migration, key, configuration value or deployment. WP-02 and WP-03 still need the operator's word after G1.
