use serde_json::json;
use crate::models::email_contact::email_contact::EmailContact;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for EmailContact {

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

            // Email-related
            "address": self.address.clone().unwrap().to_owned(),
            "display_name": self.display_name.clone().unwrap().to_owned(),
            "addr_spec": self.addr_spec.clone().unwrap().to_owned(),
        })
    }

}