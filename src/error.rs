mod tw {
    pub use twilight_http::error::Error;
    pub use twilight_http::response::DeserializeBodyError;
}

/// Nebula's error type.
///
/// Nebula's errors are not errors that should be matched on, and should instead
/// be logged and reported. Thus, nebula's error type is a simple wrapper around
/// a string message, and implements the [`std::error::Error`] trait.
#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}

impl From<tw::Error> for Error {
    fn from(value: tw::Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl From<tw::DeserializeBodyError> for Error {
    fn from(value: tw::DeserializeBodyError) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}
