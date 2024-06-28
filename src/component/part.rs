use core::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq, Debug)]
/// The component parts of a SemVer version.
pub enum PartType {
    /// The major version number in a version string.
    ///
    /// For example, in the version string `1.9.8`, `1` denotes
    /// the major version.
    Major,
    /// The minor version number in a version string.
    ///
    /// For example, in the version string `1.9.8`, `9` denotes
    /// the minor version.
    Minor,
    /// The patch version number in a version string.
    ///
    /// For example, in the version string `1.9.8`, `8` denotes
    /// the patch version.
    Patch,
    /// The prerelease identifier in a version string.
    ///
    /// For example, in the version string `0.1.0-alpha.1`, `alpha.1` denotes
    /// the prerelease identifier.
    Prerelease,
    /// The build metadata in a version string.
    ///
    /// For example, in the version string `0.1.0-alpha.1+a14`, `a14` denotes
    /// the build metadata.
    BuildMetadata,
}

impl Display for PartType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PartType::Major => "major",
                PartType::Minor => "minor",
                PartType::Patch => "patch",
                PartType::Prerelease => "prerelease",
                PartType::BuildMetadata => "build metadata",
            }
        )
    }
}
