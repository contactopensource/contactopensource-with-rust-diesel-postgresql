//! Locale script code.
//!
//! Examples:
//!
//!   * Latn is Latin
//!   * Hans is Han (simplified script)
//!   * Deva is Devanagari (Nagari)
//!   * Lina is Linear A
//!   * Arab is Arabic
//!   * Mlym is Malayalam

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type LocaleScriptCode = Supertype;
type T = LocaleScriptCode;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("Latn"), // Latin
            T::from("Hans"), // Han (simplified script)
            T::from("Deva"), // Devanagari (Nagari)
            T::from("Lina"), // Linear A
            T::from("Arab"), // Arabic
            T::from("Mlym"), // Malayalam
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{locale_script_code as t, locale_script_code::LocaleScriptCode as T};

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
