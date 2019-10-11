use std::cmp::PartialOrd;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter}; //, Result};
use std::result::Result;
use std::str::FromStr;
//use std::fmt::*;
//use std::ops::*;
//use std::error::*;
//use core::ops::Range;
//use core::ops::RangeToInclusive;
use core::ops::RangeBounds;


////
//
// Parse to Rust standard types
//
///

pub fn option_str_to_option_string(s: Option<&str>) -> Option<String> {
    match s {
        None => None,
        Some(s) => Some(String::from(s)),
    }
}

pub fn option_str_to_option_t<T: FromStr + Debug + Display>(s: Option<&str>) -> Option<T> {
    match s {
        None => None,
        Some(s) => {
            match s.parse::<T>() {
                Ok(x) => Some(x),
                Err(_e) => panic!("Cannot parse str. string:{} err:TODO", s), //TODO add err msg
            }
        }
    }
}

// pub fn option_str_to_option_t_with_lt<T: PartialOrd + FromStr + Debug + Display>(s: Option<&str>, y: T) -> Option<T> {
//     match option_str_to_option_t::<T>(s) {
//         None => None,
//         Some(ref x) if !(x < y) => panic!("Cannot parse str with x < min. x:{} min:{}", x, y),
//         Some(ref x) => Some(x),
//     }
// }

// pub fn option_str_to_option_t_with_le<T: PartialOrd + FromStr + Debug + Display>(s: Option<&str>, y: T) -> Option<T> {
//     match option_str_to_option_t::<T>(s) {
//         None => None,
//         Some(ref x) if !(x <= &y) => panic!("Cannot parse str with x <= y. x:{} y{}", x, y),
//         Some(ref x) => Some(x),
//     }
// }

// pub fn option_str_to_option_t_with_gt<T: PartialOrd + FromStr + Debug + Display>(s: Option<&str>, y: T) -> Option<T> {
//     match option_str_to_option_t::<T>(s) {
//         None => None,
//         Some(ref x) if !(x > &y) => panic!("Cannot parse str with x > y. x:{} y{}", x, y),
//         Some(ref x) => Some(x),
//     }
// }

// pub fn option_str_to_option_t_with_ge<T: PartialOrd + FromStr + Debug + Display>(s: Option<&str>, y: T) -> Option<T> {
//     match option_str_to_option_t::<T>(s) {
//         None => None,
//         Some(ref x) if !(x >= &y) => panic!("Cannot parse str with x >= y. x:{} y{}", x, y),
//         Some(ref x) => Some(x),
//     }
// }

// pub fn option_str_to_option_t_with_min_inc_and_max_inc<T: PartialOrd + FromStr + Debug + Display>(s: Option<&str>, min: T, max: T) -> Option<T> {
//     match option_str_to_option_t::<T>(s) {
//         None => None,
//         Some(ref x) if x < &min => panic!("Cannot parse str with x >= min. x:{} min:{} max:{}", x, min, max),
//         Some(ref x) if x > &max => panic!("Cannot parse str with x <= max. x:{} min:{} max:{}", x, min, max),
//         Some(ref x) => Some(x),
//     }
// }

// pub fn option_str_to_option_t_with_min_inc_and_max_exc<T: PartialOrd + FromStr + Debug + Display>(s: Option<&str>, min: T, max: T) -> Option<T> {
//     match option_str_to_option_t::<T>(s) {
//         None => None,
//         Some(ref x) if x < &min => panic!("Cannot parse str with x >= min. x:{} min:{} max:{}", x, min, max),
//         Some(ref x) if x >= &max => panic!("Cannot parse str with x < max. x:{} min:{} max:{}", x, min, max),
//         Some(ref x) => Some(x),
//     }
// }

// pub fn option_str_to_option_t_with_min_exc_and_max_inc<T: PartialOrd + FromStr + Debug + Display>(s: Option<&str>, min: T, max: T) -> Option<T> {
//     match option_str_to_option_t::<T>(s) {
//         None => None,
//         Some(ref x) if x <= &min => panic!("Cannot parse str with x > min. x:{} min:{} max:{}", x, min, max),
//         Some(ref x) if x > &max => panic!("Cannot parse str with x <= max. x:{} min:{} max:{}", x, min, max),
//         Some(ref x) => Some(x),
//     }
// }

// pub fn option_str_to_option_t_with_min_exc_and_max_exc<T: PartialOrd + FromStr + Debug + Display>(s: Option<&str>, min: T, max: T) -> Option<T> {
//     match option_str_to_option_t::<T>(s) {
//         None => None,
//         Some(ref x) if x <= &min => panic!("Cannot parse str to x with x > min. x:{} min:{} max:{}", x, min, max),
//         Some(ref x) if x >= &max => panic!("Cannot parse str to x with x < max. x:{} min:{} max:{}", x, min, max),
//         Some(ref x) => Some(x),
//     }
// }

// pub fn option_str_to_option_t_with_in_range_bounds<T: PartialOrd + FromStr + Debug + Display>(s: Option<&str>, r: impl RangeBounds<T>+ Display) -> Option<T> {
//     match option_str_to_option_t::<T>(s) {
//         None => None,
//         Some(ref x) if !r.contains(&x) => panic!("Cannot parse str to x with in range bounds. x:{} range bounds:{}", x, r),
//         Some(ref x) => Some(x),
//     }
// }

// pub fn option_str_to_option_t_with_not_in_range_bounds<T: PartialOrd + FromStr + Debug + Display>(s: Option<&str>, r: impl RangeBounds<T> + Display) -> Option<T> {
//     match option_str_to_option_t::<T>(s) {
//         None => None,
//         Some(ref x) if r.contains(&x) => panic!("Cannot parse str to x with not in range bounds. x:{} range bounds:{}", x, r),
//         Some(ref x) => Some(x),
//     }
// }

////
//
// Vet a.k.a. validate
//
////

#[derive(Debug, Clone)]
pub struct VetError;

// This is important for other errors to wrap this one.
impl Error for VetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl Display for VetError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "vet error")
    }
}

pub fn vet_lt<T: PartialOrd + Display>(x: T, y: T) -> Result<T, VetError> {
    if x < y { Ok(x) } else { Err(VetError{}) } // format!("Vet x < y. x:{} y:{}", x, y))
}

pub fn vet_le<T: PartialOrd + Display>(x: T, y: T) -> Result<T, VetError> {
    if x <= y { Ok(x) } else { Err(VetError{}) } // format!("Vet x <= y. x:{} y:{}", x, y))
}

pub fn vet_eq<T: PartialOrd + Display>(x: T, y: T) -> Result<T, VetError> {
    if x == y { Ok(x) } else { Err(VetError{}) } // format!("Vet x == y. x:{} y:{}", x, y))
}

pub fn vet_ne<T: PartialOrd + Display>(x: T, y: T) -> Result<T, VetError> {
    if x != y { Ok(x) } else { Err(VetError{}) } // format!("Vet x != y. x:{} y:{}", x, y))
}

pub fn vet_gt<T: PartialOrd + Display>(x: T, y: T) -> Result<T, VetError> {
    if x > y { Ok(x) } else { Err(VetError{}) } // format!("Vet x > y. x:{} y:{}", x, y))
}

pub fn vet_ge<T: PartialOrd + Display>(x: T, y: T) -> Result<T, VetError> {
    if x >= y { Ok(x) } else { Err(VetError{}) } // format!("Vet x >= y. x:{} y:{}", x, y))
}

pub fn vet_in_range_bounds<T: PartialOrd + Display>(x: T, r: impl RangeBounds<T>) -> Result<T, VetError> {
    if r.contains(&x) { Ok(x) } else { Err(VetError{}) } // format!("Vet x in range. x:{} range:{}", x, r)) }
}

pub fn vet_not_in_range_bounds<T: PartialOrd + Display>(x: T, r: impl RangeBounds<T>) -> Result<T, VetError> {
    if !r.contains(&x) { Ok(x) } else { Err(VetError{}) } // format!("Vet x not in range. x:{} range:{}", x, r)) }
}
