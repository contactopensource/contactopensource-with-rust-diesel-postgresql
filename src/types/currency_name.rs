//! A currency name.
//!
//! Examples:
//!
//!   * United States Dollar
//!   * Chinese Yuan
//!   * Indian Rupee
//!   * European Euro
//!   * Egyptian Pound
//!   * Indonesian Rupiah

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type CurrencyName = Supertype;
type T = CurrencyName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("United States Dollar"),
            T::from("Chinese Yuan"),
            T::from("Indian Rupee"),
            T::from("European Euro"),
            T::from("Egyptian Pound"),
            T::from("Indonesian Rupiah"),
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{currency_name as t, currency_name::CurrencyName as T};

    #[test]
    fn test_from_str() {
        let s = "United States Dollar";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "United States Dollar";

        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
