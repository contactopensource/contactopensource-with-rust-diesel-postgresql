INSERT INTO arcs VALUES (
  CAST('264b87a8f92fdbd265f4b5543f829cc2' as uuid), -- id

  -- Programming-related
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state

  -- Update-related
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Subject
  'http://example.com/alpha.html', -- subject_uri
  'example_database_1', -- subject_database
  'example_schema_1', -- subject_schema
  'example_table_1',  -- subject_table
  CAST('694492e037227acf0a264d235f18d1e9' as uuid), -- subject_id

  -- Predicate
  'http://example.com/bravo.html', -- predicate_uri
  'example_database_2', -- predicate_database
  'example_schema_2', -- predicate_schema
  'example_table_2',  -- predicate_table
  CAST('694492e037227acf0a264d235f18d1e9' as uuid), -- predicate_id

  -- Object
  'http://example.com/charlie.html', -- object_uri
  'example_database', -- object_database
  'example_schema', -- object_schema
  'example_table',  -- object_table
  CAST('694492e037227acf0a264d235f18d1e9' as uuid), -- object_id

  -- Lifecycle
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- start_at_timestamp_utc
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- stop_at_timestamp_utc

  -- Modifiers
  8, -- count
  8.8, -- weight
  0.8 -- probability

);
