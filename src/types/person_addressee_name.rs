//! Person addressee name.
//!
//! Example: "Dr. Alice Adams Jr."
//!
//! Note: this is freeform text, and also suitable for user input.

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type PersonAddresseeName = Supertype;
type T = PersonAddresseeName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("Dr. Alice Adams Jr."),
            T::from("Hon. Bob Brown Sr."),
            T::from("Rev. Carol Curtis III"),
            T::from("Pres. Dave Davis M.D."),
            T::from("Mister Edwards Ph.D."),
            T::from("Madame Frank Franklin Esq."),
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{person_addressee_name as t, person_addressee_name::PersonAddresseeName as T};

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
