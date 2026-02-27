//! Utilities for multiple-ranged types

use core::{iter, num::NonZero, ops::RangeInclusive};

pub use crate::num::ranged::Ranged;
use crate::{range::Range, *};

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
        iter::once(range::range_inclusive::<U, T>())
    }
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

nonzero_multirange_impl!(RangedNonZeroI8, i8);
nonzero_multirange_impl!(RangedNonZeroI16, i16);
nonzero_multirange_impl!(RangedNonZeroI32, i32);
nonzero_multirange_impl!(RangedNonZeroI64, i64);
nonzero_multirange_impl!(RangedNonZeroI128, i128);

nonzero_impl_multirange!(i8);
nonzero_impl_multirange!(i16);
nonzero_impl_multirange!(i32);
nonzero_impl_multirange!(i64);
nonzero_impl_multirange!(i128);

multirange_nonzero_impl!(RangedNonZeroI8, i8);
multirange_nonzero_impl!(RangedNonZeroI16, i16);
multirange_nonzero_impl!(RangedNonZeroI32, i32);
multirange_nonzero_impl!(RangedNonZeroI64, i64);
multirange_nonzero_impl!(RangedNonZeroI128, i128);
