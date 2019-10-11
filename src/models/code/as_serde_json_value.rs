use serde_json::json;
use crate::models::code::code::Code;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for Code {

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

            // Code-related
            "set_id": self.set_id.clone().unwrap().to_owned(),
            "text": self.text.clone().unwrap().to_owned(),
            "name": self.name.clone().unwrap().to_owned(),
        })
    }

}