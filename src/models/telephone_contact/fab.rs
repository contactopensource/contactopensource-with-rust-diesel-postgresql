use crate::types;
use crate::models::telephone_contact::telephone_contact::TelephoneContact as T;

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

            // Telephone-related
            label: Some(types::telephone_label::fab()),
            number_text: Some(types::telephone_number_text::fab()),

            // E.164-related
            e164_text: Some(types::telephone_e164_text::fab()),
            e164_country_code: Some(types::telephone_e164_country_code::fab()),
            e164_national_destination_code: Some(types::telephone_e164_national_destination_code::fab()),
            e164_group_identification_code: Some(types::telephone_e164_group_identification_code::fab()),
            e164_trial_identification_code: Some(types::telephone_e164_trial_identification_code::fab()),
            e164_subscriber_number: Some(types::telephone_e164_subscriber_number::fab()),

        }
    }
}
