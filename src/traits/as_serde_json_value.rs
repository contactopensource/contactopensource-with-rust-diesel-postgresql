pub trait AsSerdeJsonValue {
    fn as_serde_json_value(&self) -> serde_json::Value;
}
