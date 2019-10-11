//! Global location number.
//!
//! See: https://schema.org/globalLocationNumber https://en.wikipedia.org/wiki/Global_Location_Number
//!
//! Example: TODO

use ::rand::{Rng, thread_rng};
use crate::types::{text as supertype, text::Text as Supertype};

pub type GlobalLocationNumberText = Supertype;
type T = GlobalLocationNumberText;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    thread_rng()
    .sample_iter(&rand::distributions::Alphanumeric)
    .take(13)
    .collect()
}

#[cfg(test)]
mod tests {
    use crate::types::{global_location_number_text as t, global_location_number_text::GlobalLocationNumberText as T};

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
