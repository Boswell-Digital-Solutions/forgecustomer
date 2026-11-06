# 01 — Binding rulings and explicit non-goals

This component lists the rulings that bind the plan once it is authorized, and the work it does not do.

## Rulings

**R01 Reuse first.** A work package may write new code only for a capability that the 03 reuse matrix marks "no existing code". Every other capability is built by wiring, porting or calling the code named in the matrix. A deviation needs a recorded reason and the operator's approval (operator direction 2026-11-06).

**R02 ForgeCustomer holds every entitlement key.** Private keys live only in ForgeCustomer's deployment secrets. No agent creates, reads or sets them. This copies DataForge's rule (§19 "Receipt signing key"). forge-command and Author-Forge hold public keys only.

**R03 One variable, one shape.** Retained and pending public keys reach ForgeCustomer through one configuration value, `ENTITLEMENT_VERIFY_PUBLIC_KEYS`, a JSON object `{key_id: base64_public_key}`. It mirrors the shape DataForge Local already uses (`XA_RUN_TOKEN_PUBLIC_KEYS`), without the issuer field ForgeCustomer does not need. It is the only new configuration value.

**R04 Publish before sign.** A new signing key is published as `pending` before ForgeCustomer signs with it. A desktop can then receive it in a release before the switch. DataForge publishes a key at its first use. That is the one deliberate difference from DataForge's model, because desktops compile keys in.

**R05 Retired keys keep verifying.** A retired key stays published until its last snapshot (24 h) and its last offline lease (14 days) have expired. A revoked key stops at once.

**R06 Keep the wire format that works.** `forge.entitlements.v1` and `forge.lease.v1`, with their struct-order byte rule, stay unchanged in this plan. Author-Forge already verifies them, with a byte-compatibility fixture. RFC 8785 bytes are a separate, later version (09 Decision 4).

**R07 The operator confirms each published key.** forge-command discovers the keys ForgeCustomer publishes, shows each fingerprint, and records the operator's confirmation. This ports the predecessor's `receipt_keys_*` flow. A key that has not been confirmed is not compiled into a release.

**R08 No configuration change by an agent.** Setting `ENTITLEMENT_SIGNING_PRIVATE_KEY`, `ENTITLEMENT_SIGNING_KEY_ID` or `ENTITLEMENT_VERIFY_PUBLIC_KEYS` on the deployed service is the operator's act (workspace `CLAUDE.md`).

## Explicit non-goals

- A new key store, KMS, HSM or certificate authority.
- A root-signed key set fetched by desktops at run time. That is a later option (forge-command plan 05 §E).
- Changes to Stripe, licensing or entitlement evaluation.
- The rest of the fleet manager: fleet reads, containment and release authorization belong to `BDS-FCO-FLEET-MANAGER-v0.1`.
- Removing Author-Forge's own HS256 commercial authority (ForgeCustomer CP1-WP06).
