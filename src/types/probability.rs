//! Typical mathemtatical probability. Range 0..1.
//!
//! Example: 0.1 means 10% likely.

use ::rand::{Rng, thread_rng};
use bigdecimal::BigDecimal;

pub type Probability = BigDecimal;
type T = Probability;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    crate::helpers::parse::a::option_str_to_option_t::<BigDecimal>(s)
    //TODO change to:
    // crate::helpers::parse::a::option_str_to_option_t_with_min_inc_and_max_inc::<BigDecimal>(s, MIN, MAX)
}

use crate::traits::type_contains::TypeContains;
impl TypeContains for Probability {
    fn type_contains(&self, x: Self) -> bool {
        x >= BigDecimal::from(0.0) && x <= BigDecimal::from(1.0)
    }
}

pub fn fab() -> T {
    BigDecimal::from(thread_rng().gen_range(0.0, 1.0))
}


#[cfg(test)]
mod tests {
    use crate::types::{probability as t, probability::Probability as T};
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
