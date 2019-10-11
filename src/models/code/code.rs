use crate::types;
use crate::schema::codes;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "codes"]
pub struct Code {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Link-related
    pub set_id: Option<types::id::Id>,
    pub text: Option<types::text::Text>,
    pub name: Option<types::text::Text>,

}
