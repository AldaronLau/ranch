#![allow(unsafe_code)]

use core::num::NonZero;

use as_repr::AsRepr;

use crate::{
    num::rangeable_primitive::RangeablePrimitive,
    range::{Range, Ranged},
    *,
};

// unsafe: `repr(transparent)` on `Ranged` is `repr(primitive)`
unsafe impl<T, R> AsRepr<T> for Ranged<T, R>
where
    T: RangeablePrimitive,
    R: Range<T::ZeroablePrimitive>,
{
}

macro_rules! as_repr {
    ($nonzero:ident, $ranged:ident, $p:ident $(,)?) => {
        // unsafe: `repr(primitive)` implies `repr(Option<NonZero<primitive>>)`
        unsafe impl<const MIN: $p, const MAX: $p> AsRepr<Option<NonZero<$p>>>
            for $ranged<MIN, MAX>
        {
        }

        // unsafe: `repr(NonZero<primitive>)` implies `repr(primitive)`
        unsafe impl<const MIN: $p, const MAX: $p> AsRepr<$p>
            for $nonzero<MIN, MAX>
        {
        }
    };
}

as_repr!(RangedNonZeroU8, RangedU8, u8);
as_repr!(RangedNonZeroU16, RangedU16, u16);
as_repr!(RangedNonZeroU32, RangedU32, u32);
as_repr!(RangedNonZeroU64, RangedU64, u64);
as_repr!(RangedNonZeroU128, RangedU128, u128);
as_repr!(RangedNonZeroI8, RangedI8, i8);
as_repr!(RangedNonZeroI16, RangedI16, i16);
as_repr!(RangedNonZeroI32, RangedI32, i32);
as_repr!(RangedNonZeroI64, RangedI64, i64);
as_repr!(RangedNonZeroI128, RangedI128, i128);
