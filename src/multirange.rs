//! Utilities for multiple-ranged types

use core::{num::NonZero, ops::RangeInclusive};

pub use crate::num::ranged::Ranged;
use crate::{
    cast::as_repr_primitive::{self, AsReprPrimitive},
    num::rangeable_primitive::RangeablePrimitive,
    range::Range,
    *,
};

/// A type with one or more valid ranges of values
pub trait MultiRange<T: Rangeable = Self>: Rangeable {
    /// Each valid subrange
    ///
    /// The ranges must be returned from lowest to highest value, and never
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
    const SUBRANGES: &'static [RangeInclusive<T>] = &[];
}

macro_rules! nonzero_impl_multirange {
    ($p:ty) => {
        impl MultiRange for NonZero<$p> {
            const SUBRANGES: &'static [RangeInclusive<NonZero<$p>>] = &[
                RangeInclusive::new(
                    Self::MIN,
                    const {
                        NonZero::new(as_repr_primitive::min::<$p>(
                            -1,
                            NonZero::<$p>::MAX.get(),
                        ))
                        .unwrap()
                    },
                ),
                RangeInclusive::new(
                    const {
                        NonZero::new(as_repr_primitive::max::<$p>(
                            1,
                            NonZero::<$p>::MIN.get(),
                        ))
                        .unwrap()
                    },
                    Self::MAX,
                ),
            ];
        }

        impl MultiRange<$p> for NonZero<$p> {
            const SUBRANGES: &'static [RangeInclusive<$p>] = &[
                RangeInclusive::new(
                    <$p>::MIN,
                    as_repr_primitive::min::<$p>(-1, <$p>::MAX),
                ),
                RangeInclusive::new(
                    as_repr_primitive::max::<$p>(1, <$p>::MIN),
                    <$p>::MAX,
                ),
            ];
        }
    };
}

macro_rules! nonzero_multirange_impl {
    ($p:ty) => {
        impl<R> MultiRange<$p> for Ranged<NonZero<$p>, R>
        where
            R: Range<$p>,
        {
            const SUBRANGES: &'static [RangeInclusive<$p>] = &[
                RangeInclusive::new(
                    min::<R, $p>(),
                    as_repr_primitive::min(-1, max::<R, $p>()),
                ),
                RangeInclusive::new(
                    as_repr_primitive::max(1, min::<R, $p>()),
                    max::<R, $p>(),
                ),
            ];
        }

        impl<R> MultiRange for Ranged<NonZero<$p>, R>
        where
            R: Range<$p>,
        {
            const SUBRANGES: &'static [RangeInclusive<Self>] = &[
                RangeInclusive::new(
                    Self::from_unchecked(NonZero::new(min::<R, $p>()).unwrap()),
                    const {
                        Self::from_unchecked(
                            NonZero::new(as_repr_primitive::min(
                                -1,
                                max::<R, $p>(),
                            ))
                            .unwrap(),
                        )
                    },
                ),
                RangeInclusive::new(
                    const {
                        Self::from_unchecked(
                            NonZero::new(as_repr_primitive::max(
                                1,
                                min::<R, $p>(),
                            ))
                            .unwrap(),
                        )
                    },
                    Self::from_unchecked(NonZero::new(max::<R, $p>()).unwrap()),
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
            const SUBRANGES: &'static [RangeInclusive<NonZero<$p>>] = &[
                RangeInclusive::new(
                    const { NonZero::new(min::<R, $p>()).unwrap() },
                    const {
                        NonZero::new(as_repr_primitive::min(-1, max::<R, $p>()))
                            .unwrap()
                    },
                ),
                RangeInclusive::new(
                    const {
                        NonZero::new(as_repr_primitive::max(1, min::<R, $p>()))
                            .unwrap()
                    },
                    const { NonZero::new(max::<R, $p>()).unwrap() },
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

    Ranges::List(R::SUBRANGES)
}

/// Rangeable type.
pub trait Rangeable: Copy + Ord + 'static {}

impl<T> Rangeable for T where T: Copy + Ord + 'static {}

pub const fn min<M, T>() -> T
where
    M: MultiRange<T>,
    T: AsReprPrimitive + Rangeable,
{
    let mut i = 0;

    while is_empty(&M::SUBRANGES[i]) {
        i += 1;
    }

    *M::SUBRANGES[i].start()
}

pub const fn max<M, T>() -> T
where
    M: MultiRange<T>,
    T: AsReprPrimitive + Rangeable,
{
    let mut i = M::SUBRANGES.len();

    while is_empty(&M::SUBRANGES[i]) {
        i -= 1;
    }

    *M::SUBRANGES[i].end()
}

const fn is_empty<T>(range: &RangeInclusive<T>) -> bool
where
    T: AsReprPrimitive + Rangeable,
{
    as_repr_primitive::le(*range.end(), *range.start())
}
