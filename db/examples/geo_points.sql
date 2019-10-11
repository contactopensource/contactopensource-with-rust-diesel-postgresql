INSERT INTO geo_points VALUES (
  -- New York Grand Central
  CAST('c8fb5187e718484eefb9e8dceef4cd68' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Geo-related
  40.752687, -- latitude
  73.977188, -- longitude
  39.6, -- altitude
  39.6 -- elevation
),(
  -- San Francisco Ferry Building
  CAST('6e7e03c894c96caeee4719e3dcccd339' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Geo-related
  37.795563, -- latitude
  -122.393312, -- longitude
  12.1, -- altitude
  12.1 -- elevation
);
