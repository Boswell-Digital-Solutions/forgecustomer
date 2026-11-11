-- 0017_rake_access_issuer.sql
-- BDS-FOC-RAKE-ACCESS-ISSUER-v0.1 WP-02: ForgeCustomer issues short-lived Rake access
-- tokens to registered applications (plan component 04 §5).
--
-- All three tables are server-only. Row level security is enabled and forced with no
-- customer policy, so a customer JWT reaches no row through PostgREST. The API reaches
-- them through its own database role.

-- ---------------------------------------------------------------------------
-- app_clients — applications that may exchange a login for a Rake token
-- ---------------------------------------------------------------------------
create table if not exists public.app_clients (
  app_id               text primary key,
  audience             text not null,
  product_key          text not null,
  allowed_scopes       text[] not null,
  required_feature_key text not null,
  client_auth_method   text not null
                         check (client_auth_method in ('installation_device_key')),
  status               text not null default 'active'
                         check (status in ('active','disabled')),
  created_at           timestamptz not null default now(),
  check (cardinality(allowed_scopes) > 0)
);

-- DEC-2 = A (installation device key); DEC-6 = A (rake:approve allowed, alone, single-use).
insert into public.app_clients
  (app_id, audience, product_key, allowed_scopes, required_feature_key, client_auth_method)
values
  ('authorforge', 'rake.application', 'authorforge',
   array['rake:read','rake:execute','rake:approve'],
   'authorforge.cloud.enabled', 'installation_device_key')
on conflict (app_id) do update set
  audience = excluded.audience,
  product_key = excluded.product_key,
  allowed_scopes = excluded.allowed_scopes,
  required_feature_key = excluded.required_feature_key,
  client_auth_method = excluded.client_auth_method;

-- ---------------------------------------------------------------------------
-- rake_tenant_pseudonyms — the ids Rake sees (DEC-3 = A). Random, never derived from
-- customer_id, never sent to DataForge. Revocation leaves the row and a new mint
-- creates a fresh one.
-- ---------------------------------------------------------------------------
create table if not exists public.rake_tenant_pseudonyms (
  id               uuid primary key default gen_random_uuid(),
  customer_id      uuid not null references public.customer_profiles(id) on delete cascade,
  audience         text not null,
  tenant_pseudonym uuid not null default gen_random_uuid() unique,
  user_pseudonym   uuid not null default gen_random_uuid() unique,
  created_at       timestamptz not null default now(),
  revoked_at       timestamptz
);

create unique index if not exists uq_rake_tenant_pseudonyms_active
  on public.rake_tenant_pseudonyms (customer_id, audience) where revoked_at is null;

-- ---------------------------------------------------------------------------
-- app_token_mints — one audit row per issued token. No token value, no request body.
-- client_nonce is unique, so a replayed client proof cannot mint twice.
-- ---------------------------------------------------------------------------
create table if not exists public.app_token_mints (
  jti                uuid primary key,
  customer_id        uuid not null references public.customer_profiles(id) on delete cascade,
  app_id             text not null references public.app_clients(app_id),
  audience           text not null,
  kid                text not null,
  scopes             text[] not null,
  installation_id    uuid references public.installations(id) on delete set null,
  client_auth_method text not null,
  client_nonce       uuid not null unique,
  issued_at          timestamptz not null,
  expires_at         timestamptz not null,
  check (expires_at > issued_at)
);

create index if not exists idx_app_token_mints_customer
  on public.app_token_mints (customer_id, issued_at desc);

alter table public.app_clients enable row level security;
alter table public.app_clients force row level security;
alter table public.rake_tenant_pseudonyms enable row level security;
alter table public.rake_tenant_pseudonyms force row level security;
alter table public.app_token_mints enable row level security;
alter table public.app_token_mints force row level security;
