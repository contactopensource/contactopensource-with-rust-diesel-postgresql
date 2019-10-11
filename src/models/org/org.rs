use crate::types;
use crate::schema::orgs;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "orgs"]
pub struct Org {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Lifetime-related
    pub start_date: Option<types::date::Date>,
    pub stop_date: Option<types::date::Date>,

}
