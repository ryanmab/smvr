![Coverage](https://api.coveragerobot.com/v1/graph/github/ryanmab/smvr/badge.svg?token=c63b202336b40790d0e8963cc595bd90eb0e6d46630e222511)
[![Crates.io Version](https://img.shields.io/crates/v/smvr)](https://crates.io/crates/smvr)
[![docs.rs](https://img.shields.io/docsrs/smvr)](https://docs.rs/smvr)
[![Build](https://github.com/ryanmab/smvr/actions/workflows/build.yml/badge.svg)](https://github.com/ryanmab/smvr/actions/workflows/build.yml)
![GitHub License](https://img.shields.io/github/license/ryanmab/smvr)

<!-- cargo-rdme start -->

# smvr

A flexible parser and evaluator for interacting with Semantic Version strings.

[Semantic Versioning](https://semver.org/) is a specification outlining how version strings should
be formed, incremented and evaluated.

## Usage

```toml
[dependencies]
smvr = "0.1.1"
```

## Dialects

Dialects reflect varying implementations and interpretations of the SemVer specification.

Dialects implement a distinct parsing method for a version string, based on the version constraint's
origin. For example, differing package managers.

Dialect | Description
-|-
`smvr::Dialect::Standard` | Follows the [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) specification.

### Parsing version strings

Version strings should be parsed against a dialect to produce a `smvr::Version` instance.

This enforces validation while parsing inline with the dialect implementation.

```rust
use smvr::{BuildMetadata, Prerelease, PrereleaseComponent, Version};
use smvr::Dialect;
use smvr::Error;

let version = Version::parse(
    "10.2.1-alpha.1+build-1",
    Dialect::Standard
)?;

assert_eq!(version.major, 10);
assert_eq!(version.minor, 2);
assert_eq!(version.patch, 1);
assert_eq!(
    version.prerelease,
    Prerelease::Identifier(
        vec![
            PrereleaseComponent::String("alpha".to_string()),
            PrereleaseComponent::Number(1)
        ]
    )
);
assert_eq!(version.build_metadata, BuildMetadata::Identifier("build-1".to_string()));
```

### Comparing versions

`smvr::Version`'s can be compared inline with the original dialect implementation used to
parse the version string.

**Note:** In order to maintain consistency, only versions of the same dialect can be compared.

```rust
use smvr::{Dialect, Version};
use smvr::Error;

let version_1 = Version::parse("1.0.0", Dialect::Standard)?;
let version_1_0_1_alpha_9 = Version::parse("1.0.1-alpha.9", Dialect::Standard)?;
let version_1_0_1_beta_2 = Version::parse("1.0.1-beta.2", Dialect::Standard)?;
let version_1_0_1_beta_10 = Version::parse("1.0.1-beta.10", Dialect::Standard)?;
let version_1_0_1 = Version::parse("1.0.1", Dialect::Standard)?;

assert_ne!(version_1, version_1_0_1);
assert!(version_1 < version_1_0_1);

assert!(version_1_0_1_alpha_9 < version_1_0_1_beta_10);
assert!(version_1_0_1_beta_2 < version_1_0_1_beta_10);
assert!(version_1_0_1_beta_10 < version_1_0_1);
```

### Handling errors

While parsing, each byte will be read inline with the dialect's implementation. If any bytes are uncounted which
do not conform with how the rules the dialect implements, an error will be returned.

These errors indicate, at a high level, what the error was caused by (an invalid character, for example), which
part of the version is invalid.

These errors are eagerly returned - meaning the **first** error encountered will be returned, even if there are more
violations in the version string.

```rust
use smvr::{Dialect, PartType, Version};
use smvr::Error;

let invalid_preceding_zero_version = Version::parse("1.001.0", Dialect::Standard);
let invalid_character_version = Version::parse("abc.1.0", Dialect::Standard);

assert!(invalid_preceding_zero_version.is_err());
assert!(invalid_character_version.is_err());

if let Err(error) = invalid_preceding_zero_version {
    assert_eq!(error, Error::InvalidPrecedingZero(PartType::Minor))
}

if let Err(error) = invalid_character_version {
    assert_eq!(error, Error::InvalidCharacter(PartType::Major))
}
```

<!-- cargo-rdme end -->
