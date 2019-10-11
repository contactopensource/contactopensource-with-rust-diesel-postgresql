//! A fruit-series word.
//!
//! Examples: apple, banana, cherry, etc.

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type FruitWord = Supertype;
type T = FruitWord;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("apple"),
            T::from("banana"),
            T::from("cherry"),
            T::from("dewberry"),
            T::from("elderberry"),
            T::from("fig"),
            T::from("grape"),
            T::from("huckleberry"),
            T::from("imbe"),
            T::from("jackfruit"),
            T::from("kiwi"),
            T::from("lime"),
            T::from("mango"),
            T::from("nectarine"),
            T::from("orange"),
            T::from("papaya"),
            T::from("quince"),
            T::from("raspberry"),
            T::from("strawberry"),
            T::from("tangerine"),
            T::from("ugni"),
            T::from("voavanga"),
            T::from("watermelon"),
            T::from("xigua"),
            T::from("yangmei"),
            T::from("zuchinni"),
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{fruit_word as t, fruit_word::FruitWord as T};

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
