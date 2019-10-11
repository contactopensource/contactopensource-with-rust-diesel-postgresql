use crate::types;
use crate::models::locale::locale::Locale as T;

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

        // Code-related
        text: Some(types::locale_string::fab()),
        language_code: Some(types::locale_language_code::fab()),
        country_code: Some(types::locale_country_code::fab()),
        script_code: Some(types::locale_script_code::fab()),
        region_code: Some(types::locale_region_code::fab()),
        variant_code: Some(types::locale_variant_code::fab()),

        // Representation-related
        decimal_separator: Some(types::locale_decimal_separator::fab()),
        grouping_separator: Some(types::locale_grouping_separator::fab()),
        currency_code: Some(types::currency_code::fab()),
        currency_symbol: Some(types::currency_symbol::fab()),
        quotation_start_delimiter: Some(types::quotation_start_delimiter::fab()),
        quotation_stop_delimiter: Some(types::quotation_stop_delimiter::fab()),

    }
}
