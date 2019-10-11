use ::rand::{Rng, thread_rng};
use bigint::{ToBigInt, RandBigInt};
use bigdecimal::BigDecimal;

pub fn fab() -> BigDecimal {
    let mut rng = rand::thread_rng();
    digits: BigInt = rng.gen_bigint(1000);
    scale: i64 = rng.gen_bigint(1000);
    BigDecimal::new(digits, scale)
}

/// Example:
///
//      let low = -10000.to_bigint().unwrap();
//      let high = 10000.to_bigint().unwrap();
///     let b = rng.gen_bigint_range(&low, &high);
///
pub fn fab_range(min, max) -> BigDecimal {
    let mut rng = rand::thread_rng();
    rng.gen_bigint_range(&low, &high)
}
