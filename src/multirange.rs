//! Utilities for multiple-ranged types

use core::{num::NonZero, ops::RangeInclusive};

pub use crate::num::ranged::Ranged;
use crate::{cmp::Cmp, range::Range, *};

/// A type with multiple valid ranges of values
pub trait MultiRange<T: Rangeable = Self>: Rangeable {
    /// The minimum value of the type
    const MIN: T;
    /// The maximum value of the type
    const MAX: T;
    /// Each valid subrange (if range is continuous, should be empty).
    ///
    /// The ranges should be returned from lowest to highest value, and never
    /// overlap (although this isn't enforced by the trait).
    ///
    /// The defined ranges may be empty.
    const SUBRANGES: &'static [RangeInclusive<T>];
}

impl<T, U> MultiRange<T> for U
where
    T: Rangeable,
    U: Range<T>,
{
    const MAX: T = U::MAX;
    const MIN: T = U::MIN;
    const SUBRANGES: &'static [RangeInclusive<T>] = &[];
}

macro_rules! nonzero_impl_multirange {
    ($p:ty) => {
        impl MultiRange for NonZero<$p> {
            const MAX: Self = Self::MAX;
            const MIN: Self = Self::MIN;
            const SUBRANGES: &'static [RangeInclusive<NonZero<$p>>] = &[
                RangeInclusive::new(
                    Self::MIN,
                    const {
                        NonZero::new(min::<$p>(-1, Self::MAX.get())).unwrap()
                    },
                ),
                RangeInclusive::new(
                    const {
                        NonZero::new(max::<$p>(1, Self::MIN.get())).unwrap()
                    },
                    Self::MAX,
                ),
            ];
        }

        impl MultiRange<$p> for NonZero<$p> {
            const MAX: $p = <$p>::MAX;
            const MIN: $p = <$p>::MIN;
            const SUBRANGES: &'static [RangeInclusive<$p>] = &[
                RangeInclusive::new(<$p>::MIN, min::<$p>(-1, Self::MAX.get())),
                RangeInclusive::new(max::<$p>(1, Self::MIN.get()), <$p>::MAX),
            ];
        }
    };
}

macro_rules! nonzero_multirange_impl {
    ($p:ty) => {
        impl<R> MultiRange<$p> for Ranged<NonZero<$p>, R>
        where
            R: MultiRange<$p>,
        {
            const MAX: $p = R::MAX;
            const MIN: $p = R::MIN;
            const SUBRANGES: &'static [RangeInclusive<$p>] = &[
                RangeInclusive::new(R::MIN, min(-1, R::MAX)),
                RangeInclusive::new(max(1, R::MIN), R::MAX),
            ];
        }

        impl<R> MultiRange for Ranged<NonZero<$p>, R>
        where
            R: MultiRange<$p>,
        {
            const MAX: Self = Self::from_unchecked(R::MAX);
            const MIN: Self = Self::from_unchecked(R::MIN);
            const SUBRANGES: &'static [RangeInclusive<Self>] = &[
                RangeInclusive::new(
                    Self::MIN,
                    const {
                        Self::from_unchecked(
                            NonZero::new(min(-1, R::MAX)).unwrap(),
                        )
                    },
                ),
                RangeInclusive::new(
                    const {
                        Self::from_unchecked(
                            NonZero::new(max(1, R::MIN)).unwrap(),
                        )
                    },
                    Self::MAX,
                ),
            ];
        }
    };
}

macro_rules! multirange_nonzero_impl {
    ($p:ty) => {
        impl<R> MultiRange<NonZero<$p>> for Ranged<NonZero<$p>, R>
        where
            R: MultiRange<$p>,
        {
            const MAX: NonZero<$p> = const { NonZero::new(R::MAX).unwrap() };
            const MIN: NonZero<$p> = const { NonZero::new(R::MIN).unwrap() };
            const SUBRANGES: &'static [RangeInclusive<NonZero<$p>>] = &[
                RangeInclusive::new(
                    const { NonZero::new(R::MIN).unwrap() },
                    const { NonZero::new(min(-1, R::MAX)).unwrap() },
                ),
                RangeInclusive::new(
                    const { NonZero::new(max(1, R::MIN)).unwrap() },
                    const { NonZero::new(R::MAX).unwrap() },
                ),
            ];
        }
    };
}

nonzero_multirange_impl!(i8);
nonzero_multirange_impl!(i16);
nonzero_multirange_impl!(i32);
nonzero_multirange_impl!(i64);
nonzero_multirange_impl!(i128);

nonzero_impl_multirange!(i8);
nonzero_impl_multirange!(i16);
nonzero_impl_multirange!(i32);
nonzero_impl_multirange!(i64);
nonzero_impl_multirange!(i128);

multirange_nonzero_impl!(i8);
multirange_nonzero_impl!(i16);
multirange_nonzero_impl!(i32);
multirange_nonzero_impl!(i64);
multirange_nonzero_impl!(i128);

/// Return an iterator of ranges from a [`MultiRange`].
///
/// ```rust
/// # use std::{num::NonZero, ops::RangeInclusive};
/// # use ranch::*;
/// assert_eq!(
///     multirange::subranges::<i8, i8>()
///         .collect::<Vec<_>>(),
///     [RangeInclusive::new(-128, 127)],
/// );
/// assert_eq!(
///     multirange::subranges::<NonZero<i8>, i8>()
///         .collect::<Vec<_>>(),
///     [
///         RangeInclusive::new(-128, -1),
///         RangeInclusive::new(1, 127),
///     ],
/// );
/// assert_eq!(
///     multirange::subranges::<RangedNonZeroI8<50, 100>, i8>()
///         .collect::<Vec<_>>(),
///     [RangeInclusive::new(50, 100)],
/// );
/// assert_eq!(
///     multirange::subranges::<RangedNonZeroI8<-100, -50>, i8>()
///         .collect::<Vec<_>>(),
///     [RangeInclusive::new(-100, -50)],
/// );
/// ```
pub const fn subranges<R, T>() -> impl Iterator<Item = RangeInclusive<T>>
where
    R: MultiRange<T>,
    T: Rangeable,
{
    struct InclusiveRange<T: Rangeable> {
        min: T,
        max: T,
    }

    enum Ranges<T: Rangeable> {
        Once(InclusiveRange<T>),
        List(&'static [RangeInclusive<T>]),
    }

    impl<T> Iterator for Ranges<T>
    where
        T: Rangeable,
    {
        type Item = RangeInclusive<T>;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let range = match self {
                    Self::Once(range) => {
                        let range = RangeInclusive::new(range.min, range.max);

                        *self = Self::List(&[]);
                        range
                    }
                    Self::List(rest) => {
                        let (range, rest) = rest.split_first()?;

                        *self = Self::List(rest);
                        range.clone()
                    }
                };

                if !range.is_empty() {
                    return Some(range);
                }
            }
        }
    }

    if const { R::SUBRANGES.is_empty() } {
        Ranges::Once(InclusiveRange {
            min: R::MIN,
            max: R::MAX,
        })
    } else {
        Ranges::List(R::SUBRANGES)
    }
}

/// Rangeable type.
pub trait Rangeable: Copy + Ord + 'static {}

impl<T> Rangeable for T where T: Copy + Ord + 'static {}

const fn max<T>(a: T, b: T) -> T
where
    T: Cmp,
{
    if cmp::gt(a, b) { a } else { b }
}

const fn min<T>(a: T, b: T) -> T
where
    T: Cmp,
{
    if cmp::lt(a, b) { a } else { b }
}
