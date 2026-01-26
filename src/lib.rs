//! Ranged integer types and math
//!
//! Do you ever need to restrict a [`u8`] from 0 to 100 or restrict any other
//! integer type to any other range?  Then this crate is for you!  The ranges
//! are encoded in the type system, so you only need to
//! [validate the range once] (and it can even be [at compile time]!).  This
//! crate is sort of like a combination of similar crates [deranged] and [ux].
//!
//! This crate heavily leverages the type system to allow for powerful ranged
//! integer mathematics, covering [arbitrary `i{N}` / `u{N}` types](bitwise),
//! [unit integers](mod@unit), non-zero divisions, [ASCII](ascii), and const
//! operations.  Enable the _**`serde`**_ feature for range-validated
//! deserialization / serialization (implements [`Serialize`] and
//! [`Deserialize`] for `Ranged*` types).
//!
//! [deranged]: https://docs.rs/crate/deranged
//! [ux]: https://docs.rs/crate/ux
//! [validate the range once]: RangedI32::with_i32()
//! [at compile time]: RangedI32::new()
//! [`Serialize`]: serde_core::Serialize
//! [`Deserialize`]: serde_core::Deserialize

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg"
)]
#![no_std]
#![deny(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]
#![deny(
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    rustdoc::unescaped_backticks,
    rustdoc::redundant_explicit_links
)]

pub mod ascii;
mod assertions;
mod assign;
pub mod bitwise;
mod convert;
mod error;
mod format;
mod impl_ascii;
mod ops;
mod ord;
pub mod parsing;
mod quotient;
mod ranged {
    pub(super) mod i128;
    pub(super) mod i16;
    pub(super) mod i32;
    pub(super) mod i64;
    pub(super) mod i8;
    pub(super) mod u128;
    pub(super) mod u16;
    pub(super) mod u32;
    pub(super) mod u64;
    pub(super) mod u8;
}
mod nonzero {
    pub(super) mod i128;
    pub(super) mod i16;
    pub(super) mod i32;
    pub(super) mod i64;
    pub(super) mod i8;
    pub(super) mod u128;
    pub(super) mod u16;
    pub(super) mod u32;
    pub(super) mod u64;
    pub(super) mod u8;
}
mod repr;
#[cfg(feature = "serde")]
mod serde;
pub mod unit;

pub use self::{
    error::{Error, Result},
    nonzero::{
        i8::RangedNonZeroI8, i16::RangedNonZeroI16, i32::RangedNonZeroI32,
        i64::RangedNonZeroI64, i128::RangedNonZeroI128, u8::RangedNonZeroU8,
        u16::RangedNonZeroU16, u32::RangedNonZeroU32, u64::RangedNonZeroU64,
        u128::RangedNonZeroU128,
    },
    quotient::Quotient,
    ranged::{
        i8::RangedI8, i16::RangedI16, i32::RangedI32, i64::RangedI64,
        i128::RangedI128, u8::RangedU8, u16::RangedU16, u32::RangedU32,
        u64::RangedU64, u128::RangedU128,
    },
};
use crate::parsing::{Error as ParsingError, Result as ParsingResult};
