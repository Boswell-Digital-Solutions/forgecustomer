-- rake_access_http_smoke.sql
-- Fixture rows for the HTTP/live-PostgreSQL Rake access-token exchange smoke
-- (BDS-FOC-RAKE-ACCESS-ISSUER-v0.1 WP-02, AC-02 to AC-04).
--
-- Five customers. Device keys are the public halves of fixed test seeds
-- (0x11, 0x22, 0x33, 0x44, 0x55 repeated 32 times); scripts/rake_access_http_smoke.sh signs
-- the client proofs with the same seeds. No real key is involved.
--   A entitled  - active installation, device, license; active Pro subscription
--   B denied    - same shape, no cloud entitlement
--   C mfa       - mfa_required with no grace window; an aal1 token must be refused
--   D revoked   - entitled, but its device is revoked
--   E deleted   - entitled like A; the smoke runs an account deletion for it (AC-04)

begin;

do $$
declare
  v_product_id uuid;
  v_pro_version_id uuid;
  v_billing_id uuid;
  r record;
begin
  select id into strict v_product_id from public.products where key = 'authorforge';
  select pv.id into strict v_pro_version_id
  from public.plan_versions pv
  join public.plans pl on pl.id = pv.plan_id
  where pl.product_id = v_product_id and pl.key = 'authorforge_pro'
  order by pv.version desc
  limit 1;

  for r in
    select * from (values
      ('a', '00000000-0000-4000-8000-00000000a001'::uuid, '00000000-0000-4000-8000-00000000a011'::uuid,
            '00000000-0000-4000-8000-00000000a002'::uuid, '00000000-0000-4000-8000-00000000a003'::uuid,
            '00000000-0000-4000-8000-00000000a004'::uuid, '0EqyMnQrtKs6E2i9RhXk5tAiSrcaAWuvhSCjMsl3hzc=',
            false, true, 'active'),
      ('b', '00000000-0000-4000-8000-00000000b001'::uuid, '00000000-0000-4000-8000-00000000b011'::uuid,
            '00000000-0000-4000-8000-00000000b002'::uuid, '00000000-0000-4000-8000-00000000b003'::uuid,
            '00000000-0000-4000-8000-00000000b004'::uuid, 'oJql9HpnWYAv+VX43C0qFKXJnSO+l/hkEn/5ODRVpPA=',
            false, false, 'active'),
      ('c', '00000000-0000-4000-8000-00000000c001'::uuid, '00000000-0000-4000-8000-00000000c011'::uuid,
            '00000000-0000-4000-8000-00000000c002'::uuid, '00000000-0000-4000-8000-00000000c003'::uuid,
            '00000000-0000-4000-8000-00000000c004'::uuid, 'F8t5+ytBIPKx7GXkGY1uCLKOgT/rAeSkAIObheGAgM4=',
            true, true, 'active'),
      ('d', '00000000-0000-4000-8000-00000000d001'::uuid, '00000000-0000-4000-8000-00000000d011'::uuid,
            '00000000-0000-4000-8000-00000000d002'::uuid, '00000000-0000-4000-8000-00000000d003'::uuid,
            '00000000-0000-4000-8000-00000000d004'::uuid, '11l5O7wTooGagnx2rbb7qKSa7gB/SfLQmS2ZuCWtLEg=',
            false, true, 'revoked'),
      ('e', '00000000-0000-4000-8000-00000000e001'::uuid, '00000000-0000-4000-8000-00000000e011'::uuid,
            '00000000-0000-4000-8000-00000000e002'::uuid, '00000000-0000-4000-8000-00000000e003'::uuid,
            '00000000-0000-4000-8000-00000000e004'::uuid, 'xoImN8fTEOxXYnvgC6JZ0lN0n0qvZERwz/vlOjX3MkI=',
            false, true, 'active')
    ) as t(tag, customer_id, auth_user_id, device_id, installation_id, license_id, public_key,
           mfa_required, entitled, device_status)
  loop
    -- The deletion run leaves append-only audit rows for E, so its profile cannot be
    -- deleted again. Stop with a clear message instead of a trigger error.
    if exists (select 1 from public.customer_profiles
               where id = r.customer_id and status = 'anonymized') then
      raise exception 'rake access smoke needs a fresh database: customer % was deleted by an earlier run', r.tag;
    end if;
    delete from public.app_token_mints where customer_id = r.customer_id;
    delete from public.customer_profiles where id = r.customer_id;

    insert into public.customer_profiles
        (id, auth_user_id, status, display_name, mfa_required, mfa_grace_period_ends_at)
    values (r.customer_id, r.auth_user_id, 'active', 'rake access smoke ' || r.tag,
            r.mfa_required, null);

    insert into public.devices (id, customer_id, public_key, public_key_fpr, status)
    values (r.device_id, r.customer_id, r.public_key, 'smoke-fpr-' || r.tag, r.device_status);

    insert into public.installations (id, customer_id, device_id, install_key, product_id, status)
    values (r.installation_id, r.customer_id, r.device_id, 'rake-access-smoke-' || r.tag,
            v_product_id, 'active');

    insert into public.licenses (id, customer_id, product_id, status, device_limit)
    values (r.license_id, r.customer_id, v_product_id, 'active', 1);

    insert into public.license_activations (license_id, installation_id, device_id, status)
    values (r.license_id, r.installation_id, r.device_id, 'active');

    -- Cloud features need an entitling subscription; an override alone cannot grant
    -- them (domain::entitlement). So entitled customers hold an active Pro subscription,
    -- the real path. Customer B has none and stays on the included plan.
    if r.entitled then
      insert into public.billing_accounts (customer_id) values (r.customer_id)
      returning id into v_billing_id;
      insert into public.subscriptions (customer_id, billing_account_id, plan_version_id, status)
      values (r.customer_id, v_billing_id, v_pro_version_id, 'active');
    end if;
  end loop;
end
$$;

commit;
