//! Typical mathematical normalized weight (not mass weight). Range -1..1.
//!
//! Example: 0.1 means 10% weight.

use ::rand::{Rng, thread_rng};
use bigdecimal::BigDecimal;

pub type Weight = BigDecimal;
type T = Weight;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    crate::helpers::parse::a::option_str_to_option_t::<BigDecimal>(s)
    //TODO change to:
    //crate::helpers::parse::a::option_str_to_option_t_with_min_inc_and_max_inc::<BigDecimal>(s, MIN, MAX)
}

//TODO learn
//use crate::traits::range_bounds::RangeBounds;
// impl crate::traits::range_bounds::RangeBounds for Weight {
//     fn range_bounds() -> dyn(::core::ops::RangeBounds) {
//         BigDecimal::from(-1.0)..=BigDecimal::from(1.0);
//     }
// }

pub fn fab() -> T {
    BigDecimal::from(thread_rng().gen_range(-1.0, 1.0))
}

#[cfg(test)]
mod tests {
    use crate::types::{weight as t, weight::Weight as T};
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
