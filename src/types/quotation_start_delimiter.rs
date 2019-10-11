//! Quotation start delimiter, suitable for locales.
//!
//! Examples
//!
//!   * “ is English (U+201C left double quotation mark)
//!   * 「 is Chinese (U+300C left corner bracket)
//!   * « is French gullemet (U+00AB left angle quotes)
//!   * ( is Unix subcommand (left parenthesis)

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type QuotationStartDelimiter = Supertype;
type T = QuotationStartDelimiter;

pub const CHARS_COUNT: usize = 1;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("“"), // English (U+201C left double quotation mark)
            T::from("「"), // Chinese (U+300C left corner bracket)
            T::from("«"), // French gullemet (U+00AB left angle quotes)
            T::from("("), // Unix subcommand (left parenthesis)
            T::from("{"), // fake
            T::from("["), // fake
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{quotation_start_delimiter as t, quotation_start_delimiter::QuotationStartDelimiter as T};

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
        assert_eq!(x.chars().count(), t::CHARS_COUNT);
    }

}