INSERT INTO person_pronouns VALUES

-- Example: she, her, etc.
(
  CAST('cc1143129505d87f5f0a044b7dbef236' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Pronoun-related
  'she', -- subject_pronoun
  'her', -- object_pronoun
  'her', -- dependent_possessive_pronoun
  'hers', -- independent_possessive_pronoun
  'herself', -- reflexive_pronoun
  'herself', -- intensive_pronoun
  'her' -- disjunctive_pronoun
),

-- Example: he, him, etc.
(
  CAST('b16790a15e769541cd1b81ca4fe33a72' as uuid), -- id
  CAST('7bd380209cd310d3ad4e7f980298cbea' as uuid), -- tenant_id
  '', -- type
  '', -- state
  TO_TIMESTAMP('2020-01-01 00:00:00', 'YYYY-MM-DD HH24:MI:SS'), -- updated_at_timestamp_utc
  0, -- updated_at_clock_count
  'updated by example', -- updated_by_text

  -- Pronoun-related
  'he', -- subject_pronoun
  'him', -- object_pronoun
  'his', -- dependent_possessive_pronoun
  'his', -- independent_possessive_pronoun
  'himself', -- reflexive_pronoun
  'himself', -- intensive_pronoun
  'him' -- disjunctive_pronoun
);
