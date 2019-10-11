//! Media type text
//!
//! Examples:
//!
//!   * text/plain
//!   * image/jpeg
//!   * application/json

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type MediaTypeText = Supertype;
type T = MediaTypeText;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("text/plain"),
            T::from("image/jpeg"),
            T::from("audio/ogg"),
            T::from("video/mpeg"),
            T::from("application/json"),
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{media_type_text as t, media_type_text::MediaTypeText as T};

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
