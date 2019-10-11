use serde_json::json;
use crate::models::postal_contact::postal_contact::PostalContact;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for PostalContact {

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

            // Postal-related
            "country_text": self.country_text.clone().unwrap().to_owned(),
            "region_text": self.region_text.clone().unwrap().to_owned(),
            "locality_text": self.locality_text.clone().unwrap().to_owned(),
            "neighborhood_text": self.neighborhood_text.clone().unwrap().to_owned(),
            "postal_code_text": self.postal_code_text.clone().unwrap().to_owned(),
            "street_address_text": self.street_address_text.clone().unwrap().to_owned(),
            "premise_address_text": self.premise_address_text.clone().unwrap().to_owned(),
            "global_location_number_text": self.global_location_number_text.clone().unwrap().to_owned(),
        })
    }

}