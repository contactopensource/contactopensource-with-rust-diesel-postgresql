INSERT INTO link_contacts VALUES

-- Example: New York + Grand Central + Wikipedia link
(
  CAST('2a7b687d89d02d308347c7a207de826e' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Link-related
  'New York Grand Central', -- label
  'https://en.wikipedia.org/wiki/Grand_Central_Terminal' -- uri
),

-- Example: San Francisco + Ferry Building + Wikipedia link
(
  CAST('90aa301cc861104c2646c15c2e7db529' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Link-related
  'San Franciso Ferry Building', -- label
  'https://en.wikipedia.org/wiki/San_Francisco_Ferry_Building' -- uri
);
