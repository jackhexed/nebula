mod tw {
    pub use twilight_http::error::Error;
    pub use twilight_http::response::DeserializeBodyError;
}

/// Nebula's error type.
///
/// Errors are meant to be logged or reported, not matched on, so this is a
/// plain message implementing [`std::error::Error`]. It is returned only for
/// I/O and Discord API failures; misusing Nebula's API panics instead.
#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    /// Creates an error with the given message.
    pub fn new(msg: impl Into<String>) -> Self {
        Self { msg: msg.into() }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}

#[doc(hidden)]
impl From<tw::Error> for Error {
    fn from(value: tw::Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

#[doc(hidden)]
impl From<tw::DeserializeBodyError> for Error {
    fn from(value: tw::DeserializeBodyError) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}
