CREATE TABLE passport_contacts (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Place-related
  country_text text, -- example: 'US' is United States
  number_text text, -- example: '000-000-000-000'

  -- Time-related
  valid_start_date date, -- example: 2020-01-01
  valid_stop_date date -- example: 2030-01-01

);

-- Programming-related
CREATE INDEX ix_passport_contacts_tenant_id on passport_contacts(tenant_id);
CREATE INDEX ix_passport_contacts_typecast on passport_contacts(typecast);
CREATE INDEX ix_passport_contacts_state on passport_contacts(state);

-- Update-related
CREATE INDEX ix_passport_contacts_updated_at_timestamp_utc on passport_contacts(updated_at_timestamp_utc);
CREATE INDEX ix_passport_contacts_updated_at_clock_count on passport_contacts(updated_at_clock_count);
CREATE INDEX ix_passport_contacts_updated_by_text on passport_contacts(updated_by_text);

-- Place-related
CREATE INDEX ix_passport_contacts_country_text on passport_contacts(country_text);
CREATE INDEX ix_passport_contacts_number_text on passport_contacts(number_text);

-- Time-related
CREATE INDEX ix_passport_contacts_valid_start_date on passport_contacts(valid_start_date);
CREATE INDEX ix_passport_contacts_valid_stop_date on passport_contacts(valid_stop_date);
