//! Abstract metrics units for measurements.
//!
//! Example: 8 means 8 units.
//!
//! This type is useful for its subtypes:
//!
//!   * Meters for length metrics
//!   * Liters for volume metrics
//!   * Seconds for time metrics
//!   * etc.

use ::bigdecimal::BigDecimal;
use ::rand::{Rng, thread_rng};

pub type Metrics = BigDecimal;
type T = Metrics;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    crate::helpers::parse::big_decimal::option_str_to_option_big_decimal(s)
}

pub fn fab() -> T {
    BigDecimal::from(thread_rng().gen_range(-888.888, 888.888))
}

#[cfg(test)]
mod tests {
    use crate::types::{metrics as t, metrics::Metrics as T};
    use ::bigdecimal::Zero;
    use ::std::str::FromStr;

    #[test]
    fn test_from_str() {
        let s = "0";
        let e = T::zero();
        let x: T = T::from_str(&s).unwrap();
        assert_eq!(x, e);
    }

    #[test]
    fn test_from_option_str() {
        let s = "0";
        let e = T::zero();
        let x: T = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, e);
    }

    #[test]
    fn test_fab() {
        let _x: T = t::fab();
    }

}
