#![allow(missing_debug_implementations)] // FIXME

use core::marker::PhantomData;

use as_repr::{inherent::AsReprInherent, int::Integer};

use crate::{
    multirange::{self, MultiRange},
    num::rangeable_primitive::RangeablePrimitive,
    range::Range,
};

/// A value restricted to be within one of multiple ranges
#[derive(Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Ranged<T, R>(pub(crate) T, PhantomData<fn() -> R>)
where
    T: RangeablePrimitive,
    R: MultiRange<T::ZeroablePrimitive>;

impl<T, R> Ranged<T, R>
where
    T: RangeablePrimitive,
    R: MultiRange<T::ZeroablePrimitive>,
{
    /// The size of this integer type in bits.
    pub const BITS: u32 = <T::ZeroablePrimitive as Integer>::BITS;
    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Self::from_unchecked(multirange::max::<R, T>());
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self::from_unchecked(multirange::min::<R, T>());

    // This isn't unsafe since UB isn't possible with misuse (yet, keeping
    // private for now for flexibility), but misuse may cause logic bugs.
    pub(crate) const fn from_unchecked(primitive: T) -> Self {
        Self(primitive, PhantomData)
    }
}

impl<T, R> AsReprInherent for Ranged<T, R>
where
    T: RangeablePrimitive,
    R: MultiRange<T::ZeroablePrimitive>,
{
    type InherentRepr = <T as RangeablePrimitive>::ZeroablePrimitive;
}
