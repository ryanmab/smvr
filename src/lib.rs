#![crate_name = "smvr"]
#![no_std]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_debug_implementations, rust_2018_idioms, rustdoc::all)]
#![allow(rustdoc::private_doc_tests)]
#![forbid(unsafe_code)]

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
//! smvr = "0.1.3"
//! ```
//!
//! ## Dialects
//!
//! Dialects reflect interpretations of the Semantic Versioning specification.
//!
//! A dialect must implement a method for parsing a version string in accordance with a deterministic set of
//! rules. For example, differing package managers may impose different constraints to the style of a version string. This is
//! the perfect use case for a dedicated dialect.
//!
//! Currently only Semver Versioning 2.0.0 is supported.
//!
//! Dialect | Description
//! -|-
//! `smvr::Dialect::Standard` | Follows the [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) specification.
//!
//! ## Parsing version strings
//!
//! Version strings are parsed to produce a `smvr::Version` instance. When attempting to parse a version string, the dialect
//! to use must be provided.
//!
//! Validation is enforced by the dialect and occurs while parsing. This helps ensure only valid version strings are returned.
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
//! ## Comparing versions
//!
//! Instances of `smvr::Version`, which were parsed using the same dialect, can be compared against one another.
//!
//! The comparison behaviour is specific to the dialect, and can be used to deterministically evaluate the chronology of two or more version strings.
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
//! ## Handling errors
//!
//! While parsing, each byte is be read, and if any bytes are encountered which do not conform with the rules implemented by
//! the dialect, an error will be returned.
//!
//! These errors indicate, at a high level, what the error was caused by (an invalid character, for example) and where
//! the error occurred (i.e. inside one of the parts: Major, Minor, Patch, Prerelease, Build Metadata).
//!
//! Errors are eagerly returned, which means **the first** invalid byte encountered will trigger an error. This does not guarantee there are no more
//! violations in the rest of the version string.
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
