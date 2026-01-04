//! Ranged integer types and math
//!
//! This crate heavily leverages the type system to allow for powerful ranged
//! integer mathematics, covering [arbitrary `i{N}` / `u{N}` types](bitwise),
//! [unit integers](mod@unit), non-zero divisions, [ASCII](ascii), and const
//! operations.

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
pub mod bitwise;
mod convert;
mod error;
mod format;
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
