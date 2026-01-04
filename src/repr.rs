#![allow(unsafe_code)]

use core::num::NonZero;

use as_repr::AsRepr;

use super::*;

macro_rules! as_repr {
    ($ranged:ident, $p:ident $(,)?) => {
        // unsafe: `repr(transparent)` on ranged types is `repr(primitive)`
        unsafe impl<const MIN: $p, const MAX: $p> AsRepr<$p>
            for $ranged<MIN, MAX>
        {
        }

        // unsafe: `repr(primitive)` implies `repr(Option<NonZero<primitive>>)`
        unsafe impl<const MIN: $p, const MAX: $p> AsRepr<Option<NonZero<$p>>>
            for $ranged<MIN, MAX>
        {
        }
    };
}

macro_rules! as_repr_nonzero {
    ($ranged:ident, $p:ident $(,)?) => {
        // unsafe: `repr(transparent)` on non-zero ranged types is
        // `repr(NonZero<primitive>)`
        unsafe impl<const MIN: $p, const MAX: $p> AsRepr<NonZero<$p>>
            for $ranged<MIN, MAX>
        {
        }

        // unsafe: `repr(NonZero<primitive>)` implies `repr(primitive)`
        unsafe impl<const MIN: $p, const MAX: $p> AsRepr<$p>
            for $ranged<MIN, MAX>
        {
        }
    };
}

as_repr!(RangedU8, u8);
as_repr!(RangedU16, u16);
as_repr!(RangedU32, u32);
as_repr!(RangedU64, u64);
as_repr!(RangedU128, u128);
as_repr!(RangedI8, i8);
as_repr!(RangedI16, i16);
as_repr!(RangedI32, i32);
as_repr!(RangedI64, i64);
as_repr!(RangedI128, i128);

as_repr_nonzero!(RangedNonZeroU8, u8);
as_repr_nonzero!(RangedNonZeroU16, u16);
as_repr_nonzero!(RangedNonZeroU32, u32);
as_repr_nonzero!(RangedNonZeroU64, u64);
as_repr_nonzero!(RangedNonZeroU128, u128);
as_repr_nonzero!(RangedNonZeroI8, i8);
as_repr_nonzero!(RangedNonZeroI16, i16);
as_repr_nonzero!(RangedNonZeroI32, i32);
as_repr_nonzero!(RangedNonZeroI64, i64);
as_repr_nonzero!(RangedNonZeroI128, i128);
