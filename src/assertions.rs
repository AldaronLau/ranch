use super::*;

macro_rules! impl_assertions {
    (
        $type:ident,
        $p:ty $(,)?
    ) => {
        impl<const MIN: $p, const MAX: $p> $type<MIN, MAX> {
            pub(crate) const fn assert_range() {
                if MIN > MAX {
                    panic!("MIN cannot be more than MAX")
                }
            }
        }
    };
}

macro_rules! impl_assertions_nonzero {
    (
        $type:ident,
        $p:ty $(,)?
    ) => {
        impl<const MIN: $p, const MAX: $p> $type<MIN, MAX> {
            pub(crate) const fn assert_range() {
                if MIN > MAX {
                    panic!("MIN cannot be more than MAX")
                }

                if MIN == 0 {
                    panic!("A non-zero integer's minimum cannot be zero");
                }

                if MAX == 0 {
                    panic!("A non-zero integer's maximum cannot be zero");
                }
            }
        }
    };
}

impl_assertions!(RangedU8, u8);
impl_assertions!(RangedU16, u16);
impl_assertions!(RangedU32, u32);
impl_assertions!(RangedU64, u64);
impl_assertions!(RangedU128, u128);
impl_assertions!(RangedI8, i8);
impl_assertions!(RangedI16, i16);
impl_assertions!(RangedI32, i32);
impl_assertions!(RangedI64, i64);
impl_assertions!(RangedI128, i128);

impl_assertions_nonzero!(RangedNonZeroU8, u8);
impl_assertions_nonzero!(RangedNonZeroU16, u16);
impl_assertions_nonzero!(RangedNonZeroU32, u32);
impl_assertions_nonzero!(RangedNonZeroU64, u64);
impl_assertions_nonzero!(RangedNonZeroU128, u128);
impl_assertions_nonzero!(RangedNonZeroI8, i8);
impl_assertions_nonzero!(RangedNonZeroI16, i16);
impl_assertions_nonzero!(RangedNonZeroI32, i32);
impl_assertions_nonzero!(RangedNonZeroI64, i64);
impl_assertions_nonzero!(RangedNonZeroI128, i128);
