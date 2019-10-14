use crate::types;
use crate::models::media_type::media_type::MediaType as T;

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

            // Content-related
            text: Some(types::media_type_text::fab()),
            supertype: Some(types::media_type_supertype::fab()),
            subtype: Some(types::media_type_subtype::fab()),
            tree: Some(types::media_type_tree::fab()),
            suffix: Some(types::media_type_suffix::fab()),
            parameters: Some(types::media_type_parameters::fab()),

        }
    }
}