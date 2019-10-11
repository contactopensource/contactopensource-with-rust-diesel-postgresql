//! DB row id.
//!
//! Example: TODO

use ::uuid::Uuid;

pub type DbRowId = Uuid;
type T = DbRowId;

pub type DbRowIdParseError = ::uuid::parser::ParseError;
type E = DbRowIdParseError;

pub const LEN: usize = 32;

pub fn from_str(s: &str) -> Result<T, E> {
    T::parse_str(&s)
}

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    crate::helpers::parse::uuid::option_str_to_option_uuid(s)
}

pub fn fab() -> T {
    T::new_v4()
}

#[cfg(test)]
mod tests {
    use crate::types::{db_row_id as t, db_row_id::DbRowId as T};
    use ::std::str::FromStr;

    #[test]
    fn test_from_str() {
        let s = "0a65e039256765dac2d7bfceabd94842";
        let x: T = T::from_str(&s).unwrap();
        assert_eq!(x.to_simple().to_string(), s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "617970d3d1b7ea397b054db5daa3417f";
        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x.to_simple().to_string(), s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert_eq!(x.to_simple().to_string().len(), t::LEN);
    }

}
