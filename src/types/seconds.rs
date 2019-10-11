//! Measurement metrics that use seconds, such as for time.
//!
//! Example: 8 means 8 seconds.

use crate::types::{metrics as supertype, metrics::Metrics as Supertype};

pub type Seconds = Supertype;
type T = Seconds;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    supertype::fab()
}
//FIX