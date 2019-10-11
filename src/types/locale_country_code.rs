//! Locale country code.
//!
//! Examples:
//!
//!  * US is United States
//!  * CN is China
//!  * IN is India
//!  * ES is Spain
//!  * EG is Egypt
//!  * ID is Indonesia

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type LocaleCountryCode = Supertype;
type T = LocaleCountryCode;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("US"), // United States
            T::from("CN"), // China
            T::from("IN"), // India
            T::from("ES"), // Spain
            T::from("EG"), // Egypt
            T::from("ID"), // Indonesia
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{locale_country_code as t, locale_country_code::LocaleCountryCode as T};

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
