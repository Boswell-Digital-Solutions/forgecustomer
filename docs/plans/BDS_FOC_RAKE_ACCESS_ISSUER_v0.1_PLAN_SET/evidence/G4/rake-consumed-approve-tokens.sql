create table public.consumed_approve_tokens (
  jti varchar(64) primary key,
  exp bigint not null
);
comment on column public.consumed_approve_tokens.exp is 'Token expiry, seconds since the epoch';
alter table public.consumed_approve_tokens enable row level security;
revoke all on table public.consumed_approve_tokens from public, anon, authenticated;
