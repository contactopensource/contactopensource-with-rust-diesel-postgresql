//! Locale string, such as for language code and country code.
//!
//! Examples:
//!
//!   * "en-US" is English - United States
//!   * "zh-CN" is Chinese - China (Simplified variant)
//!   * "hi-IN" is Hindi - India
//!   * "es-ES" is Spanish - Spain
//!   * "ar-EG" is Arabic - Egypt
//!   * "ms-ID" is Malay - Indonesia

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type LocaleString = Supertype;
type T = LocaleString;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("en-US"), // English - United States
            T::from("zh-CN"), // Chinese - China (Simplified variant)
            T::from("hi-IN"), // Hindi - India
            T::from("es-ES"), // Spanish - Spain
            T::from("ar-EG"), // Arabic - Egypt
            T::from("ms-ID"), // Malay - Indonesia
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{locale_string as t, locale_string::LocaleString as T};

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
