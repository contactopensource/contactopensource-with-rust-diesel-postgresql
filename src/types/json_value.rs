//! JSON value.
//!
//! Example: `{"alpha":"apple"}`

use crate::types;

pub type JSONValue = serde_json::Value;
type T = JSONValue;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    match s {
        None => None,
        Some(s) => {
            match serde_json::from_str(s) {
                Ok(x) => Some(x),
                Err(e) => panic!("Cannot parse string to JSON. string:{} err:{:?}", s, e),
            }
        }
    }
}

pub fn fab() -> T {
    serde_json::from_str(&types::json_string::fab()).unwrap()
}


#[cfg(test)]
mod tests {
    use crate::types::{json_string as t, json_string::JSONString as T};

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
//FIX
