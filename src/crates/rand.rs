use core::{num::NonZero, ops::RangeInclusive};

use rand::{
    Rng,
    distr::{Distribution, StandardUniform, Uniform},
};

use crate::{range::MultiRange, *};

macro_rules! impl_rand {
    ($r:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> Distribution<$r<MIN, MAX>>
            for StandardUniform
        {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $r<MIN, MAX> {
                $r::<MIN, MAX>::from_unchecked(
                    Uniform::new_inclusive(MIN, MAX).unwrap().sample(rng),
                )
            }
        }
    };
}

macro_rules! impl_rand_nonzero_unsigned {
    ($r:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> Distribution<$r<MIN, MAX>>
            for StandardUniform
        {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $r<MIN, MAX> {
                const {
                    if MIN == 0 || MAX == 0 {
                        panic!("MIN or MAX cannot be zero for nonzero ints");
                    }
                }

                $r::<MIN, MAX>::from_unchecked(
                    NonZero::new(
                        Uniform::new_inclusive(MIN, MAX).unwrap().sample(rng),
                    )
                    .unwrap(),
                )
            }
        }
    };
}

macro_rules! impl_rand_nonzero_signed {
    ($r:ident, $p:ty) => {
        impl<const MIN: $p, const MAX: $p> Distribution<$r<MIN, MAX>>
            for StandardUniform
        {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $r<MIN, MAX> {
                let range = const {
                    if MIN == 0 || MAX == 0 {
                        panic!("MIN or MAX cannot be zero for nonzero ints");
                    }

                    RangeInclusive::new(
                        <$r<MIN, MAX> as MultiRange<$p>>::MIN,
                        <$r<MIN, MAX> as MultiRange<$p>>::MAX,
                    )
                };

                $r::<MIN, MAX>::from_unchecked(
                    NonZero::new(if range.contains(&0) {
                        let value = Uniform::new(MIN, MAX).unwrap().sample(rng);

                        if value >= 0 { value + 1 } else { value }
                    } else {
                        Uniform::new_inclusive(MIN, MAX).unwrap().sample(rng)
                    })
                    .unwrap(),
                )
            }
        }
    };
}

impl_rand!(RangedU8, u8);
impl_rand!(RangedU16, u16);
impl_rand!(RangedU32, u32);
impl_rand!(RangedU64, u64);
impl_rand!(RangedU128, u128);

impl_rand!(RangedI8, i8);
impl_rand!(RangedI16, i16);
impl_rand!(RangedI32, i32);
impl_rand!(RangedI64, i64);
impl_rand!(RangedI128, i128);

impl_rand_nonzero_unsigned!(RangedNonZeroU8, u8);
impl_rand_nonzero_unsigned!(RangedNonZeroU16, u16);
impl_rand_nonzero_unsigned!(RangedNonZeroU32, u32);
impl_rand_nonzero_unsigned!(RangedNonZeroU64, u64);
impl_rand_nonzero_unsigned!(RangedNonZeroU128, u128);

impl_rand_nonzero_signed!(RangedNonZeroI8, i8);
impl_rand_nonzero_signed!(RangedNonZeroI16, i16);
impl_rand_nonzero_signed!(RangedNonZeroI32, i32);
impl_rand_nonzero_signed!(RangedNonZeroI64, i64);
impl_rand_nonzero_signed!(RangedNonZeroI128, i128);
