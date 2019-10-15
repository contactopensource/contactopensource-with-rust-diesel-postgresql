INSERT INTO media_types VALUES 

-- Example: text/plain + ASCII
(
  CAST('bb40d818933e8e525d54d6cea27d573e' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Media-related
  'text/plain;charset=ASCII;boundary=alpha', -- text
  'text', -- supertype
  'html', -- subtype
  '', -- tree
  '', -- suffix
  array['charset=ASCII', 'boundary=alpha'] -- parameters

),

-- Example: application/json + UTF-8
(
  CAST('bf029cf3b57788687fe5b0823c80e901' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Media-related
  'application/x.example+json;charset=UTF-8;boundary=something', -- text
  'application', -- supertype
  'example', -- subtype
  'x.', -- tree
  '+json', -- suffix
  array['charset=UTF-16, 'boundary=bravo'] -- parameters

);

