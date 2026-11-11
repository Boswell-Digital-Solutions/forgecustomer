//! AC-01 of BDS-FOC-RAKE-ACCESS-ISSUER-v0.1: the Rake access token v1 contract.
//!
//! Checks every vector in `contracts/vectors/rake-access-token-v1/vectors.json`:
//!  * against `contracts/rake-access-token-v1.schema.json`, with a validator for the
//!    exact keyword subset that schema uses (an unknown keyword fails the test);
//!  * against the two rules the schema cannot express (lifetime, `sub` == `user_id`);
//!  * for vectors with a compact token, the EdDSA signature, verified with
//!    `jsonwebtoken` and the test public key derived here from the published label.
//!
//! No route, key or configuration is involved. The test key is derived from a label
//! and exists only for these vectors.

use std::collections::HashSet;
use std::path::PathBuf;

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine as _;
use ed25519_dalek::SigningKey;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

fn contracts_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../contracts")
}

fn load(rel: &str) -> Value {
    let path = contracts_dir().join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {path:?}: {e}"))
}

/// Keywords this validator implements. Annotation-only keywords are accepted and ignored.
const ASSERTIONS: &[&str] = &[
    "type",
    "additionalProperties",
    "required",
    "properties",
    "$ref",
    "const",
    "enum",
    "minLength",
    "maxLength",
    "format",
    "minItems",
    "uniqueItems",
    "items",
    "minimum",
];
const ANNOTATIONS: &[&str] = &["$schema", "$id", "title", "description", "$defs"];

fn resolve<'a>(root: &'a Value, reference: &str) -> &'a Value {
    let name = reference
        .strip_prefix("#/$defs/")
        .unwrap_or_else(|| panic!("unsupported $ref {reference}"));
    &root["$defs"][name]
}

fn type_matches(t: &str, v: &Value) -> bool {
    match t {
        "object" => v.is_object(),
        "array" => v.is_array(),
        "string" => v.is_string(),
        "integer" => v.as_i64().is_some() || v.as_u64().is_some(),
        other => panic!("unsupported type {other}"),
    }
}

/// Returns the first violation as a path and a message, or None.
fn validate(root: &Value, schema: &Value, v: &Value, path: &str) -> Option<String> {
    let s = schema.as_object().expect("schema node is an object");
    for k in s.keys() {
        assert!(
            ASSERTIONS.contains(&k.as_str()) || ANNOTATIONS.contains(&k.as_str()),
            "schema uses keyword {k:?}, which this validator does not implement"
        );
    }
    if let Some(r) = s.get("$ref") {
        return validate(root, resolve(root, r.as_str().unwrap()), v, path);
    }
    if let Some(t) = s.get("type") {
        if !type_matches(t.as_str().unwrap(), v) {
            return Some(format!("{path}: not of type {t}"));
        }
    }
    if let Some(c) = s.get("const") {
        if v != c {
            return Some(format!("{path}: not equal to {c}"));
        }
    }
    if let Some(e) = s.get("enum") {
        if !e.as_array().unwrap().contains(v) {
            return Some(format!("{path}: not in {e}"));
        }
    }
    if let Some(str_v) = v.as_str() {
        let n = str_v.chars().count() as u64;
        if s.get("minLength")
            .and_then(Value::as_u64)
            .is_some_and(|m| n < m)
        {
            return Some(format!("{path}: shorter than minLength"));
        }
        if s.get("maxLength")
            .and_then(Value::as_u64)
            .is_some_and(|m| n > m)
        {
            return Some(format!("{path}: longer than maxLength"));
        }
        if let Some(f) = s.get("format") {
            assert_eq!(f, "uuid", "unsupported format {f}");
            if uuid::Uuid::parse_str(str_v).is_err() {
                return Some(format!("{path}: not a uuid"));
            }
        }
    }
    if let Some(m) = s.get("minimum").and_then(Value::as_i64) {
        if v.as_i64().is_some_and(|x| x < m) {
            return Some(format!("{path}: below minimum"));
        }
    }
    if let Some(arr) = v.as_array() {
        if s.get("minItems")
            .and_then(Value::as_u64)
            .is_some_and(|m| (arr.len() as u64) < m)
        {
            return Some(format!("{path}: fewer than minItems"));
        }
        if s.get("uniqueItems") == Some(&Value::Bool(true)) {
            let set: HashSet<String> = arr.iter().map(Value::to_string).collect();
            if set.len() != arr.len() {
                return Some(format!("{path}: items not unique"));
            }
        }
        if let Some(item_schema) = s.get("items") {
            for (i, item) in arr.iter().enumerate() {
                if let Some(err) = validate(root, item_schema, item, &format!("{path}[{i}]")) {
                    return Some(err);
                }
            }
        }
    }
    if let Some(obj) = v.as_object() {
        let props = s
            .get("properties")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        if let Some(req) = s.get("required").and_then(Value::as_array) {
            for r in req {
                if !obj.contains_key(r.as_str().unwrap()) {
                    return Some(format!("{path}: missing required {r}"));
                }
            }
        }
        if let Some(ap) = s.get("additionalProperties") {
            assert_eq!(
                ap,
                &Value::Bool(false),
                "only additionalProperties: false is supported"
            );
            if let Some(extra) = obj.keys().find(|k| !props.contains_key(*k)) {
                return Some(format!("{path}: unexpected property {extra}"));
            }
        }
        for (k, sub_schema) in &props {
            if let Some(child) = obj.get(k) {
                if let Some(err) = validate(root, sub_schema, child, &format!("{path}.{k}")) {
                    return Some(err);
                }
            }
        }
    }
    None
}

/// Rules that compare two claims. Run only on schema-valid claims.
fn rule_violation(claims: &Map<String, Value>, max_lifetime: i64) -> Option<String> {
    let iat = claims["iat"].as_i64().unwrap();
    let exp = claims["exp"].as_i64().unwrap();
    if exp <= iat {
        return Some("exp is not after iat".into());
    }
    if exp - iat > max_lifetime {
        return Some(format!("lifetime {} s exceeds {max_lifetime} s", exp - iat));
    }
    if claims["sub"] != claims["user_id"] {
        return Some("sub differs from user_id".into());
    }
    None
}

fn spki_pem(public_key: &[u8; 32]) -> String {
    // SubjectPublicKeyInfo for Ed25519 (RFC 8410): fixed 12-byte prefix + raw key.
    let mut der = hex::decode("302a300506032b6570032100").unwrap();
    der.extend_from_slice(public_key);
    format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n",
        B64.encode(der)
    )
}

fn signature_ok(token: &str, public_key_pem: &str, iss: &str) -> Result<(Value, Value), String> {
    let header = decode_header(token).map_err(|e| format!("header: {e}"))?;
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.validate_exp = false; // the vectors carry fixed, past timestamps
    validation.required_spec_claims = HashSet::new();
    validation.set_audience(&["rake.application"]);
    validation.set_issuer(&[iss]);
    let key = DecodingKey::from_ed_pem(public_key_pem.as_bytes()).map_err(|e| e.to_string())?;
    let data = decode::<Value>(token, &key, &validation).map_err(|e| format!("verify: {e}"))?;
    Ok((serde_json::to_value(&header).unwrap(), data.claims))
}

/// A refusal must name the field the vector targets, so that an unrelated failure
/// cannot stand in for the intended one.
fn expect_field(id: &str, field: Option<&str>, err: &str) {
    let f = field.unwrap_or_else(|| panic!("{id}: an invalid vector must name its field"));
    assert!(
        err.contains(f),
        "{id}: refused for the wrong reason: {err:?} does not name {f:?}"
    );
}

#[test]
fn rake_access_contract_vectors() {
    let schema = load("rake-access-token-v1.schema.json");
    let doc = load("vectors/rake-access-token-v1/vectors.json");

    // The published test public key must match the key derived from the label.
    let label = doc["test_key"]["label"].as_str().unwrap();
    let seed: [u8; 32] = Sha256::digest(label.as_bytes()).into();
    let public_key = SigningKey::from_bytes(&seed).verifying_key().to_bytes();
    assert_eq!(
        doc["test_key"]["public_key_base64"].as_str().unwrap(),
        B64.encode(public_key),
        "vectors.json test public key does not match the label"
    );
    let pem = spki_pem(&public_key);
    let max_lifetime = doc["max_lifetime_seconds"].as_i64().unwrap();

    let vectors = doc["vectors"].as_array().unwrap();
    assert!(vectors.len() >= 16, "expected the full vector set");
    let (mut valid, mut invalid) = (0, 0);

    for v in vectors {
        let id = v["id"].as_str().unwrap();
        let expect = v["expect"].as_str().unwrap();
        let reason = v["reason"].as_str();
        let field = v["field"].as_str();
        let decoded = serde_json::json!({ "header": v["header"], "claims": v["claims"] });

        let schema_err = validate(&schema, &schema, &decoded, "$");
        let rule_err = if schema_err.is_none() {
            rule_violation(v["claims"].as_object().unwrap(), max_lifetime)
        } else {
            None
        };
        // Tokens are stored as their three parts (see generate.py); rejoin them.
        let sig = v.get("jws_parts").map(|parts| {
            let token = parts
                .as_array()
                .unwrap()
                .iter()
                .map(|p| p.as_str().unwrap())
                .collect::<Vec<_>>()
                .join(".");
            signature_ok(&token, &pem, v["claims"]["iss"].as_str().unwrap())
        });

        match expect {
            "valid" => {
                assert!(
                    schema_err.is_none(),
                    "{id}: schema rejected a valid vector: {schema_err:?}"
                );
                assert!(
                    rule_err.is_none(),
                    "{id}: rule rejected a valid vector: {rule_err:?}"
                );
                let (header, claims) = sig
                    .unwrap_or_else(|| panic!("{id}: a valid vector must carry jws_parts"))
                    .unwrap_or_else(|e| panic!("{id}: signature check failed: {e}"));
                assert_eq!(
                    claims, v["claims"],
                    "{id}: token claims differ from the vector"
                );
                assert_eq!(header["alg"], "EdDSA", "{id}");
                assert_eq!(header["typ"], v["header"]["typ"], "{id}");
                assert_eq!(header["kid"], v["header"]["kid"], "{id}");
                valid += 1;
            }
            "invalid" => {
                match reason {
                    Some("schema") => {
                        let e = schema_err.unwrap_or_else(|| panic!("{id}: schema accepted it"));
                        expect_field(id, field, &e);
                        println!("{id}: refused by the schema: {e}");
                    }
                    Some("rule") => {
                        assert!(
                            schema_err.is_none(),
                            "{id}: expected a rule failure, got schema {schema_err:?}"
                        );
                        let e = rule_err.unwrap_or_else(|| panic!("{id}: rule accepted it"));
                        expect_field(id, field, &e);
                        println!("{id}: refused by a rule: {e}");
                    }
                    Some("signature") => {
                        assert!(
                            schema_err.is_none() && rule_err.is_none(),
                            "{id}: must fail only on the signature"
                        );
                        let e = sig
                            .unwrap_or_else(|| panic!("{id}: needs jws_parts"))
                            .err()
                            .unwrap_or_else(|| panic!("{id}: a tampered token verified"));
                        expect_field(id, field, &e);
                        println!("{id}: refused by the signature check: {e}");
                    }
                    other => panic!("{id}: unknown reason {other:?}"),
                }
                invalid += 1;
            }
            other => panic!("{id}: unknown expect {other}"),
        }
    }
    println!("rake-access-token-v1: {valid} valid, {invalid} invalid vectors checked");
    assert_eq!((valid, invalid), (2, 14));
}
