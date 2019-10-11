-- Geo Codes
--
-- Examples:
--
--   * [Open Location Code](https://github.com/google/open-location-code)
--   * [What Three Words](https://what3words.com)
--   * [What Free Words](https://whatfreewords.org)

CREATE TABLE geo_codes (
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
  coder_id uuid, -- example: fda15956587d3766862f72fe5ab1feea is Open Location Code
  text text, -- example: '6GCRPR6C+24' is Open Location Code geolocation demo area
  latitude numeric(12,9), -- example: '37.8199' is 37.8199° N which is Golden Gate Bridge
  longitude numeric(12,9), -- example: '122.4783' is 122.4783° W (Golden Gate Bridge)
  altitude numeric(20,9), -- example: '67.056' is 67.056 meters to local surface of the earth
  elevation numeric(20,9) -- example: '67.056' is 67.056 meters to global sea level

);

-- Programming-related
CREATE INDEX ix_geo_codes_tenant_id on geo_codes(tenant_id);
CREATE INDEX ix_geo_codes_typecast on geo_codes(typecast);
CREATE INDEX ix_geo_codes_state on geo_codes(state);

-- Update-related
CREATE INDEX ix_geo_codes_updated_at_timestamp_utc on geo_codes(updated_at_timestamp_utc);
CREATE INDEX ix_geo_codes_updated_at_clock_count on geo_codes(updated_at_clock_count);
CREATE INDEX ix_geo_codes_updated_by_text on geo_codes(updated_by_text);

-- Code-related
CREATE INDEX ix_geo_codes_coder_id on geo_codes(coder_id);
CREATE INDEX ix_geo_codes_text on geo_codes(text);
CREATE INDEX ix_geo_codes_latitude on geo_codes(latitude);
CREATE INDEX ix_geo_codes_longitude on geo_codes(longitude);
CREATE INDEX ix_geo_codes_altitude on geo_codes(altitude);
CREATE INDEX ix_geo_codes_elevation on geo_codes(elevation);
