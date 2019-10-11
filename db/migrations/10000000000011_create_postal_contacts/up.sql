CREATE TABLE postal_contacts (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Postal-related
  country_text text, -- example: 'US' is United States
  region_text text, -- example: 'CA' is California (a.k.a. United States state, Canada province, etc.)
  locality_text text, -- example: 'San Francisco' (a.k.a. city, town, etc.)
  neighborhood_text text, -- example: 'Mission District' (a.k.a. city area, town zone, etc.)
  postal_code_text text, -- example: '94101' is San Francisco downtown area (a.k.a. United States ZIP code, Cananda mail routing number, etc.)
  street_address_text text, -- example: '123 Main Street'
  premise_address_text text, -- example: 'Apartment A1' (a.k.a. room name, box number, etc.)
  global_location_number_text text -- see: https://schema.org/globalLocationNumber https://en.wikipedia.org/wiki/Global_Location_Number

);

-- Programming-related
CREATE INDEX ix_postal_contacts_tenant_id on postal_contacts(tenant_id);
CREATE INDEX ix_postal_contacts_typecast on postal_contacts(typecast);
CREATE INDEX ix_postal_contacts_state on postal_contacts(state);

-- Update-related
CREATE INDEX ix_postal_contacts_updated_at_timestamp_utc on postal_contacts(updated_at_timestamp_utc);
CREATE INDEX ix_postal_contacts_updated_at_clock_count on postal_contacts(updated_at_clock_count);
CREATE INDEX ix_postal_contacts_updated_by_text on postal_contacts(updated_by_text);

-- Postal-related
CREATE INDEX ix_postal_contacts_country_text on postal_contacts(country_text);
CREATE INDEX ix_postal_contacts_region_text on postal_contacts(region_text);
CREATE INDEX ix_postal_contacts_locality_text on postal_contacts(locality_text);
CREATE INDEX ix_postal_contacts_neighborhood_text on postal_contacts(neighborhood_text);
CREATE INDEX ix_postal_contacts_postal_code_text on postal_contacts(postal_code_text);
CREATE INDEX ix_postal_contacts_street_address_text on postal_contacts(street_address_text);
CREATE INDEX ix_postal_contacts_premise_address_text on postal_contacts(premise_address_text);
CREATE INDEX ix_postal_contacts_global_location_number_text on postal_contacts(global_location_number_text);
