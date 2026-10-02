#!/usr/bin/env bash
# Decide whether a change is code or documentation only (standing rule, 2026-10-01).
#
# Reads changed file paths on stdin, one per line. Prints `code=true` or `code=false`.
# A documentation-only change prints `code=false`, and the code jobs in ci.yml skip.
# Anything else prints `code=true`, including an empty list: when the scope is unknown,
# the code CI runs.
#
# Documentation is `docs/**`, `doc/**` and any `*.md`. A documentation path that code or a
# test reads is code. List each such path in REINCLUDE below. No path is listed today:
# the API, the tests, the smoke scripts and deploy/Dockerfile read no documentation.
set -euo pipefail

REINCLUDE='^$'
DOCUMENTATION='^(docs/|doc/)|\.md$'

seen=0
while IFS= read -r path; do
  [ -z "$path" ] && continue
  seen=1
  if [[ "$path" =~ $REINCLUDE ]] || ! [[ "$path" =~ $DOCUMENTATION ]]; then
    echo "code=true"
    exit 0
  fi
done

if [ "$seen" -eq 0 ]; then
  echo "code=true"
else
  echo "code=false"
fi
