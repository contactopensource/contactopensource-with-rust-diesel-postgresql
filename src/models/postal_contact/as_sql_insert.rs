use crate::models::postal_contact::postal_contact::PostalContact;

impl crate::traits::as_sql_insert::AsSqlInsert for PostalContact {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"postal_contacts\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"country_text\", \
            \"region_text\", \
            \"locality_text\", \
            \"neighborhood_text\", \
            \"postal_code_text\", \
            \"street_address_text\", \
            \"premise_address_text\", \
            \"global_location_number_text\"\
            ) \
            VALUES \
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15) \
            -- binds: [{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",

            self.id.clone(),

            // Programming-related
            self.tenant_id.clone().unwrap(),
            self.typecast.clone().unwrap(),
            self.state.clone().unwrap(),

            // Update-related
            self.updated_at_timestamp_utc.clone().unwrap(),
            self.updated_at_clock_count.clone().unwrap(),
            self.updated_by_text.clone().unwrap(),

            // Postal-related
            self.country_text.clone().unwrap(),
            self.region_text.clone().unwrap(),
            self.locality_text.clone().unwrap(),
            self.neighborhood_text.clone().unwrap(),
            self.postal_code_text.clone().unwrap(),
            self.street_address_text.clone().unwrap(),
            self.premise_address_text.clone().unwrap(),
            self.global_location_number_text.clone().unwrap(),

        );
    }

}