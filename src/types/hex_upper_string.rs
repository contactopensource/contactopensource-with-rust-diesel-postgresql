//! Hexadecimal uppercase text.
//!
//! Example: "0123456789ABCDEF".

use ::rand::{Rng, thread_rng};
use crate::types::{text as supertype, text::Text as Supertype};

pub type HexUpperString = Supertype;
type T = HexUpperString;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub const DIGITS: &'static [&'static str] = &["0","1","2","3","4","5","6","7","8","9","A","B","C","D","E","F"];

pub fn fab() -> T {
    fab_count(thread_rng().gen_range(1, 8))
}

pub fn fab_count(count: usize) -> T {
    crate::types::text::fab_digits_count(DIGITS, count)
}

#[cfg(test)]
mod tests {
    use crate::types::{hex_upper_string as t, hex_upper_string::HexUpperString as T};
    use ::rand::{Rng, thread_rng};

    #[test]
    fn test_from_str() {
        let s = "alpha";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "alpha";
        let x: T = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

    #[test]
    fn test_fab_count() {
        let count = thread_rng().gen_range(1, 88);
        let x: T = t::fab_count(count);
        assert_eq!(x.len(), count);
    }

}
