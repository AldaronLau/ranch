use core::num::NonZero;

use as_repr::{
    AsRepr,
    cmp::{self, Cmp},
    int::Integer,
    num::Number,
    ops::{self, SaturatingAdd, SaturatingSub},
};

use crate::multirange::Rangeable;

pub trait IntegerPrimitive:
    Rangeable
    + Cmp
    + Integer
    + Number<ToRepr = Self>
    + SaturatingAdd
    + SaturatingSub
{
}

impl<T> IntegerPrimitive for T where
    T: Rangeable
        + Cmp
        + Integer
        + Number<ToRepr = Self>
        + SaturatingAdd
        + SaturatingSub
{
}

pub trait RangeablePrimitive:
    Rangeable + AsRepr<Self::ZeroablePrimitive>
{
    type ZeroablePrimitive: IntegerPrimitive;
}

macro_rules! rangeable_primitive {
    ($p:ty) => { rangeable_primitive!($p, $p); };
    ($p:ty, $zp:ty) => {
        impl RangeablePrimitive for $p {
            type ZeroablePrimitive = $zp;
        }
    };
}

rangeable_primitive!(u8);
rangeable_primitive!(u16);
rangeable_primitive!(u32);
rangeable_primitive!(u64);
rangeable_primitive!(u128);
rangeable_primitive!(i8);
rangeable_primitive!(i16);
rangeable_primitive!(i32);
rangeable_primitive!(i64);
rangeable_primitive!(i128);

rangeable_primitive!(NonZero<u8>, u8);
rangeable_primitive!(NonZero<u16>, u16);
rangeable_primitive!(NonZero<u32>, u32);
rangeable_primitive!(NonZero<u64>, u64);
rangeable_primitive!(NonZero<u128>, u128);
rangeable_primitive!(NonZero<i8>, i8);
rangeable_primitive!(NonZero<i16>, i16);
rangeable_primitive!(NonZero<i32>, i32);
rangeable_primitive!(NonZero<i64>, i64);
rangeable_primitive!(NonZero<i128>, i128);

pub(crate) const fn zeroable<P>(p: P) -> P::ZeroablePrimitive
where
    P: RangeablePrimitive,
{
    as_repr::as_repr(p)
}

pub(crate) const fn is_one<P>(p: P) -> bool
where
    P: IntegerPrimitive,
{
    cmp::cmp(p, <P::ZeroablePrimitive as Number>::REPR_ONE).is_eq()
}

pub(crate) const fn is_minus_one<P>(p: P) -> bool
where
    P: IntegerPrimitive,
{
    let minus_one =
        ops::saturating_sub(p, <P::ZeroablePrimitive as Number>::REPR_ONE);

    if cmp::is_zero(minus_one) {
        return false;
    }

    cmp::cmp(p, minus_one).is_eq()
}
