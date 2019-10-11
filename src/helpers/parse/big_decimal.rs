use std::str::FromStr;
use bigdecimal::BigDecimal;

pub fn option_str_to_option_big_decimal(s: Option<&str>) -> Option<BigDecimal> {
    match s {
        None => None,
        Some(s) => {
            match BigDecimal::from_str(s) {
                Ok(x) => Some(x),
                Err(e) => panic!("Cannot parse string to BigDecimal. string:{} err:{:?}", s, e),
            }
        }
    }
}
