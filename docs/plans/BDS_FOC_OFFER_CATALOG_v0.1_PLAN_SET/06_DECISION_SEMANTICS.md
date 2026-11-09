# 06 — Decision semantics

This component states how each operator decision and each system decision is made and recorded.

## Operator decisions (DEC-0 to DEC-9)

- Each decision in 09 takes one of: **accept** the recommendation, **choose** a named alternative, **revise** (the operator states the change), or **defer** (the decision stays open and the work packages it blocks stay blocked).
- A ruling is recorded in 09 with its date and the operator's words, and in `plan_set.yaml` `decisions`, in the same change.
- A reviewer recommendation, a pasted review or an agent summary is not a ruling.
- A ruling on a decision authorizes no work package. Authorization is a separate operator act, recorded in 09.

## System decisions (for the implementation work packages)

| Decision | Inputs | Outcomes | Fail-closed default |
|---|---|---|---|
| Issue a grant | a verified, deduplicated webhook fact; the offer's `requires` list | issue once; or refuse with a reason code | refuse |
| Reserve a founder seat | the program row; free seats | reserve the lowest free number; or "program full" | no founder session |
| Confirm a founder seat | a verified payment event for the reserving session | confirm once | the reservation expires |
| Show an add-on as active (desktop) | the signed snapshot's key | active; or absent with a reason | absent |
| Allow a build to install (desktop) | `major_version` or `updates_until` from the signed snapshot; the build's version and date | allow; or refuse with the reason | refuse the install; never stop the installed build |
| Open local work | none; the recovery certificate when the license is revoked | always opens | opens (REQ-09) |

A refusal never deletes, encrypts or hides manuscript data.
