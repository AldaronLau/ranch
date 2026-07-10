use core::{error, fmt, num::NonZero, result, str::FromStr};

use crate::{error::*, *};

macro_rules! parse {
    ($nonzero:ident, $ranged:ident, $p:ident, $with:ident) => {
        impl<const MIN: $p, const MAX: $p> FromStr for $ranged<MIN, MAX> {
            type Err = ParseIntError;

            fn from_str(src: &str) -> ParseIntResult<Self> {
                let parsed = src.parse::<$p>()?;

                Self::$with(parsed).map_err(From::from)
            }
        }

        impl<const MIN: $p, const MAX: $p> FromStr for $nonzero<MIN, MAX> {
            type Err = ParseNonZeroIntError;

            fn from_str(src: &str) -> ParseNonZeroIntResult<Self> {
                Self::with_nonzero(src.parse::<NonZero<$p>>()?)
                    .map_err(From::from)
            }
        }
    };
}

parse!(RangedNonZeroI8, RangedI8, i8, with_i8);
parse!(RangedNonZeroI16, RangedI16, i16, with_i16);
parse!(RangedNonZeroI32, RangedI32, i32, with_i32);
parse!(RangedNonZeroI64, RangedI64, i64, with_i64);
parse!(RangedNonZeroI128, RangedI128, i128, with_i128);

parse!(RangedNonZeroU8, RangedU8, u8, with_u8);
parse!(RangedNonZeroU16, RangedU16, u16, with_u16);
parse!(RangedNonZeroU32, RangedU32, u32, with_u32);
parse!(RangedNonZeroU64, RangedU64, u64, with_u64);
parse!(RangedNonZeroU128, RangedU128, u128, with_u128);
