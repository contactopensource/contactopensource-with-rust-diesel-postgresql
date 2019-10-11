//! Passport number text.
//!
//! Exampl: "A1B2-C3D4"

use ::rand::{Rng, thread_rng};
use crate::types::{text as supertype, text::Text as Supertype};

pub type PassportNumberText = Supertype;
type T = PassportNumberText;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    T::from(format!(
        "{}-{}",
        thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(4).collect::<String>(),
        thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(4).collect::<String>(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::types::{passport_number_text as t, passport_number_text::PassportNumberText as T};

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
