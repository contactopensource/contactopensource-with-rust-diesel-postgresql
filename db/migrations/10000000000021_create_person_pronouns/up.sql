CREATE TABLE person_pronouns (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Pronoun-related
  subject_pronoun text, -- example: 'she' as in 'She likes Alice.'; see: https://wikipedia.org/wiki/Subject_pronoun
  object_pronoun text, -- example: 'her' as in 'Alice likes her.'; see: https://wikipedia.org/wiki/Object_pronoun
  dependent_possessive_pronoun text, -- example: 'her' as in 'Her ideas are good.'; see: https://wikipedia.org/wiki/Possessive_pronoun
  independent_possessive_pronoun text, -- example: 'hers' as in 'The ideas are hers.'; see: https://wikipedia.org/wiki/Possessive_determiner
  reflexive_pronoun text, -- example: 'herself' as in 'She likes herself'; see https://wikipedia.org/wiki/Reflexive_pronoun
  intensive_pronoun text, -- example: 'herself' as in 'She does it herself.'; see: https://en.wikipedia.org/wiki/Intensive_pronoun
  disjunctive_pronoun text -- example: 'her' as in 'It is her.'; see https://wikipedia.org/wiki/Disjunctive_pronoun

);

-- Programming-related
CREATE INDEX ix_person_pronouns_tenant_id on person_pronouns(tenant_id);
CREATE INDEX ix_person_pronouns_typecast on person_pronouns(typecast);
CREATE INDEX ix_person_pronouns_state on person_pronouns(state);

-- Update-related
CREATE INDEX ix_person_pronouns_updated_at_timestamp_utc on person_pronouns(updated_at_timestamp_utc);
CREATE INDEX ix_person_pronouns_updated_at_clock_count on person_pronouns(updated_at_clock_count);
CREATE INDEX ix_person_pronouns_updated_by_text on person_pronouns(updated_by_text);

-- Pronoun-related
CREATE INDEX ix_person_pronouns_subject_pronoun on person_pronouns(subject_pronoun);
CREATE INDEX ix_person_pronouns_object_pronoun on person_pronouns(object_pronoun);
CREATE INDEX ix_person_pronouns_dependent_possessive_pronoun on person_pronouns(dependent_possessive_pronoun);
CREATE INDEX ix_person_pronouns_independent_possessive_pronoun on person_pronouns(independent_possessive_pronoun);
CREATE INDEX ix_person_pronouns_reflexive_pronoun on person_pronouns(reflexive_pronoun);
CREATE INDEX ix_person_pronouns_intensive_pronoun on person_pronouns(intensive_pronoun);
CREATE INDEX ix_person_pronouns_disjunctive_pronoun on person_pronouns(disjunctive_pronoun);
