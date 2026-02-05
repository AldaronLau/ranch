#![allow(unsafe_code)]

use bytemuck::{NoUninit, checked::CheckedBitPattern};

use crate::*;

macro_rules! impl_bytemuck {
    ($type:ident, $p:ty) => {
        unsafe impl<const MIN: $p, const MAX: $p> NoUninit for $type<MIN, MAX> {}

        unsafe impl<const MIN: $p, const MAX: $p> CheckedBitPattern
            for $type<MIN, MAX>
        {
            type Bits = $p;

            fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
                Self::try_from(*bits).is_ok()
            }
        }
    };
}

impl_bytemuck!(RangedU8, u8);
impl_bytemuck!(RangedU16, u16);
impl_bytemuck!(RangedU32, u32);
impl_bytemuck!(RangedU64, u64);
impl_bytemuck!(RangedU128, u128);

impl_bytemuck!(RangedI8, i8);
impl_bytemuck!(RangedI16, i16);
impl_bytemuck!(RangedI32, i32);
impl_bytemuck!(RangedI64, i64);
impl_bytemuck!(RangedI128, i128);

impl_bytemuck!(RangedNonZeroU8, u8);
impl_bytemuck!(RangedNonZeroU16, u16);
impl_bytemuck!(RangedNonZeroU32, u32);
impl_bytemuck!(RangedNonZeroU64, u64);
impl_bytemuck!(RangedNonZeroU128, u128);

impl_bytemuck!(RangedNonZeroI8, i8);
impl_bytemuck!(RangedNonZeroI16, i16);
impl_bytemuck!(RangedNonZeroI32, i32);
impl_bytemuck!(RangedNonZeroI64, i64);
impl_bytemuck!(RangedNonZeroI128, i128);
