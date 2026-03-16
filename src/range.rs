//! Utilities for ranged types

use core::{error, fmt, num::NonZero, ops::RangeInclusive, result};

pub use super::{num::marker::*, random::*};
use crate::{
    multirange::{MultiRange, Rangeable, Ranged},
    *,
};

/// Validating an integer is within a range result
pub type Result<T = (), E = Error> = result::Result<T, E>;

/// Error validating an integer is within a range
///
/// Error returned when converting from integer to non-zero ranged integer.
///
/// It is also returned from [`result()`].
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Error {
    /// Integer is too large to store in target integer type
    PosOverflow,
    /// Integer is too small to store in target integer type
    NegOverflow,
    /// Integer is zero
    Zero,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Zero => "zero cannot be converted to target integer type",
            Self::PosOverflow => {
                "integer is too large to convert to target integer type"
            }
            Self::NegOverflow => {
                "integer is too small to convert to target integer type"
            }
        })
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

/// A type with a valid range of values
pub trait Range<T: Rangeable = Self>: Rangeable {
    /// The minimum value of the type
    const MIN: T;
    /// The maximum value of the type
    const MAX: T;
}

/// Get the inclusive range of a type with a range.
///
/// ```rust
/// # use core::ops::RangeInclusive;
/// # use ranch::{range, RangedI32};
/// assert_eq!(
///     range::range_inclusive::<u8, _>(),
///     RangeInclusive::new(0, 255),
/// );
///
/// assert_eq!(
///     range::range_inclusive::<RangedI32<0, 10>, _>(),
///     RangeInclusive::new(0, 10),
/// );
/// assert_eq!(
///     range::range_inclusive::<RangedI32<0, 10>, _>(),
///     RangeInclusive::new(RangedI32::new::<0>(), RangedI32::new::<10>()),
/// );
/// ```
pub const fn range_inclusive<T, U>() -> RangeInclusive<U>
where
    T: Range<U>,
    U: Rangeable,
{
    RangeInclusive::new(T::MIN, T::MAX)
}

/// Convert a result of an option to a range result.
///
/// ```rust
/// # use ranch::{range::{self, Error}, RangedNonZeroI32};
/// let a = RangedNonZeroI32::<-100, 100>::new::<50>();
///
/// assert_eq!(
///     (
///         range::result(a.checked_add(51)),
///         range::result(a.checked_sub(151)),
///         range::result(a.checked_sub(50)),
///         range::result(a.checked_add(1)),
///     ),
///     (
///         Err(Error::PosOverflow),
///         Err(Error::NegOverflow),
///         Err(Error::Zero),
///         Ok(RangedNonZeroI32::new::<51>()),
///     ),
/// );
/// ```
pub const fn result<T>(result: crate::Result<Option<T>>) -> Result<T>
where
    T: Copy + Clone,
{
    match result {
        Ok(Some(x)) => Ok(x),
        Ok(None) => Err(Error::Zero),
        Err(crate::Error::PosOverflow) => Err(Error::PosOverflow),
        Err(crate::Error::NegOverflow) => Err(Error::NegOverflow),
    }
}

macro_rules! primitive_impl_range {
    ($p:ty) => {
        impl Range for $p {
            const MAX: $p = <$p>::MAX;
            const MIN: $p = <$p>::MIN;
        }
    };
}

macro_rules! nonzero_impl_range {
    ($p:ty) => {
        impl Range<$p> for NonZero<$p> {
            const MAX: $p = <$p>::MAX;
            const MIN: $p = 1;
        }
    };
}

macro_rules! ranged_impl_range {
    ($p:ty) => {
        impl<R> Range<$p> for Ranged<$p, R>
        where
            R: MultiRange<$p>,
        {
            const MAX: $p = R::MAX;
            const MIN: $p = R::MIN;
        }

        impl<R> Range for Ranged<$p, R>
        where
            R: MultiRange<$p>,
        {
            const MAX: Self = Self::from_unchecked(R::MAX);
            const MIN: Self = Self::from_unchecked(R::MIN);
        }
    };
}

macro_rules! range_nonzero_impl {
    ($p:ty) => {
        impl<R> Range<$p> for Ranged<NonZero<$p>, R>
        where
            R: MultiRange<$p>,
        {
            const MAX: $p = R::MAX;
            const MIN: $p = R::MIN;
        }

        impl<R> Range for Ranged<NonZero<$p>, R>
        where
            R: MultiRange<$p>,
        {
            const MAX: Self =
                Self::from_unchecked(NonZero::new(R::MAX).unwrap());
            const MIN: Self =
                Self::from_unchecked(NonZero::new(R::MIN).unwrap());
        }

        impl<R> Range<NonZero<$p>> for Ranged<NonZero<$p>, R>
        where
            R: MultiRange<$p>,
        {
            const MAX: NonZero<$p> = const { NonZero::new(R::MAX).unwrap() };
            const MIN: NonZero<$p> = const { NonZero::new(R::MIN).unwrap() };
        }
    };
}

ranged_impl_range!(u8);
ranged_impl_range!(u16);
ranged_impl_range!(u32);
ranged_impl_range!(u64);
ranged_impl_range!(u128);
ranged_impl_range!(i8);
ranged_impl_range!(i16);
ranged_impl_range!(i32);
ranged_impl_range!(i64);
ranged_impl_range!(i128);

primitive_impl_range!(u8);
primitive_impl_range!(u16);
primitive_impl_range!(u32);
primitive_impl_range!(u64);
primitive_impl_range!(u128);
primitive_impl_range!(i8);
primitive_impl_range!(i16);
primitive_impl_range!(i32);
primitive_impl_range!(i64);
primitive_impl_range!(i128);

primitive_impl_range!(NonZero<u8>);
primitive_impl_range!(NonZero<u16>);
primitive_impl_range!(NonZero<u32>);
primitive_impl_range!(NonZero<u64>);
primitive_impl_range!(NonZero<u128>);

nonzero_impl_range!(u8);
nonzero_impl_range!(u16);
nonzero_impl_range!(u32);
nonzero_impl_range!(u64);
nonzero_impl_range!(u128);

range_nonzero_impl!(u8);
range_nonzero_impl!(u16);
range_nonzero_impl!(u32);
range_nonzero_impl!(u64);
range_nonzero_impl!(u128);
