//! An alpha word series.
//!
//! Examples: alpha, bravo, charlie, etc.

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type AlphaWord = Supertype;
type T = AlphaWord;

//TODO learn
// impl FromStr for AlphaWord {
//     type Err = AppParseError;
//     pub fn from_str(s: <&str>) -> Result<Self, Self::Err> {
//         AlphaWord::from(s)
//     }
// }

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("alpha"),
            T::from("bravo"),
            T::from("charlie"),
            T::from("delta"),
            T::from("echo"),
            T::from("foxtrot"),
            T::from("golf"),
            T::from("hotel"),
            T::from("india"),
            T::from("juliet"),
            T::from("kilo"),
            T::from("lima"),
            T::from("mike"),
            T::from("november"),
            T::from("oscar"),
            T::from("papa"),
            T::from("quebec"),
            T::from("rome"),
            T::from("sierra"),
            T::from("tango"),
            T::from("uniform"),
            T::from("victor"),
            T::from("whiskey"),
            T::from("xray"),
            T::from("yankee"),
            T::from("zulu"),
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{alpha_word as t, alpha_word::AlphaWord as T};

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
