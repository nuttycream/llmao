use crate::Provider;

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
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            ErrorKind::NoData => {
                write!(f, "No data extracted")
            }
            ErrorKind::DeserializationFailed => {
                write!(f, "Failed to deserialize data")
            }
            ErrorKind::BadSchema => write!(
                f,
                "Response violated schema constraints"
            ),
        }
    }
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

/// The Extraction trait
/// # Examples
///
/// See the [example](https://github.com/nuttycream/llmao/blob/main/examples/extract.rs)
/// for a working implementation.
pub trait Extract<T>: Provider
where
    Self::Error: Error,
{
    type Prompt;
    type Content;

    /// Extract data from supplied content
    ///
    fn extract(
        &mut self,
        prompt: Self::Prompt,
        content: Self::Content,
    ) -> Result<T, Self::Error>;
}

impl<C, T> Extract<T> for &mut C
where
    C: Extract<T> + ?Sized,
    C::Error: Error,
{
    type Prompt = C::Prompt;
    type Content = C::Content;

    fn extract(
        &mut self,
        prompt: Self::Prompt,
        content: Self::Content,
    ) -> Result<T, Self::Error> {
        C::extract(self, prompt, content)
    }
}
