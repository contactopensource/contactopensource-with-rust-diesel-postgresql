use crate::models::geo_code::geo_code::GeoCode;

impl crate::traits::as_sql_insert::AsSqlInsert for GeoCode {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"geo_codes\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"coder_id\", \
            \"text\", \
            \"latitude\", \
            \"longitude\", \
            \"altitude\", \
            \"elevation\"\
            ) \
            VALUES \
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) \
            -- binds: [{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",

            self.id.clone(),

            // Programming-related
            self.tenant_id.clone().unwrap(),
            self.typecast.clone().unwrap(),
            self.state.clone().unwrap(),

            // Update-related
            self.updated_at_timestamp_utc.clone().unwrap(),
            self.updated_at_clock_count.clone().unwrap(),
            self.updated_by_text.clone().unwrap(),

            // Code-related
            self.coder_id.clone().unwrap(),
            self.text.clone().unwrap(),
            self.latitude.clone().unwrap(),
            self.longitude.clone().unwrap(),
            self.altitude.clone().unwrap(),
            self.elevation.clone().unwrap(),

        );
    }

}