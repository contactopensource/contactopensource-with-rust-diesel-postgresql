use crate::models::person_name::person_name::PersonName;

impl crate::traits::as_sql_insert::AsSqlInsert for PersonName {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"person_names\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"given_name\", \
            \"given_name_phonetic\", \
            \"middle_name\", \
            \"middle_name_phonetic\", \
            \"family_name\", \
            \"family_name_phonetic\", \
            \"legal_name\", \
            \"legal_name_phonetic\", \
            \"prefix_name\", \
            \"prefix_name_phonetic\", \
            \"suffix_name\", \
            \"suffix_name_phonetic\", \
            \"salutation_name\", \
            \"salutation_name_phonetic\", \
            \"addressee_name\", \
            \"addressee_name_phonetic\", \
            \"nickname\", \
            \"nickname_phonetic\"\
            ) \
            VALUES \
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25) \
            -- binds: [{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",

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
            self.given_name.clone().unwrap(),
            self.given_name_phonetic.clone().unwrap(),
            self.middle_name.clone().unwrap(),
            self.middle_name_phonetic.clone().unwrap(),
            self.family_name.clone().unwrap(),
            self.family_name_phonetic.clone().unwrap(),
            self.legal_name.clone().unwrap(),
            self.legal_name_phonetic.clone().unwrap(),
            self.prefix_name.clone().unwrap(),
            self.prefix_name_phonetic.clone().unwrap(),
            self.suffix_name.clone().unwrap(),
            self.suffix_name_phonetic.clone().unwrap(),
            self.salutation_name.clone().unwrap(),
            self.salutation_name_phonetic.clone().unwrap(),
            self.addressee_name.clone().unwrap(),
            self.addressee_name_phonetic.clone().unwrap(),
            self.nickname.clone().unwrap(),
            self.nickname_phonetic.clone().unwrap(),

        );
    }

}