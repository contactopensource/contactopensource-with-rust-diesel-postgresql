use crate::models::contact::contact::Contact;

impl crate::traits::as_sql_insert::AsSqlInsert for Contact {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"contacts\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"name\", \
            \"emoji\", \
            \"image_uri\", \
            \"color_hex\", \
            \"css_class\", \
            \"star_count\"\
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

            // General-related
            self.name.clone().unwrap(),
            self.emoji.clone().unwrap(),

            // Dislay-related
            self.image_uri.clone().unwrap(),
            self.color_hex.clone().unwrap(),
            self.css_class.clone().unwrap(),
            self.star_count.clone().unwrap(),

        );
    }

}