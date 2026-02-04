use core::num::NonZero;

use zeroize::Zeroize;

use crate::*;

macro_rules! impl_zeroize {
    ($type:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> Zeroize for $type<MIN, MAX> {
            fn zeroize(&mut self) {
                self.0.zeroize();
                self.0 = self.0.clamp(MIN, MAX)
            }
        }
    };
}

macro_rules! impl_zeroize_nonzero {
    ($type:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> Zeroize for $type<MIN, MAX> {
            fn zeroize(&mut self) {
                self.0.zeroize();
                self.0 = self.0.clamp(
                    NonZero::new(MIN).unwrap(),
                    NonZero::new(MAX).unwrap(),
                );
            }
        }
    };
}

impl_zeroize!(RangedU8, u8);
impl_zeroize!(RangedU16, u16);
impl_zeroize!(RangedU32, u32);
impl_zeroize!(RangedU64, u64);
impl_zeroize!(RangedU128, u128);

impl_zeroize!(RangedI8, i8);
impl_zeroize!(RangedI16, i16);
impl_zeroize!(RangedI32, i32);
impl_zeroize!(RangedI64, i64);
impl_zeroize!(RangedI128, i128);

impl_zeroize_nonzero!(RangedNonZeroU8, u8);
impl_zeroize_nonzero!(RangedNonZeroU16, u16);
impl_zeroize_nonzero!(RangedNonZeroU32, u32);
impl_zeroize_nonzero!(RangedNonZeroU64, u64);
impl_zeroize_nonzero!(RangedNonZeroU128, u128);

impl_zeroize_nonzero!(RangedNonZeroI8, i8);
impl_zeroize_nonzero!(RangedNonZeroI16, i16);
impl_zeroize_nonzero!(RangedNonZeroI32, i32);
impl_zeroize_nonzero!(RangedNonZeroI64, i64);
impl_zeroize_nonzero!(RangedNonZeroI128, i128);
