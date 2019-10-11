//! DB column name.
//!
//! Example: "updated_at" means DB column `updated_at`.

use crate::types::{self, text as supertype, text::Text as Supertype};

pub type DbColumnName = Supertype;
type T = DbColumnName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    types::alpha_word::fab()
}

#[cfg(test)]
mod tests {
    use crate::types::{db_column_name as t, db_column_name::DbColumnName as T};

    #[test]
    fn test_from_str() {
        let s = "hello";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "hello";
        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
