//! XML string.
//!
//! Example: `<x>alpha</x>`

use crate::types::{self, text as supertype, text::Text as Supertype};

pub type XMLString = Supertype;
type T = XMLString;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    let tag_name = fab_tag_name();
    let tag_content = fab_tag_content();
    T::from(format!(
        "<{}>{}</{}>",
        tag_name,
        tag_content,
        tag_name,
        ))
}

pub fn fab_tag_name() -> String {
    types::alpha_word::fab()
}

pub fn fab_tag_content() -> String {
    types::alpha_word::fab()
}

#[cfg(test)]
mod tests {
    use crate::types::{xml_string as t, xml_string::XMLString as T};

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

    #[test]
    fn test_fab_tag_name() {
        let x: String = t::fab_tag_name();
        assert!(!x.is_empty());
    }

    #[test]
    fn test_fab_tag_content() {
        let x: String = t::fab_tag_content();
        assert!(!x.is_empty());
    }

}
