CREATE TABLE persons (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Lifetime-related
  birth_date date, -- example: '2000-01-01'
  death_date date, -- example: '2099-01-01'

  -- Physical-related
  mass_as_grams numeric(12,9), -- example: TODO
  height_as_meters numeric(10,9), -- example: TODO

  -- Org-related
  org_name text, -- example: 'Acme Company'
  org_team text, -- example: 'Department of Widgets'
  org_role text -- example: 'Manager of Widgets'

);

-- Programming-related
CREATE INDEX ix_persons_tenant_id on persons(tenant_id);
CREATE INDEX ix_persons_typecast on persons(typecast);
CREATE INDEX ix_persons_state on persons(state);

-- Update-related
CREATE INDEX ix_persons_updated_at_timestamp_utc on persons(updated_at_timestamp_utc);
CREATE INDEX ix_persons_updated_at_clock_count on persons(updated_at_clock_count);
CREATE INDEX ix_persons_updated_by_text on persons(updated_by_text);

-- Lifetime-related
CREATE INDEX ix_persons_birth_date on persons(birth_date);
CREATE INDEX ix_persons_death_date on persons(death_date);

-- Physical-related
CREATE INDEX ix_persons_mass_as_grams on persons(mass_as_grams);
CREATE INDEX ix_persons_height_as_meters on persons(height_as_meters);

-- Org-related
CREATE INDEX ix_persons_org_name on persons(org_name);
CREATE INDEX ix_persons_org_team on persons(org_team);
CREATE INDEX ix_persons_org_role on persons(org_role);
