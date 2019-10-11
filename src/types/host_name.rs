//! Typical host name.
//!
//! Example: "www.example.com".

use crate::types::{self, text as supertype, text::Text as Supertype};

pub type HostName = Supertype;
type T = HostName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    T::from(format!(
        "{}.{}.{}",
        types::node_name::fab(),
        types::node_name::fab(),
        types::tld_name::fab(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::types::{host_name as t, host_name::HostName as T};

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
