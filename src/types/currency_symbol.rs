//! A currency symbol.
//!
//! Examples:
//!
//!   * $ is United States Dollar
//!   * ¥ is Chinese Yuan
//!   * ₹ is Indian Rupee
//!   * € is European Euro
//!   * ج.م is Egyptian Pound
//!   * Rp is Indonesian Rupiah

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type CurrencySymbol = Supertype;
type T = CurrencySymbol;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("$"), // United States Dollar
            T::from("¥"), // Chinese Yuan
            T::from("₹"), // Indian Rupee
            T::from("€"), // European Euro
            T::from("ج.م"), // Egyptian Pound
            T::from("Rp"), // Indonesian Rupiah
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{currency_symbol as t, currency_symbol::CurrencySymbol as T};

    #[test]
    fn test_from_str() {
        let s = "$";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "$";
        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
