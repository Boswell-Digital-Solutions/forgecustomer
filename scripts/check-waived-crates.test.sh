#!/usr/bin/env bash
# Tests for scripts/check-waived-crates.sh, with a fake cargo and fixture files.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

# Fake cargo: FAKE_<crate with - as _>_OUT is printed; FAKE_<crate>_RC is the exit code.
cat >"$tmp/cargo" <<'FAKE'
#!/usr/bin/env bash
crate=""; prev=""
for a in "$@"; do [ "$prev" = "-i" ] && crate="$a"; prev="$a"; done
key="$(echo "$crate" | tr '-' '_')"
out_var="FAKE_${key}_OUT"; rc_var="FAKE_${key}_RC"
[ -n "${!out_var:-}" ] && printf '%s\n' "${!out_var}"
exit "${!rc_var:-0}"
FAKE
chmod +x "$tmp/cargo"

printf '[advisories]\nignore = [\n    "RUSTSEC-2026-0459",\n]\n' >"$tmp/waived.toml"
printf '[advisories]\nignore = [\n]\n' >"$tmp/clear.toml"
printf 'name = "libsqlite3-sys"\nname = "sqlx-postgres"\n' >"$tmp/full.lock"
printf 'name = "sqlx-postgres"\n' >"$tmp/nosqlite.lock"
printf 'name = "libsqlite3-sys"\n' >"$tmp/nocontrol.lock"

pass=0; failed=0
run() { # name, expected exit (0|1), toml, lock, then env assignments
  local name="$1" want="$2" toml="$3" lock="$4"; shift 4
  local got=0
  env CARGO="$tmp/cargo" AUDIT_TOML="$tmp/$toml" LOCKFILE="$tmp/$lock" "$@" \
    bash "$here/check-waived-crates.sh" >"$tmp/out" 2>&1 || got=$?
  [ "$got" -ne 0 ] && got=1
  if [ "$got" = "$want" ]; then pass=$((pass+1)); else failed=$((failed+1)); echo "FAIL: $name (want $want, got $got)"; cat "$tmp/out"; fi
}

CTRL=FAKE_sqlx_postgres_OUT="sqlx-postgres v0.8.6"
run "absent and control reachable passes"   0 waived.toml full.lock      "$CTRL"
run "reachable crate fails"                 1 waived.toml full.lock      "$CTRL" FAKE_libsqlite3_sys_OUT="libsqlite3-sys v0.30.1"
run "cargo tree error on crate fails"       1 waived.toml full.lock      "$CTRL" FAKE_libsqlite3_sys_RC=101
run "control prints nothing fails"          1 waived.toml full.lock
run "cargo tree error on control fails"     1 waived.toml full.lock      "$CTRL" FAKE_sqlx_postgres_RC=101
run "crate left the lockfile fails"         1 waived.toml nosqlite.lock  "$CTRL"
run "control missing from lockfile fails"   1 waived.toml nocontrol.lock "$CTRL"
run "waiver removed skips"                  0 clear.toml  full.lock      FAKE_libsqlite3_sys_OUT="libsqlite3-sys v0.30.1"

echo "check-waived-crates tests: ${pass} passed, ${failed} failed"
[ "$failed" -eq 0 ]
