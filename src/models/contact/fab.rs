use crate::types;
use crate::models::contact::contact::Contact as T;

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

        // General-related
        name: Some(types::text::fab()),
        emoji: Some(types::text::fab()),

        // Display-related
        image_uri: Some(types::uri_string::fab()),
        color_hex: Some(types::color_hex_upper_string::fab()),
        css_class: Some(types::text::fab()),
        star_count: Some(types::star_count::fab()),

    }
}
