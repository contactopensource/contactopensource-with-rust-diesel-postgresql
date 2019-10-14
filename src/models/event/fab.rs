use crate::types;
use crate::models::event::event::Event as T;

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

            // Name-related
            name: Some(types::text::fab()),

            // Lifetime-related
            start_timestamp_utc: Some(types::timestamp::fab()),
            stop_timestamp_utc: Some(types::timestamp::fab()),
            duration_as_seconds: Some(types::seconds::fab()),

        }
    }
}
