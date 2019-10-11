//! Typical color hexadecimal uppercase string.
//!
//! Examples:
//!
//!   * "FF0088" is deep pink

use crate::types::{hex_upper_string as supertype, hex_upper_string::HexUpperString as Supertype};

pub type ColorHexUpperString = Supertype;
type T = ColorHexUpperString;

pub const LEN: usize = 6;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    supertype::fab_count(LEN)
}

#[cfg(test)]
mod tests {
    use crate::types::{color_hex_upper_string as t, color_hex_upper_string::ColorHexUpperString as T};

    #[test]
    fn test_from_str() {
        let s = "AAAAAA";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "AAAAAA";
        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert_eq!(x.len(), t::LEN);
    }

}
