//! Telephone E.164 national destination code.
//!
//! Example: TODO

use ::rand::{Rng, thread_rng};
use crate::types::{text as supertype, text::Text as Supertype};

pub type TelephoneE164NationalDestinationCode = Supertype;
type T = TelephoneE164NationalDestinationCode;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    crate::types::text::fab_digits_count(crate::types::text::DIGIT, thread_rng().gen_range(1, 3))
}

#[cfg(test)]
mod tests {
    use crate::types::{telephone_e164_national_destination_code as t, telephone_e164_national_destination_code::TelephoneE164NationalDestinationCode as T};

    #[test]
    fn test_from_str() {
        let s = "a";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "a";
        let x: T = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
