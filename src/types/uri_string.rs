//! Typical URI string, that may or may not be parseable.
//!
//! Example: "http://example.com/index.html"

use crate::types::{self, text as supertype, text::Text as Supertype};

pub type URIString = Supertype;
type T = URIString;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    T::from(
        format!("{}://{}/{}",
        types::uri_scheme::fab(),
        types::host_name::fab(),
        types::path_base_name::fab(),
        ))
}

#[cfg(test)]
mod tests {
    use crate::types::{uri_string as t, uri_string::URIString as T};

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
