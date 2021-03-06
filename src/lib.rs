//! provides a number of utilities for writing media-type parsers in rust
//!
//! Utils:
//!
//! - `quoted_string`: provides implementations for a number of traits from the `quoted-string`
//!   crate including `ParsingImpl`, `QuotingClassifier` and `WithoutQuotingValidator`. Implementations
//!   are provided for a number of different use cases including media-types in http, mime as well as
//!   a more strict impl for media types compatible with all other implementations and a impl being
//!   usable with media-types compatible with any (/at last one) of the other implementations.
//!
//! - `lookup_table`: provides a `lut` lookup table for bytes/us-ascii chars used in context of
//!   media-type parsing.
//!
//! **Note: Currently is crate is rather unstable. It will still keep to
//!  semver but changing to a newer (braking) version might induce large
//!  api changes. Sill bug-fixes to older versions can be done if requested
//!  so it can be used as a internal dependency. Just don't expect that all
//!  braking changes will try to keep as much api compatibility as possible
//!  as it should be done with more stable crates**
#![warn(missing_docs)]

#[macro_use]
extern crate lut;
extern crate quoted_string as qs;
extern crate percent_encoding as penc;

/// lut lookup tables for parsing media types
pub mod lookup_tables;
/// impl of traits from the quoted-string crate for parsing media types
pub mod quoted_string;
/// impl of EncodingSet's for encoding parameter values if needed
pub mod percent_encoding;