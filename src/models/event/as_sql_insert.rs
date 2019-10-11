use crate::models::event::event::Event;

impl crate::traits::as_sql_insert::AsSqlInsert for Event {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"events\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"name\", \
            \"start_timestamp_utc\", \
            \"stop_timestamp_utc\", \
            \"duration_as_seconds\"\
            ) \
            VALUES \
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) \
            -- binds: [{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",

            self.id.clone(),

            // Programming-related
            self.tenant_id.clone().unwrap(),
            self.typecast.clone().unwrap(),
            self.state.clone().unwrap(),

            // Update-related
            self.updated_at_timestamp_utc.clone().unwrap(),
            self.updated_at_clock_count.clone().unwrap(),
            self.updated_by_text.clone().unwrap(),

            // Name-related
            self.name.clone().unwrap(),

            // Lifetime-related
            self.start_timestamp_utc.clone().unwrap(),
            self.stop_timestamp_utc.clone().unwrap(),
            self.duration_as_seconds.clone().unwrap(),

        );
    }

}