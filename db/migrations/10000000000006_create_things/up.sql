CREATE TABLE things (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- General-related
  name text -- example: 'tool'

);

-- Programming-related
CREATE INDEX ix_things_tenant_id on things(tenant_id);
CREATE INDEX ix_things_typecast on things(typecast);
CREATE INDEX ix_things_state on things(state);

-- Update-related
CREATE INDEX ix_things_updated_at_timestamp_utc on things(updated_at_timestamp_utc);
CREATE INDEX ix_things_updated_at_clock_count on things(updated_at_clock_count);
CREATE INDEX ix_things_updated_by_text on things(updated_by_text);

-- General-related
CREATE INDEX ix_things_name on things(name);
