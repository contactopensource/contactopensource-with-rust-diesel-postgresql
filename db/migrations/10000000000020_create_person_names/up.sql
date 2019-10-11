CREATE TABLE person_names (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Name-related
  given_name text, -- example: 'Alice'
  given_name_phonetic text, -- example: 'Alice'
  middle_name text, -- example: 'Amy'
  middle_name_phonetic text, -- example: 'Amy'
  family_name text, -- example: 'Adams'
  family_name_phonetic text, -- example: 'Adams'
  legal_name text, -- example: 'Alice Amy Adams'
  legal_name_phonetic text, -- example: 'Alice Amy Adams'
  prefix_name text, -- example: 'Dr.'
  prefix_name_phonetic text, -- example: 'Dr.'
  suffix_name text, -- example: 'Jr.'
  suffix_name_phonetic text, -- example: 'Jr.'
  salutation_name text, -- example: 'Doctor Adams'
  salutation_name_phonetic text, -- example: 'Doctor Adams'
  addressee_name text, -- example: 'Dr. Alice Adams Jr.'
  addressee_name_phonetic text, -- example: 'Dr. Alice Adams Jr.'
  nickname text, -- example: 'Ali'
  nickname_phonetic text -- example: 'Ali'

);

-- Programming-related
CREATE INDEX ix_person_names_tenant_id on person_names(tenant_id);
CREATE INDEX ix_person_names_typecast on person_names(typecast);
CREATE INDEX ix_person_names_state on person_names(state);

-- Update-related
CREATE INDEX ix_person_names_updated_at_timestamp_utc on person_names(updated_at_timestamp_utc);
CREATE INDEX ix_person_names_updated_at_clock_count on person_names(updated_at_clock_count);
CREATE INDEX ix_person_names_updated_by_text on person_names(updated_by_text);

-- Person-related
CREATE INDEX ix_person_names_given_name on person_names(given_name);
CREATE INDEX ix_person_names_given_name_phonetic on person_names(given_name_phonetic);
CREATE INDEX ix_person_names_middle_name on person_names(middle_name);
CREATE INDEX ix_person_names_middle_name_phonetic on person_names(middle_name_phonetic);
CREATE INDEX ix_person_names_family_name on person_names(family_name);
CREATE INDEX ix_person_names_family_name_phonetic on person_names(family_name_phonetic);
CREATE INDEX ix_person_names_legal_name on person_names(legal_name);
CREATE INDEX ix_person_names_legal_name_phonetic on person_names(legal_name_phonetic);
CREATE INDEX ix_person_names_salutation on person_names(salutation_name);
CREATE INDEX ix_person_names_salutation_phonetic on person_names(salutation_name_phonetic);
CREATE INDEX ix_person_names_addressee on person_names(addressee_name);
CREATE INDEX ix_person_names_addressee_phonetic on person_names(addressee_name_phonetic);
CREATE INDEX ix_person_names_nickname on person_names(nickname);
CREATE INDEX ix_person_names_nickname_phonetic on person_names(nickname_phonetic);
