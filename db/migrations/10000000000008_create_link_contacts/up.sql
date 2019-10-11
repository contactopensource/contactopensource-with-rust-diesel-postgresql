CREATE TABLE link_contacts (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Link-related
  label text, -- example: 'Example web page'
  uri text -- example: 'http://example.com/example.html'

);

-- Programming-related
CREATE INDEX ix_link_contacts_tenant_id on link_contacts(tenant_id);
CREATE INDEX ix_link_contacts_typecast on link_contacts(typecast);
CREATE INDEX ix_link_contacts_state on link_contacts(state);

-- Update-related
CREATE INDEX ix_link_contacts_updated_at_timestamp_utc on link_contacts(updated_at_timestamp_utc);
CREATE INDEX ix_link_contacts_updated_at_clock_count on link_contacts(updated_at_clock_count);
CREATE INDEX ix_link_contacts_updated_by_text on link_contacts(updated_by_text);

-- Link-related
CREATE INDEX ix_link_contacts_label on link_contacts(label);
CREATE INDEX ix_link_contacts_uri on link_contacts(uri);
