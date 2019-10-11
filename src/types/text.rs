//! Typical freeform text.
//!
//! Example: "hello world"

use ::rand::{Rng, thread_rng};

pub type Text = String;
type T = Text;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    crate::helpers::parse::a::option_str_to_option_string(s)
}

// Character classes for old-style Unix ASCII
// See https://www.gnu.org/software/grep/manual/html_node/Character-Classes-and-Bracket-Expressions.html
pub const BLANK:  &'static[&'static str] = &[" ","\t"];
pub const UPPER:  &'static[&'static str] = &["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
pub const LOWER:  &'static[&'static str] = &["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];
pub const ALPHA:  &'static[&'static str] = &["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];
pub const ALNUM:  &'static[&'static str] = &["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","0","1","2","3","4","5","6","7","8","9"];
pub const PUNCT:  &'static[&'static str] = &["!","\"","#","$","%","&","'","(",")","*","+",",","-",".","/",":",";","<","=",">","?","","[","\\","]","^","_","`","{","","}","~","."];
pub const WORD:   &'static[&'static str] = &["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","_"];
pub const DIGIT:  &'static[&'static str] = &["0","1","2","3","4","5","6","7","8","9"];
pub const XDIGIT: &'static[&'static str] = &["0","1","2","3","4","5","6","7","8","9","A","B","C","D","E","F"];

pub fn fab() -> T {
    fab_count(thread_rng().gen_range(1, 8))
}

pub fn fab_count(count: usize) -> T {
    fab_digits_count(ALNUM, count)
}

pub fn fab_digits_count(digits: &[&str], count: usize) -> T {
    (0..count)
    .map(|_|
        digits[thread_rng().gen_range(0, digits.len())]
    )
    .collect()
}

#[cfg(test)]
mod tests {
    use crate::types::{text as t, text::Text as T};

    #[test]
    fn test_from_str() {
        let s = "a";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "a";
        let x: T = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
