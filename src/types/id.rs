//! Typical id for our data.
//!
//! Example: TODO

use ::uuid::Uuid;

pub type Id = Uuid;
type T = Id;

pub type IdParseError = ::uuid::parser::ParseError;
type E = IdParseError;

pub const LEN: usize = 32;

pub fn from_str(s: &str) -> Result<T, E> {
    T::parse_str(&s)
}

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    crate::helpers::parse::uuid::option_str_to_option_uuid(s)
}

pub fn fab() -> T {
    Uuid::new_v4()
}

#[cfg(test)]
mod tests {
    use crate::types::{id as t, id::Id as T};
    use ::std::str::FromStr;

    #[test]
    fn test_from_str() {
        let s = "ad30577b1e237cfbf63bafd5d19b940b";
        let x: T = T::from_str(&s).unwrap();
        assert_eq!(x.to_simple().to_string(), s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "09f0bc9fd611bcdc55bf5d23fea9cc44";
        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x.to_simple().to_string(), s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert_eq!(x.to_simple().to_string().len(), t::LEN);
    }

}
