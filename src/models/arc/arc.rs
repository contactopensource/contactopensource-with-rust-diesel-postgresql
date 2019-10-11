use crate::types;
use crate::schema::arcs;

#[derive(Debug, Identifiable, Deserialize, Queryable, PartialEq, Insertable, AsChangeset)]
#[table_name = "arcs"]
pub struct Arc {
    pub id: types::id::Id,

    // Programming-related
    pub tenant_id: Option<types::id::Id>,
    pub typecast: Option<types::typecast::Typecast>,
    pub state: Option<types::state::State>,

    // Update-related
    pub updated_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub updated_at_clock_count: Option<types::count::Count>,
    pub updated_by_text: Option<types::text::Text>,

    // Subject
    pub subject_uri: Option<types::uri_string::URIString>,
    pub subject_database: Option<types::db_database_name::DbDatabaseName>,
    pub subject_schema: Option<types::db_schema_name::DbSchemaName>,
    pub subject_table: Option<types::db_table_name::DbTableName>,
    pub subject_id: Option<types::db_row_id::DbRowId>,

    // Predicate
    pub predicate_uri: Option<types::uri_string::URIString>,
    pub predicate_database: Option<types::db_database_name::DbDatabaseName>,
    pub predicate_schema: Option<types::db_schema_name::DbSchemaName>,
    pub predicate_table: Option<types::db_table_name::DbTableName>,
    pub predicate_id: Option<types::db_row_id::DbRowId>,

    // Object
    pub object_uri: Option<types::uri_string::URIString>,
    pub object_database: Option<types::db_database_name::DbDatabaseName>,
    pub object_schema: Option<types::db_schema_name::DbSchemaName>,
    pub object_table: Option<types::db_table_name::DbTableName>,
    pub object_id: Option<types::db_row_id::DbRowId>,

    // Lifecycle
    pub start_at_timestamp_utc: Option<types::timestamp::Timestamp>,
    pub stop_at_timestamp_utc: Option<types::timestamp::Timestamp>,

    // Display-related
    pub count: Option<types::count::Count>,
    pub weight: Option<types::weight::Weight>,
    pub probability: Option<types::probability::Probability>,

}
