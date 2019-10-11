use crate::types;
use crate::schema::items;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "items"]
pub struct Item {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Meta-related
    pub uri: Option<String>,

    // Content-related
    pub text: Option<types::text::Text>,
    pub json: Option<types::json_value::JSONValue>,
    pub xml: Option<types::xml_string::XMLString>,
    pub number: Option<types::number::Number>,

}
