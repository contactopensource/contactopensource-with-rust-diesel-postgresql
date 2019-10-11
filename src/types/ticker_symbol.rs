//! Ticker symbol for e.g. stock market trading
//!
//! See https://wikipedia.org/wiki/Ticker_symbol
//!
//! Example: "AAPL" is Apple Inc.

use ::rand::{Rng, thread_rng};
use crate::types::{text as supertype, text::Text as Supertype};

pub type TickerSymbol = Supertype;
type T = TickerSymbol;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    let count = thread_rng().gen_range(1, 4);
    thread_rng()
    .sample_iter(&rand::distributions::Alphanumeric)
    .take(count)
    .collect()
}

#[cfg(test)]
mod tests {
    use crate::types::{ticker_symbol as t, ticker_symbol::TickerSymbol as T};

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
