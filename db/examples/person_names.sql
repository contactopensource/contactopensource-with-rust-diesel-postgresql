INSERT INTO person_names VALUES 

-- Example: Dr. Alice Amy Adams Jr.
(
  CAST('cc1143129505d87f5f0a044b7dbef236' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Name-related
  'Alice', -- given_name
  'Alice', -- given_name_phonetic
  'Amy', -- middle_name
  'Amy', -- middle_name_phonetic
  'Adams', -- family_name
  'Adams', -- family_name_phonetic
  'Alice Amy Adams', -- legal_name
  'Alice Amy Adams', -- legal_name_phonetic
  'Dr.', -- prefix_name
  'Dr.', -- prefix_name_phonetic
  'Jr.', -- suffix_name
  'Jr.', -- suffix_name_phonetic
  'Doctor Adams', -- salutation_name
  'Doctor Adams', -- salutation_name_phonetic
  'Dr. Alice Adams Jr.', -- addressee_name
  'Dr. Alice Adams Jr.', -- addressee_name_phonetic
  'Ali', -- nickname
  'Ali' -- nickname_phonetic
), 

-- Example: Hon. Bob Brian Brown Sr.
(
  CAST('925561d3c5d097b690d029ef03d08721' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Name-related
  'Bob', -- given_name
  'Bob', -- given_name_phonetic
  'Brian', -- middle_name
  'Brian', -- middle_name_phonetic
  'Brown', -- family_name
  'Brown', -- family_name_phonetic
  'Bob Brian Brown', -- legal_name
  'Bob Amy Brown', -- legal_name_phonetic
  'Hon.', -- prefix_name
  'Hon.', -- prefix_name_phonetic
  'Sr.', -- suffix_name
  'Sr.', -- suffix_name_phonetic
  'Honorable Brown', -- salutation_name
  'Honorable Brown', -- salutation_name_phonetic
  'Hon. Bob Brown Sr.', -- addressee_name
  'Hon. Bob Brown Sr.', -- addressee_name_phonetic
  'Bobby', -- nickname
  'Bobby' -- nickname_phonetic
);
