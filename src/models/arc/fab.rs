use crate::types;
use crate::models::arc::arc::Arc as T;

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

            // Subject
            subject_uri: Some(types::uri_string::fab()),
            subject_database: Some(types::db_database_name::fab()),
            subject_schema: Some(types::db_schema_name::fab()),
            subject_table: Some(types::db_table_name::fab()),
            subject_id: Some(types::db_row_id::fab()),

            // Predicate
            predicate_uri: Some(types::uri_string::fab()),
            predicate_database: Some(types::db_database_name::fab()),
            predicate_schema: Some(types::db_schema_name::fab()),
            predicate_table: Some(types::db_table_name::fab()),
            predicate_id: Some(types::db_row_id::fab()),

            // Object
            object_uri: Some(types::uri_string::fab()),
            object_database: Some(types::db_database_name::fab()),
            object_schema: Some(types::db_schema_name::fab()),
            object_table: Some(types::db_table_name::fab()),
            object_id: Some(types::db_row_id::fab()),

            // Lifecycle
            start_at_timestamp_utc: Some(types::timestamp::fab()),
            stop_at_timestamp_utc: Some(types::timestamp::fab()),

            // Modifiers
            count: Some(types::count::fab()),
            weight:  Some(types::weight::fab()),
            probability: Some(types::probability::fab()),

        }
    }
}
