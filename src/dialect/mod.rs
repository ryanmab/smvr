use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;

pub(crate) use standard::Standard;

use crate::component::PartType;
use crate::error::Error;
use crate::{BuildMetadata, Prerelease, Version};

mod standard;

pub(crate) type CapturedBytes = Vec<u8>;
pub(crate) type RemainingUnparsedBytes = [u8];
pub(crate) type NextPartType = Option<PartType>;

/// The specification to follow when parsing, validating, ordering and formatting of a particular version.
///
/// Dialects implement a distinct parsing method for a version string, based on the version constraint's
/// origin. For example, differing package managers.
///
/// Every version has to be parsed following a particular dialect - likely standard SemVer. However,
/// dialects open up support for version comparisons following particular behaviour outlined by
/// Cargo, or wider support for other languages like Composer (for PHP), npm (for JavaScript), etc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dialect {
    /// The standard dialect follows the [Semver Versioning 2.0.0](https://github.com/semver/semver/blob/master/semver.md#semantic-versioning-200) specification
    Standard,
}

pub(crate) trait DialectParser {
    fn parse_byte(
        byte: &u8,
        part: (PartType, &CapturedBytes),
        remaining_bytes: &RemainingUnparsedBytes,
    ) -> Result<NextPartType, Error> {
        if (part.0 == PartType::Patch || part.0 == PartType::Prerelease) && byte == &b'+' {
            return Ok(Some(PartType::BuildMetadata));
        }

        if part.0 == PartType::Patch && byte == &b'-' {
            return Ok(Some(PartType::Prerelease));
        }

        if byte == &b'.' {
            match part.0 {
                PartType::Major => return Ok(Some(PartType::Minor)),
                PartType::Minor => return Ok(Some(PartType::Patch)),
                PartType::Patch => return Ok(Some(PartType::Minor)),

                // The prerelease part is special, in that it doesn't have to transition to another
                // part of the version when encountering a dot. Specifically, the dot represents a new
                // piece of the same prerelease identifier, and not a transition to a different part
                // of the version.
                //
                // The parser will handle internal transitions in the prerelease identifier and ensure
                // the part is treated additively inside the prerelease.
                PartType::Prerelease => return Ok(Some(PartType::Prerelease)),
                _ => {}
            }
        }

        match part.0 {
            PartType::Major => {
                if !(&b'0'..=&b'9').contains(&byte) {
                    // Major, minor and patch versions can only be digits
                    return Err(Error::InvalidCharacter(part.0));
                }

                let is_first_digit = part.1.is_empty();
                let is_last_digit = remaining_bytes.is_empty() || remaining_bytes[0] == b'.';

                if byte == &b'0' && is_first_digit && !is_last_digit {
                    // Major can begin with zero, only when it's the only digit (like 0.1.0)
                    return Err(Error::InvalidPrecedingZero(part.0));
                }
            }
            PartType::Minor => {
                if !(&b'0'..=&b'9').contains(&byte) {
                    // Major, minor and patch versions can only be digits
                    return Err(Error::InvalidCharacter(part.0));
                }

                let is_first_digit = part.1.is_empty();
                let is_last_digit = remaining_bytes.is_empty() || remaining_bytes[0] == b'.';

                if byte == &b'0' && (is_first_digit && !is_last_digit) {
                    // Minor and patch can never start with a zero
                    return Err(Error::InvalidPrecedingZero(part.0));
                }
            }
            PartType::Patch => {
                if !(&b'0'..=&b'9').contains(&byte) {
                    // Major, minor and patch versions can only be digits
                    return Err(Error::InvalidCharacter(part.0));
                }

                let is_first_digit = part.1.is_empty();
                let is_last_digit = remaining_bytes.is_empty()
                    || (remaining_bytes[0] == b'+' || remaining_bytes[0] == b'-');

                if byte == &b'0' && (is_first_digit && !is_last_digit) {
                    // Minor and patch can never start with a zero
                    return Err(Error::InvalidPrecedingZero(part.0));
                }
            }
            PartType::Prerelease => {
                if !byte.is_ascii_alphanumeric() && byte != &b'-' {
                    return Err(Error::InvalidCharacter(part.0));
                }
            }
            PartType::BuildMetadata => {
                if !byte.is_ascii_alphanumeric() && byte != &b'-' && byte != &b'.' {
                    return Err(Error::InvalidCharacter(part.0));
                }
            }
        }

        Ok(None)
    }

    /// Compare two versions and impose the intended order of each version, based on the dialect.
    ///
    /// For example: `1.0.0-alpha.1` < `1.0.0-alpha.2` < `1.0.0-beta` < `1.0.0` < `1.0.1`
    ///
    /// The default implementation follows the [Semantic Versioning 2.0.0 specification](https://github.com/semver/semver/blob/master/semver.md#semantic-versioning-specification-semver).
    fn cmp(a: &Version, b: &Version) -> Ordering {
        if a.major != b.major {
            return if a.major > b.major {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }

        if a.minor != b.minor {
            return if a.minor > b.minor {
                return Ordering::Greater;
            } else {
                Ordering::Less
            };
        }

        if a.patch != b.patch {
            return if a.patch > b.patch {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }

        if let Prerelease::Identifier(a) = &a.prerelease {
            if let Prerelease::Identifier(b) = &b.prerelease {
                return if a < b {
                    Ordering::Less
                } else if a.eq(b) {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                };
            }

            return Ordering::Less;
        } else if b.prerelease != Prerelease::Empty {
            return Ordering::Greater;
        }

        Ordering::Equal
    }

    /// Compare two versions and decide if they're considered equal, based on the dialect.
    ///
    /// The default implementation follows the [Semantic Versioning 2.0.0 specification](https://github.com/semver/semver/blob/master/semver.md#semantic-versioning-specification-semver).
    fn eq(a: &Version, b: &Version) -> bool {
        a.major.eq(&b.major)
            && a.minor.eq(&b.minor)
            && a.patch.eq(&b.patch)
            && a.prerelease.eq(&b.prerelease)
    }

    /// Format a version back into a human-readable string.
    ///
    /// The output of this should match the original un-parsed version passed in.
    ///
    /// The default implementation follows the [Semantic Versioning 2.0.0 specification](https://github.com/semver/semver/blob/master/semver.md#semantic-versioning-specification-semver).
    fn format(version: &Version) -> String {
        let mut string = format!("{}.{}.{}", version.major, version.minor, version.patch);

        if let Prerelease::Identifier(identifier) = &version.prerelease {
            string.push_str(&format!(
                "-{}",
                identifier
                    .iter()
                    .fold(String::new(), |mut str, part| {
                        str.push_str(&format!(".{}", part));

                        str
                    })
                    .trim_start_matches('.')
            ));
        }

        if let BuildMetadata::Identifier(identifier) = &version.build_metadata {
            string.push_str(&format!("+{}", identifier));
        }

        string
    }
}
