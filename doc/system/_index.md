# ForgeCustomer — Compiled System Reference

**Designation:** FOC
**Document role:** Canonical compiled technical reference for the ForgeCustomer customer/commercial authority
**Source:** `doc/system/`
**Build command:** `bash doc/system/BUILD.sh`
**Document version:** 2.0 (2026-06-19) — BDS canonical-compliance migration (7-group class-aware structure, truth classes, designation-bound fail-closed assembly, governance trio)
**Protocol:** BDS Documentation Protocol v2.0; BDS Repo Documentation System Canonical Compliance Standard

> **Generated artifact warning:** `doc/FOCSYSTEM.md` is assembled output. Edit the
> source modules under `doc/system/` and rebuild. Hand edits to the compiled
> artifact are overwritten by the next build.

Assembly contract:

- Command: `bash doc/system/BUILD.sh`
- Validation: `bash doc/system/validate_snapshots.sh` runs during assembly
- Primary output: `doc/FOCSYSTEM.md`

This `doc/system/` tree is the canonical source of truth for ForgeCustomer. It uses
explicit **truth classes**: *canonical facts* define the customer/commercial
authority, licensing/entitlement model, mutation discipline, and ecosystem
boundaries; *snapshot facts* are dated, audit-derived counts (routes, tables,
migrations, tests). See §7 for the scope/authority boundary and §8 for ownership
and designation doctrine.

| Part | File | Contents |
| --- | --- | --- |
| §1 | `00_overview/01-overview.md` | Service identity, customer/commercial authority |
| §2 | `00_overview/02-architecture-runtime.md` | Architecture & runtime (Rust/Axum + Supabase) |
| §3 | `10_service-contract/03-api-contract.md` | API contract (incl. `/v1/admin/*`) |
| §4 | `20_runtime/04-data-model.md` | Data model |
| §5 | `20_runtime/05-domain-subsystems.md` | Domain subsystems (licensing, entitlements, usage, billing) |
| §6 | `30_dependencies/06-integrations-events.md` | Integrations & events (Stripe, DataForge outbox) |
| §7 | `40_governance/07-scope.md` | Service authority boundary, mutation discipline, truth classes |
| §8 | `40_governance/08-governance.md` | Ownership, designation doctrine, authority hierarchy |
| §9 | `40_governance/09-change-control.md` | Change control |
| §10 | `40_governance/10-authority-boundaries.md` | Detailed authority/ownership boundaries |
| §11 | `40_governance/11-security-privacy.md` | Security & privacy |
| §12 | `50_operations/12-configuration-operations.md` | Configuration & operations |
| §13 | `50_operations/13-verification-status.md` | Verification & status |
| §14 | `99_appendices/14-appendices.md` | Glossary, cross-references, revision history |

## Quick Assembly

```bash
bash doc/system/BUILD.sh
```
