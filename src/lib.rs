#![doc = include_str!("../README.md")]

pub mod extract;
pub mod params;

/// Provider trait
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
