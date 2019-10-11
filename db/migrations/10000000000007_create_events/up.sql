CREATE TABLE events (
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
  name text, -- example: 'Acme Agency'

  -- Lifetime-related
  start_timestamp_utc timestamp, --  example: 2020-01-01T00:00:00 always UTC
  stop_timestamp_utc timestamp, --  example: 2020-01-01T00:00:00 always UTC
  duration_as_seconds numeric(20,9) -- example: 3600 seconds is 1 hour

);

-- Programming-related
CREATE INDEX ix_events_tenant_id on events(tenant_id);
CREATE INDEX ix_events_typecast on events(typecast);
CREATE INDEX ix_events_state on events(state);

-- Update-related
CREATE INDEX ix_events_updated_at_timestamp_utc on events(updated_at_timestamp_utc);
CREATE INDEX ix_events_updated_at_clock_count on events(updated_at_clock_count);
CREATE INDEX ix_events_updated_by_text on events(updated_by_text);

-- General-related
CREATE INDEX ix_events_name on events(name);

-- Lifetime-related
CREATE INDEX ix_events_start_timestamp_utc on events(start_timestamp_utc);
CREATE INDEX ix_events_stop_timestamp_utc on events(stop_timestamp_utc);
CREATE INDEX ix_events_duration_as_seconds on events(duration_as_seconds);
