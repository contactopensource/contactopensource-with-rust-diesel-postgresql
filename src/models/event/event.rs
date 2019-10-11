use crate::types;
use crate::schema::events;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "events"]
pub struct Event {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Name-related
    pub name: Option<types::text::Text>,

    // Lifetime-related
    pub start_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub stop_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub duration_as_seconds: Option<types::seconds::Seconds>,

}
