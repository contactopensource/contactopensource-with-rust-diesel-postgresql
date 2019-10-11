pub fn option_str_to_option_json(s: Option<&str>) -> Option<serde_json::Value> {
    match s {
        None => None,
        Some(s) => {
            match serde_json::from_str(s) {
                Ok(x) => Some(x),
                Err(e) => panic!("Cannot parse string to JSON. string:{} err:{:?}", s, e),
            }
        }
    }
}
