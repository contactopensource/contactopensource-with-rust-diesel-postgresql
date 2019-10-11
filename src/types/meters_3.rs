//! Measurement metrics that use meters cubed, such as for volume.
//!
//! Example: 8 means 8 meters cubed.

use crate::types::{metrics as supertype, metrics::Metrics as Supertype};

pub type Meters3 = Supertype;
type T = Meters3;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    supertype::fab()
}

#[cfg(test)]
mod tests {
    use crate::types::{meters_3 as t, meters_3::Meters3 as T};
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
