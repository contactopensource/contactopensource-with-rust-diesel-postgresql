//! User name.
//!
//! Example: "alice_anderson"

use crate::types::{self, text as supertype, text::Text as Supertype};

pub type UserName = Supertype;
type T = UserName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    T::from(format!(
        "{}_{}",
        types::person_given_name::fab(),
        types::person_family_name::fab(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::types::{user_name as t, user_name::UserName as T};

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
