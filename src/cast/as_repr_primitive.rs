// FIXME: Move to as_repr crate as `AsReprIntPrimitive`

use core::num::NonZero;

use as_repr::AsRepr;

use super::as_primitive::Primitive;
use crate::{
    cmp::Cmp,
    multirange::{MultiRange, Ranged},
};

/// Has a representation as a primitive
pub trait AsReprPrimitive: AsRepr<Self::Repr> + Copy {
    type Repr: Primitive;

    const SIGNED: bool;
}

impl<T> AsReprPrimitive for T
where
    T: Primitive,
{
    type Repr = T;

    const SIGNED: bool = T::SIGNED;
}

// Implement for `Ranged`
impl<T, R> AsReprPrimitive for Ranged<T, R>
where
    T: AsReprPrimitive,
    R: MultiRange<T::Repr>,
{
    type Repr = T::Repr;

    const SIGNED: bool = T::SIGNED;
}

macro_rules! nonzero {
    ($p:ty) => {
        impl AsReprPrimitive for NonZero<$p> {
            type Repr = $p;

            const SIGNED: bool = <$p as AsReprPrimitive>::SIGNED;
        }
    };
}

nonzero!(i8);
nonzero!(i16);
nonzero!(i32);
nonzero!(i64);
nonzero!(i128);
nonzero!(u8);
nonzero!(u16);
nonzero!(u32);
nonzero!(u64);
nonzero!(u128);
