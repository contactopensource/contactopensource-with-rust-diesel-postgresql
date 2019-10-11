//! Media type parameters
//!
//! Examples:
//!
//!   * charset=UTF-8
//!   * boundary=alpha

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{self, media_type_parameters as t};

pub type MediaTypeParameters = Vec<types::media_type_parameter::MediaTypeParameter>;
type T = MediaTypeParameters;

pub type MediaTypeParametersParseError = String; //TODO write custom error
type E = MediaTypeParametersParseError;

pub const SEPARATOR: &str = ";";

pub fn from_str(s: &str) -> Result<T, E> {
    let tokens = s.split(t::SEPARATOR);
    let v: MediaTypeParameters = tokens.map(|token| types::media_type_parameter::from_str(token).unwrap()).collect();
    Ok(v)
}

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    match s {
        None => None,
        Some(s) => {
            let s = String::from(s);
            let split = s.split(";");
            let v: MediaTypeParameters = split.map(|s| s.to_string()).collect();
            Some(v)
        },
    }
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        let v0: T = vec![
            types::media_type_parameter::MediaTypeParameter::from("charset=UTF-8"),
            types::media_type_parameter::MediaTypeParameter::from("boundary=alpha"),
        ];
        let v1: T = vec![
            types::media_type_parameter::MediaTypeParameter::from("charset=UTF-16"),
            types::media_type_parameter::MediaTypeParameter::from("boundary=bravo"),
        ];
        let v2: T = vec![
            types::media_type_parameter::MediaTypeParameter::from("charset=ASCII"),
            types::media_type_parameter::MediaTypeParameter::from("boundary=charlie"),
        ];
        vec![
            v0,
            v1,
            v2,
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{self, media_type_parameters as t, media_type_parameters::MediaTypeParameters as T};

    #[test]
    fn test_from_str() {
        let s = "alpha";
        let e: T = vec![
            types::media_type_parameter::MediaTypeParameter::from("alpha"),
        ];
        let x: T = t::from_str(&s).unwrap();
        assert_eq!(x, e);
    }

    #[test]
    fn test_from_option_str() {
        let s = "alpha";
        let e: T = vec![
            types::media_type_parameter::MediaTypeParameter::from("alpha"),
        ];
        let x: T = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, e);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(x.len() > 0);
    }

}
