#!/usr/bin/env bash
# Live-PostgreSQL HTTP smoke for the Rake access-token exchange
# (BDS-FOC-RAKE-ACCESS-ISSUER-v0.1 WP-02: AC-02, AC-03, AC-04).
#
# Needs: psql, cargo, curl, python3 (standard library only) and openssl 3 (Ed25519).
# Every key here is a fixed test seed. Nothing here is a deployed secret.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

DB_NAME="${DB_NAME:-fc}"
PSQL_CMD="${PSQL:-psql -v ON_ERROR_STOP=1}"
SMOKE_HOST="${SMOKE_HOST:-127.0.0.1}"
SMOKE_PORT="${SMOKE_PORT:-19134}"
JWT_SECRET="rake-access-smoke-supabase-secret"
JWT_ISSUER="https://proj.supabase.co/auth/v1"

tmp_dir="$(mktemp -d)"
api_pid=""
cleanup() {
  if [[ -n "$api_pid" ]] && kill -0 "$api_pid" 2>/dev/null; then
    kill "$api_pid" 2>/dev/null || true
    wait "$api_pid" 2>/dev/null || true
  fi
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

$PSQL_CMD -d "$DB_NAME" -f supabase/tests/rake_access_http_smoke.sql >/dev/null

if [[ -z "${DATABASE_URL:-}" ]]; then
  if [[ -n "${PGHOST:-}" ]]; then
    export DATABASE_URL="postgres:///${DB_NAME}?host=${PGHOST}&port=${PGPORT:-5432}"
  else
    export DATABASE_URL="postgres://localhost/${DB_NAME}"
  fi
fi

export HOST="$SMOKE_HOST" PORT="$SMOKE_PORT"
export SUPABASE_JWT_ISSUER="$JWT_ISSUER" SUPABASE_JWT_AUDIENCE="authenticated" SUPABASE_JWT_SECRET="$JWT_SECRET"
export ADMIN_JWT_ISSUER="https://operators.local" ADMIN_JWT_AUDIENCE="forgecustomer-admin" ADMIN_JWT_PUBLIC_KEY=""
# Test seeds: entitlement key = 0x03 x32, Rake access key = 0x05 x32, standby = public half of 0x06 x32.
export ENTITLEMENT_SIGNING_PRIVATE_KEY="AwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwM="
export ENTITLEMENT_SIGNING_KEY_ID="rake-smoke-entitlement-key"
export RAKE_ACCESS_SIGNING_KEY_ID="rake-access-smoke-1"
export RAKE_ACCESS_ISSUER="bds-forgecustomer"
export DATABASE_ACQUIRE_TIMEOUT_SECS=5 REQUEST_TIMEOUT_SECS=10 RATE_LIMIT_PER_MINUTE=0

# Base64 raw Ed25519 public key of a test seed (one byte, repeated 32 times).
public_key_of_seed() {
  local byte="$1" pem="$tmp_dir/seed-$1.pem"
  {
    echo "-----BEGIN PRIV""ATE KEY-----"
    python3 -I -c "import base64,sys; print(base64.b64encode(bytes.fromhex('302e020100300506032b657004220420') + bytes([int(sys.argv[1], 16)]) * 32).decode(), end='')" "$byte"
    echo
    echo "-----END PRIV""ATE KEY-----"
  } > "$pem"
  openssl pkey -in "$pem" -pubout -outform DER | tail -c 32 | base64 -w0
}

start_api() {
  local label="$1"
  cargo run -q -p forgecustomer-api > "$tmp_dir/api-$label.log" 2>&1 &
  api_pid="$!"
  for _ in $(seq 1 240); do
    if ! kill -0 "$api_pid" 2>/dev/null; then
      echo "forgecustomer-api exited during startup ($label)" >&2
      cat "$tmp_dir/api-$label.log" >&2
      exit 1
    fi
    if curl -fsS "http://$SMOKE_HOST:$SMOKE_PORT/v1/health" >/dev/null 2>&1; then
      return 0
    fi
    sleep 1
  done
  echo "forgecustomer-api did not become healthy ($label)" >&2
  cat "$tmp_dir/api-$label.log" >&2
  exit 1
}

stop_api() {
  kill "$api_pid" 2>/dev/null || true
  wait "$api_pid" 2>/dev/null || true
  api_pid=""
}

run_checks() {
  local phase="$1"
  PHASE="$phase" BASE="http://$SMOKE_HOST:$SMOKE_PORT" TMP="$tmp_dir" \
  JWT_SECRET="$JWT_SECRET" JWT_ISSUER="$JWT_ISSUER" \
  PSQL_CMD="$PSQL_CMD" DB_NAME="$DB_NAME" \
    python3 -I scripts/rake_access_http_smoke.py
}

# Phase 1: the issuer has a key.
export RAKE_ACCESS_SIGNING_PRIVATE_KEY="BQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQU="
# The standby public key is derived from its test seed at run time, not stored.
standby_pub="$(public_key_of_seed 06)"
export RAKE_ACCESS_VERIFY_PUBLIC_KEYS="{\"rake-access-smoke-2\":\"${standby_pub}\"}"
start_api keyed
run_checks keyed
stop_api

# Phase 2: no key configured, so the exchange must fail closed with 503.
export RAKE_ACCESS_SIGNING_PRIVATE_KEY=""
export RAKE_ACCESS_VERIFY_PUBLIC_KEYS=""
start_api unkeyed
run_checks unkeyed
stop_api

echo "rake access HTTP smoke: PASS"
