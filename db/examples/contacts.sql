INSERT INTO contacts VALUES

-- Example: Acme Agency
(
  CAST('423f36dcba577aecb7c127b898c00183' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- General-related
  'Acme Agency', -- name
  'U+1F60A', -- emoji (e.g. happy face)

  -- Display-related
  'https://example.com/alpha.jpg', -- image_uri
  'FF0000', -- color_hex e.g. red
  'selected', -- css_class
  5 -- star_count
),

-- Example: Bravo Business
(
  CAST('423f36dcba577aecb7c127b898c00183' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- General-related
  'Bravo Business', -- name
  'U+1F610', -- emoji (e.g. neutral face)

  -- Display-related
  'https://example.com/bravo.jpg', -- image_uri
  '00FF00', -- color_hex e.g. blue
  'unselected', -- css_class
  3 -- star_count
);
