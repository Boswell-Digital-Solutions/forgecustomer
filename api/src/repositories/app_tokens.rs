//! Storage for Rake access-token issuance (migration 0017): application clients, the
//! installation device key used for client authentication, tenant pseudonyms and the mint
//! audit record.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// A registered application client.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AppClient {
    pub app_id: String,
    pub audience: String,
    pub product_key: String,
    pub allowed_scopes: Vec<String>,
    pub required_feature_key: String,
    pub client_auth_method: String,
    pub status: String,
}

pub async fn find_app_client(
    pool: &PgPool,
    app_id: &str,
) -> Result<Option<AppClient>, sqlx::Error> {
    sqlx::query_as::<_, AppClient>(
        r#"
        select app_id, audience, product_key, allowed_scopes, required_feature_key,
               client_auth_method, status
        from public.app_clients
        where app_id = $1
        "#,
    )
    .bind(app_id)
    .fetch_optional(pool)
    .await
}

/// Why an installation cannot authenticate the application.
#[derive(Debug, PartialEq, Eq)]
pub enum InstallationAuthError {
    /// Not the caller's installation, or not for the application's product.
    NotFound,
    /// Deactivated installation, no registered device key, or a revoked device.
    NoActiveDeviceKey,
    /// No active, unexpired license activation, or an explicit revocation applies.
    NotActivated,
}

#[derive(Debug, sqlx::FromRow)]
struct InstallationKeyRow {
    installation_status: String,
    product_key: String,
    device_id: Option<Uuid>,
    device_status: Option<String>,
    device_public_key: Option<String>,
    license_id: Option<Uuid>,
    license_status: Option<String>,
    license_expires_at: Option<DateTime<Utc>>,
}

/// The registered device public key (base64) of an activated installation that belongs to
/// `customer_id` and runs `product_key`. The rules match offline-lease issuance.
pub async fn installation_device_key(
    pool: &PgPool,
    customer_id: Uuid,
    installation_id: Uuid,
    product_key: &str,
) -> Result<Result<String, InstallationAuthError>, sqlx::Error> {
    let Some(row) = sqlx::query_as::<_, InstallationKeyRow>(
        r#"
        select i.status as installation_status,
               p.key as product_key,
               i.device_id,
               d.status as device_status,
               d.public_key as device_public_key,
               l.id as license_id,
               l.status as license_status,
               l.expires_at as license_expires_at
        from public.installations i
        join public.products p on p.id = i.product_id
        left join public.devices d on d.id = i.device_id
        left join public.license_activations a
               on a.installation_id = i.id and a.status = 'active'
        left join public.licenses l on l.id = a.license_id
        where i.id = $1 and i.customer_id = $2
        order by a.activated_at desc nulls last
        limit 1
        "#,
    )
    .bind(installation_id)
    .bind(customer_id)
    .fetch_optional(pool)
    .await?
    else {
        return Ok(Err(InstallationAuthError::NotFound));
    };

    if row.product_key != product_key {
        return Ok(Err(InstallationAuthError::NotFound));
    }
    if row.installation_status != "active" || row.device_status.as_deref() != Some("active") {
        return Ok(Err(InstallationAuthError::NoActiveDeviceKey));
    }
    let Some(public_key) = row.device_public_key else {
        return Ok(Err(InstallationAuthError::NoActiveDeviceKey));
    };
    let (Some(license_id), Some("active")) = (row.license_id, row.license_status.as_deref()) else {
        return Ok(Err(InstallationAuthError::NotActivated));
    };
    if matches!(row.license_expires_at, Some(at) if at <= Utc::now()) {
        return Ok(Err(InstallationAuthError::NotActivated));
    }
    let revoked = sqlx::query_scalar::<_, bool>(
        r#"
        select exists (
            select 1 from public.license_revocations r
            where (r.license_id = $1 and r.installation_id is null and r.device_id is null)
               or r.installation_id = $2
               or ($3::uuid is not null and r.device_id = $3)
        )
        "#,
    )
    .bind(license_id)
    .bind(installation_id)
    .bind(row.device_id)
    .fetch_one(pool)
    .await?;
    if revoked {
        return Ok(Err(InstallationAuthError::NotActivated));
    }
    Ok(Ok(public_key))
}

/// One mint, recorded in the same transaction that resolves the pseudonyms.
#[derive(Debug)]
pub struct MintRecord<'a> {
    pub jti: Uuid,
    pub customer_id: Uuid,
    pub app_id: &'a str,
    pub audience: &'a str,
    pub kid: &'a str,
    pub scopes: &'a [String],
    pub installation_id: Option<Uuid>,
    pub client_auth_method: &'a str,
    pub client_nonce: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum MintError {
    /// The client nonce was used before: a replayed proof.
    NonceReplayed,
    Db(sqlx::Error),
}

impl From<sqlx::Error> for MintError {
    fn from(e: sqlx::Error) -> Self {
        MintError::Db(e)
    }
}

/// Get or create the active pseudonym pair for (customer, audience), then write the mint
/// row. Returns `(tenant_pseudonym, user_pseudonym)`. Nothing is committed unless both
/// succeed, so a failed audit write leaves no token and no half state.
pub async fn record_mint(pool: &PgPool, mint: &MintRecord<'_>) -> Result<(Uuid, Uuid), MintError> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        insert into public.rake_tenant_pseudonyms (customer_id, audience)
        values ($1, $2)
        on conflict (customer_id, audience) where revoked_at is null do nothing
        "#,
    )
    .bind(mint.customer_id)
    .bind(mint.audience)
    .execute(&mut *tx)
    .await?;
    let (tenant, user) = sqlx::query_as::<_, (Uuid, Uuid)>(
        r#"
        select tenant_pseudonym, user_pseudonym
        from public.rake_tenant_pseudonyms
        where customer_id = $1 and audience = $2 and revoked_at is null
        "#,
    )
    .bind(mint.customer_id)
    .bind(mint.audience)
    .fetch_one(&mut *tx)
    .await?;

    let inserted = sqlx::query(
        r#"
        insert into public.app_token_mints
            (jti, customer_id, app_id, audience, kid, scopes, installation_id,
             client_auth_method, client_nonce, issued_at, expires_at)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        on conflict (client_nonce) do nothing
        "#,
    )
    .bind(mint.jti)
    .bind(mint.customer_id)
    .bind(mint.app_id)
    .bind(mint.audience)
    .bind(mint.kid)
    .bind(mint.scopes)
    .bind(mint.installation_id)
    .bind(mint.client_auth_method)
    .bind(mint.client_nonce)
    .bind(mint.issued_at)
    .bind(mint.expires_at)
    .execute(&mut *tx)
    .await?
    .rows_affected();
    if inserted == 0 {
        return Err(MintError::NonceReplayed);
    }

    tx.commit().await?;
    Ok((tenant, user))
}
