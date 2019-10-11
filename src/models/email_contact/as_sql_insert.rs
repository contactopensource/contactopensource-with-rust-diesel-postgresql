use crate::models::email_contact::email_contact::EmailContact;

impl crate::traits::as_sql_insert::AsSqlInsert for EmailContact {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"email_contacts\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"address\", \
            \"display_name\", \
            \"addr_spec\"\
            ) \
            VALUES \
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) \
            -- binds: [{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",

            self.id.clone(),

            // Programming-related
            self.tenant_id.clone().unwrap(),
            self.typecast.clone().unwrap(),
            self.state.clone().unwrap(),

            // Update-related
            self.updated_at_timestamp_utc.clone().unwrap(),
            self.updated_at_clock_count.clone().unwrap(),
            self.updated_by_text.clone().unwrap(),

            // Email-related
            self.address.clone().unwrap(),
            self.display_name.clone().unwrap(),
            self.addr_spec.clone().unwrap(),

        );
    }

}