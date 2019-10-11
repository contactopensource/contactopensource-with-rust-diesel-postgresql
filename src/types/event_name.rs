//! Event name.
//!
//! Example: "lunch" is an event name.

use ::rand::{Rng, thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type EventName = Supertype;
type T = EventName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("breakfast"),
            T::from("brunch"),
            T::from("lunch"),
            T::from("tea"),
            T::from("dinner"),
            T::from("supper"),
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{event_name as t, event_name::EventName as T};

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
