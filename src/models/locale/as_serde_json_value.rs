use serde_json::json;
use crate::models::locale::locale::Locale;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for Locale {

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
            "text": self.text.clone().unwrap().to_owned(),
            "language_code": self.language_code.clone().unwrap().to_owned(),
            "country_code": self.country_code.clone().unwrap().to_owned(),
            "script_code": self.script_code.clone().unwrap().to_owned(),
            "region_code": self.region_code.clone().unwrap().to_owned(),
            "variant_code": self.variant_code.clone().unwrap().to_owned(),

            // Representation-related
            "decimal_separator": self.decimal_separator.clone().unwrap().to_owned(),
            "grouping_separator": self.grouping_separator.clone().unwrap().to_owned(),
            "currency_code": self.currency_code.clone().unwrap().to_owned(),
            "currency_symbol": self.currency_symbol.clone().unwrap().to_owned(),
            "quotation_start_delimiter": self.quotation_start_delimiter.clone().unwrap().to_owned(),
            "quotation_stop_delimiter": self.quotation_stop_delimiter.clone().unwrap().to_owned(),
        })
    }

}