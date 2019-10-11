CREATE TABLE orgs (
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
  emoji text, -- example: 'U+1F60A' is smiling face with smiling eyes

  -- Display-related
  image_uri text, -- example: 'https://example.com/image.jpg'
  color_hex text, -- example: 'FF0000' is red
  css_class text, -- example: 'friend' is a cascading style sheet class (a space-separated list)
  star_count int2, -- example: '5' means 5-star rating

  -- Lifetime-related
  start_date date, -- example: '2000-01-01'
  stop_date date -- example: '2099-01-01'

);

-- Programming-related
CREATE INDEX ix_orgs_tenant_id on orgs(tenant_id);
CREATE INDEX ix_orgs_typecast on orgs(typecast);
CREATE INDEX ix_orgs_state on orgs(state);

-- Update-related
CREATE INDEX ix_orgs_updated_at_timestamp_utc on orgs(updated_at_timestamp_utc);
CREATE INDEX ix_orgs_updated_at_clock_count on orgs(updated_at_clock_count);
CREATE INDEX ix_orgs_updated_by_text on orgs(updated_by_text);

-- General-related
CREATE INDEX ix_orgs_name on orgs(name);
CREATE INDEX ix_orgs_emoji on orgs(emoji);

-- Display-related
CREATE INDEX ix_orgs_image_uri on orgs(image_uri);
CREATE INDEX ix_orgs_color_hex on orgs(color_hex);
CREATE INDEX ix_orgs_css_class on orgs(css_class);
CREATE INDEX ix_orgs_star_count on orgs(star_count);

-- Lifetime-related
CREATE INDEX ix_orgs_start_date on orgs(start_date);
CREATE INDEX ix_orgs_stop_date on orgs(stop_date);
