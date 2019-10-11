//! International Standard of Industrial Classification of All Economic Activities (ISIC), Revision 4, Name.
//!
//! Examples:
//!
//!   * A: Agriculture, forestry and fishing
//!   * B05: Mining of coal and lignite
//!   * C101: Processing and preserving of meat

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type ISICV4Name = Supertype;
type T = ISICV4Name;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("Agriculture.to_string(), forestry and fishing"),
            T::from("Mining of coal and lignite"),
            T::from("Processing and preserving of meat"),
            T::from("Electric power generation, transmission and distribution"),
            T::from("Sewerage"),
            T::from("Construction of buildings"),
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{isic_v4_name as t, isic_v4_name::ISICV4Name as T};

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
