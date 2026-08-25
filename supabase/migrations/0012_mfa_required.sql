-- 0012_mfa_required.sql
-- Tracks whether a customer has enabled TOTP MFA on their Supabase auth
-- account, so the API can require the corresponding session actually be at
-- Authenticator Assurance Level 2 (aal2) rather than trusting the client.
--
-- This is additive. It does not read or duplicate Supabase's own
-- auth.mfa_factors state; ForgeCustomer only ever learns this flag from the
-- customer themselves, and only while their own current token is already at
-- aal2 (see POST /v1/account/mfa-status) — a caller who cannot produce an
-- aal2 token (i.e. has never passed a real TOTP challenge) can never set it.

alter table public.customer_profiles
  add column if not exists mfa_required boolean not null default false;
