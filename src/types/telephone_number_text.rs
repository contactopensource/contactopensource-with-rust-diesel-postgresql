//! Telephone number text.
//!
//! Example: "U.S. 415 555-1000 ext. 2000"
//!
//! Note: this is freeform text, and also suitable for user input.

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type TelephoneNumberText = Supertype;
type T = TelephoneNumberText;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("555-1000"),
            T::from("(415) 555-1000"),
            T::from("1 (415) 555-1000"),
            T::from("+1-415-555-1000"),
            T::from("U.S. 415 555-1000"),
            T::from("555-1000 extension 2000"),
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{telephone_number_text as t, telephone_number_text::TelephoneNumberText as T};

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
