CREATE TABLE email_contacts (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Email-related
  address text, -- example: 'Alice Adams <alice@example.com>' -- see https://tools.ietf.org/html/rfc5322
  display_name text, -- example: 'Alice Adams'
  addr_spec text -- example: 'alice@example.com'

);

-- Programming-related
CREATE INDEX ix_email_contacts_tenant_id on email_contacts(tenant_id);
CREATE INDEX ix_email_contacts_typecast on email_contacts(typecast);
CREATE INDEX ix_email_contacts_state on email_contacts(state);

-- Update-related
CREATE INDEX ix_email_contacts_updated_at_timestamp_utc on email_contacts(updated_at_timestamp_utc);
CREATE INDEX ix_email_contacts_updated_at_clock_count on email_contacts(updated_at_clock_count);
CREATE INDEX ix_email_contacts_updated_by_text on email_contacts(updated_by_text);

-- Email-related
CREATE INDEX ix_email_contacts_address on email_contacts(address);
CREATE INDEX ix_email_contacts_display_name on email_contacts(display_name);
CREATE INDEX ix_email_contacts_addr_spec on email_contacts(addr_spec);
