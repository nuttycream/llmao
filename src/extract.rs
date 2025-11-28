pub trait Error: std::fmt::Debug {
    fn kind(&self) -> ErrorKind;
}

/// Extract error kinds
///
/// This represents set of extraction errors
/// some of which ripped from `rig-core`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// No data extracted from Provider Response
    NoData,

    /// Failed to deserialize the data
    DeserializationFailed,

    /// Provider response violated the schema constraints
    /// for structured outputs
    BadSchema,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::NoData => write!(f, "No data extracted"),
            ErrorKind::DeserializationFailed => write!(f, "Failed to deserialize data"),
            ErrorKind::BadSchema => write!(f, "Response violated schema constraints"),
        }
    }
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

/// Extraction eerror type trait.
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

// todo impl extract
// decide whether or not, we should have
// serde for deserialization generic
