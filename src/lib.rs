pub mod extract;
pub mod generation;
pub mod tool;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    Other,
}
