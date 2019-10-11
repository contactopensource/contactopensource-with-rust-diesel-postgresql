
-- Media types are also known and MIME types.
--
-- See https://en.wikipedia.org/wiki/Media_type
--
-- This table is baed on IANA media type standard.
--
-- Examples:
--
--     text/plain
--     image/jpg
--
-- Example tree items:
--
--     application/x.foo (the 'x' tree means unregistered)
--     application/vnd.foo (the 'vnd' tree means vendor)
--
-- Example suffix items:
--
--     application/foo+json (the '+json' means JSON format)
--     application/foo+xml (the '+xml' means XML format)
--
-- Example parameter items:
--
--     charset=UTF-8
--     boundary=something
--
-- Example that uses all of the above:
--
--     application/x.foo+json;charset=UTF-8;boundary=something
--
CREATE TABLE media_types (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Media-related
  text text, -- example: 'application/x.foo;charset=UTF-8'
  supertype text, -- example: 'text'
  subtype text, -- example 'plain'
  tree text, -- example: 'x.' means unregistered
  suffix text, -- example: '+zip' means ZIP compressed file
  parameters text[] -- example: 'charset=UTF-8',

);

-- Programming-related
CREATE INDEX ix_media_types_tenant_id on media_types(tenant_id);
CREATE INDEX ix_media_types_typecast on media_types(typecast);
CREATE INDEX ix_media_types_state on media_types(state);

-- Update-related
CREATE INDEX ix_media_types_updated_at_timestamp_utc on media_types(updated_at_timestamp_utc);
CREATE INDEX ix_media_types_updated_at_clock_count on media_types(updated_at_clock_count);
CREATE INDEX ix_media_types_updated_by_text on media_types(updated_by_text);

-- Media-related
CREATE INDEX ix_media_types_text on media_types(text);
CREATE INDEX ix_media_types_supertype on media_types(supertype);
CREATE INDEX ix_media_types_subtype on media_types(subtype);
CREATE INDEX ix_media_types_tree on media_types(tree);
CREATE INDEX ix_media_types_suffix on media_types(suffix);
CREATE INDEX ix_media_types_parameters on media_types(parameters);
