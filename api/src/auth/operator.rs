//! Operator token verification against a set of trusted keys (P3 gate G5).
//!
//! `ADMIN_JWT_TRUSTED_KEYS` holds `{kid: {"key": <Ed25519 SPKI PEM>, "issuer": <issuer>}}`
//! (DEC-G5-1). The header `kid` selects exactly one entry. The token's `iss` and `sub` must
//! both equal that entry's issuer (SC-3, DEC-G5-4). The whole set is validated when it is
//! loaded; one invalid entry refuses the whole set, and a refused or unset set refuses every
//! operator token. No key material is logged or returned in an error.
//!
//! **Binding.** Every key is *bound* (SC-1: the token must name its route's action and
//! target) except a key whose issuer is [`UNBOUND_PREDECESSOR_ISSUER`]. That exemption is
//! DEC-G5-2 option (b): the predecessor Forge_Command keeps scope `admin` until P3 gate G7,
//! which removes its key and this constant. Until then, SC-1 holds for bound keys only.

use std::collections::{BTreeMap, HashSet};
use std::fmt;

use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::Claims;
use crate::error::{AppError, ErrorCode};

/// The predecessor's issuer string. Tokens from a key with this issuer are not route-bound
/// (DEC-G5-2 (b)). Removed at P3 gate G7, with the predecessor's key.
pub const UNBOUND_PREDECESSOR_ISSUER: &str = "forge_command_local";

/// The environment variable that holds the trusted-key set.
pub const TRUSTED_KEYS_VAR: &str = "ADMIN_JWT_TRUSTED_KEYS";

/// Longest accepted token, in bytes. A larger value is refused before any parse.
const MAX_TOKEN_BYTES: usize = 16 * 1024;
/// Longest accepted `jti`.
const MAX_JTI_CHARS: usize = 128;
/// DER prefix of an Ed25519 SubjectPublicKeyInfo (RFC 8410), followed by the 32-byte key.
const ED25519_SPKI_PREFIX: [u8; 12] = [
    0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
];

/// Why a trusted-key set was refused. Each reason is a fixed category; none holds key
/// material.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum KeySetError {
    #[error("unset")]
    Unset,
    #[error("not a JSON object without duplicate members")]
    NotStrictJson,
    #[error("not a non-empty object of kid to entry")]
    NotAnObject,
    #[error("an entry has an empty kid")]
    EmptyKid,
    #[error("an entry is not exactly {{key, issuer}}")]
    EntryShape,
    #[error("an entry has no valid issuer")]
    BadIssuer,
    #[error("an entry key is not exactly one Ed25519 SPKI PEM block")]
    BadKey,
    #[error("an entry kid is not derived from its key")]
    KidNotDerived,
    #[error("one key appears in two entries")]
    KeyInTwoEntries,
}

/// The two published `kid` derivations of one raw Ed25519 key (SEC-2 as amended 2026-11-09):
/// the predecessor's `base64url(SHA-256(raw))[:8]` and the rebuild's D-10 form, the first 8
/// bytes of `SHA-256(raw)` in lower-case hex.
pub fn derived_kids(raw: &[u8; 32]) -> [String; 2] {
    let digest = Sha256::digest(raw);
    [
        URL_SAFE_NO_PAD.encode(digest)[..8].to_string(),
        hex::encode(&digest[..8]),
    ]
}

/// One verified entry of the set.
#[derive(Clone)]
struct TrustedKey {
    issuer: String,
    bound: bool,
    key: DecodingKey,
}

/// A validated trusted-key set.
#[derive(Clone)]
pub struct TrustedKeySet {
    entries: BTreeMap<String, TrustedKey>,
}

impl fmt::Debug for TrustedKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // kids and issuers are public labels; never print a key.
        f.debug_struct("TrustedKeySet")
            .field("kids", &self.entries.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl TrustedKeySet {
    /// Validate the whole set. One invalid entry refuses the whole set (SEC-3, K-1).
    pub fn parse(raw: &str) -> Result<Self, KeySetError> {
        if raw.trim().is_empty() {
            return Err(KeySetError::Unset);
        }
        let declared = match parse_strict(raw) {
            Ok(Strict::Obj(members)) if !members.is_empty() => members,
            Ok(_) => return Err(KeySetError::NotAnObject),
            Err(_) => return Err(KeySetError::NotStrictJson),
        };
        let mut entries = BTreeMap::new();
        let mut seen_keys: HashSet<[u8; 32]> = HashSet::new();
        for (kid, entry) in declared {
            if kid.is_empty() {
                return Err(KeySetError::EmptyKid);
            }
            let Strict::Obj(fields) = entry else {
                return Err(KeySetError::EntryShape);
            };
            let mut issuer = None;
            let mut pem = None;
            for (name, value) in fields {
                match (name.as_str(), value) {
                    ("issuer", Strict::Str(s)) => issuer = Some(s),
                    ("key", Strict::Str(s)) => pem = Some(s),
                    ("issuer", _) => return Err(KeySetError::BadIssuer),
                    ("key", _) => return Err(KeySetError::BadKey),
                    _ => return Err(KeySetError::EntryShape),
                }
            }
            let (Some(issuer), Some(pem)) = (issuer, pem) else {
                return Err(KeySetError::EntryShape);
            };
            if issuer.is_empty() || issuer != issuer.trim() {
                return Err(KeySetError::BadIssuer);
            }
            let raw_key = raw_ed25519_from_spki_pem(&pem).ok_or(KeySetError::BadKey)?;
            if !derived_kids(&raw_key).contains(&kid) {
                return Err(KeySetError::KidNotDerived);
            }
            if !seen_keys.insert(raw_key) {
                return Err(KeySetError::KeyInTwoEntries);
            }
            let key = DecodingKey::from_ed_der(&raw_key);
            let bound = issuer != UNBOUND_PREDECESSOR_ISSUER;
            entries.insert(kid, TrustedKey { issuer, bound, key });
        }
        Ok(Self { entries })
    }

    /// The `kid`s in the set, for a log line. Never key material.
    pub fn kids(&self) -> Vec<String> {
        self.entries.keys().cloned().collect()
    }
}

/// Extract the raw 32-byte key from exactly one Ed25519 SPKI PEM block. Accepts `\n`
/// written as the two characters `\` `n`, as an environment variable often carries it.
fn raw_ed25519_from_spki_pem(pem: &str) -> Option<[u8; 32]> {
    let pem = pem.replace("\\n", "\n");
    let pem = pem.trim();
    if pem.matches("-----BEGIN ").count() != 1 {
        return None;
    }
    let body = pem
        .strip_prefix("-----BEGIN PUBLIC KEY-----")?
        .strip_suffix("-----END PUBLIC KEY-----")?;
    let b64: String = body.chars().filter(|c| !c.is_whitespace()).collect();
    let der = STANDARD.decode(b64).ok()?;
    if der.len() != ED25519_SPKI_PREFIX.len() + 32 || der[..12] != ED25519_SPKI_PREFIX {
        return None;
    }
    let mut raw = [0u8; 32];
    raw.copy_from_slice(&der[12..]);
    Some(raw)
}

/// A token that passed signature, registered-claim, issuer and subject checks.
#[derive(Debug, Clone)]
pub struct VerifiedOperator {
    pub claims: Claims,
    pub kid: String,
    pub issuer: String,
    /// True when SC-1 route binding applies to this token's key.
    pub bound: bool,
    /// The token's `jti`, checked non-empty and bounded.
    pub jti: String,
}

/// Verifies operator tokens for one audience against the trusted-key set.
#[derive(Clone)]
pub struct OperatorVerifier {
    keys: Result<TrustedKeySet, KeySetError>,
    audience: String,
}

impl OperatorVerifier {
    pub fn new(keys: Result<TrustedKeySet, KeySetError>, audience: &str) -> Self {
        Self {
            keys,
            audience: audience.to_string(),
        }
    }

    /// True when the trusted-key set loaded. Readiness reports false as not ready, so a
    /// refused or unset `ADMIN_JWT_TRUSTED_KEYS` cannot pass a deploy health check.
    pub fn is_configured(&self) -> bool {
        self.keys.is_ok()
    }

    /// Steps 1 and 2 of the G5 order: signature and registered claims, then issuer, `sub`
    /// and audience. Binding (SC-1) and the replay claim (SC-2) are the caller's next steps.
    pub fn verify(&self, token: &str) -> Result<VerifiedOperator, AppError> {
        let Ok(keys) = &self.keys else {
            // Fail closed: an unset or refused set never accepts a token.
            return Err(AppError::new(
                ErrorCode::Internal,
                "token verification is not configured",
            ));
        };
        if token.len() > MAX_TOKEN_BYTES {
            return Err(invalid());
        }
        let header = strict_header(token)?;
        if header.alg.as_deref() != Some("EdDSA") || header.has_crit {
            return Err(invalid());
        }
        let kid = header.kid.ok_or_else(invalid)?;
        let entry = keys.entries.get(&kid).ok_or_else(invalid)?;

        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_issuer(&[entry.issuer.as_str()]);
        validation.set_audience(&[self.audience.as_str()]);
        validation.set_required_spec_claims(&["exp", "aud", "iss"]);
        validation.leeway = 0;
        let claims = decode::<Claims>(token, &entry.key, &validation)
            .map_err(|e| super::map_jwt_error(&e))?
            .claims;

        // DEC-G5-4: `sub` names the issuer of the selected entry.
        if claims.sub != entry.issuer {
            return Err(AppError::new(
                ErrorCode::InvalidToken,
                "The token subject is invalid.",
            ));
        }
        let jti = match claims.jti.as_deref() {
            Some(j) if !j.is_empty() && j.chars().count() <= MAX_JTI_CHARS => j.to_string(),
            _ => {
                return Err(AppError::new(
                    ErrorCode::InvalidToken,
                    "The token has no valid identifier.",
                ))
            }
        };
        Ok(VerifiedOperator {
            claims,
            kid,
            issuer: entry.issuer.clone(),
            bound: entry.bound,
            jti,
        })
    }
}

fn invalid() -> AppError {
    AppError::new(ErrorCode::InvalidToken, "The access token is invalid.")
}

struct StrictHeader {
    alg: Option<String>,
    kid: Option<String>,
    has_crit: bool,
}

/// Read the JOSE header with duplicate members refused, so a second `kid` or `alg` cannot
/// hide behind the first.
fn strict_header(token: &str) -> Result<StrictHeader, AppError> {
    let segment = token.split('.').next().ok_or_else(invalid)?;
    let bytes = URL_SAFE_NO_PAD.decode(segment).map_err(|_| invalid())?;
    let text = std::str::from_utf8(&bytes).map_err(|_| invalid())?;
    let Strict::Obj(members) = parse_strict(text).map_err(|_| invalid())? else {
        return Err(invalid());
    };
    let mut header = StrictHeader {
        alg: None,
        kid: None,
        has_crit: false,
    };
    for (name, value) in members {
        match (name.as_str(), value) {
            ("alg", Strict::Str(s)) => header.alg = Some(s),
            ("kid", Strict::Str(s)) => header.kid = Some(s),
            ("alg", _) | ("kid", _) => return Err(invalid()),
            // No critical extension is understood, so any is refused (RFC 7515 4.1.11).
            ("crit", _) => header.has_crit = true,
            _ => {}
        }
    }
    Ok(header)
}

/// A JSON value whose objects keep member order and refuse duplicate names at every depth.
#[derive(Debug, PartialEq)]
enum Strict {
    Null,
    Bool,
    Num,
    Str(String),
    Arr,
    Obj(Vec<(String, Strict)>),
}

fn parse_strict(text: &str) -> Result<Strict, serde_json::Error> {
    let mut de = serde_json::Deserializer::from_str(text);
    let value = Strict::deserialize(&mut de)?;
    de.end()?;
    Ok(value)
}

impl<'de> Deserialize<'de> for Strict {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Strict;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("JSON")
            }
            fn visit_unit<E>(self) -> Result<Strict, E> {
                Ok(Strict::Null)
            }
            fn visit_bool<E>(self, _: bool) -> Result<Strict, E> {
                Ok(Strict::Bool)
            }
            fn visit_i64<E>(self, _: i64) -> Result<Strict, E> {
                Ok(Strict::Num)
            }
            fn visit_u64<E>(self, _: u64) -> Result<Strict, E> {
                Ok(Strict::Num)
            }
            fn visit_f64<E>(self, _: f64) -> Result<Strict, E> {
                Ok(Strict::Num)
            }
            fn visit_str<E>(self, s: &str) -> Result<Strict, E> {
                Ok(Strict::Str(s.to_string()))
            }
            fn visit_string<E>(self, s: String) -> Result<Strict, E> {
                Ok(Strict::Str(s))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Strict, A::Error> {
                while seq.next_element::<Strict>()?.is_some() {}
                Ok(Strict::Arr)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Strict, A::Error> {
                let mut members: Vec<(String, Strict)> = Vec::new();
                while let Some(name) = map.next_key::<String>()? {
                    if members.iter().any(|(n, _)| n == &name) {
                        return Err(de::Error::custom("duplicate member"));
                    }
                    let value = map.next_value::<Strict>()?;
                    members.push((name, value));
                }
                Ok(Strict::Obj(members))
            }
        }
        deserializer.deserialize_any(V)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PEM_A: &str = "-----BEGIN PUBLIC KEY-----\nMCowBQYDK2VwAyEAYOHufz/uvwEGo+RUA4tUz7spzpbs9LKrY6FtlgydMGY=\n-----END PUBLIC KEY-----\n";

    fn raw_a() -> [u8; 32] {
        raw_ed25519_from_spki_pem(PEM_A).expect("parses")
    }

    fn set(kid: &str, issuer: &str, pem: &str) -> String {
        serde_json::json!({ kid: { "key": pem, "issuer": issuer } }).to_string()
    }

    #[test]
    fn both_published_derivations_load() {
        let [pred, rebuild] = derived_kids(&raw_a());
        assert_eq!(pred.len(), 8);
        assert_eq!(rebuild.len(), 16);
        assert!(TrustedKeySet::parse(&set(&pred, "forge_command_local", PEM_A)).is_ok());
        assert!(TrustedKeySet::parse(&set(&rebuild, "fcn_local", PEM_A)).is_ok());
    }

    #[test]
    fn escaped_newlines_are_accepted() {
        let [pred, _] = derived_kids(&raw_a());
        let escaped = PEM_A.replace('\n', "\\n");
        assert!(TrustedKeySet::parse(&set(&pred, "forge_command_local", &escaped)).is_ok());
    }

    #[test]
    fn whole_set_refusals() {
        let [pred, _] = derived_kids(&raw_a());
        assert_eq!(TrustedKeySet::parse("").unwrap_err(), KeySetError::Unset);
        assert_eq!(
            TrustedKeySet::parse("{}").unwrap_err(),
            KeySetError::NotAnObject
        );
        assert_eq!(
            TrustedKeySet::parse("[]").unwrap_err(),
            KeySetError::NotAnObject
        );
        assert_eq!(
            TrustedKeySet::parse("{").unwrap_err(),
            KeySetError::NotStrictJson
        );
        // A kid that is not derived from its key.
        assert_eq!(
            TrustedKeySet::parse(&set("AAAAAAAA", "x", PEM_A)).unwrap_err(),
            KeySetError::KidNotDerived
        );
        // No issuer, an empty issuer, a padded issuer.
        let no_issuer = serde_json::json!({ &pred: { "key": PEM_A } }).to_string();
        assert_eq!(
            TrustedKeySet::parse(&no_issuer).unwrap_err(),
            KeySetError::EntryShape
        );
        assert_eq!(
            TrustedKeySet::parse(&set(&pred, "", PEM_A)).unwrap_err(),
            KeySetError::BadIssuer
        );
        assert_eq!(
            TrustedKeySet::parse(&set(&pred, " x", PEM_A)).unwrap_err(),
            KeySetError::BadIssuer
        );
        // An extra member.
        let extra = serde_json::json!({ &pred: { "key": PEM_A, "issuer": "x", "bound": false } })
            .to_string();
        assert_eq!(
            TrustedKeySet::parse(&extra).unwrap_err(),
            KeySetError::EntryShape
        );
        // A key that is not one Ed25519 SPKI block.
        assert_eq!(
            TrustedKeySet::parse(&set(&pred, "x", "nope")).unwrap_err(),
            KeySetError::BadKey
        );
        let two = format!("{PEM_A}{PEM_A}");
        assert_eq!(
            TrustedKeySet::parse(&set(&pred, "x", &two)).unwrap_err(),
            KeySetError::BadKey
        );
    }

    #[test]
    fn duplicate_kid_and_one_key_under_two_kids_refuse_the_set() {
        let [pred, rebuild] = derived_kids(&raw_a());
        let entry = format!(
            r#"{{"key":{},"issuer":"x"}}"#,
            serde_json::to_string(PEM_A).unwrap()
        );
        let dup = format!(r#"{{"{pred}":{entry},"{pred}":{entry}}}"#);
        assert_eq!(
            TrustedKeySet::parse(&dup).unwrap_err(),
            KeySetError::NotStrictJson
        );
        let two_kids = format!(r#"{{"{pred}":{entry},"{rebuild}":{entry}}}"#);
        assert_eq!(
            TrustedKeySet::parse(&two_kids).unwrap_err(),
            KeySetError::KeyInTwoEntries
        );
    }

    #[test]
    fn duplicate_member_inside_an_entry_refuses_the_set() {
        let [pred, _] = derived_kids(&raw_a());
        let pem = serde_json::to_string(PEM_A).unwrap();
        let raw = format!(r#"{{"{pred}":{{"key":{pem},"issuer":"a","issuer":"b"}}}}"#);
        assert_eq!(
            TrustedKeySet::parse(&raw).unwrap_err(),
            KeySetError::NotStrictJson
        );
    }

    #[test]
    fn only_the_predecessor_issuer_is_unbound() {
        let [pred, rebuild] = derived_kids(&raw_a());
        let a = TrustedKeySet::parse(&set(&pred, UNBOUND_PREDECESSOR_ISSUER, PEM_A)).unwrap();
        assert!(!a.entries[&pred].bound);
        let b = TrustedKeySet::parse(&set(&rebuild, "fcn_local", PEM_A)).unwrap();
        assert!(b.entries[&rebuild].bound);
    }

    #[test]
    fn debug_output_holds_no_key_material() {
        let [pred, _] = derived_kids(&raw_a());
        let s = TrustedKeySet::parse(&set(&pred, "x", PEM_A)).unwrap();
        let text = format!("{s:?}");
        assert!(text.contains(&pred));
        assert!(!text.contains("MCow"));
    }
}
