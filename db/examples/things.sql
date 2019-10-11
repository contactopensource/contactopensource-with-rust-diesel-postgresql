INSERT INTO things VALUES

-- Example: a train (such as for examples with New York Grand Central)
(
  CAST('d5c7802ecf25359ddc7dde71074f0132' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Thing-related
  'train' -- name
),

-- Example: a boat (such as for examples with San Francisco Ferry Building)
(
  CAST('deabb3e6a944fd07d5e83de4a3a237b6' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Thing-related
  'boat' -- name
);
