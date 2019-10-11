CREATE TABLE telephone_contacts (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Telephone-related
  label text, -- example: 'mobile phone'
  number_text text, -- example: '1 (415) 555-0000'

  -- E.164-related -- see https://en.wikipedia.org/wiki/E.164
  e164_text varchar(15), -- example: '14155550000' means United States country code, San Franicsco area code, etc.
  e164_country_code varchar(3), -- example: '1' means United States -- see https://en.wikipedia.org/wiki/E.164
  e164_national_destination_code varchar(13), -- example: '415' is the United States area code of San Francisco
  e164_group_identification_code varchar(4), -- example: TODO
  e164_trial_identification_code varchar(3), -- example: TODO
  e164_subscriber_number varchar(14) -- example: '5550000' -- see https://en.wikipedia.org/wiki/E.164

);

-- Programming-related
CREATE INDEX ix_telephone_contacts_tenant_id on telephone_contacts(tenant_id);
CREATE INDEX ix_telephone_contacts_typecast on telephone_contacts(typecast);
CREATE INDEX ix_telephone_contacts_state on telephone_contacts(state);

-- Update-related
CREATE INDEX ix_telephone_contacts_updated_at_timestamp_utc on telephone_contacts(updated_at_timestamp_utc);
CREATE INDEX ix_telephone_contacts_updated_at_clock_count on telephone_contacts(updated_at_clock_count);
CREATE INDEX ix_telephone_contacts_updated_by_text on telephone_contacts(updated_by_text);

-- Telephone-related
CREATE INDEX ix_telephone_contacts_label on telephone_contacts(label);
CREATE INDEX ix_telephone_contacts_number_text on telephone_contacts(number_text);

-- E.164-related
CREATE INDEX ix_telephone_contacts_e164_text on telephone_contacts(e164_text);
CREATE INDEX ix_telephone_contacts_e164_country_code on telephone_contacts(e164_country_code);
CREATE INDEX ix_telephone_contacts_e164_national_destination_code on telephone_contacts(e164_national_destination_code);
CREATE INDEX ix_telephone_contacts_e164_group_identification_code on telephone_contacts(e164_group_identification_code);
CREATE INDEX ix_telephone_contacts_e164_trial_identification_code on telephone_contacts(e164_trial_identification_code);
CREATE INDEX ix_telephone_contacts_e164_subscriber_number on telephone_contacts(e164_subscriber_number);
