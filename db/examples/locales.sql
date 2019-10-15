INSERT INTO locales VALUES 

-- Example: English language + United States
(
  CAST('01e35cdb11f1441a22fffdbbcc398747' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Code-related
  'en-US', -- text
  'en', -- language_code
  'US', -- country_code
  'Latn', -- script_code
  '', -- region_code
  '', -- variant_code

  -- Representation-related
  '.', -- decimal_separator
  ',', -- grouping_separator
  'USD', -- currency_code
  '$', -- currency_symbol
  '“', -- quotation_start_delimiter
  '”' -- quotation_stop_delimiter
),

-- Example: Chinese language (specifically simplified Mandarin) + China
(
  CAST('0efde58a9b43066ebfb11bd1bf8d7074' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Code-related
  'zh-CN', -- text
  'zh', -- language_code
  'CN', -- country_code
  'Hans', -- script_code
  '', -- region_code
  '', -- variant_code

  -- Representation-related
  '.', -- decimal_separator
  ' ', -- grouping_separator
  'CNY', -- currency_code
  '¥', -- currency_symbol
  '「', -- quotation_start_delimiter
  '」' -- quotation_stop_delimiter
);
