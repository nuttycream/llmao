use crate::Provider;

pub trait Error: std::fmt::Debug {
    fn kind(&self) -> ErrorKind;
}

/// Chat error kinds
///
/// This represents set of chat errors
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    Other,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            ErrorKind::Other => write!(f, "Other"),
        }
    }
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

/// Chat trait
pub trait Chat: Provider
where
    Self::Error: Error,
{
    type Message;
    type Response;

    fn chat(
        &mut self,
        messages: &[Self::Message],
    ) -> Result<Self::Response, Self::Error>;
}
