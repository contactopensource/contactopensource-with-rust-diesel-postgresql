use crate::types;
use crate::models::link_contact::link_contact::LinkContact as T;

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

            // Link-related
            label: Some(types::label::fab()),
            uri: Some(types::uri_string::fab()),

        }
    }
}
