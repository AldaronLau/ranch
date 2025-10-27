use super::*;

macro_rules! impl_ranged_conversion {
    ($type:ident, $p:ty $(,)?) => {
        impl<const MIN: $p, const MAX: $p> TryFrom<$p> for $type::<MIN, MAX> {
            type Error = Error;

            fn try_from(primitive: $p) -> Result<Self> {
                <$type::<MIN, MAX>>::new(primitive)
            }
        }

        impl<const MIN: $p, const MAX: $p> From<$type::<MIN, MAX>> for $p {
            fn from(ranged: $type::<MIN, MAX>) -> Self {
                ranged.get()
            }
        }
    };
}

impl_ranged_conversion!(RangedI8, i8);
impl_ranged_conversion!(RangedU8, u8);
impl_ranged_conversion!(RangedU16, u16);
impl_ranged_conversion!(RangedU32, u32);
impl_ranged_conversion!(RangedU64, u64);
impl_ranged_conversion!(RangedU128, u128);
