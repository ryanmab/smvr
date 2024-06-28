use crate::component::PartType;

/// Error parsing a version string.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// A part of the provided version string contains an invalid character.
    InvalidCharacter(PartType),

    /// A part of the provided version string includes a preceding zero, which is not
    /// allowed.
    InvalidPrecedingZero(PartType),
}
