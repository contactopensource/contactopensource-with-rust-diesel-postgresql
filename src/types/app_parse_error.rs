
/// An error which can be returned when parsing an app type.
///
/// This error is used as the error type for the `/types` implementations.
///
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppParseError(());

#[stable(feature = "app_parse_error_error", since = "1.4.0")]
impl fmt::Display for AppParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

#[stable(feature = "app_parse_error_error", since = "1.4.0")]
impl Error for AppParseError {
    fn description(&self) -> &str {
        "invalid syntax"
    }
}