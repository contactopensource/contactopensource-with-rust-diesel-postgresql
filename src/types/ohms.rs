//! Measurement metrics that use Ohms, such as for electricy and resistance.
//!
//! Example: 8 means 8 Ohms.

use crate::types::{metrics as supertype, metrics::Metrics as Supertype};

pub type Ohms = Supertype;
type T = Ohms;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    supertype::fab()
}

#[cfg(test)]
mod tests {
    use crate::types::{ohms as t, ohms::Ohms as T};
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
