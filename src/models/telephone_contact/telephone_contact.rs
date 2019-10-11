use crate::types;
use crate::schema::telephone_contacts;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "telephone_contacts"]
pub struct TelephoneContact {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Telephone-related
    pub label: Option<types::label::Label>,
    pub number_text: Option<types::telephone_number_text::TelephoneNumberText>,

    // E.164-related
    pub e164_text: Option<types::telephone_e164_text::TelephoneE164Text>,
    pub e164_country_code: Option<types::telephone_e164_country_code::TelephoneE164CountryCode>,
    pub e164_national_destination_code: Option<types::telephone_e164_national_destination_code::TelephoneE164NationalDestinationCode>,
    pub e164_group_identification_code: Option<types::telephone_e164_group_identification_code::TelephoneE164GroupIdentificationCode>,
    pub e164_trial_identification_code: Option<types::telephone_e164_trial_identification_code::TelephoneE164TrialIdentificationCode>,
    pub e164_subscriber_number: Option<types::telephone_e164_subscriber_number::TelephoneE164SubscriberNumber>,

}
