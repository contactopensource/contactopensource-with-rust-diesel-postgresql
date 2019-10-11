//! Typical state suitable for a typical state machine.
//!
//! Example: TODO
//!
//! We prefer implemenation using String (over number, enum, etc.)
//! because we prefer the flexibility of freeform text.

use crate::types::{self, text as supertype, text::Text as Supertype};

pub type State = Supertype;
type T = State;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    types::alpha_word::fab()
}

#[cfg(test)]
mod tests {
    use crate::types::{state as t, state::State as T};

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