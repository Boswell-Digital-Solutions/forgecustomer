# AC-00 — source lock

Checked 2026-11-11 by the agent, on the operator's machine, with `git fetch` then `git diff --stat <plan pin> <head> -- <paths>`.

| Repository | Plan pin | Head checked | Paths compared | Result |
|---|---|---|---|---|
| forgecustomer `main` | `3a2ce487e` | `a4223af7c` | `api`, `contracts`, `supabase`, `doc/system` | no change (the only commit since the pin adds this plan set) |
| rake `master` | `caac54c1a` | `b502298c1` | `auth`, `config.py`, `forge_keys.py`, `tests/conftest.py`, `main.py`, `service_contract.v1.json`, `render.yaml`, `api` | no change (rake #187 changed docs only) |
| Author-Forge `main` | `1856b3b73` | `1856b3b73` | `apps/api/src/routes/rake.ts`, `apps/api/tests/routes/rake.test.ts`, `apps/api/src/services/workspaceAuthority.ts`, `src-tauri/src/fleet_updates.rs` | same commit |
| bds_website `main` | `3b6a2e51d` | `3b6a2e51d` | `git grep -i rake` | same commit; no match |
| forge `main` | `88fda8241` | `a2b0d5f82` | not compared by path | read for the registry row only |

Each "no change" is an empty `git diff --stat` output for the listed paths.
