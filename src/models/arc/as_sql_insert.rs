use crate::models::arc::arc::Arc;

impl crate::traits::as_sql_insert::AsSqlInsert for Arc {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"arcs\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"subject_uri\", \
            \"subject_database\", \
            \"subject_schema\", \
            \"subject_table\", \
            \"subject_id\", \
            \"predicate_uri\", \
            \"predicate_database\", \
            \"predicate_schema\", \
            \"predicate_table\", \
            \"predicate_id\", \
            \"object_uri\", \
            \"object_database\", \
            \"object_schema\", \
            \"object_table\", \
            \"object_id\", \
            \"start_at_timestamp_utc\", \
            \"stop_at_timestamp_utc\", \
            \"count\", \
            \"weight\", \
            \"probability\"\
            ) \
            VALUES \
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27) \
            -- binds: [{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",

            self.id.clone(),

            // Programming-related
            self.tenant_id.clone().unwrap(),
            self.typecast.clone().unwrap(),
            self.state.clone().unwrap(),

            // Update-related
            self.updated_at_timestamp_utc.clone().unwrap(),
            self.updated_at_clock_count.clone().unwrap(),
            self.updated_by_text.clone().unwrap(),

            // Subject
            self.subject_uri.clone().unwrap(),
            self.subject_database.clone().unwrap(),
            self.subject_schema.clone().unwrap(),
            self.subject_table.clone().unwrap(),
            self.subject_id.clone().unwrap(),

            // Predicate
            self.predicate_uri.clone().unwrap(),
            self.predicate_database.clone().unwrap(),
            self.predicate_schema.clone().unwrap(),
            self.predicate_table.clone().unwrap(),
            self.predicate_id.clone().unwrap(),

            // Object
            self.object_uri.clone().unwrap(),
            self.object_database.clone().unwrap(),
            self.object_schema.clone().unwrap(),
            self.object_table.clone().unwrap(),
            self.object_id.clone().unwrap(),

            // Lifecycle
            self.start_at_timestamp_utc.clone().unwrap(),
            self.stop_at_timestamp_utc.clone().unwrap(),

            // Modifiers
            self.count.clone().unwrap(),
            self.weight.clone().unwrap(),
            self.probability.clone().unwrap(),

        );
    }

}