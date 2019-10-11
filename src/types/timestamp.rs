//! Timestamp that must be in UTC time, and that must be compatible with SQL standard timestamp.
//!
//! Example: TODO

use crate::types::{timestamp as t};

pub type Timestamp = ::chrono::NaiveDateTime;
type T = Timestamp;

pub type TimestampParseError = ::chrono::format::ParseError;
type E = TimestampParseError;

pub const FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

pub fn from_str(s: &str) -> Result<T, E> {
    ::chrono::NaiveDateTime::parse_from_str(s, t::FORMAT)
}

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    crate::helpers::parse::chrono::option_str_to_option_naive_date_time(s)
}

pub fn fab() -> T {
    crate::helpers::fab::chrono::timestamp_utc()
}

#[cfg(test)]
mod tests {
    use crate::types::{timestamp as t, timestamp::Timestamp as T};

    #[test]
    fn test_from_str() {
        let s = "2020-12-31T23:59:59Z";
        let e = ::chrono::NaiveDate::from_ymd(2020, 12, 31).and_hms(23, 59, 59);
        let x: T = t::from_str(s).unwrap();
        assert_eq!(x, e);
    }

    #[test]
    fn test_from_option_str() {
        let s = "2020-12-31T23:59:59Z";
        let e = ::chrono::NaiveDate::from_ymd(2020, 12, 31).and_hms(23, 59, 59);
        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, e);
    }

    #[test]
    fn test_fab() {
        let _x: T = t::fab();
    }

}
