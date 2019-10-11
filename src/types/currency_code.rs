//! A currency code.
//!
//! Examples:
//!
//!   * USD is United States Dollar
//!   * CNY is Chinese Yuan
//!   * INR is Indian Rupee
//!   * EUR is European Euro
//!   * EGP is Egyptian Pound
//!   * IDR is Indonesian Rupiah

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type CurrencyCode = Supertype;
type T = CurrencyCode;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("USD"), // United States Dollar
            T::from("CNY"), // Chinese Yuan
            T::from("INR"), // Indian Rupee
            T::from("EUR"), // European Euro
            T::from("EGP"), // Egyptian Pound
            T::from("IDR"), // Indonesian Rupiah
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{currency_code as t, currency_code::CurrencyCode as T};

    #[test]
    fn test_from_str() {
        let s = "USD";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "USD";
        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
