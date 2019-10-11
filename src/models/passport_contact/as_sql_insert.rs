use crate::models::passport_contact::passport_contact::PassportContact;

impl crate::traits::as_sql_insert::AsSqlInsert for PassportContact {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"passport_contacts\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"country_text\", \
            \"number_text\", \
            \"valid_start_date\", \
            \"valid_stop_date\"\
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

            // Place-related
            self.country_text.clone().unwrap(),
            self.number_text.clone().unwrap(),

            // Time-related
            self.valid_start_date.clone().unwrap(),
            self.valid_stop_date.clone().unwrap(),

        );
    }

}
