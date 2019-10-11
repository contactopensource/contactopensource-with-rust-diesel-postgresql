//! Cascading Style Sheet (CSS) class name.
//!
//! Example: "help" is a CSS class name.

use crate::types::{self, text as supertype, text::Text as Supertype};

pub type CSSClassName = Supertype;
type T = CSSClassName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    types::alpha_word::fab()
}

#[cfg(test)]
mod tests {
    use crate::types::{css_class_name as t, css_class_name::CSSClassName as T};

    #[test]
    fn test_from_str() {
        let s = "hello";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "hello";
        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
