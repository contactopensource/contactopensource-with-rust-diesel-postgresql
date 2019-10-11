//! Legal entity identifier
//!
//! See: https://schema.org/leiCode
//! See: https://wikipedia.org/wiki/Legal_Entity_Identifier
//!
//! Example: "01234567890123456789"

use crate::{types::text as supertype, types::text::Text as Supertype};

pub type LegalEntityIdentifier = Supertype;
type T = LegalEntityIdentifier;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

const DIGITS: &'static[&'static str] = &["0","1","2","3","4","5","6","7","8","9"]; //TODO fix to real values
const LEN: usize = 20;

pub fn fab() -> T {
    supertype::fab_digits_count(DIGITS, LEN)
}

#[cfg(test)]
mod tests {
    use crate::types::{legal_entity_identifier as t, legal_entity_identifier::LegalEntityIdentifier as T};

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
