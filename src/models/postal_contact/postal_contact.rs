use crate::types;
use crate::schema::postal_contacts;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "postal_contacts"]
pub struct PostalContact {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Postal-related
    pub country_text: Option<types::text::Text>,
    pub region_text: Option<types::text::Text>,
    pub locality_text: Option<types::text::Text>,
    pub neighborhood_text: Option<types::text::Text>,
    pub postal_code_text: Option<types::text::Text>,
    pub street_address_text: Option<types::text::Text>,
    pub premise_address_text: Option<types::text::Text>,
    pub global_location_number_text: Option<types::text::Text>,

}
