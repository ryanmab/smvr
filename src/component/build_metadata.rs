use alloc::string::String;

#[derive(Debug, PartialEq)]
/// The build metadata for a particular version.
pub enum BuildMetadata {
    Empty,
    Identifier(String),
}
