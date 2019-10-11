use crate::types;
use crate::schema::email_contacts;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "email_contacts"]
pub struct EmailContact {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Email-related
    pub address: Option<types::text::Text>,
    pub display_name: Option<types::text::Text>,
    pub addr_spec: Option<types::text::Text>,

}
