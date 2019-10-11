use crate::models::telephone_contact::telephone_contact::TelephoneContact;

impl crate::traits::as_sql_insert::AsSqlInsert for TelephoneContact {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"telephone_contacts\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"label\", \
            \"number_text\", \
            \"e164_text\", \
            \"e164_country_code\", \
            \"e164_national_destination_code\", \
            \"e164_group_identification_code\", \
            \"e164_trial_identification_code\", \
            \"e164_subscriber_number\"\
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

            // Telephone-related
            self.label.clone().unwrap(),
            self.number_text.clone().unwrap(),

            // E.164-related
            self.e164_text.clone().unwrap(),
            self.e164_country_code.clone().unwrap(),
            self.e164_national_destination_code.clone().unwrap(),
            self.e164_group_identification_code.clone().unwrap(),
            self.e164_trial_identification_code.clone().unwrap(),
            self.e164_subscriber_number.clone().unwrap(),

        );
    }

}
