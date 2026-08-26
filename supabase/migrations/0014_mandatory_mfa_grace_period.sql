-- 0014_mandatory_mfa_grace_period.sql
-- Makes TOTP MFA mandatory for every customer account (product decision — MFA
-- moves from opt-in to required), phased in with a 30-day grace period so the
-- existing customer base isn't locked out the moment this ships.
--
-- `mfa_grace_period_ends_at` is set only by this migration and by new-account
-- provisioning going forward. The self-service path (POST /v1/account/mfa-status)
-- and the operator "require MFA" path (POST /v1/admin/customers/{id}/mfa-required,
-- phase 4b) never touch it: self-service can only ever be called by a caller who
-- already holds an aal2 token, meaning they already have a verified factor and
-- have nothing to wait out; the operator path is deliberate incident response
-- that must not wait for a grace period by design. `CustomerContext::require_active()`
-- only ever consults this column while `mfa_required` is true, so it's inert for
-- every other case.

alter table public.customer_profiles
  add column if not exists mfa_grace_period_ends_at timestamptz;

-- Every account that doesn't already require MFA (the overwhelming majority,
-- since MFA has been opt-in until now) enters its 30-day grace period as of
-- this migration's rollout. Accounts that already require MFA (self-service
-- opt-in, or an operator-forced requirement) are untouched — they're already
-- either voluntarily enrolled or under deliberate immediate enforcement.
update public.customer_profiles
  set mfa_required = true,
      mfa_grace_period_ends_at = now() + interval '30 days'
  where mfa_required = false;

-- New accounts are mandatory-by-default from here on, each getting its own
-- 30-day grace period starting at its own signup time.
alter table public.customer_profiles
  alter column mfa_required set default true;
alter table public.customer_profiles
  alter column mfa_grace_period_ends_at set default (now() + interval '30 days');
