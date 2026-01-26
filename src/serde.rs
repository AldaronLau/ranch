use serde_core::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use super::*;

macro_rules! impl_serde {
    ($type:ident, $p:ty) => {
        impl<'de, const MIN: $p, const MAX: $p> Deserialize<'de>
            for $type<MIN, MAX>
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                <$p as Deserialize>::deserialize(deserializer)?
                    .try_into()
                    .map_err(Error::custom)
            }
        }

        impl<const MIN: $p, const MAX: $p> Serialize for $type<MIN, MAX> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                <$p as Serialize>::serialize(&self.get(), serializer)
            }
        }
    };
}

macro_rules! impl_serde_nonzero {
    ($type:ident, $p:ty, $r:ident) => {
        impl<'de, const MIN: $p, const MAX: $p> Deserialize<'de>
            for $type<MIN, MAX>
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let r: $r<MIN, MAX> =
                    <$p as Deserialize>::deserialize(deserializer)?
                        .try_into()
                        .map_err(Error::custom)?;

                r.to_ranged_nonzero()
                    .ok_or("must not be zero")
                    .map_err(Error::custom)
            }
        }

        impl<const MIN: $p, const MAX: $p> Serialize for $type<MIN, MAX> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                <$p as Serialize>::serialize(&self.get(), serializer)
            }
        }
    };
}

impl_serde!(RangedU8, u8);
impl_serde!(RangedU16, u16);
impl_serde!(RangedU32, u32);
impl_serde!(RangedU64, u64);
impl_serde!(RangedU128, u128);

impl_serde!(RangedI8, i8);
impl_serde!(RangedI16, i16);
impl_serde!(RangedI32, i32);
impl_serde!(RangedI64, i64);
impl_serde!(RangedI128, i128);

impl_serde_nonzero!(RangedNonZeroU8, u8, RangedU8);
impl_serde_nonzero!(RangedNonZeroU16, u16, RangedU16);
impl_serde_nonzero!(RangedNonZeroU32, u32, RangedU32);
impl_serde_nonzero!(RangedNonZeroU64, u64, RangedU64);
impl_serde_nonzero!(RangedNonZeroU128, u128, RangedU128);

impl_serde_nonzero!(RangedNonZeroI8, i8, RangedI8);
impl_serde_nonzero!(RangedNonZeroI16, i16, RangedI16);
impl_serde_nonzero!(RangedNonZeroI32, i32, RangedI32);
impl_serde_nonzero!(RangedNonZeroI64, i64, RangedI64);
impl_serde_nonzero!(RangedNonZeroI128, i128, RangedI128);
