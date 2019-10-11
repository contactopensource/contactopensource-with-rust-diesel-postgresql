//! Typical typecast name, suitable for single type inheritance.
//!
//! Example: "item" means use the single table inheritance `item` class.
//!
//! We prefer implemenation using type String (over number, enum, etc.)
//! because we prefer the flexibility of freeform text.

use crate::types::{text as supertype, text::Text as Supertype};

pub type Typecast = Supertype;
type T = Typecast;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    crate::types::alpha_word::fab()
}

#[cfg(test)]
mod tests {
    use crate::types::{typecast as t, typecast::Typecast as T};

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
