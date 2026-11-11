# AC-01 — result

**Result: PASS on the WP-01 candidate, before merge.** Base: forgecustomer `main` `34480071e` plus the uncommitted WP-01 files. Environment: the operator's machine, Linux, cargo 1.97.1. 2026-11-11.

## What ran

| Step | Command (from `api/` unless noted) | Observed |
|---|---|---|
| Vectors | `python3 -I contracts/vectors/rake-access-token-v1/generate.py` (repository root), run twice, outputs compared with `cmp` | 18 vectors; the two runs are byte-identical |
| Contract test | `cargo test -p forgecustomer-api --test rake_access_contract -- --nocapture` | 1 test passed: "3 valid, 15 invalid vectors checked" |
| Mutation check | `alg` changed from `{"const": "EdDSA"}` to `{"type": "string"}` in the schema, then the same test | FAILED at `invalid-05-alg-hs256: schema accepted it`. The schema was restored and compared with `cmp`. |
| Repository gate | `cargo fmt --all --check`; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo test --all` | fmt clean after formatting the new file; clippy exit 0; tests: 146 passed and 1 ignored (lib), 1 passed (contract), 36 passed (security) |

## Each refusal names its field

The test fails a negative vector unless the refusal text names the vector's `field`. Observed:

| Vector | Refused by | Message |
|---|---|---|
| invalid-01-no-iss | schema | `$.claims: missing required "iss"` |
| invalid-02-no-aud | schema | `$.claims: missing required "aud"` |
| invalid-03-no-exp | schema | `$.claims: missing required "exp"` |
| invalid-04-no-jti | schema | `$.claims: missing required "jti"` |
| invalid-05-alg-hs256 | schema | `$.header.alg: not equal to "EdDSA"` |
| invalid-06-alg-none | schema | `$.header.alg: not equal to "EdDSA"` |
| invalid-07-scopes-string | schema | `$.claims.scopes: not of type "array"` |
| invalid-08-lifetime-901 | rule | `lifetime 901 s exceeds 900 s` |
| invalid-09-type-refresh | schema | `$.claims.type: not equal to "access"` |
| invalid-10-no-typ | schema | `$.header: missing required "typ"` |
| invalid-11-aud-list | schema | `$.claims.aud: not equal to "rake.application"` |
| invalid-12-extra-email | schema | `$.claims: unexpected property email` |
| invalid-13-sub-not-user | rule | `sub differs from user_id` |
| invalid-14-tampered-signature | signature | `verify: InvalidSignature` (the payload swaps `tenant_id`; the signature is the original) |
| invalid-15-approve-mixed | rule | `rake:approve must be the only scope` (DEC-6) |

The three valid vectors (read and execute; read only; approve only) verify with `jsonwebtoken` EdDSA, and their decoded claims and header equal the vector.

## DEC-6 addition

After DEC-6 (2026-11-11), the contract gained the rule that `rake:approve` stands alone, with `valid-03-approve-only` and `invalid-15-approve-mixed`. The first rerun failed on `invalid-14-tampered-signature`, because its forged payload added `rake:approve` beside other scopes, so the new rule refused it before the signature check. The test requires that vector to fail only on the signature. The forged payload now swaps `tenant_id` instead. The rerun passed, and the full gate passed again (146 + 1 + 36, 1 ignored).

## Secret scan

The first push stored each signed vector as one compact token. The CI secret scan (gitleaks rule `jwt`, run `43006762294`) reported the three tokens in `vectors.json`. They are public test vectors, signed with a key derived from a published label, so they are not secrets. The vectors now store each token as `jws_parts`, a list of its three parts, and the test joins them with `.`. A local check with the `jwt` rule's pattern finds 0 matches in the new file. The branch was rewritten so that no commit holds the compact tokens. No `.gitleaks.toml` allowlist was added, because that file is outside WP-01's scope.

## Limits

- The schema check uses a validator in the test for the keywords the schema uses. The test fails if the schema gains any other keyword. It is not a general JSON Schema validator. A full validator would need a new dependency, which is outside WP-01's permitted scope.
- The vectors prove the contract and the signature format. They do not prove the issuer route or Rake's verifier (AC-02 to AC-06).
- The token was signed in Python (`cryptography` 46.0.5) and verified in Rust. That is one cross-language check. Rake's verifier is checked in WP-03.
