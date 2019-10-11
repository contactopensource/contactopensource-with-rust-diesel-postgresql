use ::uuid::Uuid;

pub fn option_str_to_option_uuid(s: Option<&str>) -> Option<Uuid> {
    match s {
        None => None,
        Some(s) => {
            match Uuid::parse_str(&s) {
                Ok(x) => Some(x),
                Err(e) => panic!("Cannot parse string to UUID; string:{} err:{:?}", s, e),
            }
        }
    }
}
