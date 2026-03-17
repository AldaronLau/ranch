#![allow(unsafe_code)]

use core::num::NonZero;

use as_repr::AsRepr;

use crate::{
    multirange::{MultiRange, Ranged},
    num::rangeable_primitive::RangeablePrimitive,
    cast::{as_primitive::Primitive, to::IsNonZero},
};

// unsafe: `repr(transparent)` on `Ranged<T, R>` is `repr(T)`
unsafe impl<T, R, A> AsRepr<A> for Ranged<T, R>
where
    T: RangeablePrimitive + AsRepr<A>,
    R: MultiRange<T::ZeroablePrimitive>,
    A: RangeablePrimitive,
{
}

// unsafe: `repr(primitive)` implies `repr(Option<NonZero<primitive>>)`
unsafe impl<N, P, R> AsRepr<Option<N>> for Ranged<P, R>
where
    N: RangeablePrimitive<ZeroablePrimitive = P> + IsNonZero,
    P: Primitive,
    R: MultiRange<P>
{
}

macro_rules! as_repr {
    ($p:ident $(,)?) => {
        // unsafe: `repr(primitive)` implies `repr(Option<NonZero<primitive>>)`
        unsafe impl<R> AsRepr<Option<Ranged<NonZero<$p>, R>>> for Ranged<$p, R>
        where
            R: MultiRange<$p>
        {
        }

        // unsafe: `repr(NonZero<primitive>)` implies `repr(primitive)`
        unsafe impl<R> AsRepr<Ranged<$p, R>> for Ranged<NonZero<$p>, R>
        where
            R: MultiRange<$p>
        {
        }
    };
}

as_repr!(u8);
as_repr!(u16);
as_repr!(u32);
as_repr!(u64);
as_repr!(u128);
as_repr!(i8);
as_repr!(i16);
as_repr!(i32);
as_repr!(i64);
as_repr!(i128);
