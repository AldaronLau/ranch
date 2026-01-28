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
//! # Types of operations
//!
//! Like the std library, ranch provide [`strict`](#strict),
//! [`checked`](#checked), and [`saturating`](#saturating) integer operations.
//! In addition, ranch also provides [`constant`](#constant) and
//! [`ranged`](#ranged) operations, which modify the output ranges.
//!
//! ## Strict
//!
//! Strict operations panic when out of range, or a division by nonzero occurs.
//! This is exposed in ranch with `+`, `-`, `/`, `*`, `%`.  Using the other
//! provided operation methods will never result in panics or UB (even if unsafe
//! is used to set the inner value to something out of range).
//!
//! ```rust
//! # use ranch::RangedI32;
//! assert_eq!(
//!     RangedI32::<2, 7>::new::<2>() + 5,
//!     RangedI32::<2, 7>::new::<7>(),
//! );
//! ```
//!
//! Panics:
//!
//! ```rust,should_panic
//! # use ranch::RangedI32;
//! let _ = RangedI32::<2, 7>::new::<2>() + 6;
//! ```
//!
//! ## Checked
//!
//! Checked operations are methods starting with `checked_`; They return an
//! [`Option`] (unsigned) or [`Result`] (signed).  Divisions by zero-able types
//! additionally wrap the result in a [`Quotient`].
//!
//! ```rust
//! # use ranch::RangedI32;
//! assert_eq!(
//!     RangedI32::<2, 7>::new::<2>().checked_add(5).unwrap(),
//!     RangedI32::<2, 7>::new::<7>(),
//! );
//! RangedI32::<2, 7>::new::<2>().checked_add(6).unwrap_err();
//! ```
//!
//! ## Saturating
//!
//! Saturating operations are similar to checked, except that the `Option` and
//! `Result` are stripped and `Self::MIN` or `Self::MAX` is returned on
//! overflow.
//!
//! ```rust
//! # use ranch::RangedI32;
//! assert_eq!(
//!     RangedI32::<2, 7>::new::<2>().saturating_add(5),
//!     RangedI32::<2, 7>::new::<7>(),
//! );
//! assert_eq!(
//!     RangedI32::<2, 7>::new::<2>().saturating_add(6),
//!     RangedI32::<2, 7>::new::<7>(),
//! );
//! ```
//!
//! ## Constant
//!
//! Constant operations add a constant value and modify the output's range
//! accordingly.  Due to limitations in Rust, you have to specify the output
//! range, but ranch will check your work and tell you if you're wrong.
//!
//! ```rust
//! # use ranch::RangedI32;
//! assert_eq!(
//!     RangedI32::<2, 7>::new::<2>().add::<5, 7, 12>(),
//!     RangedI32::<7, 12>::new::<7>(),
//! );
//! assert_eq!(
//!     RangedI32::<2, 7>::new::<2>().add::<6, 8, 13>(),
//!     RangedI32::<8, 13>::new::<8>(),
//! );
//! ```
//!
//! ## Ranged
//!
//! Similar to constant operations, these modify the output's range, but also
//! allow for some runtime variation.  In this example, the value added to a
//! number between 2 and 7 can be either 6 or 7:
//!
//! ```rust
//! # use ranch::RangedI32;
//! assert_eq!(
//!     RangedI32::<2, 7>::new::<2>()
//!         .add_ranged(RangedI32::<6, 7>::new::<6>()),
//!     RangedI32::<8, 14>::new::<8>(),
//! );
//! assert_eq!(
//!     RangedI32::<2, 7>::new::<2>()
//!         .add_ranged(RangedI32::<6, 7>::new::<7>()),
//!     RangedI32::<8, 14>::new::<9>(),
//! );
//! ```
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
pub mod range;
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
