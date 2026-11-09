#!/usr/bin/env bash
# Fail closed when a waived advisory's crate becomes reachable (KI-FOC-20261109-003).
#
# A waiver in .cargo/audit.toml is justified only while its crate is outside the resolved
# dependency graph. This script checks each guarded waiver that is still in the ignore list:
#   - the crate is reachable              -> fail: the exclusion evidence is invalid
#   - the crate has left Cargo.lock       -> fail: removal condition 2 holds; remove the waiver
#   - `cargo tree` exits non-zero         -> fail: a command failure is not "absent"
#   - the positive control prints nothing -> fail: the check cannot read the graph
# A guarded waiver that is no longer in the ignore list is skipped.
#
# Inputs, for tests: CARGO (default `cargo`), AUDIT_TOML (default .cargo/audit.toml),
# LOCKFILE (default Cargo.lock). scripts/check-waived-crates.test.sh covers this script.
set -euo pipefail

CARGO="${CARGO:-cargo}"
AUDIT_TOML="${AUDIT_TOML:-.cargo/audit.toml}"
LOCKFILE="${LOCKFILE:-Cargo.lock}"
TREE_FLAGS=(--workspace --all-features --target all -e all --locked)

# advisory id | waived crate | positive-control crate (must be reachable)
GUARDED=(
  "RUSTSEC-2026-0459|libsqlite3-sys|sqlx-postgres"
)

fail=0
tree() { "$CARGO" tree -q -i "$1" "${TREE_FLAGS[@]}"; }

for row in "${GUARDED[@]}"; do
  IFS='|' read -r id crate control <<<"$row"

  if ! grep -Eq "^[[:space:]]*\"${id}\"," "$AUDIT_TOML"; then
    echo "skip: ${id} is not in the ignore list of ${AUDIT_TOML}"
    continue
  fi

  if ! grep -Eq "^name = \"${control}\"$" "$LOCKFILE"; then
    echo "FAIL: ${id}: control crate ${control} is not in ${LOCKFILE}; the check cannot run"
    fail=1; continue
  fi
  if ! out="$(tree "$control" 2>&1)"; then
    echo "FAIL: ${id}: cargo tree failed for control crate ${control}:"; echo "$out"
    fail=1; continue
  fi
  if [ -z "$out" ]; then
    echo "FAIL: ${id}: control crate ${control} prints no tree; the check cannot read the graph"
    fail=1; continue
  fi

  if ! grep -Eq "^name = \"${crate}\"$" "$LOCKFILE"; then
    echo "FAIL: ${id}: ${crate} has left ${LOCKFILE}. Removal condition 2 holds: remove the waiver."
    fail=1; continue
  fi
  if ! out="$(tree "$crate" 2>&1)"; then
    echo "FAIL: ${id}: cargo tree failed for ${crate}; a command failure is not absence:"; echo "$out"
    fail=1; continue
  fi
  if [ -n "$out" ]; then
    echo "FAIL: ${id}: ${crate} is reachable through the resolved dependency graph."
    echo "The exclusion evidence is invalid. Remove the waiver, pending a security reassessment."
    echo "$out"
    fail=1; continue
  fi
  echo "ok: ${id}: ${crate} is in ${LOCKFILE} and not reachable; control ${control} is reachable"
done

exit "$fail"
