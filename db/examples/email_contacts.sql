INSERT INTO email_contacts VALUES 

-- Example: Alice Adams <alice@example.com>
(
  CAST('dc3200ef1e8066ab892964f9292cd15e' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Email-related
  'Alice Adams <alice@example.com>', -- address
  'Alice Adams', -- display_name
  'alice@example.com' -- addr_spec
),

-- Example: Bob Brown <bob@example.com>
(
  CAST('92543cb0d9829d78b6188c108e25be9e' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Email-related
  'Bob Brown <bob@example.com>', -- address
  'Bob Brown', -- display_name
  'bob@example.com' -- addr_spec
);

