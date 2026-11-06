# 06 — Decision semantics

## Key statuses

| Status | Published | Signs | Verifies on ForgeCustomer |
|---|---|---|---|
| `pending` | yes | no | yes |
| `active` | yes | yes | yes |
| `retired` | yes, until removed from configuration | no | yes |
| `revoked` | no | no | no |

Allowed transitions: `pending → active` (only when the deployed signing key matches); `active → retired` (at the switch); `pending | active | retired → revoked` (terminal).

## Gate dispositions

PASS | FAIL | BLOCKED | NOT_RUN. A skip is NOT_RUN.

## Gates

| Gate | Needs | Covers |
|---|---|---|
| G0 | AC-00, Decision 1 ruled | Plan admitted; source lock |
| G1 | AC-01, AC-02 | Overlap wired (ForgeCustomer) |
| G2 | G1, AC-03, AC-04 | Registry and statuses |
| G3 | G2, AC-05; AUTHORITY_GAP-01 closed for the register step | Operator confirmation |
| G4 | G1, AC-06 | Release-time trust set |
| G5 | G2, G3, G4, AC-07 | First rotation rehearsal |

Order: G0 → G1 → {G2, G4} → G3 → G5. G1 alone closes KI-FOC-20261105-001's overlap half.
