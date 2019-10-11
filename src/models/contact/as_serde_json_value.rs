use serde_json::json;
use crate::models::contact::contact::Contact;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for Contact {

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

            // General-related
            "name": self.name.clone().unwrap().to_owned(),
            "emoji": self.emoji.clone().unwrap().to_owned(),

            // Display-related
            "image_uri": self.image_uri.clone().unwrap().to_owned(),
            "color_hex": self.color_hex.clone().unwrap().to_owned(),
            "css_class": self.css_class.clone().unwrap().to_owned(),
            "star_count": self.star_count.clone().unwrap().to_owned(),
        })
    }

}