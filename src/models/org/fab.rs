use crate::types;
use crate::models::org::org::Org as T;

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

            // Lifetime-related
            start_date: Some(crate::helpers::fab::chrono::date_in_past_as_naive_utc()),
            stop_date: Some(crate::helpers::fab::chrono::date_in_future_as_naive_utc()),

        }
    }
}
