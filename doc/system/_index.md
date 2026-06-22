        # ForgeCustomer - Compiled System Reference

        **Designation:** FOC
        **Document role:** Canonical compiled technical reference for the ForgeCustomer cloud service
        **Source:** `doc/system/`
        **Build command:** `bash doc/system/BUILD.sh`
        **Document version:** 2.0 (2026-06-22) - canonical compliance migration
        **Protocol:** BDS Documentation Protocol v2.0; BDS Repo Documentation System Canonical Compliance Standard

        > **Generated artifact warning:** `doc/FOCSYSTEM.md` is assembled output. Edit
        > the source modules under `doc/system/` and rebuild. Hand edits to the
        > compiled artifact are overwritten by the next build.

        Assembly contract:

        - Command: `bash doc/system/BUILD.sh`
        - Validation: `bash doc/system/validate_snapshots.sh` runs during assembly
        - Primary output: `doc/FOCSYSTEM.md`

        This `doc/system/` tree is the canonical source of truth for ForgeCustomer. It uses
        explicit **truth classes**: canonical facts define repo role, authority
        boundaries, contract behavior, runtime behavior, and verification doctrine;
        snapshot facts are dated, audit-derived counts and current implementation
        inventory that may drift between audits.

        | Part | File | Contents |
        | --- | --- | --- |
        | §1 | `00_overview/00-overview.md` | 1. Overview |
| §2 | `00_overview/02-architecture-runtime.md` | 3. Architecture and Runtime |
| §3 | `10_service-contract/03-api-contract.md` | 4. API Contract |
| §4 | `20_runtime/04-data-model.md` | 5. Data Model |
| §5 | `20_runtime/05-domain-subsystems.md` | 6. Domain Subsystems |
| §6 | `30_dependencies/06-integrations-events.md` | 7. Integrations and Events |
| §7 | `40_governance/01-authority-boundaries.md` | 2. Authority Boundaries |
| §8 | `40_governance/07-security-privacy.md` | 8. Security and Privacy |
| §9 | `40_governance/90-governance-change-control.md` | 11. Governance and Change Control |
| §10 | `50_operations/08-configuration-operations.md` | 9. Configuration and Operations |
| §11 | `50_operations/09-verification-status.md` | 10. Verification and Status |
| §12 | `99_appendices/90-appendices.md` | Appendices |

        ## Quick Assembly

        ```bash
        bash doc/system/BUILD.sh
        ```
