//! Rake access-token issuance (BDS-FOC-RAKE-ACCESS-ISSUER-v0.1, components 04 and 05 §2).
//!
//! ForgeCustomer exchanges a verified Supabase login, an active profile, an installation
//! device proof and an entitlement for a 300-second EdDSA token that Rake verifies with a
//! pinned public key. This module holds the pure parts: the signing key, scope resolution,
//! the device proof and the claims. Database work is in `repositories::app_tokens`.

use std::collections::BTreeSet;

use base64::engine::general_purpose::{STANDARD as B64, URL_SAFE_NO_PAD as B64URL};
use base64::Engine;
use ed25519_dalek::{Signature, VerifyingKey};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;
use uuid::Uuid;

use crate::services::signing::{Signer25519, SigningError, VerifyingKeyRing};

/// Header `typ` of every Rake access token (04 §1).
pub const TOKEN_TYP: &str = "rake-access+jwt";
/// Lifetime of every issued token, in seconds (DEC-4).
pub const TOKEN_LIFETIME_SECS: i64 = 300;
/// The approve scope. It is never combined with another scope (DEC-6).
pub const APPROVE_SCOPE: &str = "rake:approve";
/// How far a client proof's `iat` may be from server time, in seconds.
pub const PROOF_SKEW_SECS: i64 = 60;
/// Domain-separation prefix of the installation device proof message.
pub const PROOF_DOMAIN: &str = "forgecustomer.rake-access-exchange.v1";

/// The active Rake access signing key, its publication ring and the issuer id.
pub struct RakeAccessIssuer {
    signer: Signer25519,
    encoding_key: EncodingKey,
    ring: VerifyingKeyRing,
    issuer: String,
}

#[derive(Debug, thiserror::Error)]
pub enum RakeAccessKeyError {
    #[error("RAKE_ACCESS_SIGNING_PRIVATE_KEY: {0}")]
    Signing(#[from] SigningError),
    #[error("RAKE_ACCESS_SIGNING_PRIVATE_KEY must differ from the entitlement signing key")]
    SameAsEntitlementKey,
    #[error("RAKE_ACCESS_VERIFY_PUBLIC_KEYS: {0}")]
    VerifyKeys(String),
    #[error("RAKE_ACCESS_ISSUER must not be empty")]
    EmptyIssuer,
}

impl RakeAccessIssuer {
    /// Build the issuer from a base64 Ed25519 seed. `entitlement_public_key` is the active
    /// entitlement key: one key, one purpose, so an equal key fails start-up (AC-03).
    pub fn new(
        key_id: &str,
        b64_seed: &str,
        verify_keys: &[(String, String)],
        issuer: &str,
        entitlement_public_key: &str,
    ) -> Result<Self, RakeAccessKeyError> {
        if issuer.trim().is_empty() {
            return Err(RakeAccessKeyError::EmptyIssuer);
        }
        let signer = Signer25519::from_base64_seed(key_id, b64_seed)?;
        if signer.public_key_base64() == entitlement_public_key {
            return Err(RakeAccessKeyError::SameAsEntitlementKey);
        }
        let seed = B64
            .decode(b64_seed.trim())
            .map_err(|e| SigningError::InvalidKey(e.to_string()))?;
        // PKCS#8 v1 wrapping of an Ed25519 seed (RFC 8410): fixed 16-byte prefix + seed.
        let mut pkcs8 = hex::decode("302e020100300506032b657004220420")
            .map_err(|e| SigningError::InvalidKey(e.to_string()))?;
        pkcs8.extend_from_slice(&seed);
        let encoding_key = EncodingKey::from_ed_der(&pkcs8);

        let mut ring = VerifyingKeyRing::new();
        ring.add_signer(&signer);
        let active = signer.public_key_base64();
        for (id, public_key) in verify_keys {
            let mut probe = VerifyingKeyRing::new();
            probe
                .add_base64(id.clone(), public_key)
                .map_err(|e| RakeAccessKeyError::VerifyKeys(format!("key_id {id}: {e}")))?;
            if id == signer.key_id() {
                if public_key.trim() != active {
                    return Err(RakeAccessKeyError::VerifyKeys(format!(
                        "key_id {id} is the active signing key id with a different public key"
                    )));
                }
                continue;
            }
            if public_key.trim() == entitlement_public_key {
                return Err(RakeAccessKeyError::VerifyKeys(format!(
                    "key_id {id} is the entitlement signing key"
                )));
            }
            ring.add_base64(id.clone(), public_key)
                .map_err(|e| RakeAccessKeyError::VerifyKeys(format!("key_id {id}: {e}")))?;
        }
        Ok(Self {
            signer,
            encoding_key,
            ring,
            issuer: issuer.trim().to_string(),
        })
    }

    pub fn key_id(&self) -> &str {
        self.signer.key_id()
    }

    pub fn issuer(&self) -> &str {
        &self.issuer
    }

    /// Published keys, active first, then retained and standby keys by id.
    pub fn published(&self) -> Vec<PublishedKey> {
        let mut keys: Vec<PublishedKey> = self
            .ring
            .published()
            .into_iter()
            .map(|(kid, public_key)| PublishedKey {
                status: if kid == self.signer.key_id() {
                    "active"
                } else {
                    "verify_only"
                },
                kid,
                alg: "EdDSA",
                public_key,
            })
            .collect();
        keys.sort_by(|a, b| (a.status != "active", &a.kid).cmp(&(b.status != "active", &b.kid)));
        keys
    }

    /// Sign the claims as a compact JWT with header `{alg: EdDSA, typ, kid}`.
    pub fn sign(&self, claims: &RakeAccessClaims) -> Result<String, jsonwebtoken::errors::Error> {
        let mut header = Header::new(Algorithm::EdDSA);
        header.typ = Some(TOKEN_TYP.to_string());
        header.kid = Some(self.signer.key_id().to_string());
        encode(&header, claims, &self.encoding_key)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PublishedKey {
    pub kid: String,
    pub alg: &'static str,
    pub public_key: String,
    pub status: &'static str,
}

/// Claims of 04 §2. Field names are the contract; do not rename.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RakeAccessClaims {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub tenant_id: String,
    pub user_id: String,
    pub app_id: String,
    pub scopes: Vec<String>,
    #[serde(rename = "type")]
    pub token_type: &'static str,
    pub iat: i64,
    pub exp: i64,
    pub jti: String,
}

impl RakeAccessClaims {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        issuer: &str,
        audience: &str,
        tenant_pseudonym: Uuid,
        user_pseudonym: Uuid,
        app_id: &str,
        scopes: Vec<String>,
        iat: i64,
        jti: Uuid,
    ) -> Self {
        Self {
            iss: issuer.to_string(),
            aud: audience.to_string(),
            sub: user_pseudonym.to_string(),
            tenant_id: tenant_pseudonym.to_string(),
            user_id: user_pseudonym.to_string(),
            app_id: app_id.to_string(),
            scopes,
            token_type: "access",
            iat,
            exp: iat + TOKEN_LIFETIME_SECS,
            jti: jti.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScopeError {
    /// A requested scope is not in the application's list.
    NotAllowed(String),
    /// `rake:approve` was combined with another scope (DEC-6).
    ApproveNotAlone,
    /// The request named no scope, or the same scope twice.
    Invalid,
}

/// Resolve the scopes to grant. With no request, grant every allowed scope except
/// `rake:approve`, which a caller must request alone (DEC-6). The result is sorted.
pub fn resolve_scopes(
    requested: Option<&[String]>,
    allowed: &[String],
) -> Result<Vec<String>, ScopeError> {
    let allowed: BTreeSet<&str> = allowed.iter().map(String::as_str).collect();
    let granted: Vec<String> = match requested {
        None => allowed
            .iter()
            .filter(|s| **s != APPROVE_SCOPE)
            .map(|s| s.to_string())
            .collect(),
        Some(req) => {
            let unique: BTreeSet<&str> = req.iter().map(String::as_str).collect();
            if req.is_empty() || unique.len() != req.len() {
                return Err(ScopeError::Invalid);
            }
            if let Some(bad) = unique.iter().find(|s| !allowed.contains(**s)) {
                return Err(ScopeError::NotAllowed(bad.to_string()));
            }
            unique.into_iter().map(str::to_string).collect()
        }
    };
    if granted.is_empty() {
        return Err(ScopeError::Invalid);
    }
    if granted.iter().any(|s| s == APPROVE_SCOPE) && granted.len() != 1 {
        return Err(ScopeError::ApproveNotAlone);
    }
    Ok(granted)
}

/// The bytes the installation device key signs (DEC-2 = A). Lines joined with `\n`:
/// domain, app_id, installation_id, scopes as sent (joined with `,`, empty if omitted),
/// iat, nonce. Every value is validated before this is built, so no value holds `\n`.
pub fn proof_message(
    app_id: &str,
    installation_id: Uuid,
    requested_scopes: Option<&[String]>,
    iat: i64,
    nonce: Uuid,
) -> Vec<u8> {
    let scopes = requested_scopes.map(|s| s.join(",")).unwrap_or_default();
    format!("{PROOF_DOMAIN}\n{app_id}\n{installation_id}\n{scopes}\n{iat}\n{nonce}").into_bytes()
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProofError {
    /// `iat` is outside the allowed skew.
    Stale,
    /// The registered device key or the signature does not decode.
    Malformed,
    /// The signature does not verify.
    BadSignature,
}

/// Verify a device proof: `iat` within skew of `now`, then an Ed25519 signature over
/// `message` by the registered device key (both base64).
pub fn verify_proof(
    device_public_key_b64: &str,
    message: &[u8],
    signature_b64: &str,
    iat: i64,
    now: i64,
) -> Result<(), ProofError> {
    if (now - iat).abs() > PROOF_SKEW_SECS {
        return Err(ProofError::Stale);
    }
    let key: [u8; 32] = B64
        .decode(device_public_key_b64.trim())
        .ok()
        .and_then(|raw| raw.try_into().ok())
        .ok_or(ProofError::Malformed)?;
    let key = VerifyingKey::from_bytes(&key).map_err(|_| ProofError::Malformed)?;
    let sig: [u8; 64] = decode_signature(signature_b64).ok_or(ProofError::Malformed)?;
    key.verify_strict(message, &Signature::from_bytes(&sig))
        .map_err(|_| ProofError::BadSignature)
}

fn decode_signature(raw: &str) -> Option<[u8; 64]> {
    let raw = raw.trim();
    B64.decode(raw)
        .or_else(|_| B64URL.decode(raw))
        .ok()
        .and_then(|bytes| bytes.try_into().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};

    fn seed(b: u8) -> String {
        B64.encode([b; 32])
    }

    fn entitlement_pub(b: u8) -> String {
        Signer25519::from_base64_seed("e", &seed(b))
            .unwrap()
            .public_key_base64()
    }

    fn issuer() -> RakeAccessIssuer {
        RakeAccessIssuer::new(
            "rake-access-1",
            &seed(7),
            &[],
            "bds-forgecustomer",
            &entitlement_pub(1),
        )
        .unwrap()
    }

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn the_entitlement_key_cannot_be_the_rake_key() {
        let err = RakeAccessIssuer::new("rake-access-1", &seed(1), &[], "iss", &entitlement_pub(1))
            .err()
            .expect("must fail");
        assert!(matches!(err, RakeAccessKeyError::SameAsEntitlementKey));
    }

    #[test]
    fn the_entitlement_key_cannot_be_a_rake_verify_key() {
        let verify = vec![("old".to_string(), entitlement_pub(1))];
        let err = RakeAccessIssuer::new(
            "rake-access-1",
            &seed(7),
            &verify,
            "iss",
            &entitlement_pub(1),
        )
        .err()
        .expect("must fail");
        assert!(err.to_string().contains("entitlement signing key"), "{err}");
    }

    #[test]
    fn an_empty_issuer_fails() {
        let err = RakeAccessIssuer::new("k", &seed(7), &[], " ", &entitlement_pub(1))
            .err()
            .expect("must fail");
        assert!(matches!(err, RakeAccessKeyError::EmptyIssuer));
    }

    #[test]
    fn published_keys_list_active_first_and_public_halves_only() {
        let standby = Signer25519::from_base64_seed("rake-access-2", &seed(8)).unwrap();
        let verify = vec![("rake-access-2".to_string(), standby.public_key_base64())];
        let issuer = RakeAccessIssuer::new(
            "rake-access-1",
            &seed(7),
            &verify,
            "iss",
            &entitlement_pub(1),
        )
        .unwrap();
        let keys = issuer.published();
        assert_eq!(keys.len(), 2);
        assert_eq!(
            (keys[0].kid.as_str(), keys[0].status),
            ("rake-access-1", "active")
        );
        assert_eq!(
            (keys[1].kid.as_str(), keys[1].status),
            ("rake-access-2", "verify_only")
        );
        assert!(keys
            .iter()
            .all(|k| B64.decode(&k.public_key).unwrap().len() == 32));
    }

    #[test]
    fn scopes_default_excludes_approve_and_approve_stands_alone() {
        let allowed = s(&["rake:read", "rake:execute", "rake:approve"]);
        assert_eq!(
            resolve_scopes(None, &allowed).unwrap(),
            s(&["rake:execute", "rake:read"])
        );
        assert_eq!(
            resolve_scopes(Some(&s(&["rake:approve"])), &allowed).unwrap(),
            s(&["rake:approve"])
        );
        assert_eq!(
            resolve_scopes(Some(&s(&["rake:read", "rake:approve"])), &allowed),
            Err(ScopeError::ApproveNotAlone)
        );
        assert_eq!(
            resolve_scopes(Some(&s(&["rake:admin"])), &allowed),
            Err(ScopeError::NotAllowed("rake:admin".into()))
        );
        assert_eq!(
            resolve_scopes(Some(&[]), &allowed),
            Err(ScopeError::Invalid)
        );
        assert_eq!(
            resolve_scopes(Some(&s(&["rake:read", "rake:read"])), &allowed),
            Err(ScopeError::Invalid)
        );
    }

    #[test]
    fn a_signed_token_matches_the_contract_and_verifies_with_the_published_key() {
        let issuer = issuer();
        let tenant = Uuid::new_v4();
        let user = Uuid::new_v4();
        let jti = Uuid::new_v4();
        let claims = RakeAccessClaims::new(
            issuer.issuer(),
            "rake.application",
            tenant,
            user,
            "authorforge",
            s(&["rake:execute", "rake:read"]),
            1_794_000_000,
            jti,
        );
        let token = issuer.sign(&claims).unwrap();

        let header = decode_header(&token).unwrap();
        assert_eq!(header.alg, Algorithm::EdDSA);
        assert_eq!(header.typ.as_deref(), Some(TOKEN_TYP));
        assert_eq!(header.kid.as_deref(), Some("rake-access-1"));

        let public = B64.decode(&issuer.published()[0].public_key).unwrap();
        let mut der = hex::decode("302a300506032b6570032100").unwrap();
        der.extend_from_slice(&public);
        let pem = format!(
            "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n",
            B64.encode(der)
        );
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.validate_exp = false;
        validation.set_audience(&["rake.application"]);
        validation.set_issuer(&["bds-forgecustomer"]);
        let decoded = decode::<serde_json::Value>(
            &token,
            &DecodingKey::from_ed_pem(pem.as_bytes()).unwrap(),
            &validation,
        )
        .unwrap()
        .claims;
        assert_eq!(decoded["type"], "access");
        assert_eq!(decoded["sub"], user.to_string());
        assert_eq!(decoded["user_id"], user.to_string());
        assert_eq!(decoded["tenant_id"], tenant.to_string());
        assert_eq!(
            decoded["exp"].as_i64().unwrap() - decoded["iat"].as_i64().unwrap(),
            300
        );
        assert_eq!(decoded["jti"], jti.to_string());
        let names: BTreeSet<&str> = decoded
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            names,
            BTreeSet::from([
                "iss",
                "aud",
                "sub",
                "tenant_id",
                "user_id",
                "app_id",
                "scopes",
                "type",
                "iat",
                "exp",
                "jti"
            ]),
            "the token carries exactly the contract claims"
        );
    }

    fn device() -> (SigningKey, String) {
        let key = SigningKey::from_bytes(&[42u8; 32]);
        let public = B64.encode(key.verifying_key().to_bytes());
        (key, public)
    }

    #[test]
    fn a_device_proof_verifies_and_a_changed_field_does_not() {
        let (key, public) = device();
        let inst = Uuid::new_v4();
        let nonce = Uuid::new_v4();
        let scopes = s(&["rake:read"]);
        let msg = proof_message("authorforge", inst, Some(&scopes), 1000, nonce);
        let sig = B64.encode(key.sign(&msg).to_bytes());
        assert_eq!(verify_proof(&public, &msg, &sig, 1000, 1030), Ok(()));

        let other = proof_message(
            "authorforge",
            inst,
            Some(&s(&["rake:approve"])),
            1000,
            nonce,
        );
        assert_eq!(
            verify_proof(&public, &other, &sig, 1000, 1030),
            Err(ProofError::BadSignature)
        );
    }

    #[test]
    fn a_stale_or_future_proof_is_refused() {
        let (key, public) = device();
        let msg = proof_message("authorforge", Uuid::new_v4(), None, 1000, Uuid::new_v4());
        let sig = B64.encode(key.sign(&msg).to_bytes());
        assert_eq!(
            verify_proof(&public, &msg, &sig, 1000, 1061),
            Err(ProofError::Stale)
        );
        assert_eq!(
            verify_proof(&public, &msg, &sig, 1000, 939),
            Err(ProofError::Stale)
        );
    }

    #[test]
    fn a_malformed_key_or_signature_is_refused() {
        let (_, public) = device();
        let msg = b"x";
        assert_eq!(
            verify_proof("AAAA", msg, &B64.encode([0u8; 64]), 0, 0),
            Err(ProofError::Malformed)
        );
        assert_eq!(
            verify_proof(&public, msg, "short", 0, 0),
            Err(ProofError::Malformed)
        );
    }
}
