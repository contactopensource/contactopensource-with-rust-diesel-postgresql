//! An animal-series word.
//!
//! Examples: ant, bat, cat, etc.

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type WeatherEmoji = String;
type T = WeatherEmoji;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("🌞"), // Sun With Face
            T::from("⛅"), // Sun Behind Cloud
            T::from("⛈"), // Cloud With Lightning and Rain
            T::from("🌤"), // Sun Behind Small Cloud
            T::from("🌥"), // Sun Behind Large Cloud
            T::from("🌦"), // Sun Behind Rain Cloud
            T::from("🌧"), // Cloud With Rain
            T::from("🌨"), // Cloud With Snow
            T::from("🌩"), // Cloud With Lightning
            T::from("🌪"), // Tornado
            T::from("🌫"), // Fog
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{weather_emoji as t, weather_emoji::WeatherEmoji as T};

    #[test]
    fn test_from_str() {
        let s = "a";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "a";
        let x: T = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
