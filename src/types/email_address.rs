//! Email addr spec.
//!
//! Example: "Alice Adams <alice.anderson@example.com>"

use crate::types;
use crate::types::{text as supertype, text::Text as Supertype};

pub type EmailAddress = Supertype;
type T = EmailAddress;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    T::from(format!(
        "{} <{}>",
        types::email_display_name::fab(),
        types::email_addr_spec::fab()
        ))
}

#[cfg(test)]
mod tests {
    use crate::types::{email_address as t, email_address::EmailAddress as T};

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
