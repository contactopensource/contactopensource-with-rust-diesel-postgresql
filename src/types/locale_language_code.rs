//! Locale language code.
//!
//! Examples:
//!
//!   * en is English
//!   * zh is Chinese
//!   * hi is Hindi
//!   * es is Spanish
//!   * ar is Arabic
//!   * ms is Malay

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type LocaleLanguageCode = Supertype;
type T = LocaleLanguageCode;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("en"), // English
            T::from("zh"), // Chinese
            T::from("hi"), // Hindi
            T::from("es"), // Spanish
            T::from("ar"), // Arabic
            T::from("ms"), // Malay
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{locale_language_code as t, locale_language_code::LocaleLanguageCode as T};

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
