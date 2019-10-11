use serde_json::json;
use crate::models::person_pronoun::person_pronoun::PersonPronoun;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for PersonPronoun {

    fn as_serde_json_value(&self) -> serde_json::Value {
        json!({
            "id": self.id,

            // Programming-related
            "tenant_id": self.tenant_id.clone().unwrap().to_owned(),
            "typecast": self.typecast.clone().unwrap().to_owned(),
            "state": self.state.clone().unwrap().to_owned(),

            // Update-related
            "updated_at_timestamp_utc": self.updated_at_timestamp_utc.clone().unwrap().to_owned(),
            "updated_at_clock_count": self.updated_at_clock_count.clone().unwrap().to_owned(),
            "updated_by_text": self.updated_by_text.clone().unwrap().to_owned(),

            // Pronoun-related
            "subject_pronoun": self.subject_pronoun.clone().unwrap().to_owned(),
            "object_pronoun": self.object_pronoun.clone().unwrap().to_owned(),
            "dependent_possessive_pronoun": self.dependent_possessive_pronoun.clone().unwrap().to_owned(),
            "independent_possessive_pronoun": self.independent_possessive_pronoun.clone().unwrap().to_owned(),
            "reflexive_pronoun": self.reflexive_pronoun.clone().unwrap().to_owned(),
            "intensive_pronoun": self.intensive_pronoun.clone().unwrap().to_owned(),
            "disjunctive_pronoun": self.disjunctive_pronoun.clone().unwrap().to_owned(),

        })
    }

}