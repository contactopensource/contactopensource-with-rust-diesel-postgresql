INSERT INTO telephone_contacts VALUES

-- Example: New York + Grand Central + telephone for Master's Office
(
  CAST('8e238bdbf3b6c189f9e2ed65b2efbb45' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Telephone-related
  'Master''s Office', -- label
  '1 (212) 340-2583', -- number_text
  -- E.164-related -- see https://en.wikipedia.org/wiki/E.164
  '12123402583', -- e164_text
  '1', -- e164_country_code
  '212', -- e164_national_destination_code
  '', -- e164_group_identification_code
  '', -- e164_trial_identification_code
  '3402583' -- e164_subscriber_number
),

-- Example: San Francisco + Ferry Building + telephone for Lost & Found
(
  CAST('7ea1eb5f6d9075eaa93d12a602dfddb5' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Telephone-related
  'Lost & Found', -- label
  '1 (415) 983-8007', -- number_text
  -- E.164-related -- see https://en.wikipedia.org/wiki/E.164
  '14159838007', -- e164_text
  '1', -- e164_country_code
  '415', -- e164_national_destination_code
  '', -- e164_group_identification_code
  '', -- e164_trial_identification_code
  '4159838007' -- e164_subscriber_number
);
