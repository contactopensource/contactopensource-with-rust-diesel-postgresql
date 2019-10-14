use crate::types;
use crate::models::person_name::person_name::PersonName as T;

impl crate::traits::fab_able::FabAble<T> for T {
    fn fab() -> T {
        T {
            id: types::id::fab(),

            // Programming-related
            tenant_id: Some(types::id::fab()),
            typecast: Some(types::typecast::fab()),
            state: Some(types::state::fab()),

            // Update-related
            updated_at_timestamp_utc: Some(types::timestamp::fab()),
            updated_at_clock_count: Some(types::count::fab()),
            updated_by_text: Some(types::text::fab()),

            // Name-related
            given_name: Some(types::person_given_name::fab()),
            given_name_phonetic: Some(types::person_given_name::fab()),
            middle_name: Some(types::person_middle_name::fab()),
            middle_name_phonetic: Some(types::person_middle_name::fab()),
            family_name: Some(types::person_family_name::fab()),
            family_name_phonetic: Some(types::person_family_name::fab()),
            legal_name: Some(types::person_legal_name::fab()),
            legal_name_phonetic: Some(types::person_legal_name::fab()),
            prefix_name: Some(types::person_prefix_name::fab()),
            prefix_name_phonetic: Some(types::person_prefix_name::fab()),
            suffix_name: Some(types::person_suffix_name::fab()),
            suffix_name_phonetic: Some(types::person_suffix_name::fab()),
            salutation_name: Some(types::person_salutation_name::fab()),
            salutation_name_phonetic: Some(types::person_salutation_name::fab()),
            addressee_name: Some(types::person_addressee_name::fab()),
            addressee_name_phonetic: Some(types::person_addressee_name::fab()),
            nickname: Some(types::person_nickname::fab()),
            nickname_phonetic: Some(types::person_nickname::fab()),

        }
    }
}
