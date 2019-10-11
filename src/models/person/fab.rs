use crate::types;
use crate::models::person::person::Person as T;

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

        // Lifetime-related
        birth_date: Some(types::date::fab()),
        death_date: Some(types::date::fab()),

        // Physical-related
        mass_as_grams: Some(types::grams::fab()),
        height_as_meters: Some(types::meters::fab()),

        // Org-related
        org_name: Some(types::text::fab()),
        org_team: Some(types::text::fab()),
        org_role: Some(types::text::fab()),

    }
}
