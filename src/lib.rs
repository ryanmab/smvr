#![crate_name = "smvr"]
#![no_std]

//! # smvr
//!
//! A flexible parser and evaluator for interacting with Semantic Version strings.
//!
//! [Semantic Versioning](https://semver.org/) is a specification outlining how version strings should
//! be formed, incremented and evaluated.
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! smvr = "0.1.1"
//! ```
//!
//! ## Dialects
//!
//! Dialects reflect implementations and interpretations of the SemVer specification.
//!
//! A dialect must implement a method for parsing a version string, following a deterministic set of
//! rules. For example, differing package managers may impose specific styling constraints.
//!
//! Dialect | Description
//! -|-
//! `smvr::Dialect::Standard` | Follows the [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) specification.
//!
//! ### Parsing version strings
//!
//! Version strings should be parsed, following a dialect, to produce a `smvr::Version` instance.
//!
//! Validation is enforced while parsing occurs, and is implemented by the chosen dialect.
//!
//! ```rust
//! use smvr::{BuildMetadata, Prerelease, PrereleaseComponent, Version};
//! use smvr::Dialect;
//! use smvr::Error;
//!
//! let version = Version::parse(
//!     "10.2.1-alpha.1+build-1",
//!     Dialect::Standard
//! )?;
//!
//! assert_eq!(version.major, 10);
//! assert_eq!(version.minor, 2);
//! assert_eq!(version.patch, 1);
//! assert_eq!(
//!     version.prerelease,
//!     Prerelease::Identifier(
//!         vec![
//!             PrereleaseComponent::String("alpha".to_string()),
//!             PrereleaseComponent::Number(1)
//!         ]
//!     )
//! );
//! assert_eq!(version.build_metadata, BuildMetadata::Identifier("build-1".to_string()));
//! # Ok::<(), Error>(())
//! ```
//!
//! ### Comparing versions
//!
//! Instances of `smvr::Version`, which are parsed from the same dialect, can be compared against once another.
//!
//! The comparison behaviour is dialect specific, and can be used to deterministically evaluate the chronology of two or more version strings.
//!
//! For example: `1.0.0-alpha.1` < `1.0.0-alpha.2` < `1.0.0-beta` < `1.0.0` < `1.0.1`
//!
//! ```rust
//! use smvr::{Dialect, Version};
//! use smvr::Error;
//!
//! let version_1 = Version::parse("1.0.0", Dialect::Standard)?;
//! let version_1_0_1_alpha_9 = Version::parse("1.0.1-alpha.9", Dialect::Standard)?;
//! let version_1_0_1_beta_2 = Version::parse("1.0.1-beta.2", Dialect::Standard)?;
//! let version_1_0_1_beta_10 = Version::parse("1.0.1-beta.10", Dialect::Standard)?;
//! let version_1_0_1 = Version::parse("1.0.1", Dialect::Standard)?;
//!
//! assert_ne!(version_1, version_1_0_1);
//! assert!(version_1 < version_1_0_1);
//!
//! assert!(version_1_0_1_alpha_9 < version_1_0_1_beta_10);
//! assert!(version_1_0_1_beta_2 < version_1_0_1_beta_10);
//! assert!(version_1_0_1_beta_10 < version_1_0_1);
//! # Ok::<(), Error>(())
//! ```
//!
//! ### Handling errors
//!
//! While parsing, each byte will be read, adhereing to a chosen dialect. If any bytes are encountered which do not
//! conform with the rules implemented by the dialect, an error will be returned.
//!
//! These errors indicate, at a high level, what the error was caused by (an invalid character, for example) and which
//! part of the version is invalid (i.e. Major, Minor, Patch, Prerelease, Build Metadata).
//!
//! The error is eagerly returned, which means only the **first** error encountered will provided, even if there are more
//! violations in the version string.
//!
//! ```rust
//! use smvr::{Dialect, PartType, Version};
//! use smvr::Error;
//!
//! let invalid_preceding_zero_version = Version::parse("1.001.0", Dialect::Standard);
//! let invalid_character_version = Version::parse("abc.1.0", Dialect::Standard);
//!
//! assert!(invalid_preceding_zero_version.is_err());
//! assert!(invalid_character_version.is_err());
//!
//! if let Err(error) = invalid_preceding_zero_version {
//!     assert_eq!(error, Error::InvalidPrecedingZero(PartType::Minor))
//! }
//!
//! if let Err(error) = invalid_character_version {
//!     assert_eq!(error, Error::InvalidCharacter(PartType::Major))
//! }
//! # Ok::<(), Error>(())
//! ```

extern crate alloc;

pub(crate) mod component;
pub(crate) mod dialect;
pub(crate) mod error;
pub(crate) mod version;

pub use component::*;
pub use dialect::Dialect;
pub use error::Error;
pub use version::*;
