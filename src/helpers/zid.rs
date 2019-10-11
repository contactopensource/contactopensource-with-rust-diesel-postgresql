extern crate rand;

use ::rand::{Rng, thread_rng};

pub fn zid() -> String {
    const CHARSET: &[u8] = b"0123456789abcdef";
    const LEN: usize = 32;
    let mut rng = thread_rng();
    let s: String = (0..LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        })
        .collect();
    s
}
