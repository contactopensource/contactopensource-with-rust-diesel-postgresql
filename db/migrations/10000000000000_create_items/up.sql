CREATE TABLE items (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Meta-related -- the intent is to describe the content fields below.
  uri text, -- example: 'https://example.com/example.txt'

  -- Content-related -- intended as a mutex of available database types
  text text, -- example: 'hello world'
  json jsonb, -- example: '{"hello": "world"}'
  xml xml, -- example: '<?xml version=\'1.0\'?><example>hello world</example>'
  number numeric(20,9) -- example: 1234.5678

);

-- Programming-related
CREATE INDEX ix_items_tenant_id on items(tenant_id);
CREATE INDEX ix_items_typecast on items(typecast);
CREATE INDEX ix_items_state on items(state);

-- Update-related
CREATE INDEX ix_items_updated_at_timestamp_utc on items(updated_at_timestamp_utc);
CREATE INDEX ix_items_updated_at_clock_count on items(updated_at_clock_count);
CREATE INDEX ix_items_updated_by_text on items(updated_by_text);

-- Meta-related
CREATE INDEX ix_items_uri on items(uri);

-- Content-related
CREATE INDEX ix_items_text on items(text);
CREATE INDEX ix_items_json on items(json);
CREATE INDEX ix_items_xml on items USING BTREE (cast(xpath('/', xml) as text[]));
CREATE INDEX ix_items_number on items(number);
