use core::ops::Neg;

use crate::*;

macro_rules! impl_neg {
    ($type:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> Neg for $type<MIN, MAX> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                const {
                    if MIN != -MAX {
                        panic!("Cannot negate asymmetrical types");
                    }
                }

                Self(-self.0)
            }
        }
    };
}

impl_neg!(RangedI8, i8);
impl_neg!(RangedI16, i16);
impl_neg!(RangedI32, i32);
impl_neg!(RangedI64, i64);
impl_neg!(RangedI128, i128);

impl_neg!(RangedNonZeroI8, i8);
impl_neg!(RangedNonZeroI16, i16);
impl_neg!(RangedNonZeroI32, i32);
impl_neg!(RangedNonZeroI64, i64);
impl_neg!(RangedNonZeroI128, i128);
