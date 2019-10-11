//! Typical star count, such as a rating. Range 0..=5.
//!
//! Example: 5 means 5 star count, such as the best.

use ::rand::{Rng, thread_rng};

pub type StarCount = i16; // For now, must be equivalent to Diesel small int.
type T = StarCount;

pub type StarCountParseError = ::std::num::ParseIntError;
type E = StarCountParseError;

pub fn from_str(s: &str) -> Result<T, E> {
    s.parse::<T>()
}

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    crate::helpers::parse::a::option_str_to_option_t::<T>(s)
}

pub fn fab() -> T {
    thread_rng().gen_range(0, 5)
}

#[cfg(test)]
mod tests {
    use crate::types::{star_count as t, star_count::StarCount as T};

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
