use serde_json::json;
use crate::models::item::item::Item;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for Item {

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

            // Meta-data-related
            "uri": self.uri.clone().unwrap().to_owned(),

            // Item-related
            "text": self.text.clone().unwrap().to_owned(),
            "json": self.json.clone().unwrap().to_owned(),
            "xml": self.xml.clone().unwrap().to_owned(),
            "number": self.number.clone().unwrap().to_owned(),
        })
    }

}