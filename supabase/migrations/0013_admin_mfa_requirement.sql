-- 0013_admin_mfa_requirement.sql
-- Append-only record of operator-driven mfa_required transitions (phase 4b:
-- Forge_Command's "require MFA" incident-response action). Self-service changes
-- via POST /v1/account/mfa-status are not recorded here — this table exists
-- specifically because an operator, unlike the account owner, is not proving
-- anything about their own aal2 status by flipping someone else's flag, so
-- who did it and why has to be durable and queryable (GET /v1/admin/audit
-- reads commercial_audit_events for that; this table is the same shape as
-- customer_status_history, kept alongside it for the same reason).

create table if not exists public.customer_mfa_history (
  id            uuid primary key default gen_random_uuid(),
  customer_id   uuid not null references public.customer_profiles(id) on delete cascade,
  from_required boolean not null,
  to_required   boolean not null,
  reason        text not null,
  actor_type    text not null default 'operator'
                  check (actor_type in ('system','operator')),
  actor_id      text,
  created_at    timestamptz not null default now()
);

create index if not exists idx_customer_mfa_history_customer
  on public.customer_mfa_history(customer_id, created_at);

-- Privileged: operator-audit detail, not directly customer-readable (same
-- treatment as license_revocations) — the customer already sees the effect
-- via GET /v1/account's mfa_required field.
do $$
begin
  execute 'alter table public.customer_mfa_history enable row level security;';
  execute 'alter table public.customer_mfa_history force row level security;';
end $$;
