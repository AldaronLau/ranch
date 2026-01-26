//! Error types related to parsing

use core::{error, fmt, num::NonZero, result};

use super::*;

/// Parsing ranged integer result
pub type Result<T = (), E = Error> = result::Result<T, E>;

/// Error parsing ranged integer
#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    /// Integer is too large to store in target integer type
    PosOverflow,
    /// Integer is too small to store in target integer type
    NegOverflow,
    /// Internal parsing error
    ParseInt(core::num::ParseIntError),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseInt(err) => err.fmt(f),
            Self::PosOverflow => f.write_str(
                "Integer is too large to store in target integer type",
            ),
            Self::NegOverflow => f.write_str(
                "Integer is too small to store in target integer type",
            ),
        }
    }
}

impl From<core::num::ParseIntError> for Error {
    fn from(error: core::num::ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}

impl From<crate::Error> for Error {
    fn from(error: crate::Error) -> Self {
        match error {
            crate::Error::PosOverflow => Self::PosOverflow,
            crate::Error::NegOverflow => Self::NegOverflow,
        }
    }
}

macro_rules! parse_nonzero {
    ($nonzero:ident, $p:ident) => {
        impl<const MIN: $p, const MAX: $p> core::str::FromStr
            for $nonzero<MIN, MAX>
        {
            type Err = Error;

            fn from_str(src: &str) -> Result<Self> {
                Self::with_nonzero(src.parse::<NonZero<$p>>()?)
                    .map_err(From::from)
            }
        }
    };
}

parse_nonzero!(RangedNonZeroI8, i8);
parse_nonzero!(RangedNonZeroI16, i16);
parse_nonzero!(RangedNonZeroI32, i32);
parse_nonzero!(RangedNonZeroI64, i64);
parse_nonzero!(RangedNonZeroI128, i128);

parse_nonzero!(RangedNonZeroU8, u8);
parse_nonzero!(RangedNonZeroU16, u16);
parse_nonzero!(RangedNonZeroU32, u32);
parse_nonzero!(RangedNonZeroU64, u64);
parse_nonzero!(RangedNonZeroU128, u128);
