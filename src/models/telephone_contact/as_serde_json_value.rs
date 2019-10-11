use serde_json::json;
use crate::models::telephone_contact::telephone_contact::TelephoneContact;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for TelephoneContact {

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

            // Telephone-related
            "label": self.label.clone().unwrap().to_owned(),
            "number_text": self.number_text.clone().unwrap().to_owned(),

            // E.164-related
            "e164_text": self.e164_text.clone().unwrap().to_owned(),
            "e164_country_code": self.e164_country_code.clone().unwrap().to_owned(),
            "e164_national_destination_code": self.e164_national_destination_code.clone().unwrap().to_owned(),
            "e164_group_identification_code": self.e164_group_identification_code.clone().unwrap().to_owned(),
            "e164_trial_identification_code": self.e164_trial_identification_code.clone().unwrap().to_owned(),
            "e164_subscriber_number": self.e164_subscriber_number.clone().unwrap().to_owned(),
        })
    }

}
