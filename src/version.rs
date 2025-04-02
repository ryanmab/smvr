use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt::{Debug, Display};

use crate::component::{BuildMetadata, PartType, Prerelease, PrereleaseComponent};
use crate::dialect;
use crate::dialect::Dialect::Standard;
use crate::dialect::{CapturedBytes, Dialect, DialectParser, NextPartType, RemainingUnparsedBytes};
use crate::error::Error;

#[derive(Debug)]
/// A parsed version string, conforming to a specific dialect.
///
/// Instances of `Version` can be compared against one another, and formatted as a string.
///
/// ## Example
///
/// ```rust
/// use smvr::{Dialect, Version};
/// let version = Version::parse("0.1.4-beta", Dialect::Standard);
///
/// # assert_eq!(version.unwrap().to_string(), "0.1.4-beta".to_string())
/// ```
pub struct Version {
    dialect: Dialect,

    /// The major version number.
    pub major: usize,

    /// The minor version number.
    pub minor: usize,

    /// The patch version number.
    pub patch: usize,

    /// The prerelease identifier, if provided.
    pub prerelease: Prerelease,

    /// The build metadata, if provided.
    pub build_metadata: BuildMetadata,
}

impl Version {
    /// Parse a string into a Version instance, following a specific Semver dialect.
    ///
    /// ```
    /// use smvr::{Dialect, Version};
    /// let version = Version::parse("0.1.4-beta", Dialect::Standard);
    /// # assert_eq!(version.unwrap().to_string(), "0.1.4-beta".to_string())
    /// ```
    ///
    /// ## Errors
    ///
    /// If the version string is not valid for the chosen dialect, the _first_ error encountered will be returned.
    ///
    /// ## Panics
    ///
    /// If the version string is not valid utf-8, a panic will occur.
    pub fn parse(version: &str, dialect: Dialect) -> Result<Self, Error> {
        let version_bytes = version.as_bytes();

        let (mut major, mut minor, mut patch, mut prerelease, mut build_metadata) =
            (vec![], vec![], vec![], vec![], vec![]);

        let mut current_part_type = PartType::Major;
        let mut remaining = version_bytes;
        loop {
            let part = Self::parse_part(remaining, dialect, current_part_type)?;

            let (part, r, next_part_type) = part;

            match current_part_type {
                PartType::Major => major = part,
                PartType::Minor => minor = part,
                PartType::Patch => patch = part,
                PartType::Prerelease => prerelease.push(part),
                PartType::BuildMetadata => build_metadata = part,
            }

            if next_part_type.is_none() {
                break;
            }

            remaining = r;
            current_part_type = next_part_type.unwrap();
        }

        Ok(Self::new(
            alloc::str::from_utf8(&major[..])
                .unwrap()
                .parse::<usize>()
                .unwrap_or_default(),
            alloc::str::from_utf8(&minor[..])
                .unwrap()
                .parse::<usize>()
                .unwrap_or_default(),
            alloc::str::from_utf8(&patch[..])
                .unwrap()
                .parse::<usize>()
                .unwrap_or_default(),
            if prerelease.is_empty() {
                None
            } else {
                Some(
                    prerelease
                        .iter()
                        .map(|part| {
                            if part.iter().all(|i| (&b'0'..=&b'9').contains(&i)) {
                                PrereleaseComponent::Number(
                                    alloc::str::from_utf8(&part[..])
                                        .unwrap()
                                        .parse::<usize>()
                                        .unwrap_or_default(),
                                )
                            } else {
                                PrereleaseComponent::String(
                                    alloc::str::from_utf8(&part[..]).unwrap().to_string(),
                                )
                            }
                        })
                        .collect(),
                )
            },
            if build_metadata.is_empty() {
                None
            } else {
                Some(
                    alloc::str::from_utf8(&build_metadata[..])
                        .unwrap()
                        .to_string(),
                )
            },
            dialect,
        ))
    }

    /// Progressively parse and return one particular part of a version string.
    ///
    /// The end point of a part is determined by the chosen dialect.
    ///
    /// With each byte parsed the dialect decides whether:
    /// 1. The byte is valid inside the part (i.e. it's a digit when inside the minor part of a
    ///    version string).
    /// 2. Whether to consume the byte, or include
    fn parse_part(
        version_bytes: &[u8],
        dialect: Dialect,
        current_part: PartType,
    ) -> Result<(CapturedBytes, &RemainingUnparsedBytes, NextPartType), Error> {
        let mut part = vec![];

        for (i, byte) in version_bytes.iter().enumerate() {
            let next_part = match dialect {
                Standard => dialect::Standard::parse_byte(
                    *byte,
                    (current_part, &part),
                    &version_bytes[i + 1..],
                ),
            }?;

            if next_part.is_some() {
                return Ok((part, &version_bytes[i + 1..], next_part));
            }

            part.push(byte.to_owned());
        }

        Ok((part, &[], None))
    }

    /// Create a new Version instance, using pre-parsed Semantic Version content.
    fn new(
        major: usize,
        minor: usize,
        patch: usize,
        prerelease: Option<Vec<PrereleaseComponent>>,
        build_metadata: Option<String>,
        dialect: Dialect,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
            prerelease: prerelease.map_or(Prerelease::Empty, |prerelease| {
                Prerelease::Identifier(prerelease)
            }),
            build_metadata: build_metadata.map_or(BuildMetadata::Empty, |metadata| {
                BuildMetadata::Identifier(metadata)
            }),
            dialect,
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        if self.dialect != other.dialect {
            // Only versions originating from the same dialect can be compared. This
            // prevents issues with inconsistent comparisons based on comparator order
            return false;
        }

        match self.dialect {
            Standard => dialect::Standard::eq(self, other),
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.dialect != other.dialect {
            // Only versions originating from the same dialect can be compared. This
            // prevents issues with inconsistent comparisons based on comparator order
            return None;
        }

        Some(match self.dialect {
            Standard => dialect::Standard::cmp(self, other),
        })
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self.dialect {
                Standard => dialect::Standard::format(self),
            }
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec;

    use proptest::prelude::*;

    use crate::dialect::Dialect;
    use crate::error::Error;
    use crate::version::{BuildMetadata, PartType, Prerelease, PrereleaseComponent, Version};

    #[test]
    fn parsing_minimal_version_with_standard_dialect() {
        let version = Version::parse("1", Dialect::Standard).unwrap();

        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
    }

    #[test]
    fn parsing_only_version_with_standard_dialect() {
        let version = Version::parse("1.2.2", Dialect::Standard).unwrap();

        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 2);
    }

    #[test]
    fn parsing_version_with_standard_dialect() {
        let version = Version::parse("12.19.1-alpha.12+build1234", Dialect::Standard).unwrap();

        assert_eq!(version.major, 12);
        assert_eq!(version.minor, 19);
        assert_eq!(version.patch, 1);
        assert_eq!(
            version.prerelease,
            Prerelease::Identifier(vec![
                PrereleaseComponent::String("alpha".to_string()),
                PrereleaseComponent::Number(12)
            ])
        );
        assert_eq!(
            version.build_metadata,
            BuildMetadata::Identifier("build1234".to_string())
        );

        assert_eq!("12.19.1-alpha.12+build1234", version.to_string());
    }

    #[test]
    fn parsing_version_with_proceeding_major_zero_standard_dialect() {
        let version = Version::parse("0.1.0", Dialect::Standard).unwrap();

        assert_eq!(version.major, 0);
        assert_eq!(version.minor, 1);
        assert_eq!(version.patch, 0);
    }

    #[test]
    fn parsing_version_with_only_build_metadata_standard_dialect() {
        let version = Version::parse("0.1.12+build.1234", Dialect::Standard).unwrap();

        assert_eq!(version.major, 0);
        assert_eq!(version.minor, 1);
        assert_eq!(version.patch, 12);
        assert_eq!(version.prerelease, Prerelease::Empty);
        assert_eq!(
            version.build_metadata,
            BuildMetadata::Identifier("build.1234".to_string())
        );
    }

    #[test]
    fn parsing_version_with_proceeding_zero_standard_dialect() {
        let version = Version::parse("12.019.1", Dialect::Standard);

        let Err(error) = version else {
            panic!("Parsing should have returned an error")
        };

        assert_eq!(error, Error::InvalidPrecedingZero(PartType::Minor));
    }

    proptest! {
        #[test]
        fn parses_various_valid_version_strings(
            version in "(0|[1-9][0-9]*)[.](0|[1-9][0-9]*)[.](0|[1-9][0-9]*)([-](0|[1-9A-Za-z-][0-9A-Za-z-]*)([.][0-9A-Za-z-]+)*)?([+][0-9A-Za-z-]+([.][0-9A-Za-z-]+)*)?"
        ) {
            let version = Version::parse(&version, Dialect::Standard);

            assert!(version.is_ok());
        }
    }
}
