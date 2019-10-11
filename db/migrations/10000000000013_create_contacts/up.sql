CREATE TABLE contacts (
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
  name text, -- example: 'Alice Adams'
  emoji text, -- example: 'U+1F60A' is smiling face with smiling eyes

  -- Display-related
  image_uri text, -- example: 'https://example.com/image.jpg'
  color_hex text, -- example: 'FF0000' is red
  css_class text, -- example: 'friend' is a cascading style sheet class (a space-separated list)
  star_count int2 -- example: '5' means 5-star rating

);

-- Programming-related
CREATE INDEX ix_contacts_tenant_id on contacts(tenant_id);
CREATE INDEX ix_contacts_typecast on contacts(typecast);
CREATE INDEX ix_contacts_state on contacts(state);

-- Update-related
CREATE INDEX ix_contacts_updated_at_timestamp_utc on contacts(updated_at_timestamp_utc);
CREATE INDEX ix_contacts_updated_at_clock_count on contacts(updated_at_clock_count);
CREATE INDEX ix_contacts_updated_by_text on contacts(updated_by_text);

-- General-related
CREATE INDEX ix_contacts_name on contacts(name);
CREATE INDEX ix_contacts_emoji on contacts(emoji);

-- Display-related
CREATE INDEX ix_contacts_image_uri on contacts(image_uri);
CREATE INDEX ix_contacts_color_hex on contacts(color_hex);
CREATE INDEX ix_contacts_css_class on contacts(css_class);
CREATE INDEX ix_contacts_star_count on contacts(star_count);
