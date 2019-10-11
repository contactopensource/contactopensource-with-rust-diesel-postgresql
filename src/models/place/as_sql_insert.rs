use crate::models::place::place::Place;

impl crate::traits::as_sql_insert::AsSqlInsert for Place {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"places\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"name\"\
            ) \
            VALUES \
            ($1, $2, $3, $4, $5, $6, $7, $8) \
            -- binds: [{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",

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

        );
    }

}