CREATE TABLE codes (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Code-related
  set_id uuid, -- example: '8bafabcc469dbae2c04e84a42bbfd19d' is ISO
  text text, -- example: 'A'
  name text -- example: 'Agriculture'
);

-- Programming-related
CREATE INDEX ix_codes_tenant_id on codes(tenant_id);
CREATE INDEX ix_codes_typecast on codes(typecast);
CREATE INDEX ix_codes_state on codes(state);

-- Update-related
CREATE INDEX ix_codes_updated_at_timestamp_utc on codes(updated_at_timestamp_utc);
CREATE INDEX ix_codes_updated_at_clock_count on codes(updated_at_clock_count);
CREATE INDEX ix_codes_updated_by_text on codes(updated_by_text);

-- General-related
CREATE INDEX ix_codes_set_id on codes(set_id);
CREATE INDEX ix_codes_text on codes(text);
CREATE INDEX ix_codes_name on codes(name);
