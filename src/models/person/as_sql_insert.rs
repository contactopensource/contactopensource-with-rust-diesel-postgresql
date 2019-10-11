use crate::models::person::person::Person;

impl crate::traits::as_sql_insert::AsSqlInsert for Person {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"persons\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"birth_date\", \
            \"death_date\", \
            \"mass_as_grams\", \
            \"height_as_meters\", \
            \"org_name\", \
            \"org_team\", \
            \"org_role\"\
            ) \
            VALUES \
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) \
            -- binds: [{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",

            self.id.clone(),

            // Programming-related
            self.tenant_id.clone().unwrap(),
            self.typecast.clone().unwrap(),
            self.state.clone().unwrap(),

            // Update-related
            self.updated_at_timestamp_utc.clone().unwrap(),
            self.updated_at_clock_count.clone().unwrap(),
            self.updated_by_text.clone().unwrap(),

            // Lifetime-related
            self.birth_date.clone().unwrap(),
            self.death_date.clone().unwrap(),

            // Physical-related
            self.mass_as_grams.clone().unwrap(),
            self.height_as_meters.clone().unwrap(),

            // Org-related
            self.org_name.clone().unwrap(),
            self.org_team.clone().unwrap(),
            self.org_role.clone().unwrap(),

        );
    }

}