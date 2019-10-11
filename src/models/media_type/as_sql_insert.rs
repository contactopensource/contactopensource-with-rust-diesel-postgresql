use crate::models::media_type::media_type::MediaType;

impl crate::traits::as_sql_insert::AsSqlInsert for MediaType {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"media_types\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"text\", \
            \"supertype\", \
            \"subtype\", \
            \"tree\", \
            \"suffix\", \
            \"parameters\"\
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

            // Media-related
            self.text.clone().unwrap(),
            self.supertype.clone().unwrap(),
            self.subtype.clone().unwrap(),
            self.tree.clone().unwrap(),
            self.suffix.clone().unwrap(),
            self.parameters.clone().unwrap(),

        );
    }

}