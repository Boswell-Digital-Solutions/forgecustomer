//! SC-1: a bound key's operator token is bound to the action and target it authorizes.
//!
//! Each operator route maps to exactly one action scope. A bound token must carry that scope
//! and nothing else, and its `target` claim must equal the route's path parameters joined with
//! `/`, in route order. A route with no path parameter needs no `target` claim, and refuses one.
//! A route that is not in [`ROUTES`] refuses every bound token.
//!
//! The predecessor's key is exempt until P3 gate G7 (DEC-G5-2 (b),
//! [`super::operator::UNBOUND_PREDECESSOR_ISSUER`]). SC-1 holds for bound keys only.

use axum::http::Method;

use crate::error::{AppError, ErrorCode};

use super::Claims;

/// One operator route: method, the router's path template, and the one action scope it
/// accepts from a bound token.
pub struct RouteBinding {
    pub method: &'static str,
    pub path: &'static str,
    pub action: &'static str,
}

const fn r(method: &'static str, path: &'static str, action: &'static str) -> RouteBinding {
    RouteBinding {
        method,
        path,
        action,
    }
}

/// Every operator route. `tests::every_operator_route_is_bound` keeps this table and the
/// router in step.
pub const ROUTES: &[RouteBinding] = &[
    r("GET", "/v1/posture/customers", "posture:read"),
    r("GET", "/v1/admin/customers", "customers:read"),
    r(
        "POST",
        "/v1/admin/customers/:id/suspend",
        "customers:suspend",
    ),
    r(
        "POST",
        "/v1/admin/customers/:id/restore",
        "customers:restore",
    ),
    r(
        "POST",
        "/v1/admin/customers/:id/mfa-required",
        "customers:require_mfa",
    ),
    r(
        "POST",
        "/v1/admin/subscriptions/:id/resync",
        "subscriptions:resync",
    ),
    r("POST", "/v1/admin/licenses", "licenses:issue"),
    r("POST", "/v1/admin/licenses/:id/revoke", "licenses:revoke"),
    r(
        "POST",
        "/v1/admin/entitlements/override",
        "entitlements:override",
    ),
    r("POST", "/v1/admin/usage/adjust", "usage:adjust"),
    r(
        "GET",
        "/v1/admin/products/authorforge/commercial-policy",
        "commercial_policy:read",
    ),
    r(
        "POST",
        "/v1/admin/products/authorforge/commercial-policy/versions",
        "commercial_policy:publish",
    ),
    r("GET", "/v1/admin/audit", "audit:read"),
    r("GET", "/v1/admin/fleets", "fleets:read"),
    r("GET", "/v1/admin/fleets/:id", "fleets:read"),
    r("POST", "/v1/admin/fleets/:id/policy", "fleets:set_policy"),
    r("GET", "/v1/admin/releases", "releases:read"),
    r("POST", "/v1/admin/releases", "releases:create"),
    r("GET", "/v1/admin/releases/:id", "releases:read"),
    r(
        "POST",
        "/v1/admin/releases/:id/artifacts",
        "releases:register_artifact",
    ),
    r(
        "POST",
        "/v1/admin/releases/:id/validate",
        "releases:validate",
    ),
    r("POST", "/v1/admin/releases/:id/publish", "releases:publish"),
    // S-4, a stop route of the T-13 amendment.
    r("POST", "/v1/admin/releases/:id/block", "releases:block"),
    r(
        "POST",
        "/v1/admin/update-campaigns",
        "update_campaigns:create",
    ),
    r(
        "GET",
        "/v1/admin/update-campaigns/:id",
        "update_campaigns:read",
    ),
    // S-1, a stop route.
    r(
        "POST",
        "/v1/admin/update-campaigns/:id/pause",
        "update_campaigns:pause",
    ),
    r(
        "POST",
        "/v1/admin/update-campaigns/:id/resume",
        "update_campaigns:resume",
    ),
    // S-2, a stop route.
    r(
        "POST",
        "/v1/admin/update-campaigns/:id/revoke",
        "update_campaigns:revoke",
    ),
    r(
        "POST",
        "/v1/admin/update-campaigns/:id/rollout",
        "update_campaigns:rollout",
    ),
    // S-3, a stop route.
    r(
        "POST",
        "/v1/admin/update-campaigns/:campaign_id/holds",
        "update_campaigns:add_hold",
    ),
    r(
        "DELETE",
        "/v1/admin/update-campaigns/:campaign_id/holds/:fleet_id",
        "update_campaigns:remove_hold",
    ),
    r("GET", "/v1/admin/update-failures", "update_failures:read"),
    // S-5, a stop route.
    r(
        "POST",
        "/v1/admin/release-artifacts/:id/quarantine",
        "release_artifacts:quarantine",
    ),
    r(
        "GET",
        "/v1/admin/deletion-requests",
        "deletion_requests:read",
    ),
    r(
        "POST",
        "/v1/admin/deletion-requests/:id/advance",
        "deletion_requests:advance",
    ),
    r(
        "POST",
        "/v1/admin/deletion-requests/:id/reject",
        "deletion_requests:reject",
    ),
    r(
        "POST",
        "/v1/admin/deletion-requests/:id/execute",
        "deletion_requests:execute",
    ),
];

/// The binding for a method and router path template, if the route is an operator route.
pub fn route_binding(method: &Method, path: &str) -> Option<&'static RouteBinding> {
    ROUTES
        .iter()
        .find(|b| b.method == method.as_str() && b.path == path)
}

/// The path parameter names of a template, in order (`:id` and `:campaign_id` style).
///
/// The target joins parameter values with `/`. A percent-encoded `%2F` decodes into a value, so
/// on a two-parameter route `("a/b", "c")` and `("a", "b/c")` give the same target. That is
/// harmless while both parameters are UUIDs, which every current two-parameter route uses.
fn param_names(path: &str) -> impl Iterator<Item = &str> {
    path.split('/').filter_map(|seg| seg.strip_prefix(':'))
}

/// Check a bound token against its route. `params` are the request's path parameters as the
/// router matched them.
pub fn check_bound(
    method: &Method,
    matched_path: Option<&str>,
    params: &[(String, String)],
    claims: &Claims,
) -> Result<(), AppError> {
    // SC-1R: a role can never stand in for the route check.
    if claims.roles.is_some() {
        return Err(refused("The token must not carry a roles claim."));
    }
    let binding = matched_path
        .and_then(|p| route_binding(method, p))
        .ok_or_else(|| refused("This route accepts no bound operator token."))?;
    // Exactly one scope token, equal to the route's action. `admin` and `*` never match.
    if claims.scope.as_deref() != Some(binding.action) {
        return Err(refused("The token scope does not authorize this action."));
    }
    let mut expected = Vec::new();
    for name in param_names(binding.path) {
        let value = params
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.as_str())
            .ok_or_else(|| refused("The route target is missing."))?;
        expected.push(value);
    }
    let expected = (!expected.is_empty()).then(|| expected.join("/"));
    if claims.target != expected {
        return Err(refused("The token target does not match this request."));
    }
    Ok(())
}

fn refused(message: &str) -> AppError {
    AppError::new(ErrorCode::Forbidden, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claims(scope: Option<&str>, target: Option<&str>, roles: Option<Vec<&str>>) -> Claims {
        Claims {
            sub: "fcn_local".into(),
            email: None,
            role: None,
            aal: None,
            roles: roles.map(|r| r.into_iter().map(String::from).collect()),
            scope: scope.map(String::from),
            exp: 0,
            jti: Some("j".into()),
            target: target.map(String::from),
        }
    }

    fn p(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect()
    }

    const PAUSE: &str = "/v1/admin/update-campaigns/:id/pause";

    #[test]
    fn right_action_and_target_pass() {
        let c = claims(Some("update_campaigns:pause"), Some("c1"), None);
        assert!(check_bound(&Method::POST, Some(PAUSE), &p(&[("id", "c1")]), &c).is_ok());
    }

    #[test]
    fn another_action_or_target_fails() {
        let other_action = claims(Some("update_campaigns:resume"), Some("c1"), None);
        assert!(check_bound(
            &Method::POST,
            Some(PAUSE),
            &p(&[("id", "c1")]),
            &other_action
        )
        .is_err());
        let other_target = claims(Some("update_campaigns:pause"), Some("c2"), None);
        assert!(check_bound(
            &Method::POST,
            Some(PAUSE),
            &p(&[("id", "c1")]),
            &other_target
        )
        .is_err());
        let no_target = claims(Some("update_campaigns:pause"), None, None);
        assert!(check_bound(&Method::POST, Some(PAUSE), &p(&[("id", "c1")]), &no_target).is_err());
    }

    #[test]
    fn admin_scope_and_extra_scopes_fail() {
        for scope in [
            "admin",
            "*",
            "update_campaigns:pause admin",
            "update_campaigns:pause ",
        ] {
            let c = claims(Some(scope), Some("c1"), None);
            assert!(
                check_bound(&Method::POST, Some(PAUSE), &p(&[("id", "c1")]), &c).is_err(),
                "{scope}"
            );
        }
    }

    #[test]
    fn any_roles_claim_fails() {
        for roles in [vec!["admin"], vec![]] {
            let c = claims(Some("update_campaigns:pause"), Some("c1"), Some(roles));
            assert!(check_bound(&Method::POST, Some(PAUSE), &p(&[("id", "c1")]), &c).is_err());
        }
    }

    #[test]
    fn a_route_without_parameters_refuses_a_target() {
        let c = claims(Some("customers:read"), Some("x"), None);
        assert!(check_bound(&Method::GET, Some("/v1/admin/customers"), &[], &c).is_err());
        let ok = claims(Some("customers:read"), None, None);
        assert!(check_bound(&Method::GET, Some("/v1/admin/customers"), &[], &ok).is_ok());
    }

    #[test]
    fn two_parameters_join_in_route_order() {
        let path = "/v1/admin/update-campaigns/:campaign_id/holds/:fleet_id";
        let params = p(&[("fleet_id", "f9"), ("campaign_id", "c1")]);
        let ok = claims(Some("update_campaigns:remove_hold"), Some("c1/f9"), None);
        assert!(check_bound(&Method::DELETE, Some(path), &params, &ok).is_ok());
        let swapped = claims(Some("update_campaigns:remove_hold"), Some("f9/c1"), None);
        assert!(check_bound(&Method::DELETE, Some(path), &params, &swapped).is_err());
    }

    #[test]
    fn an_unlisted_route_or_method_fails() {
        let c = claims(Some("customers:read"), None, None);
        assert!(check_bound(&Method::GET, Some("/v1/admin/unknown"), &[], &c).is_err());
        assert!(check_bound(&Method::DELETE, Some("/v1/admin/customers"), &[], &c).is_err());
        assert!(check_bound(&Method::GET, None, &[], &c).is_err());
    }

    #[test]
    fn each_action_is_unique_per_method_and_path_and_no_route_is_listed_twice() {
        let mut seen = std::collections::HashSet::new();
        for b in ROUTES {
            assert!(
                seen.insert((b.method, b.path)),
                "{} {} listed twice",
                b.method,
                b.path
            );
            assert!(!b.action.is_empty() && !b.action.contains(' ') && b.action != "admin");
        }
    }

    /// Every operator route in the router source has a binding, and every binding names a
    /// routed path. A new admin route without a binding fails here.
    #[test]
    fn every_operator_route_is_bound() {
        let source = include_str!("../routes/mod.rs");
        let mut routed = std::collections::HashSet::new();
        let mut rest = source;
        while let Some(i) = rest.find(".route(") {
            rest = &rest[i + ".route(".len()..];
            let Some(start) = rest.find('"') else { break };
            let after = &rest[start + 1..];
            let Some(end) = after.find('"') else { break };
            let path = &after[..end];
            // The handler list runs to the next `.route(` or the end of the statement.
            let tail = &after[end..];
            let body_end = [tail.find(".route("), tail.find(';')]
                .into_iter()
                .flatten()
                .min()
                .unwrap_or(tail.len());
            let handlers = &after[end..end + body_end];
            if path.starts_with("/v1/admin") || path.starts_with("/v1/posture") {
                for (method, needle) in [
                    ("GET", "get("),
                    ("POST", "post("),
                    ("DELETE", "delete("),
                    ("PUT", "put("),
                    ("PATCH", "patch("),
                ] {
                    if handlers.contains(needle) {
                        routed.insert((method, path.to_string()));
                    }
                }
            }
        }
        assert!(
            !routed.is_empty(),
            "no operator route found: the parser is broken"
        );
        let bound: std::collections::HashSet<_> = ROUTES
            .iter()
            .map(|b| (b.method, b.path.to_string()))
            .collect();
        let missing: Vec<_> = routed.difference(&bound).collect();
        let stale: Vec<_> = bound.difference(&routed).collect();
        assert!(missing.is_empty(), "routes with no binding: {missing:?}");
        assert!(stale.is_empty(), "bindings with no route: {stale:?}");
    }
}
