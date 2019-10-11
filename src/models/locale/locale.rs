use crate::types;
use crate::schema::locales;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "locales"]
pub struct Locale {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Code-related
    pub text: Option<String>,
    pub language_code: Option<String>,
    pub country_code: Option<String>,
    pub script_code: Option<String>,
    pub region_code: Option<String>,
    pub variant_code: Option<String>,

    // Representation-related
    pub decimal_separator: Option<String>,
    pub grouping_separator: Option<String>,
    pub currency_code: Option<String>,
    pub currency_symbol: Option<String>,
    pub quotation_start_delimiter: Option<String>,
    pub quotation_stop_delimiter: Option<String>,

}
