use chrono::NaiveDate;
use chrono::NaiveDateTime;

pub fn option_str_to_option_naive_date(s: Option<&str>) -> Option<NaiveDate> {
    match s {
        None => None,
        Some(s) => {
            match chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                Ok(x) => Some(x),
                Err(e) => panic!("Cannot parse string to NaiveDate; string:{} err:{:?}", s, e),
            }
        }
    }
}

pub fn option_str_to_option_naive_date_time(s: Option<&str>) -> Option<NaiveDateTime> {
    match s {
        None => None,
        Some(s) => {
            match chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%SZ") {
                Ok(x) => Some(x),
                Err(e) => panic!("Cannot parse string to NaiveDateTime; string:{} err:{:?}", s, e),
            }
        }
    }
}
