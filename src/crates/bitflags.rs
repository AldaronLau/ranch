use bitflags::Bits;

use crate::{
    bitops::{BitwiseSigned, BitwiseUnsigned},
    *,
};

macro_rules! impl_bitflags {
    ($unsigned:ident, $signed:ident, $u:ty, $s:ty) => {
        impl<const MIN: $s, const MAX: $s> Bits for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
        {
            const ALL: Self = Self(-1);
            const EMPTY: Self = Self(0);
        }

        impl<const MIN: $u, const MAX: $u> Bits for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
        {
            const ALL: Self = Self(MAX);
            const EMPTY: Self = Self(MIN);
        }
    };
}

impl_bitflags!(RangedU8, RangedI8, u8, i8);
impl_bitflags!(RangedU16, RangedI16, u16, i16);
impl_bitflags!(RangedU32, RangedI32, u32, i32);
impl_bitflags!(RangedU64, RangedI64, u64, i64);
impl_bitflags!(RangedU128, RangedI128, u128, i128);
