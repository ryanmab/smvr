use alloc::string::String;

#[derive(Debug, PartialEq, Eq)]
/// The build metadata for a particular version.
pub enum BuildMetadata {
    /// No build metadata was provided.
    Empty,

    /// The build metadata identifier.
    ///
    /// For example, in the version string `0.1.0-alpha.1+a14`, `a14` denotes the build metadata.
    Identifier(String),
}
