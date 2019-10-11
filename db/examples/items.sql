INSERT INTO items VALUES

-- Example: Alice
(
  CAST('c9d5a052adab471f2d0e63054a535793' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Meta-related -- the intent is to describe the content fields below.
  'https://example.com/alice.html', -- uri

  -- Content-related - the intent is to offer a variety of database types.
  'Alice', -- text
  '{"item": "Alice"}', -- json
  '<?xml version="1.0"?><item>Bob</item>' -- xml

),

-- Example: Bob
(
  CAST('88f9eba69345b623bafcc2a50e53f2f8' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Meta-related -- the intent is to describe the content fields below.
  'https://example.com/bob.html', -- uri

  -- Content-related - the intent is to offer a variety of database types.
  'Bob', -- text
  '{"item": "Bob"}', -- json
  '<?xml version="1.0"?><item>Bob</item>' -- xml

);
