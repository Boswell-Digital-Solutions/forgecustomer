#!/usr/bin/env python3
"""Regenerate vectors.json for the Rake access token v1 contract.

Run from the repository root:  python3 -I contracts/vectors/rake-access-token-v1/generate.py

The test key is derived from TEST_KEY_LABEL, so no private key is stored.
It exists only for these vectors and must never be deployed.
Needs the `cryptography` package. The output is deterministic (Ed25519 is deterministic).
"""
import base64, copy, hashlib, json, pathlib
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.hazmat.primitives import serialization

TEST_KEY_LABEL = b"BDS rake-access-token-v1 test key - never deployed"
OUT = pathlib.Path(__file__).with_name("vectors.json")

seed = hashlib.sha256(TEST_KEY_LABEL).digest()
key = Ed25519PrivateKey.from_private_bytes(seed)
pub = key.public_key().public_bytes(serialization.Encoding.Raw, serialization.PublicFormat.Raw)

def b64u(b): return base64.urlsafe_b64encode(b).rstrip(b"=").decode()
def compact(obj): return json.dumps(obj, separators=(",", ":"), sort_keys=True).encode()
def sign(header, claims):
    signing_input = b64u(compact(header)) + "." + b64u(compact(claims))
    return signing_input + "." + b64u(key.sign(signing_input.encode()))

HEADER = {"alg": "EdDSA", "typ": "rake-access+jwt", "kid": "rake-access-test-1"}
CLAIMS = {
    "iss": "bds-forgecustomer",
    "aud": "rake.application",
    "sub": "6f1d2c3b-4a59-4e8d-9c7b-1a2b3c4d5e6f",
    "tenant_id": "0b9a8c7d-6e5f-4a3b-8c2d-1e0f9a8b7c6d",
    "user_id": "6f1d2c3b-4a59-4e8d-9c7b-1a2b3c4d5e6f",
    "app_id": "authorforge",
    "scopes": ["rake:read", "rake:execute"],
    "type": "access",
    "iat": 1794000000,
    "exp": 1794000300,
    "jti": "3c2b1a09-8f7e-4d6c-9b5a-4f3e2d1c0b9a",
}

def vec(vid, expect, reason=None, header=None, claims=None, token=False, tamper=False, field=None):
    h = copy.deepcopy(HEADER if header is None else header)
    c = copy.deepcopy(CLAIMS if claims is None else claims)
    v = {"id": vid, "expect": expect, "reason": reason, "field": field, "header": h, "claims": c}
    if token:
        t = sign(h, c)
        if tamper:
            head, body, sig = t.split(".")
            c2 = dict(c, scopes=["rake:read", "rake:execute", "rake:approve"])
            t = head + "." + b64u(compact(c2)) + "." + sig
            v["claims"] = c2
        # Stored as its three parts, not one compact string, so that secret scanners
        # do not report these public test vectors as leaked tokens. Join with ".".
        v["jws_parts"] = t.split(".")
    return v

def without(k): c = copy.deepcopy(CLAIMS); del c[k]; return c
def withc(**kw): c = copy.deepcopy(CLAIMS); c.update(kw); return c
def withh(**kw): h = copy.deepcopy(HEADER); h.update(kw); return h

vectors = [
    vec("valid-01-read-execute", "valid", token=True),
    vec("valid-02-read-only", "valid", claims=withc(scopes=["rake:read"], jti="7a6b5c4d-3e2f-4a1b-9c8d-7e6f5a4b3c2d"), token=True),
    vec("invalid-01-no-iss", "invalid", "schema", claims=without("iss"), field="iss"),
    vec("invalid-02-no-aud", "invalid", "schema", claims=without("aud"), field="aud"),
    vec("invalid-03-no-exp", "invalid", "schema", claims=without("exp"), field="exp"),
    vec("invalid-04-no-jti", "invalid", "schema", claims=without("jti"), field="jti"),
    vec("invalid-05-alg-hs256", "invalid", "schema", header=withh(alg="HS256"), field="alg"),
    vec("invalid-06-alg-none", "invalid", "schema", header=withh(alg="none"), field="alg"),
    vec("invalid-07-scopes-string", "invalid", "schema", claims=withc(scopes="rake:read rake:execute"), field="scopes"),
    vec("invalid-08-lifetime-901", "invalid", "rule", claims=withc(exp=CLAIMS["iat"] + 901), field="lifetime"),
    vec("invalid-09-type-refresh", "invalid", "schema", claims=withc(type="refresh"), field="type"),
    vec("invalid-10-no-typ", "invalid", "schema", header={k: v for k, v in HEADER.items() if k != "typ"}, field="typ"),
    vec("invalid-11-aud-list", "invalid", "schema", claims=withc(aud=["rake.application"]), field="aud"),
    vec("invalid-12-extra-email", "invalid", "schema", claims=withc(email="author@example.com"), field="email"),
    vec("invalid-13-sub-not-user", "invalid", "rule", claims=withc(sub="0b9a8c7d-6e5f-4a3b-8c2d-1e0f9a8b7c6d"), field="sub"),
    vec("invalid-14-tampered-signature", "invalid", "signature", token=True, tamper=True, field="verify"),
]

doc = {
    "contract": "rake-access-token-v1",
    "schema": "contracts/rake-access-token-v1.schema.json",
    "test_key": {
        "derivation": "Ed25519 seed = SHA-256 of the label, UTF-8",
        "label": TEST_KEY_LABEL.decode(),
        "kid": HEADER["kid"],
        "public_key_base64": base64.b64encode(pub).decode(),
    },
    "max_lifetime_seconds": 900,
    "vectors": vectors,
}
OUT.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
print(f"wrote {OUT} with {len(vectors)} vectors")
