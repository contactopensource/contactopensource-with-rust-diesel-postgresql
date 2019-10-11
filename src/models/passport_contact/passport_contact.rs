use crate::types;
use crate::schema::passport_contacts;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "passport_contacts"]
pub struct PassportContact {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Passport-related
    pub country_text: Option<types::passport_country_text::PassportCountryText>,
    pub number_text: Option<types::passport_number_text::PassportNumberText>,

    // Time-related
    pub valid_start_date: Option<types::date::Date>,
    pub valid_stop_date: Option<types::date::Date>,

}
