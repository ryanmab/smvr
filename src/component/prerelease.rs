use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
/// The prerelease metadata for a particular version.
pub enum Prerelease {
    /// No prerelease metadata was provided.
    Empty,

    /// The individual components of the prerelease metadata.
    ///
    /// For example, in the version string `0.1.0-alpha.1`, `alpha.1` denotes the prerelease identifier,
    /// which is broken down into two components: `alpha` and `1`.
    Identifier(Vec<PrereleaseComponent>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
/// An individual piece of a prerelease identifier, as they were interpreted.
pub enum PrereleaseComponent {
    /// A numeric component of the prerelease identifier.
    ///
    /// For example, in the version string `0.1.0-alpha.1`, `alpha.1` denotes the prerelease identifier,
    /// and `1` is a numeric component.
    Number(usize),

    /// A string component of the prerelease identifier.
    ///
    /// For example, in the version string `0.1.0-alpha.1`, `alpha.1` denotes the prerelease identifier,
    /// and `alpha` is a string component.
    String(String),
}

impl Display for PrereleaseComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::String(str) => write!(f, "{str}"),
        }
    }
}
