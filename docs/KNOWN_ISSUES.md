# Known Issues

Findings tracked here per the Forge workspace-wide protocol (`/home/charlie/Forge/CLAUDE.md`):
a bug, a root cause, a gap, or an open tracking state goes here, checked first before a finding
is reported as new, and written in the same session it is found.

---

## KI-FOC-20260924-001 — `cargo audit` fails on `main`: RUSTSEC-2026-0285 in `rustls` 0.23.40

**Date found:** 2026-09-24
**Status:** closed (fixed in the same change)

**What is wrong:** The CI `dependency audit` job fails on the `main` lockfile. `Cargo.lock`
locks `rustls` 0.23.40. RUSTSEC-2026-0285 applies to that version: "TLS 1.3 handshake messages
incorrectly accepted across encryption level boundaries" (dated 2026-09-14, severity 5.3,
medium). The advisory's solution is `rustls` 0.23.45 or later. forgecustomer#27 found the
failure. That PR does not change `Cargo.lock`.

**Root cause:** The advisory entered the RustSec database after the last CI run on `main`
(2026-09-14 07:29 UTC). That run passed with the same lockfile. No push to `main` has run CI
since then, so no check failed until the next pull request. KI-FOC-20260924-002 records this
gap.

**Fix:** `cargo update -p rustls` moves `rustls` to 0.23.45 and `rustls-webpki` to 0.103.15.
Both are semver-compatible patch releases. No other package changes. `cargo audit` passes, and
its four allowed warnings do not change.

**Scope:** closed. The change is `Cargo.lock` only. `.cargo/audit.toml` gets no new waiver.

---

## KI-FOC-20260924-002 — `cargo audit` runs only on push and pull request

**Date found:** 2026-09-24
**Status:** open

**What is wrong:** `.github/workflows/ci.yml` runs the `audit` job only on `push` and
`pull_request`. A new advisory against a locked dependency fails no check until the next push.
RUSTSEC-2026-0285 (KI-FOC-20260924-001) applied to `main` for ten days before a pull request
found it. The repository has no Dependabot configuration.

**Root cause:** The workflow has no `schedule` trigger. `cargo audit` reads the advisory
database at run time, so its result can change when the database changes, with no change to
the code.

**Fix:** None yet. A `schedule` trigger on the audit job, for example daily, finds a new
advisory against `main` within one day.

**Scope:** open. The fix changes CI only. It needs an operator decision on the schedule.
