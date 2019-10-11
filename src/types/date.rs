//! Date that must be compatible with Diesel and its SQL standard timestamp.
//!
//! Example: TODO

pub type Date = ::chrono::NaiveDate;
type T = Date;

pub type DateParseError = ::chrono::format::ParseError;
type E = DateParseError;

pub fn from_str(s: &str) -> Result<T, E> {
    ::chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    crate::helpers::parse::chrono::option_str_to_option_naive_date(s)
}

pub fn fab() -> T {
    crate::helpers::fab::chrono::today_as_naive_utc()
}

#[cfg(test)]
mod tests {
    use crate::types::{date as t, date::Date as T};

    #[test]
    fn test_from_str() {
        let s = "2020-12-31";
        let e = ::chrono::NaiveDate::from_ymd(2020, 12, 31);
        let x: T = t::from_str(s).unwrap();
        assert_eq!(x, e);
    }

    #[test]
    fn test_from_option_str() {
        let s = "2020-12-31";
        let e = ::chrono::NaiveDate::from_ymd(2020, 12, 31);
        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, e);
    }

    #[test]
    fn test_fab() {
        let _x: T = t::fab();
    }

}
