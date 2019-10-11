use crate::types;
use crate::schema::person_names;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "person_names"]
pub struct PersonName {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Name-related
    pub given_name: Option<types::person_given_name::PersonGivenName>,
    pub given_name_phonetic: Option<types::person_given_name::PersonGivenName>,
    pub middle_name: Option<types::person_middle_name::PersonMiddleName>,
    pub middle_name_phonetic: Option<types::person_middle_name::PersonMiddleName>,
    pub family_name: Option<types::person_family_name::PersonFamilyName>,
    pub family_name_phonetic: Option<types::person_family_name::PersonFamilyName>,
    pub legal_name: Option<types::person_legal_name::PersonLegalName>,
    pub legal_name_phonetic: Option<types::person_legal_name::PersonLegalName>,
    pub prefix_name: Option<types::person_prefix_name::PersonPrefixName>,
    pub prefix_name_phonetic: Option<types::person_prefix_name::PersonPrefixName>,
    pub suffix_name: Option<types::person_suffix_name::PersonSuffixName>,
    pub suffix_name_phonetic: Option<types::person_suffix_name::PersonSuffixName>,
    pub salutation_name: Option<types::person_salutation_name::PersonSalutationName>,
    pub salutation_name_phonetic: Option<types::person_salutation_name::PersonSalutationName>,
    pub addressee_name: Option<types::person_addressee_name::PersonAddresseeName>,
    pub addressee_name_phonetic: Option<types::person_addressee_name::PersonAddresseeName>,
    pub nickname: Option<types::person_nickname::PersonNickname>,
    pub nickname_phonetic: Option<types::person_nickname::PersonNickname>,

}
