#!/usr/bin/env bash
# Test scripts/ci-change-scope.sh. A wrong `code=false` skips the code CI on a code change,
# so the script must fail closed: an unknown scope is code.
set -euo pipefail
SCRIPT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/ci-change-scope.sh"
failures=0

expect() {
  local want="$1" input="$2" got
  got="$(printf '%b' "$input" | bash "$SCRIPT")"
  if [ "$got" != "$want" ]; then
    echo "FAIL: input '$input' gave '$got', want '$want'" >&2
    failures=$((failures + 1))
  fi
}

# Documentation only.
expect code=false 'docs/KNOWN_ISSUES.md\n'
expect code=false 'doc/system/13-verification-status.md\ndoc/FOCSYSTEM.md\ndocs/SECURITY.md\n'
expect code=false 'README.md\nCLAUDE.md\ncontracts/events/README.md\n.agents/skills/supabase/SKILL.md\n'
# Any other file is code.
expect code=true 'docs/a.md\napi/src/main.rs\n'
expect code=true 'Cargo.lock\n'
expect code=true 'supabase/migrations/001.sql\n'
expect code=true 'contracts/openapi.yaml\n'
expect code=true 'src/docs.ts\n'
expect code=true 'mydocs/a.rs\n'
expect code=true 'notes.md.bak\n'
expect code=true '.github/workflows/ci.yml\ndocs/a.md\n'
expect code=true 'scripts/ci-change-scope.sh\ndocs/a.md\n'
# An unknown scope is code.
expect code=true ''
expect code=true '\n\n'

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "ci-change-scope: all cases passed"
