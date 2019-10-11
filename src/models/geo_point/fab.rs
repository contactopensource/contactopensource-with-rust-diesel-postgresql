use crate::types;
use crate::models::geo_point::geo_point::GeoPoint as T;

pub fn fab() -> T {
    T {
        id: types::id::fab(),

        // Programming-related
        tenant_id: Some(types::id::fab()),
        typecast: Some(types::typecast::fab()),
        state: Some(types::state::fab()),

        // Update-related
        updated_at_timestamp_utc: Some(types::timestamp::fab()),
        updated_at_clock_count: Some(types::count::fab()),
        updated_by_text: Some(types::text::fab()),

        // Geo-related
        latitude: Some(types::latitude::fab()),
        longitude: Some(types::longitude::fab()),
        altitude: Some(types::altitude::fab()),
        elevation: Some(types::elevation::fab()),

    }
}
