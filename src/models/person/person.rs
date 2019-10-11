use crate::types;
use crate::schema::persons;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "persons"]
pub struct Person {
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
    pub birth_date: Option<types::date::Date>,
    pub death_date: Option<types::date::Date>,

    // Physical-related
    pub mass_as_grams: Option<types::grams::Grams>,
    pub height_as_meters: Option<types::meters::Meters>,

    // Org-related
    pub org_name: Option<String>,
    pub org_team: Option<String>,
    pub org_role: Option<String>,

}
