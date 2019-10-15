INSERT INTO places VALUES 

-- New York + Grand Central
(
  CAST('a1b32e52b344acb5187b1749c6697982' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Place-related
  'Grand Central in New York' -- name

  -- Postal contact:
  -- 'Grand Central', -- addressee
  -- '89 E 42nd St', -- street_address
  -- 'Master''s Office', -- premise_address
  -- 'New York', -- locality_name
  -- 'NY', -- region_code
  -- '10017', -- postal_code
  -- 'US', -- country_code
  -- 'NA', -- continent_code
  -- 'Midtown', -- neighborhood_name

  -- Telephone: 1 (212) 340-2583 = Master's Office

  -- Geo point:
  -- 40.752687, -- latitude
  -- -73.977188, -- longitude

  -- Geo codes:
  -- '87G8Q23F+34', -- https://plus.codes
  -- 'rubble.occurs.holds', -- What3Words.com
  -- 'burst.fully.things', -- WhatFreeWords.org
  -- TODO -- global location number
),

-- San Francisco + Ferry Building
(
  CAST('650044c3f694a056d370ad88dc75aa7b' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text
  'Ferry Building in San Francisco' -- name

  -- Postal contact:
  -- 'Ferry Building', -- addressee
  -- '1 Ferry Building', -- street_address
  -- 'Lost & Found', -- premise_address
  -- 'San Francisco', -- locality_name
  -- 'CA', -- region_code
  -- '94111', -- postal_code
  -- 'US', -- country_code
  -- 'NA', -- continent_code
  -- 'Downtown', -- neighborhood_name

  -- Telephone:  1 (415) 983-8007 = Lost & Found

  -- Geo point:
  -- 37.795563, -- latitude
  -- -122.393312, -- longitude

  -- Geo codes:
  -- '849VQJW4+6M', -- https://plus.codes
  -- 'banks.issues.socket', -- https://what3words.com
  -- 'solid.cracks.solar', -- https://whatfreewords.org
  -- TODO -- global location number
);