#![doc = include_str!("../README.md")]

/// Extract module - used for structured outputs or tool calling
pub mod extract;
/// Params module
pub mod params;

/// Core Provider trait
///
/// Essentially the core trait, all LLM providers must at
/// least implement this once, and create it's error types
/// Ideally, this would include Network/HTTPS requests and
/// Provider specific errors.
pub trait Provider {
    type Error;
}

impl<T: Provider + ?Sized> Provider for &mut T {
    type Error = T::Error;
}
