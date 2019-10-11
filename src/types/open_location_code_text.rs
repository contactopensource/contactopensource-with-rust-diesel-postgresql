//! Open Location Code geolocation code.
//!
//! See https://github.com/google/open-location-code
//!
//! Example: "6GCRPR6C+24" is the Parliament Buildings in Nairobi, Kenya.

use crate::types::{text as supertype, text::Text as Supertype};

pub type OpenLocationCodeText = Supertype;
type T = OpenLocationCodeText;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

// Open Location Code terminology is "digits" instead of e.g. "characters".
pub const DIGITS: &'static[&'static str] = &["2","3","4","5","6","7","8","9","C","F","G","H","J","M","P","Q","R","V","W","X"];
pub const SEPARATOR: String = "+";
pub const LEN_PREFIX: usize = 8;
pub const LEN_SUFFIX: usize = 2;

pub fn fab() -> T {
    T::from(format!(
        "{}{}{}",
        supertype::fab_digits_count(DIGITS, LEN_PREFIX),
        SEPARATOR,
        supertype::fab_digits_count(DIGITS, LEN_SUFFIX),
}

#[cfg(test)]
mod tests {
    use crate::types::{open_location_code_text as t, open_location_code_text::OpenLocationCodeText as T};

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

}
