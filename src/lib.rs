//! Ranged integers

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg"
)]
#![no_std]
#![forbid(unsafe_code)]
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

mod error;
mod parsing_error;
mod ranged_i8;
mod ranged_u128;
mod ranged_u16;
mod ranged_u32;
mod ranged_u64;
mod ranged_u8;

use core::fmt;

pub use self::{
    error::{Error, Result},
    parsing_error::{ParsingError, ParsingResult},
    ranged_i8::RangedI8,
    ranged_u8::RangedU8,
    ranged_u16::RangedU16,
    ranged_u32::RangedU32,
    ranged_u64::RangedU64,
    ranged_u128::RangedU128,
};

macro_rules! impl_ranged_fmt {
    ($type:ident, $primitive:ty, [$($Trait:ident),* $(,)?] $(,)?) => {
        $(
            impl<const MIN: $primitive, const MAX: $primitive> fmt::$Trait
            for $type<MIN, MAX>
            where
            {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    <$primitive as fmt::$Trait>::fmt(&self.get(), f)
                }
            }
        )*
    };
}

impl_ranged_fmt!(
    RangedU8,
    u8,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedU16,
    u16,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedU32,
    u32,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedU64,
    u64,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedU128,
    u128,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);

impl_ranged_fmt!(
    RangedI8,
    i8,
    [
        Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp,
    ],
);
