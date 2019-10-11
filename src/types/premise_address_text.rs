//! Premise address text.
//!
//! Example: "Suite 100"
//!
//! Note: this is freeform text, and also suitable for user input.

use crate::types::{self, text as supertype, text::Text as Supertype};

pub type PremiseAddressText = Supertype;
type T = PremiseAddressText;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    types::alpha_word::fab()
}

#[cfg(test)]
mod tests {
    use crate::types::{premise_address_text as t, premise_address_text::PremiseAddressText as T};

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