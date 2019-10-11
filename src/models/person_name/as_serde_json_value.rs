use serde_json::json;
use crate::models::person_name::person_name::PersonName;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for PersonName {

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

            // Name-related
            "given_name": self.given_name.clone().unwrap().to_owned(),
            "given_name_phonetic": self.given_name_phonetic.clone().unwrap().to_owned(),
            "middle_name": self.middle_name.clone().unwrap().to_owned(),
            "middle_name_phonetic": self.middle_name_phonetic.clone().unwrap().to_owned(),
            "family_name": self.family_name.clone().unwrap().to_owned(),
            "family_name_phonetic": self.family_name_phonetic.clone().unwrap().to_owned(),
            "legal_name": self.legal_name.clone().unwrap().to_owned(),
            "legal_name_phonetic": self.legal_name_phonetic.clone().unwrap().to_owned(),
            "prefix_name": self.prefix_name.clone().unwrap().to_owned(),
            "prefix_name_phonetic": self.prefix_name_phonetic.clone().unwrap().to_owned(),
            "suffix_name": self.suffix_name.clone().unwrap().to_owned(),
            "suffix_name_phonetic": self.suffix_name_phonetic.clone().unwrap().to_owned(),
            "salutation_name": self.salutation_name.clone().unwrap().to_owned(),
            "salutation_name_phonetic": self.salutation_name_phonetic.clone().unwrap().to_owned(),
            "addressee_name": self.addressee_name.clone().unwrap().to_owned(),
            "addressee_name_phonetic": self.addressee_name_phonetic.clone().unwrap().to_owned(),
            "nickname": self.nickname.clone().unwrap().to_owned(),
            "nickname_phonetic": self.nickname_phonetic.clone().unwrap().to_owned(),

        })
    }

}