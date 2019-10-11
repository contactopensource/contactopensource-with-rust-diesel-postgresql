//! Person name.
//!
//! Example: "Alice Adams" is a person's name.

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type PersonName = Supertype;
type T = PersonName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("Alice Adams"),
            T::from("Bob Brown"),
            T::from("Carol Curtis"),
            T::from("Dave Davis"),
            T::from("Eve Edwards"),
            T::from("Frank Franklin"),
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{person_name as t, person_name::PersonName as T};

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
