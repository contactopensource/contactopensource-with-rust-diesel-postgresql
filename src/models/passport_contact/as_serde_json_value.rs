use serde_json::json;
use crate::models::passport_contact::passport_contact::PassportContact;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for PassportContact {

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

            // Place-related
            "country_text": self.country_text.clone().unwrap().to_owned(),
            "number_text": self.number_text.clone().unwrap().to_owned(),

            // Time-related
            "valid_start_date": self.valid_start_date.clone().unwrap().to_owned(),
            "valid_stop_date": self.valid_stop_date.clone().unwrap().to_owned(),
        })
    }

}
