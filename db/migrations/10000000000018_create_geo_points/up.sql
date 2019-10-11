CREATE TABLE geo_points (
  id uuid not null primary key,

  -- Programming-related
  tenant_id uuid, -- for optional multi-tenant installation
  typecast text, -- for optional single table inheritance
  state text, -- for optional state machine transition

  -- Update-related
  updated_at_timestamp_utc timestamp, -- example: 2020-01-01T00:00:00 always UTC
  updated_at_clock_count bigint, -- example: 123456789 as suitable for a vector clock
  updated_by_text text, -- example: explanation of who updated the contact, why, how, etc.

  -- Geo-related
  latitude numeric(12,9), -- example: '37.8199' is 37.8199° N which is Golden Gate Bridge
  longitude numeric(12,9), -- example: '122.4783' is 122.4783° W (Golden Gate Bridge)
  altitude numeric(20,9), -- example: '67.056' is 67.056 meters to local surface of the earth
  elevation numeric(20,9) -- example: '67.056' is 67.056 meters to global sea level

);

-- Programming-related
CREATE INDEX ix_geo_points_tenant_id on geo_points(tenant_id);
CREATE INDEX ix_geo_points_typecast on geo_points(typecast);
CREATE INDEX ix_geo_points_state on geo_points(state);

-- Update-related
CREATE INDEX ix_geo_points_updated_at_timestamp_utc on geo_points(updated_at_timestamp_utc);
CREATE INDEX ix_geo_points_updated_at_clock_count on geo_points(updated_at_clock_count);
CREATE INDEX ix_geo_points_updated_by_text on geo_points(updated_by_text);

-- Geo-related
CREATE INDEX ix_geo_points_latitude on geo_points(latitude);
CREATE INDEX ix_geo_points_longitude on geo_points(longitude);
CREATE INDEX ix_geo_points_altitude on geo_points(altitude);
CREATE INDEX ix_geo_points_elevation on geo_points(elevation);
