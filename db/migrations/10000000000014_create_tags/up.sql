CREATE TABLE tags (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Tag-related
  text text -- example: 'trending'

);

-- Programming-related
CREATE INDEX ix_tags_tenant_id on tags(tenant_id);
CREATE INDEX ix_tags_typecast on tags(typecast);
CREATE INDEX ix_tags_state on tags(state);

-- Update-related
CREATE INDEX ix_tags_updated_at_timestamp_utc on tags(updated_at_timestamp_utc);
CREATE INDEX ix_tags_updated_at_clock_count on tags(updated_at_clock_count);
CREATE INDEX ix_tags_updated_by_text on tags(updated_by_text);

-- General-related
CREATE INDEX ix_tags_text on tags(text);
