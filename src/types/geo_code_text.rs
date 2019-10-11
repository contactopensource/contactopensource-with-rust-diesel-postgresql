//! Geocode text.
//!
//! Examples:
//!
//!   * "6GCRPR6C+24" is the Open Location Code geocode for the Parliament Buildings in Nairobi, Kenya.
//!
//!   * "joyful.nail.harmonica" the What Free Words geocode for geolocation 37.234328,-115.806657.
//!
//! See https://github.com/google/open-location-code
//!
//! See https://whatfreewords.org/

use ::rand::{Rng, thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type GeoCodeText = Supertype;
type T = GeoCodeText;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

const DIGITS: &'static[&'static str] = &["0","1","2","3","4","5","6","7","8","9","A","B","C","D","E","F"]; //TODO fix to be the correct chars

pub fn fab() -> T {
    supertype::fab_digits_count(DIGITS, thread_rng().gen_range(1, 8))
}

#[cfg(test)]
mod tests {
    use crate::types::{geo_code_text as t, geo_code_text::GeoCodeText as T};

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
