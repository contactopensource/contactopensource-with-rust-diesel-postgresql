use crate::types;
use crate::schema::geo_points;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "geo_points"]
pub struct GeoPoint {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Geo-related
    pub latitude: Option<types::latitude::Latitude>,
    pub longitude: Option<types::longitude::Longitude>,
    pub altitude: Option<types::altitude::Altitude>,
    pub elevation: Option<types::elevation::Elevation>,

}
