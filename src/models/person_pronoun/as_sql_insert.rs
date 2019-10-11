use crate::models::person_pronoun::person_pronoun::PersonPronoun;

impl crate::traits::as_sql_insert::AsSqlInsert for PersonPronoun {

    fn as_sql_insert(&self) -> String {
        return format!(
            "INSERT INTO \"person_pronouns\" (\
            \"id\", \
            \"tenant_id\", \
            \"typecast\", \
            \"state\", \
            \"updated_at_timestamp_utc\", \
            \"updated_at_clock_count\", \
            \"updated_by_text\", \
            \"subject_pronoun\", \
            \"object_pronoun\", \
            \"dependent_possessive_pronoun\", \
            \"independent_possessive_pronoun\", \
            \"reflexive_pronoun\", \
            \"intensive_pronoun\", \
            \"disjunctive_pronoun\"\
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

            // Pronoun-related
            self.subject_pronoun.clone().unwrap(),
            self.object_pronoun.clone().unwrap(),
            self.dependent_possessive_pronoun.clone().unwrap(),
            self.independent_possessive_pronoun.clone().unwrap(),
            self.reflexive_pronoun.clone().unwrap(),
            self.intensive_pronoun.clone().unwrap(),
            self.disjunctive_pronoun.clone().unwrap(),

        );
    }

}