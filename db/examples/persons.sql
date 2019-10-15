INSERT INTO persons VALUES 

-- Example: Alice
(
  CAST('cc1143129505d87f5f0a044b7dbef236' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Lifetime-related
  TO_TIMESTAMP('2000-01-01', 'YYYY-MM-DD HH24:MI:SS'), -- birth_date
  TO_TIMESTAMP('2099-01-01', 'YYYY-MM-DD HH24:MI:SS'), -- death_date

  -- Physical-related
  7000.0, -- mass_as_grams
  1.7, -- height_as_meters

  -- Org-related
  'Acme Agency', -- org_name
  'Department of Adventures', -- org_team
  'Manager of Adventures' -- org_role

),

-- Example: Bob
(
  CAST('0f8a222d89c986b4a95ad60a55fad6e6' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Lifetime-related
  TO_TIMESTAMP('2000-01-01', 'YYYY-MM-DD HH24:MI:SS'), -- birth_date
  TO_TIMESTAMP('2099-01-01', 'YYYY-MM-DD HH24:MI:SS'), -- death_date

  -- Physical-related
  7000.0, -- mass_as_grams
  1.7, -- height_as_meters

  -- Org-related
  'Bravo Business', -- org_name
  'Department of Books', -- org_team
  'Manager of Books' -- org_role

);

