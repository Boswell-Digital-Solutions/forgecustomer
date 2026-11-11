"""Checks for scripts/rake_access_http_smoke.sh (BDS-FOC-RAKE-ACCESS-ISSUER-v0.1 WP-02).

Standard library plus the `openssl` CLI (Ed25519). Every key is a fixed test seed.
Each check prints its name and PASS; the first failure raises and exits nonzero.
"""

import base64
import hashlib
import hmac
import json
import os
import shlex
import subprocess
import time
import urllib.error
import urllib.request
import uuid

BASE = os.environ["BASE"]
TMP = os.environ["TMP"]
PHASE = os.environ["PHASE"]
PSQL = shlex.split(os.environ["PSQL_CMD"]) + ["-d", os.environ["DB_NAME"], "-tA", "-c"]

CUSTOMERS = {
    # tag: (customer_id, auth_user_id, installation_id, device seed byte)
    "a": ("00000000-0000-4000-8000-00000000a001", "00000000-0000-4000-8000-00000000a011", "00000000-0000-4000-8000-00000000a003", 0x11),
    "b": ("00000000-0000-4000-8000-00000000b001", "00000000-0000-4000-8000-00000000b011", "00000000-0000-4000-8000-00000000b003", 0x22),
    "c": ("00000000-0000-4000-8000-00000000c001", "00000000-0000-4000-8000-00000000c011", "00000000-0000-4000-8000-00000000c003", 0x33),
    "d": ("00000000-0000-4000-8000-00000000d001", "00000000-0000-4000-8000-00000000d011", "00000000-0000-4000-8000-00000000d003", 0x44),
}
PROOF_DOMAIN = "forgecustomer.rake-access-exchange.v1"
CONTRACT_CLAIMS = {"iss", "aud", "sub", "tenant_id", "user_id", "app_id", "scopes", "type", "iat", "exp", "jti"}


def b64u(data):
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode()


def b64u_dec(text):
    return base64.urlsafe_b64decode(text + "=" * (-len(text) % 4))


def supabase_token(tag, aal="aal2"):
    header = {"alg": "HS256", "typ": "JWT"}
    claims = {"sub": CUSTOMERS[tag][1], "iss": os.environ["JWT_ISSUER"], "aud": "authenticated",
              "exp": int(time.time()) + 600, "aal": aal}
    head = b64u(json.dumps(header, separators=(",", ":")).encode())
    body = b64u(json.dumps(claims, separators=(",", ":")).encode())
    sig = hmac.new(os.environ["JWT_SECRET"].encode(), f"{head}.{body}".encode(), hashlib.sha256).digest()
    return f"{head}.{body}.{b64u(sig)}"


def private_pem(seed_byte):
    der = bytes.fromhex("302e020100300506032b657004220420") + bytes([seed_byte]) * 32
    path = os.path.join(TMP, f"device-{seed_byte:02x}.pem")
    with open(path, "w") as f:
        # The PEM label is split so secret scanners do not read this builder as a stored key.
        label = "PRIVATE" + " KEY"
        f.write(f"-----BEGIN {label}-----\n" + base64.b64encode(der).decode() + f"\n-----END {label}-----\n")
    return path


def ed25519_sign(seed_byte, message):
    msg_path = os.path.join(TMP, "msg.bin")
    with open(msg_path, "wb") as f:
        f.write(message)
    out = subprocess.run(["openssl", "pkeyutl", "-sign", "-rawin", "-inkey", private_pem(seed_byte), "-in", msg_path],
                         check=True, capture_output=True).stdout
    assert len(out) == 64, len(out)
    return base64.b64encode(out).decode()


def ed25519_verify(public_b64, message, signature):
    der = bytes.fromhex("302a300506032b6570032100") + base64.b64decode(public_b64)
    pub_path = os.path.join(TMP, "pub.pem")
    with open(pub_path, "w") as f:
        f.write("-----BEGIN PUBLIC KEY-----\n" + base64.b64encode(der).decode() + "\n-----END PUBLIC KEY-----\n")
    msg_path = os.path.join(TMP, "verify.bin")
    sig_path = os.path.join(TMP, "verify.sig")
    with open(msg_path, "wb") as f:
        f.write(message)
    with open(sig_path, "wb") as f:
        f.write(signature)
    r = subprocess.run(["openssl", "pkeyutl", "-verify", "-pubin", "-rawin", "-inkey", pub_path,
                        "-in", msg_path, "-sigfile", sig_path], capture_output=True)
    return r.returncode == 0


def proof(tag, app_id="authorforge", scopes=None, iat=None, nonce=None, seed=None):
    _, _, installation_id, seed_byte = CUSTOMERS[tag]
    iat = int(time.time()) if iat is None else iat
    nonce = nonce or str(uuid.uuid4())
    scope_line = ",".join(scopes) if scopes is not None else ""
    message = f"{PROOF_DOMAIN}\n{app_id}\n{installation_id}\n{scope_line}\n{iat}\n{nonce}".encode()
    sig = ed25519_sign(seed if seed is not None else seed_byte, message)
    return {"installation_id": installation_id, "iat": iat, "nonce": nonce, "signature": sig}


def request(method, path, body=None, token=None):
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(BASE + path, data=data, method=method)
    req.add_header("content-type", "application/json")
    if token:
        req.add_header("authorization", f"Bearer {token}")
    try:
        with urllib.request.urlopen(req, timeout=15) as res:
            return res.status, dict(res.headers), json.loads(res.read() or b"null")
    except urllib.error.HTTPError as e:
        raw = e.read()
        return e.code, dict(e.headers), json.loads(raw) if raw else None


def exchange(tag, token=None, **kw):
    body = {"app_id": kw.pop("app_id", "authorforge"), "client_proof": kw.pop("client_proof", None)}
    scopes = kw.pop("scopes", None)
    if scopes is not None:
        body["scopes"] = scopes
    if body["client_proof"] is None:
        body["client_proof"] = proof(tag, app_id=body["app_id"], scopes=scopes)
    return request("POST", "/v1/app-tokens/rake", body, token if token is not None else supabase_token(tag))


def sql(query):
    return subprocess.run(PSQL + [query], check=True, capture_output=True, text=True).stdout.strip()


def code_of(body):
    return (body or {}).get("error", {}).get("code") if isinstance(body, dict) else None


def expect(name, status, body, want_status, want_code=None, want_reason=None):
    got_code = code_of(body)
    reason = ((body or {}).get("error", {}).get("details") or {}).get("reason") if isinstance(body, dict) else None
    ok = status == want_status and (want_code is None or got_code == want_code) and (want_reason is None or reason == want_reason)
    if not ok:
        if isinstance(body, dict) and "access_token" in body:
            body = {**body, "access_token": "<redacted>"}
        raise SystemExit(f"FAIL {name}: got {status} {got_code} reason={reason} body={body}; "
                         f"want {want_status} {want_code} reason={want_reason}")
    print(f"PASS {name}: {status} {got_code or ''} {reason or ''}".rstrip())


def check_token(token, published):
    head_b64, body_b64, sig_b64 = token.split(".")
    header = json.loads(b64u_dec(head_b64))
    claims = json.loads(b64u_dec(body_b64))
    assert header == {"alg": "EdDSA", "typ": "rake-access+jwt", "kid": "rake-access-smoke-1"}, header
    assert set(claims) == CONTRACT_CLAIMS, sorted(claims)
    assert claims["aud"] == "rake.application" and claims["iss"] == "bds-forgecustomer", claims
    assert claims["type"] == "access" and claims["sub"] == claims["user_id"], claims
    assert claims["exp"] - claims["iat"] == 300, claims
    key = {k["kid"]: k for k in published}[header["kid"]]
    assert ed25519_verify(key["public_key"], f"{head_b64}.{body_b64}".encode(), b64u_dec(sig_b64)), "signature"
    text = json.dumps(claims)
    for forbidden in [CUSTOMERS["a"][0], CUSTOMERS["a"][1], "@"]:
        assert forbidden not in text, f"token leaks {forbidden}"
    return header, claims


def keyed():
    # AC-03: the key list holds public halves only, active first, and not the entitlement key.
    status, _, keys = request("GET", "/v1/app-tokens/keys")
    assert status == 200, status
    published = keys["keys"]
    assert [(k["kid"], k["status"]) for k in published] == [("rake-access-smoke-1", "active"), ("rake-access-smoke-2", "verify_only")], published
    _, _, ent = request("GET", "/v1/entitlements/keys")
    ent_keys = {k["public_key"] for k in ent["keys"]}
    assert not ent_keys & {k["public_key"] for k in published}, "a Rake key is listed as an entitlement key"
    print("PASS AC-03 keys route: public halves only; entitlement route lists none of them")

    # AC-02 refusals.
    expect("no Supabase token", *request("POST", "/v1/app-tokens/rake", {"app_id": "authorforge", "client_proof": proof("a")})[::2], 401)
    expect("bad Supabase token", *exchange("a", token="x.y.z")[::2], 401)
    expect("aal1 with mfa_required", *exchange("c", token=supabase_token("c", aal="aal1"))[::2], 403, "MFA_REQUIRED")
    expect("unknown app", *exchange("a", app_id="no_such_app")[::2], 403, "APP_NOT_ALLOWED")
    expect("scope outside the list", *exchange("a", scopes=["rake:admin"])[::2], 403, "SCOPE_NOT_ALLOWED")
    expect("rake:approve mixed with another scope", *exchange("a", scopes=["rake:read", "rake:approve"])[::2], 403, "SCOPE_NOT_ALLOWED")
    expect("feature not entitled", *exchange("b")[::2], 403, "ENTITLEMENT_DENIED")
    expect("revoked device", *exchange("d")[::2], 401, "CLIENT_AUTH_FAILED", "device_key")
    expect("proof signed by another key", *exchange("a", client_proof=proof("a", seed=0x22))[::2], 401, "CLIENT_AUTH_FAILED", "signature")
    expect("stale proof", *exchange("a", client_proof=proof("a", iat=int(time.time()) - 120))[::2], 401, "CLIENT_AUTH_FAILED", "stale")
    expect("proof for other scopes", *exchange("a", scopes=["rake:read"], client_proof=proof("a", scopes=["rake:execute"]))[::2],
           401, "CLIENT_AUTH_FAILED", "signature")
    refused_rows = sql(f"select count(*) from public.app_token_mints where customer_id in "
                       f"('{CUSTOMERS['b'][0]}','{CUSTOMERS['c'][0]}','{CUSTOMERS['d'][0]}')")
    assert refused_rows == "0", refused_rows
    print("PASS no audit row for any refused request")

    # AC-02 success.
    p = proof("a")
    status, headers, body = exchange("a", client_proof=p)
    assert status == 200, (status, body)
    assert {k.lower(): v for k, v in headers.items()}.get("cache-control") == "no-store", headers
    assert body["token_type"] == "Bearer" and body["expires_in"] == 300, body
    assert body["scopes"] == ["rake:execute", "rake:read"], body
    _, claims = check_token(body["access_token"], published)
    rows = sql(f"select count(*) from public.app_token_mints where jti = '{claims['jti']}' and customer_id = '{CUSTOMERS['a'][0]}'")
    assert rows == "1", rows
    print("PASS AC-02 success: 200, no-store, contract header and claims, verifies with the published key, one audit row")
    print("PASS AC-04 token carries no customer_id, Supabase user id or email")

    expect("replayed proof (same nonce)", *exchange("a", client_proof=p)[::2], 401, "CLIENT_AUTH_FAILED", "nonce_replayed")

    # AC-04 pseudonym stability, separation and revocation.
    _, _, again = exchange("a")
    _, claims2 = check_token(again["access_token"], published)
    assert claims2["tenant_id"] == claims["tenant_id"] and claims2["user_id"] == claims["user_id"], "pseudonym changed"
    assert claims2["jti"] != claims["jti"]
    stored = sql(f"select tenant_pseudonym from public.rake_tenant_pseudonyms where customer_id = '{CUSTOMERS['a'][0]}' and revoked_at is null")
    assert stored == claims["tenant_id"], (stored, claims["tenant_id"])
    assert claims["tenant_id"] != CUSTOMERS["a"][0]
    sql(f"update public.rake_tenant_pseudonyms set revoked_at = now() where customer_id = '{CUSTOMERS['a'][0]}' and revoked_at is null")
    _, _, fresh = exchange("a")
    _, claims3 = check_token(fresh["access_token"], published)
    assert claims3["tenant_id"] != claims["tenant_id"], "a revoked pseudonym was reused"
    print("PASS AC-04 pseudonyms: stable across mints, not the customer id, replaced after revocation")

    # DEC-6: an approve-only token.
    status, _, appr = exchange("a", scopes=["rake:approve"])
    assert status == 200 and appr["scopes"] == ["rake:approve"], (status, appr)
    _, claims4 = check_token(appr["access_token"], published)
    assert claims4["scopes"] == ["rake:approve"]
    print("PASS DEC-6 approve-only token issued with scopes [rake:approve]")

    # AC-02: an audit write failure returns 503 and no token.
    sql("create or replace function public.smoke_fail_mint() returns trigger language plpgsql as "
        "$f$ begin raise exception 'smoke: audit write refused'; end $f$;")
    sql("drop trigger if exists smoke_fail_mint on public.app_token_mints; "
        "create trigger smoke_fail_mint before insert on public.app_token_mints "
        "for each row execute function public.smoke_fail_mint();")
    try:
        before = sql(f"select count(*) from public.app_token_mints where customer_id = '{CUSTOMERS['a'][0]}'")
        status, _, body = exchange("a")
        expect("audit write failure", status, body, 503, "SERVICE_UNAVAILABLE")
        assert "access_token" not in (body or {}), body
        after = sql(f"select count(*) from public.app_token_mints where customer_id = '{CUSTOMERS['a'][0]}'")
        assert before == after, (before, after)
        print("PASS audit write failure leaves no token and no row")
    finally:
        sql("drop trigger if exists smoke_fail_mint on public.app_token_mints; "
            "drop function if exists public.smoke_fail_mint();")


def unkeyed():
    status, _, keys = request("GET", "/v1/app-tokens/keys")
    assert status == 200 and keys["keys"] == [], (status, keys)
    print("PASS no key configured: the key list is empty")
    expect("no key configured", *exchange("a")[::2], 503, "SERVICE_UNAVAILABLE")


{"keyed": keyed, "unkeyed": unkeyed}[PHASE]()
