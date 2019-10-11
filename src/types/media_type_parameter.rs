//! Media type parameter
//!
//! Examples:
//!
//!   * charset=UTF-8
//!   * boundary=alpha

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type MediaTypeParameter = Supertype;
type T = MediaTypeParameter;

pub type MediaTypeParameterParseError = String;
type E = MediaTypeParameterParseError;

pub fn from_str(s: &str) -> Result<T, E> {
    Ok(T::from(s))
}

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("charset=UTF-8"),
            T::from("charset=UTF-16"),
            T::from("charset=ASCII"),
            T::from("boundary=alpha"),
            T::from("boundary=bravo"),
            T::from("boundary=charlie"),
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{media_type_parameter as t, media_type_parameter::MediaTypeParameter as T};

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
