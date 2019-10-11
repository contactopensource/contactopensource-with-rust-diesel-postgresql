//! Typical count. Range 0..I.
//!
//! Example: 8 means count 8.

use ::rand::{Rng, thread_rng};

pub type Count = i64;
type T = Count;

pub type CountParseError = ::std::num::ParseIntError;
type E = CountParseError;

pub fn from_str(s: &str) -> Result<T, E> {
    s.parse::<T>()
}

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    crate::helpers::parse::a::option_str_to_option_t::<T>(s)
}

pub fn fab() -> T {
    thread_rng().gen_range(1, 888)
}

#[cfg(test)]
mod tests {
    use crate::types::{count as t, count::Count as T};

    #[test]
    fn test_from_str() {
        let s = "0";
        let x: T = t::from_str(s).unwrap();
        assert_eq!(x, 0);
    }

    #[test]
    fn test_from_option_str() {
        let s = "0";
        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, 0);
    }

    #[test]
    fn test_fab() {
        let _x: T = t::fab();
    }

}
