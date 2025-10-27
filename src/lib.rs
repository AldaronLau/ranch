//! Ranged integer types and math.

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

pub mod conversions;
mod convert;
mod error;
mod format;
pub mod parsing;
mod ranged_i8;
mod ranged_u128;
mod ranged_u16;
mod ranged_u32;
mod ranged_u64;
mod ranged_u8;

pub use self::{
    error::{Error, Result},
    ranged_i8::RangedI8,
    ranged_u8::RangedU8,
    ranged_u16::RangedU16,
    ranged_u32::RangedU32,
    ranged_u64::RangedU64,
    ranged_u128::RangedU128,
};
use crate::parsing::{Error as ParsingError, Result as ParsingResult};
