//! Range utilities
//!
//! Some convenience utilities for type ranges.

use core::{error, fmt, iter, num::NonZero, ops::RangeInclusive, result};

pub use super::random::*;
use super::*;

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

/// A type with multiple valid ranges of values
pub trait MultiRange<T = Self> {
    /// The minimum value of the type
    const MIN: T;
    /// The maximum value of the type
    const MAX: T;

    /// Return an iterator of each valid range.
    ///
    /// The ranges should be returned from lowest to highest value, and never
    /// overlap (although this isn't enforced by the trait).
    fn ranges() -> impl Iterator<Item = RangeInclusive<T>>;
}

impl<T, U> MultiRange<T> for U
where
    U: Range<T>,
{
    const MAX: T = U::MAX;
    const MIN: T = U::MIN;

    fn ranges() -> impl Iterator<Item = RangeInclusive<T>> {
        iter::once(range_inclusive::<U, T>())
    }
}

/// A type with a valid range of values
pub trait Range<T = Self> {
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

macro_rules! ranged_impl_range {
    ($r:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> Range<$p> for $r<MIN, MAX> {
            const MAX: $p = MAX;
            const MIN: $p = MIN;
        }

        impl<const MIN: $p, const MAX: $p> Range<$r<MIN, MAX>>
            for $r<MIN, MAX>
        {
            const MAX: $r<MIN, MAX> = Self::MAX;
            const MIN: $r<MIN, MAX> = Self::MIN;
        }
    };
}

macro_rules! nonzero_multirange_impl {
    ($r:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> MultiRange<$p> for $r<MIN, MAX> {
            const MAX: $p = MAX;
            const MIN: $p = MIN;

            fn ranges() -> impl Iterator<Item = RangeInclusive<$p>> {
                iter::once(RangeInclusive::new(MIN, -1))
                    .chain(iter::once(RangeInclusive::new(1, MAX)))
                    .filter(|range| !range.is_empty())
            }
        }

        impl<const MIN: $p, const MAX: $p> MultiRange<$r<MIN, MAX>>
            for $r<MIN, MAX>
        {
            const MAX: $r<MIN, MAX> = Self::MAX;
            const MIN: $r<MIN, MAX> = Self::MIN;

            fn ranges() -> impl Iterator<Item = RangeInclusive<Self>> {
                iter::once(RangeInclusive::new(Self::MIN, $r::new::<-1>()))
                    .chain(iter::once(RangeInclusive::new(
                        $r::new::<1>(),
                        Self::MAX,
                    )))
                    .filter(|range| !range.is_empty())
            }
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

macro_rules! nonzero_impl_multirange {
    ($p:ty) => {
        impl MultiRange for NonZero<$p> {
            const MAX: Self = Self::MAX;
            const MIN: Self = Self::MIN;

            fn ranges() -> impl Iterator<Item = RangeInclusive<Self>> {
                iter::once(RangeInclusive::new(
                    Self::MIN,
                    const { NonZero::new(-1).unwrap() },
                ))
                .chain(iter::once(RangeInclusive::new(
                    const { NonZero::new(1).unwrap() },
                    Self::MAX,
                )))
                .filter(|range| !range.is_empty())
            }
        }

        impl MultiRange<$p> for NonZero<$p> {
            const MAX: $p = <$p>::MAX;
            const MIN: $p = <$p>::MIN;

            fn ranges() -> impl Iterator<Item = RangeInclusive<$p>> {
                iter::once(RangeInclusive::new(<$p>::MIN, -1))
                    .chain(iter::once(RangeInclusive::new(1, <$p>::MAX)))
                    .filter(|range| !range.is_empty())
            }
        }
    };
}

macro_rules! range_nonzero_impl {
    ($r:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> Range<NonZero<$p>> for $r<MIN, MAX> {
            const MAX: NonZero<$p> = const { NonZero::new(MAX).unwrap() };
            const MIN: NonZero<$p> = const { NonZero::new(MIN).unwrap() };
        }
    };
}

macro_rules! multirange_nonzero_impl {
    ($r:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> MultiRange<NonZero<$p>>
            for $r<MIN, MAX>
        {
            const MAX: NonZero<$p> = const { NonZero::new(MAX).unwrap() };
            const MIN: NonZero<$p> = const { NonZero::new(MIN).unwrap() };

            fn ranges() -> impl Iterator<Item = RangeInclusive<NonZero<$p>>> {
                iter::once(RangeInclusive::new(
                    const { NonZero::new(MIN).unwrap() },
                    const { NonZero::new(-1).unwrap() },
                ))
                .chain(iter::once(RangeInclusive::new(
                    const { NonZero::new(1).unwrap() },
                    const { NonZero::new(MAX).unwrap() },
                )))
                .filter(|range| !range.is_empty())
            }
        }
    };
}

ranged_impl_range!(RangedU8, u8);
ranged_impl_range!(RangedU16, u16);
ranged_impl_range!(RangedU32, u32);
ranged_impl_range!(RangedU64, u64);
ranged_impl_range!(RangedU128, u128);
ranged_impl_range!(RangedI8, i8);
ranged_impl_range!(RangedI16, i16);
ranged_impl_range!(RangedI32, i32);
ranged_impl_range!(RangedI64, i64);
ranged_impl_range!(RangedI128, i128);

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

ranged_impl_range!(RangedNonZeroU8, u8);
ranged_impl_range!(RangedNonZeroU16, u16);
ranged_impl_range!(RangedNonZeroU32, u32);
ranged_impl_range!(RangedNonZeroU64, u64);
ranged_impl_range!(RangedNonZeroU128, u128);

nonzero_multirange_impl!(RangedNonZeroI8, i8);
nonzero_multirange_impl!(RangedNonZeroI16, i16);
nonzero_multirange_impl!(RangedNonZeroI32, i32);
nonzero_multirange_impl!(RangedNonZeroI64, i64);
nonzero_multirange_impl!(RangedNonZeroI128, i128);

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

nonzero_impl_multirange!(i8);
nonzero_impl_multirange!(i16);
nonzero_impl_multirange!(i32);
nonzero_impl_multirange!(i64);
nonzero_impl_multirange!(i128);

range_nonzero_impl!(RangedNonZeroU8, u8);
range_nonzero_impl!(RangedNonZeroU16, u16);
range_nonzero_impl!(RangedNonZeroU32, u32);
range_nonzero_impl!(RangedNonZeroU64, u64);
range_nonzero_impl!(RangedNonZeroU128, u128);

multirange_nonzero_impl!(RangedNonZeroI8, i8);
multirange_nonzero_impl!(RangedNonZeroI16, i16);
multirange_nonzero_impl!(RangedNonZeroI32, i32);
multirange_nonzero_impl!(RangedNonZeroI64, i64);
multirange_nonzero_impl!(RangedNonZeroI128, i128);
