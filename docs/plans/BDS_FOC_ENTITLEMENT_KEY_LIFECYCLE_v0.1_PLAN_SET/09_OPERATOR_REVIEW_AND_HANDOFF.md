# 09 — Operator review packet and handoff

## Existing authority

On 2026-11-05 the operator directed: "well build the plan ensuring current business code that works is considered instead of reinvented. - we build the plan in Drive /Forge using the protocols." That authorizes creating this plan set in BDS-PSS-001 form. The Drive connector was unavailable, and the operator then directed: "just hold it in repo - docs/plans then". So this copy in forgecustomer `docs/plans/` is the held copy. It was uploaded to the Drive destination on 2026-11-05, once the connector worked again. On 2026-11-06 the operator ruled DEC-1 to DEC-5 ("Proceed - in order authorized"). Lifecycle: `authorized`.

## Status

Planning v0.1 drafted. Rulings DEC-1 to DEC-5 approved 2026-11-06. Execution authorization: WP-00 and WP-01. No implementation or verification yet.

---

## Decision 1 — Authorize WP-00 and WP-01?

**Recommendation: yes.** WP-01 is the smallest real fix. It wires ForgeCustomer's existing multi-key ring to one new configuration value, and it closes the overlap half of KI-FOC-20261105-001. WP-00 includes one unauthenticated read of the public `GET /v1/entitlements/keys`, to learn the live `key_id`.

**Alternative:** WP-00 without the public read. Then the live `key_id` stays unknown until you supply it.

**Note:** merging the WP-01 PR redeploys ForgeCustomer on Render (KI-FOC-20261002-001). Setting the new variable is your act.

**Ruling (2026-11-06): approved.** WP-00 includes the one unauthenticated read of the public key list. WP-01 ends in a PR; its merge needs your word.

## Decision 2 — Who creates a new signing key?

**Recommendation:** you, outside any agent, set as a deployment secret. That is DataForge's rule, and it is ForgeCustomer's practice today.

**Alternative:** a ForgeCustomer admin route that generates the key. This adds private-key handling to the API, and the plan does not recommend it.

**Ruling (2026-11-06): approved, with one addition.** You create two keys, an active key and a standby key, outside any agent. Both public halves are published and compiled into every Author-Forge trust set; only the active key signs. A compromise then becomes a signer switch, not a rebuild (TUF and APK Signature Scheme v3 pre-trust the next key; the 2026-06-11 plan's staged rotation). WP-01's verify-key variable carries the standby public key.

## Decision 3 — Extend the public key list with status fields?

**Recommendation: yes, additively.** Add `status`, `valid_from` and `retired_at` (04 §B). Author-Forge's parser does not deny unknown fields (inspected at `9f3a5d1a5`), so built desktops are unaffected.

**Ruling (2026-11-06): approved.**

## Decision 4 — Keep the v1 wire format?

**Recommendation: keep `forge.entitlements.v1` with the struct-order byte rule now (R06).** It works and is verified across repositories. RFC 8785 (`serde_jcs`, already proven in forge-command) can be a `v2` later, behind its own plan.

**Alternative:** move to RFC 8785 now. That changes the bytes both sides sign and verify, and it needs a coordinated release.

**Ruling (2026-11-06): approved.** v1 keeps the struct-order rule. RFC 8785 becomes the v2 rule, triggered when a non-Rust verifier is first needed. RFC 8785 is an informational Independent Submission, so its use rests on the ecosystem precedent (forge-command's SMITH vectors), not on standard status.

## Decision 5 — Interim confirmation source for WP-04

**Question:** until forge-command's confirmation exists, what should the release step trust?

**Recommendation:** the release step accepts a key only if its fingerprint is listed in a committed file `src-tauri/entitlement_key_confirmations.json`, edited only by you. The forge-command receipts replace it when WP-03 lands.

**Alternative:** trust every `active` key ForgeCustomer publishes. This is simpler, but a compromised deployment could add a key.

**Ruling (2026-11-06): approved.** The committed, operator-edited file is the interim. Retire an old key no earlier than the longest-lived signed document plus clock skew: the 14-day lease plus 5 minutes.

---

## Dependencies

- forge-command P3 (ForgeCustomer admin credential).
- `BDS-FCO-FLEET-MANAGER-v0.1` (forge-command #49) for WP-03.
- Fleet pipeline (KI-AUF-20261101-006) for delivered-fleet evidence.

## Handoff

**Next action:** WP-00 (authorized), then WP-01 (authorized; ends in a PR).

Keep R01 (reuse first), R02 (no key outside ForgeCustomer), every AC NOT_RUN, and AUTHORITY_GAP-01 to -06.
