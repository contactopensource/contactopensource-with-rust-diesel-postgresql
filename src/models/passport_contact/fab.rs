use crate::types;
use crate::models::passport_contact::passport_contact::PassportContact as T;

pub fn fab() -> T {
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

        // Passport-related
        country_text: Some(types::passport_country_text::fab()),
        number_text: Some(types::passport_number_text::fab()),

        // Time-related
        valid_start_date: Some(types::date::fab()),
        valid_stop_date: Some(types::date::fab()),

    }
}
