//! Typical DNS top level domain (TLD) name.
//!
//! Example: "com" as in "example.com"

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type TLDName = Supertype;
type T = TLDName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("com"), // commercial
            T::from("net"), // networking
            T::from("edu"), // education
            T::from("org"), // organization
            T::from("gov"), // goverment
            T::from("mil"), // miliary
            T::from("us"), // United States
            T::from("cn"), // China
            T::from("in"), // India
            T::from("id"), // Indonesia
            T::from("br"), // Brazil
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{tld_name as t, tld_name::TLDName as T};

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
