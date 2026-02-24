#![allow(missing_debug_implementations)] // FIXME

use core::marker::PhantomData;

use crate::{num::rangeable_primitive::RangeablePrimitive, range::Range};

/// A value restricted to a range
#[derive(Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Ranged<T, R>(pub(crate) T, PhantomData<fn() -> R>)
where
    T: RangeablePrimitive,
    R: Range<T::ZeroablePrimitive>;

impl<T, R> Ranged<T, R>
where
    T: RangeablePrimitive,
    R: Range<T::ZeroablePrimitive>,
{
    /// The size of this integer type in bits.
    pub const BITS: u32 = T::BITS;

    // This isn't unsafe since UB isn't possible with misuse, but misuse may
    // cause logic bugs.
    pub(crate) const fn from_unchecked(primitive: T) -> Self {
        Self(primitive, PhantomData)
    }
}

impl<T, R> Ranged<T, R>
where
    T: RangeablePrimitive<ZeroablePrimitive = T>,
    R: Range<T>,
{
    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Self(R::MAX, PhantomData);
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self(R::MIN, PhantomData);
}
