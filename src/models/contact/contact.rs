use crate::types;
use crate::schema::contacts;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "contacts"]
pub struct Contact {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // General-related
    pub name: Option<types::text::Text>,
    pub emoji: Option<types::text::Text>,

    // Display-related
    pub image_uri: Option<types::uri_string::URIString>,
    pub color_hex: Option<types::color_hex_upper_string::ColorHexUpperString>,
    pub css_class: Option<types::css_class_name::CSSClassName>,
    pub star_count: Option<types::star_count::StarCount>,

}
