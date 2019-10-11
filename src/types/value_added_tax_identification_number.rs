//! Value added tax identification number (a.k.a. VATIN, VAT id)
//!
//! See: https://schema.org/vatID
//! See: https://wikipedia.org/wiki/VAT_identification_number
//!
//! Example: "01234567"
//!
//! Note that this type is a text code, not an integer.

use ::rand::{Rng, thread_rng};
use crate::types::{text as supertype, text::Text as Supertype};

pub type ValueAddedTaxIdentificationNumber = Supertype;
type T = ValueAddedTaxIdentificationNumber;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

const DIGITS: &'static[&'static str] = &["0","1","2","3","4","5","6","7","8","9"];

pub fn fab() -> T {
    supertype::fab_digits_count(&DIGITS, thread_rng().gen_range(1, 8))
}

#[cfg(test)]
mod tests {
    use crate::types::{value_added_tax_identification_number as t, value_added_tax_identification_number::ValueAddedTaxIdentificationNumber as T};

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
