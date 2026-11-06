# 07 — QA, verification and acceptance-to-evidence map. ALL CHECKS NOT_RUN.

**Common evidence rule.** Every record states:
- the repository SHA;
- the environment;
- the command;
- the expected and observed results;
- the retained artifacts with sha256;
- the limits.

Tests use local ForgeCustomer and test keys generated in the suite, never the deployed service or a real key. Skips do not count. A negative check must reach its named refusal.

**Evidence destination:** this plan set's `evidence/<AC-ID>/`, with the repository records referenced.

**AC-00 Structure.** 15 plan files. The manifest equals the folder, evidence files included. The lifecycle matches in four places. The IDs agree.

**AC-01 Overlap wired.**
- With `ENTITLEMENT_VERIFY_PUBLIC_KEYS` holding a second test key, `GET /v1/entitlements/keys` lists both.
- A snapshot signed by the second key verifies through `VerifyingKeyRing::verify_snapshot`.
- With the variable unset, behavior equals today's (one key).
- A malformed value fails start-up with a named error.
- A duplicate `key_id` carrying a different key fails start-up.
- `rotation_overlap_both_keys_verify` still passes.
- `cargo fmt`, `clippy -D warnings` and `cargo test --all` pass (the ForgeCustomer `doc/system/13` gates).

**AC-02 Cross-repo verification.** An Author-Forge build whose trust set holds both test keys verifies a snapshot from each, using the existing byte-compatibility method (fixtures from ForgeCustomer's own signer).

**AC-03 Registry.**
- The migration applies, re-applies deterministically and passes the RLS write-denial matrix (ForgeCustomer CI `migrations` job).
- Status is derived from events.
- Each admin route needs `admin`, a reason, and writes audit.
- A revoked key cannot change.

**AC-04 Signing refuses a retired or revoked active key.**
- With the active key marked retired or revoked, `GET /v1/entitlements/current` and lease issuance give 503 and store nothing.
- The ported lifecycle-ordering test shows no snapshot is stored after a concurrent revocation commits. This mirrors DataForge's concurrency proof.

**AC-05 Operator confirmation.**
- forge-command discovery lists each published key with a fingerprint that equals the predecessor's `fingerprint` function output on the same key, by test vector.
- A confirmation needs presence and writes a receipt.
- An unconfirmed key is excluded by AC-06.

**AC-06 Release-time trust set.**
- The workflow step writes only confirmed `active` and `pending` keys.
- A 1.x version with none fails the build with a named message.
- A 0.x build with none passes.
- The release record carries the key ids.

**AC-07 Rotation rehearsal.** On a local ForgeCustomer and two Author-Forge builds (old: key A; new: keys A and B), across steps 1–6 of 05 §E:
- the old build verifies until the switch;
- the new build verifies before and after the switch;
- after the switch, the old build refuses with `unknown_key_id`, the expected result for an un-updated install;
- no step needs a user action.

Delivered-fleet evidence is blocked on AUTHORITY_GAP-04.

**AC-08 Reuse audit.** A review of each work package's diff against the 03 matrix. No listed capability is reimplemented, and each new-code item maps to the "no existing code" list.
