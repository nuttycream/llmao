#![doc = include_str!("../README.md")]

pub mod extract;
pub mod params;

pub trait Error: std::fmt::Debug {
    fn kind(&self) -> ErrorKind;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    Other,
}
