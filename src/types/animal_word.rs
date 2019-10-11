//! An animal word series.
//!
//! Examples: ant, bat, cat, etc.

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type AnimalWord = Supertype;
type T = AnimalWord;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("ant"),
            T::from("bat"),
            T::from("cat"),
            T::from("dog"),
            T::from("emu"),
            T::from("fish"),
            T::from("goat"),
            T::from("horse"),
            T::from("insect"),
            T::from("jaguar"),
            T::from("koala"),
            T::from("lion"),
            T::from("mouse"),
            T::from("numbat"),
            T::from("octopus"),
            T::from("pig"),
            T::from("quail"),
            T::from("rabbit"),
            T::from("snake"),
            T::from("tiger"),
            T::from("urchin"),
            T::from("vulture"),
            T::from("whale"),
            T::from("xerus"),
            T::from("yak"),
            T::from("zebra"),
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{animal_word as t, animal_word::AnimalWord as T};

    #[test]
    fn test_from_str() {
        let s = "ant";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "ant";
        let x = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
