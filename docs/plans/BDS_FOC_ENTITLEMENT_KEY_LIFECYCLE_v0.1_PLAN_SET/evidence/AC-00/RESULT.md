# AC-00 — Plan-set structure and parity

- **Date:** 2026-11-06.
- **Checker:** `~/Forge/reports/bds_plan_ac00_check.py`, sha256 `902088580ee732e9bc7d3f173f87a0d985a0f09411e4145652feef4e7c06b6bb`. It is the same checker as the fleet-manager plan's AC-00. Command: `python3 -I ~/Forge/reports/bds_plan_ac00_check.py <plan folder>`.
- **Checks:**
  - the manifest equals the folder, evidence included;
  - components 00–09 are present;
  - the lifecycle agrees in 00, `plan_set.yaml`, `FOLDER_METADATA.json` and `REGISTRY_ENTRY.json`;
  - the WP ids agree between 08 and `plan_set.yaml`;
  - the AC ids agree between 07 and `plan_set.yaml`;
  - the compiled file equals its header plus 00–09.
- **Observed:** all six checks PASS, exit 0.
- **Negative controls:** run on 2026-11-06 against the fleet-manager plan. A lifecycle mismatch, a stray file and an unmirrored component edit each exit 1 with the named failure.
- **Result:** AC-00 **PASS**. With Decision 1 ruled, G0 is **PASS**.
- **Limit:** the check is structural. It does not judge content.
