use core::{
    num::NonZero,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use as_repr::AsRepr;

use super::*;

macro_rules! assign_impl {
    ($t:ident, $p:ty, $nonzero:ident) => {
        impl<T, const MIN: $p, const MAX: $p> AddAssign<T> for $t<MIN, MAX>
        where
            T: AsRepr<$p>,
        {
            fn add_assign(&mut self, other: T) {
                *self = *self + other;
            }
        }

        impl<T, const MIN: $p, const MAX: $p> SubAssign<T> for $t<MIN, MAX>
        where
            T: AsRepr<$p>,
        {
            fn sub_assign(&mut self, other: T) {
                *self = *self - other;
            }
        }

        impl<T, const MIN: $p, const MAX: $p> MulAssign<T> for $t<MIN, MAX>
        where
            T: AsRepr<$p>,
        {
            fn mul_assign(&mut self, other: T) {
                *self = *self * other;
            }
        }

        impl<T, const MIN: $p, const MAX: $p> DivAssign<T> for $t<MIN, MAX>
        where
            T: AsRepr<NonZero<$p>>,
        {
            fn div_assign(&mut self, other: T) {
                *self = *self / other;
            }
        }

        /*
        impl<T, const MIN: $p, const MAX: $p> RemAssign<T> for $t::<MIN, MAX>
        where
            T: AsRepr<NonZero<$p>>,
        {
            fn rem_assign(&mut self, other: T) {
                *self = *self % other;
            }
        }*/
    };
}

assign_impl!(RangedI8, i8, RangedNonZeroI8);
assign_impl!(RangedI16, i16, RangedNonZeroI16);
assign_impl!(RangedI32, i32, RangedNonZeroI32);
assign_impl!(RangedI64, i64, RangedNonZeroI64);
assign_impl!(RangedI128, i128, RangedNonZeroI128);

assign_impl!(RangedU8, u8, RangedNonZeroU8);
assign_impl!(RangedU16, u16, RangedNonZeroU16);
assign_impl!(RangedU32, u32, RangedNonZeroU32);
assign_impl!(RangedU64, u64, RangedNonZeroU64);
assign_impl!(RangedU128, u128, RangedNonZeroU128);
