use std::fmt::Display;

mod tw {
    pub use twilight_http::error::Error;
}

#[derive(Debug)]
pub struct Error {
    kind: Option<ErrorKind>,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Error {
    pub(crate) fn new(
        kind: Option<ErrorKind>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self { kind, source }
    }

    pub(crate) fn internal(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::new(None, Some(Box::new(source)))
    }

    pub(crate) fn msg(msg: &'static str) -> Self {
        Self::internal(Msg(msg))
    }

    pub fn kind(&self) -> Option<&ErrorKind> {
        self.kind.as_ref()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(kind) = self.kind() {
            write!(f, "{kind}")
        } else if let Some(source) = self.source.as_ref() {
            write!(f, "internal error: {source}")
        } else {
            write!(f, "internal error: unknown")
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|s| s.as_ref() as _)
    }
}

#[doc(hidden)]
impl From<tw::Error> for Error {
    fn from(value: tw::Error) -> Self {
        Self::internal(value)
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ErrorKind {}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {}
    }
}

#[derive(Debug)]
struct Msg(&'static str);

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Msg {}
