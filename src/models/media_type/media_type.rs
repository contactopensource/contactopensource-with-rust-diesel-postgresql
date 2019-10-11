use crate::types;
use crate::schema::media_types;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "media_types"]
pub struct MediaType {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Media-related
    pub text: Option<String>,
    pub supertype: Option<String>,
    pub subtype: Option<String>,
    pub tree: Option<String>,
    pub suffix: Option<String>,
    pub parameters: Option<Vec<String>>,

}
