use crate::types;
use crate::models::postal_contact::postal_contact::PostalContact as T;

impl crate::traits::fab_able::FabAble for T {
    fn fab() -> T {
        Self {
            id: types::id::fab(),

            // Programming-related
            tenant_id: Some(types::id::fab()),
            typecast: Some(types::typecast::fab()),
            state: Some(types::state::fab()),

            // Update-related
            updated_at_timestamp_utc: Some(types::timestamp::fab()),
            updated_at_clock_count: Some(types::count::fab()),
            updated_by_text: Some(types::text::fab()),

            // Postal-related
            country_text: Some(types::text::fab()),
            region_text: Some(types::text::fab()),
            locality_text: Some(types::text::fab()),
            neighborhood_text: Some(types::text::fab()),
            postal_code_text: Some(types::text::fab()),
            street_address_text: Some(types::text::fab()),
            premise_address_text: Some(types::text::fab()),
            global_location_number_text: Some(types::global_location_number_text::fab()),

        }
    }
}
