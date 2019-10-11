//! Path base name, which is typically a file name, including an extension name.
//!
//! Examples: "readme.txt", "image.jpg", "index.html".

use crate::types::{self, text as supertype, text::Text as Supertype};

pub type PathBaseName = Supertype;
type T = PathBaseName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    T::from(format!(
        "{}.{}",
        types::alpha_word::fab(),
        types::path_ext_name::fab(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::types::{path_base_name as t, path_base_name::PathBaseName as T};

    #[test]
    fn test_from_str() {
        let s = "alpha";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "alpha";
        let x: T = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
