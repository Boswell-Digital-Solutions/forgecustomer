//! Purpose-limited customer projection. No mutations and no direct Supabase credentials.
use crate::{
    auth::bearer_token,
    error::{AppError, ErrorCode},
    repositories::admin::{self, AdminCustomerRow, CustomerFilter},
    state::AppState,
};
use axum::{
    async_trait,
    extract::{FromRequestParts, Query, State},
    http::{header, request::Parts},
    Json,
};
use serde::{Deserialize, Serialize};

pub struct PostureReader;

#[async_trait]
impl FromRequestParts<AppState> for PostureReader {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let token = bearer_token(
            parts
                .headers
                .get(header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok()),
        )?;
        let claims = state.posture_validator.validate(token)?;
        if claims.sub.trim().is_empty() || claims.scope.as_deref() != Some("posture:read") {
            return Err(AppError::new(
                ErrorCode::Forbidden,
                "Customer posture read scope required.",
            ));
        }
        Ok(Self)
    }
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Page {
    #[serde(default)]
    pub offset: u32,
}

#[derive(Serialize)]
pub struct CustomerPosture {
    pub observed_at: chrono::DateTime<chrono::Utc>,
    pub customers: Vec<CustomerSummary>,
    pub next_offset: Option<u32>,
}

/// Explicit projection: future fields on the admin row do not expand this contract.
#[derive(Serialize)]
pub struct CustomerSummary {
    pub id: uuid::Uuid,
    pub customer_type: String,
    pub display_name: Option<String>,
    pub status: String,
    pub country_code: Option<String>,
    pub primary_email: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
impl From<AdminCustomerRow> for CustomerSummary {
    fn from(row: AdminCustomerRow) -> Self {
        Self {
            id: row.id,
            customer_type: row.customer_type,
            display_name: row.display_name,
            status: row.status,
            country_code: row.country_code,
            primary_email: row.primary_email,
            created_at: row.created_at,
        }
    }
}

pub async fn customers(
    _reader: PostureReader,
    State(state): State<AppState>,
    Query(page): Query<Page>,
) -> Result<impl axum::response::IntoResponse, AppError> {
    if page.offset > 100_000 {
        return Err(AppError::new(
            ErrorCode::BadRequest,
            "Page offset exceeds supported range.",
        ));
    }
    // One bounded projection; the extra row signals another page without counting all customers.
    let mut customers = admin::list_customers(
        &state.pool,
        CustomerFilter {
            limit: 26,
            offset: i64::from(page.offset),
            ..Default::default()
        },
    )
    .await
    .map_err(|_| {
        AppError::new(
            ErrorCode::ServiceUnavailable,
            "Customer posture is unavailable.",
        )
    })?;
    let next_offset = (customers.len() > 25 && page.offset <= 99_975).then_some(page.offset + 25);
    customers.truncate(25);
    Ok((
        [(header::CACHE_CONTROL, "no-store")],
        Json(CustomerPosture {
            observed_at: chrono::Utc::now(),
            customers: customers.into_iter().map(CustomerSummary::from).collect(),
            next_offset,
        }),
    ))
}
