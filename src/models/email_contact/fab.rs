use crate::types;
use crate::models::email_contact::email_contact::EmailContact as T;

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

        // Email-related
        address: Some(types::email_address::fab()),
        display_name: Some(types::email_display_name::fab()),
        addr_spec: Some(types::email_addr_spec::fab()),

    }
}
