use core::{num::NonZero, ops::RangeInclusive};

use arbitrary::{Arbitrary, Result, Unstructured};

use crate::{range::MultiRange, *};

macro_rules! impl_arbitrary {
    ($type:ident, $p:ty) => {
        impl<'a, const MIN: $p, const MAX: $p> Arbitrary<'a>
            for $type<MIN, MAX>
        {
            fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                u.int_in_range(range::range_inclusive::<Self, _>())
                    .map(Self)
            }
        }
    };
}

macro_rules! impl_arbitrary_nonzero {
    ($type:ident, $p:ty) => {
        impl<'a, const MIN: $p, const MAX: $p> Arbitrary<'a>
            for $type<MIN, MAX>
        {
            fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                let range = const {
                    if MIN == 0 || MAX == 0 {
                        panic!("MIN or MAX cannot be zero for nonzero ints");
                    }

                    RangeInclusive::new(
                        <Self as MultiRange<$p>>::MIN,
                        <Self as MultiRange<$p>>::MAX,
                    )
                };

                if range.contains(&0) {
                    let range = const {
                        RangeInclusive::new(
                            <Self as MultiRange<$p>>::MIN + 1,
                            <Self as MultiRange<$p>>::MAX,
                        )
                    };
                    let value = u.int_in_range(range)?;

                    Ok(Self(
                        NonZero::new(if value <= 0 {
                            value - 1
                        } else {
                            value
                        })
                        .unwrap(),
                    ))
                } else {
                    u.int_in_range(range)
                        .map(|i| Self(NonZero::new(i).unwrap()))
                }
            }
        }
    };
}

impl_arbitrary!(RangedU8, u8);
impl_arbitrary!(RangedU16, u16);
impl_arbitrary!(RangedU32, u32);
impl_arbitrary!(RangedU64, u64);
impl_arbitrary!(RangedU128, u128);

impl_arbitrary!(RangedI8, i8);
impl_arbitrary!(RangedI16, i16);
impl_arbitrary!(RangedI32, i32);
impl_arbitrary!(RangedI64, i64);
impl_arbitrary!(RangedI128, i128);

impl_arbitrary_nonzero!(RangedNonZeroU8, u8);
impl_arbitrary_nonzero!(RangedNonZeroU16, u16);
impl_arbitrary_nonzero!(RangedNonZeroU32, u32);
impl_arbitrary_nonzero!(RangedNonZeroU64, u64);
impl_arbitrary_nonzero!(RangedNonZeroU128, u128);

impl_arbitrary_nonzero!(RangedNonZeroI8, i8);
impl_arbitrary_nonzero!(RangedNonZeroI16, i16);
impl_arbitrary_nonzero!(RangedNonZeroI32, i32);
impl_arbitrary_nonzero!(RangedNonZeroI64, i64);
impl_arbitrary_nonzero!(RangedNonZeroI128, i128);
