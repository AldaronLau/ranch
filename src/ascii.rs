//! ASCII Types
//!
//! This could be an alternate implementation for
//! <https://github.com/rust-lang/rust/issues/110998> using ranged integers.

use core::ascii::EscapeDefault;

use super::*;

/// ASCII uppercase character
pub type Uppercase = RangedNonZeroU8<0x41, 0x5A>;

/// ASCII lowercase character
pub type Lowercase = RangedNonZeroU8<0x61, 0x7A>;

/// ASCII graphic character
pub type Graphic = RangedNonZeroU8<0x21, 0x7E>;

/// ASCII digit character
pub type Digit = RangedNonZeroU8<0x30, 0x39>;

/// One of the 127 Unicode characters from U+0001 through U+007F.
///
/// Can be used for niche optimization with ASCII characters.
pub type NonNul = bitwise::NonZeroU7;

/// One of the 128 Unicode characters from U+0000 through U+007F, often known as
/// the ASCII subset.
pub type Char = bitwise::U7;
