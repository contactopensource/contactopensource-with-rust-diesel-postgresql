use serde_json::json;
use crate::models::person::person::Person;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for Person {

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

            // Lifetime-related
            "birth_date": self.birth_date.clone().unwrap().to_owned(),
            "death_date": self.death_date.clone().unwrap().to_owned(),

            // Physical-related
            "mass_as_grams": self.mass_as_grams.clone().unwrap().to_owned(),
            "height_as_meters": self.height_as_meters.clone().unwrap().to_owned(),

            // Org-related
            "org_name": self.org_name.clone().unwrap().to_owned(),
            "org_team": self.org_team.clone().unwrap().to_owned(),
            "org_role": self.org_role.clone().unwrap().to_owned(),
        })
    }

}