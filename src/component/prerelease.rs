use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, PartialOrd)]
/// The prerelease metadata for a particular version.
///
/// If provided, the identifier is broken down into one or more prerelease components.
pub enum Prerelease {
    Empty,
    Identifier(Vec<PrereleaseComponent>),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// An individual piece of a prerelease identifier, as they were interpreted.
pub enum PrereleaseComponent {
    Number(usize),
    String(String),
}

impl Display for PrereleaseComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            PrereleaseComponent::Number(num) => write!(f, "{}", num),
            PrereleaseComponent::String(str) => write!(f, "{}", str),
        }
    }
}
