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
//! ## Optional Features
//!
//! Ranch optionally works with a few other crates:
//!
//!  - Enable [**_`arbitrary`_**](https://docs.rs/crate/arbitrary) for
//!    generating ranged integers from raw fuzzing data
//!  - Enable [**_`bitflags`_**](https://docs.rs/crate/bitflags) for using
//!    [`bitwise`] types as bitflags
//!  - Enable [**_`bytemuck`_**](https://docs.rs/crate/bytemuck) for casting as
//!    "plain old data"
//!  - Enable [**_`fastrand`_**](https://docs.rs/crate/fastrand) for RNG support
//!  - Enable [**_`rand.v010`_**](https://docs.rs/crate/rand) for using with the
//!    rand crate
//!  - Enable [**_`serde`_**](https://docs.rs/crate/serde) for serialization and
//!    deserialization
//!  - Enable [**_`zeroize`_**](https://docs.rs/crate/zeroize) for secure secret
//!    clearing
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
//! provided operation methods will never result in UB (even if unsafe is used
//! to set the inner value to something out of range), but may result in logic
//! bugs and panics on invalid bit patterns.
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
//! # use ranch::{RangedI32, range::RangeI32};
//! assert_eq!(
//!     RangedI32::<2, 7>::new::<2>().add_to::<5, RangeI32<7, 12>>(),
//!     RangedI32::<7, 12>::new::<7>(),
//! );
//! assert_eq!(
//!     RangedI32::<2, 7>::new::<2>().add_to::<6, RangeI32<8, 13>>(),
//!     RangedI32::<8, 13>::new::<8>(),
//! );
//! ```
//!
//! **Note:** After [`feature(generic_const_exprs)`] stabilizes, new functions
//! without the `_to` suffix will remove the generic for the return range.
//!
//! ## Ranged
//!
//! Similar to constant operations, these modify the output's range, but also
//! allow for some runtime variation.  In this example, the value added to a
//! number between 2 and 7 can be either 6 or 7:
//!
//! ```rust
//! # use ranch::RangedI32;
//! let a: RangedI32<8, 14> = RangedI32::<2, 7>::new::<2>()
//!     .add_ranged_to(RangedI32::<6, 7>::new::<6>());
//! let b: RangedI32<8, 14> = RangedI32::<2, 7>::new::<2>()
//!     .add_ranged_to(RangedI32::<6, 7>::new::<7>());
//!
//! assert_eq!(a, 8);
//! assert_eq!(b, 9);
//! ```
//!
//! **Note:** After [`feature(generic_const_exprs)`] stabilizes, new functions
//! without the `_to` suffix will remove the generic for the return range.
//!
//! # Indexing arrays
//!
//! Ranch can also be used to index arrays with unsigned ranged integers.
//!
//! ```rust
//! # use ranch::{RangedU32, unit::UnitU32};
//! let a = [1, 2, 3];
//! let i = RangedU32::<0, 2>::new::<1>();
//! let j = UnitU32::<1>::default();
//!
//! assert_eq!(a[i], 2);
//! assert_eq!(a[j], 2);
//! ```
//!
//! Won't compile if the range's maximum exceeds the last index:
//!
//! ```rust,compile_fail,E0080
//! # use ranch::RangedU32;
//! let a = [1, 2, 3];
//! let i = RangedU32::<0, 3>::new::<1>();
//!
//! assert_eq!(a[i], 2);
//! ```
//!
//! [deranged]: https://docs.rs/crate/deranged
//! [ux]: https://docs.rs/crate/ux
//! [validate the range once]: RangedI32::with_i32()
//! [at compile time]: RangedI32::new()
//! [`Serialize`]: serde_core::Serialize
//! [`Deserialize`]: serde_core::Deserialize
//! [`feature(generic_const_exprs)`]: https://github.com/rust-lang/rust/issues/76560

#![cfg(feature = "full")]
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

mod cast {
    pub(super) mod as_primitive;
    mod as_repr;
    pub(super) mod as_repr_primitive;
    mod to;
}

mod crates {
    #[cfg(feature = "arbitrary.v1")]
    mod arbitrary;
    #[cfg(feature = "bitflags.v2")]
    mod bitflags;
    #[cfg(feature = "bytemuck.v1")]
    mod bytemuck;
    #[cfg(feature = "fastrand.v2")]
    mod fastrand;
    #[cfg(feature = "rand.v010")]
    mod rand;
    #[cfg(feature = "serde.v1")]
    mod serde;
    #[cfg(feature = "zeroize.v1")]
    mod zeroize;
}

mod ranged {
    mod i128;
    mod i16;
    mod i32;
    mod i64;
    mod i8;
    mod u128;
    mod u16;
    mod u32;
    mod u64;
    mod u8;
}

mod nonzero {
    mod i128;
    mod i16;
    mod i32;
    mod i64;
    mod i8;
    mod u128;
    mod u16;
    mod u32;
    mod u64;
    mod u8;
}

mod num {
    pub(super) mod aliases;
    pub(super) mod marker;
    pub(super) mod rangeable_primitive;
    pub(super) mod ranged;
}

mod ops {
    mod cmp;
    mod misc;
    mod nonzero_signed;
    mod nonzero_unsigned;
    mod signed;
    mod unsigned;
}

pub mod ascii;
mod assertions;
mod assign;
mod bitops;
pub mod bitwise;
mod convert;
mod error;
mod format;
mod from_repr;
mod impl_ascii;
mod index;
pub mod multirange;
mod neg;
mod ord;
pub mod parsing;
mod quotient;
mod random;
pub mod range;
mod shl;
mod shr;
pub mod types;
pub mod unit;

pub use self::{
    error::{Error, Result},
    num::aliases::{
        RangedI8, RangedI16, RangedI32, RangedI64, RangedI128, RangedNonZeroI8,
        RangedNonZeroI16, RangedNonZeroI32, RangedNonZeroI64,
        RangedNonZeroI128, RangedNonZeroU8, RangedNonZeroU16, RangedNonZeroU32,
        RangedNonZeroU64, RangedNonZeroU128, RangedU8, RangedU16, RangedU32,
        RangedU64, RangedU128,
    },
    quotient::Quotient,
};
use crate::parsing::{Error as ParsingError, Result as ParsingResult};
