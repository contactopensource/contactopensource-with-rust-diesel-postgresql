pub fn uuid_as_string() -> String {
    Uuid::new_v4().to_string()
}

pub fn uuid_as_uuid() -> Uuid {
    Uuid::new_v4()
}
