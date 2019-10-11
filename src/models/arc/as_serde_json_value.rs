use serde_json::json;
use crate::models::arc::arc::Arc;

impl crate::traits::as_serde_json_value::AsSerdeJsonValue for Arc {

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

            // Subject
            "subject_uri": self.subject_uri.clone().to_owned(),
            "subject_database": self.subject_database.clone().to_owned(),
            "subject_schema": self.subject_schema.clone().to_owned(),
            "subject_table": self.subject_table.clone().to_owned(),
            "subject_id": self.subject_id.clone().to_owned(),

            // Predicate
            "predicate_uri": self.predicate_uri.clone().to_owned(),
            "predicate_database": self.predicate_database.clone().to_owned(),
            "predicate_schema": self.predicate_schema.clone().to_owned(),
            "predicate_table": self.predicate_table.clone().to_owned(),
            "predicate_id": self.predicate_id.clone().to_owned(),

            // Object
            "object_uri": self.object_uri.clone().to_owned(),
            "object_database": self.object_database.clone().to_owned(),
            "object_schema": self.object_schema.clone().to_owned(),
            "object_table": self.object_table.clone().to_owned(),
            "object_id": self.object_id.clone().to_owned(),

            // Lifecycle
            "start_at_timestamp_utc": self.start_at_timestamp_utc.clone().unwrap().to_owned(),
            "stop_at_timestamp_utc": self.stop_at_timestamp_utc.clone().unwrap().to_owned(),

            // Modifiers
            "count": self.count.clone().unwrap().to_owned(),
            "weight": self.weight.clone().unwrap().to_owned(),
            "probability": self.probability.clone().unwrap().to_owned(),
        })
    }

}