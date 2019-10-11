CREATE TABLE locales (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Code-related
  text text, -- example: 'en-US' is English language in United States country
  language_code text, -- example: 'en' is English
  country_code text, -- example: 'US' is United States
  script_code text, -- example: 'Latn' is Latin script; see http://www.unicode.org/iso15924/codelists.html
  region_code text, -- example: 'QO' is Outlying Oceania
  variant_code text, -- example: 'TH' is Thai digit shapes

  -- Representation-related
  decimal_separator text, -- example: '.' is the decimal separator of 'en-US'; see https://en.wikipedia.org/wiki/Decimal_separator
  grouping_separator text, -- example: ',' is the grouping separator of 'en-US'
  currency_code text, -- example: 'USD' is United States Dollar
  currency_symbol text, -- example: '$' is United States Dollar
  quotation_start_delimiter text, -- example: '“' (U+201C) is left double quotation mark
  quotation_stop_delimiter text -- example: '”' (U+201D) is right double quotation mark

  -- TODO
  -- calendar: Calendar? -- The calendar for the locale, or the Gregorian calendar as a fallback.
  -- exemplar_character_set: CharacterSet? -- The exemplar character set for the locale, or nil if has none.
  -- collation_dentifier: String? -- The collation identifier for the locale, or nil if it has none.
  -- collator_identifier: String? -- The collator identifier of the locale.
  -- usesMetricSystem: Bool -- A Boolean that is true if the locale uses the metric system.

);

-- Programming-related
CREATE INDEX ix_locales_tenant_id on locales(tenant_id);
CREATE INDEX ix_locales_typecast on locales(typecast);
CREATE INDEX ix_locales_state on locales(state);

-- Update-related
CREATE INDEX ix_locales_updated_at_timestamp_utc on locales(updated_at_timestamp_utc);
CREATE INDEX ix_locales_updated_at_clock_count on locales(updated_at_clock_count);
CREATE INDEX ix_locales_updated_by_text on locales(updated_by_text);

-- locale-related
CREATE INDEX ix_locales_text on locales(text);
CREATE INDEX ix_locales_language_code on locales(language_code);
CREATE INDEX ix_locales_country_code on locales(country_code);
CREATE INDEX ix_locales_script_code on locales(script_code);
CREATE INDEX ix_locales_region_code on locales(region_code);
CREATE INDEX ix_locales_variant_code on locales(variant_code);
