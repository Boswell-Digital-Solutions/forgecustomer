-- 0018_operator_token_replay.sql
-- P3 gate G5, SC-2 (DEC-G5-3 option (a)): an operator token is accepted once.
--
-- One row per accepted operator token, unique on the issuer and the jti together. The API
-- claims a row with one atomic insert that does nothing on a duplicate, after the token's
-- signature, claims, issuer, subject, audience, action and target checks pass, and before the
-- request runs. A second request with the same token finds the row and is refused. Two
-- application instances share this table, so the refusal holds across instances and restarts.
--
-- This table is replay state, not a ledger. A row is useless after its token expires, so the
-- API removes rows whose expiry is more than five minutes past. An expired token fails its
-- expiry check (leeway 0) before it reaches this table.
--
-- Server-only. Row level security is enabled and forced with no customer policy.

create table if not exists public.operator_token_replay (
  issuer      text        not null,
  jti         text        not null,
  expires_at  timestamptz not null,
  claimed_at  timestamptz not null default now(),
  primary key (issuer, jti),
  check (length(issuer) between 1 and 256),
  check (length(jti) between 1 and 128)
);

create index if not exists idx_operator_token_replay_expires
  on public.operator_token_replay (expires_at);

alter table public.operator_token_replay enable row level security;
alter table public.operator_token_replay force row level security;
