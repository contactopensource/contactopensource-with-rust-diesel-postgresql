//! JSON string.
//!
//! Example: `{"alpha":"apple"}`

use crate::types::{self, text as supertype, text::Text as Supertype};

pub type JSONString = Supertype;
type T = JSONString;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    // TODO fix the JSON to be valid
    T::from(format!(
        "{{\"{}\": \"{}\"}}",
        fab_key(),
        fab_value(),
        ))
}

pub fn fab_key() -> String {
    types::alpha_word::fab()
}

pub fn fab_value() -> String {
    types::fruit_word::fab()
}

#[cfg(test)]
mod tests {
    use crate::types::{json_string as t, json_string::JSONString as T};

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
    fn test_fab_key() {
        let x: String = t::fab_key();
        assert!(!x.is_empty());
    }

    #[test]
    fn test_fab_value() {
        let x: String = t::fab_value();
        assert!(!x.is_empty());
    }

}
