//! WhatFreeWords.com geolocation code.
//!
//! See https://whatfreewords.org/
//!
//! Example: "joyful.nail.harmonica" is geolocation 37.234328,-115.806657.

use crate::types::{self, geo_code as supertype, geo_code::GeoCode as Supertype};

pub type WhatFreeWordsCodeText = Supertype;
type T = WhatFreeWordsCodeText;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    T::from(format!(
        "{}.{}.{}",
        types::alpha_word::fab(),
        types::animal_word::fab(),
        types::fruit_word::fab(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::types::{what_fre_words_code_text as t, what_fre_words_code_text::WhatFreeWordsCodeText as T};

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
