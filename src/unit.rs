//! Aliases for unit (single value ranged) integers
//!
//! Like the [`prim@unit`] type, these types have exactly one possible value.
//! However, these types are guaranteed to be represented as the underlying
//! integer.

use crate::*;

/// [`i8`] that's guaranteed to be a specific value
pub type UnitI8<const VAL: i8> = RangedI8<VAL, VAL>;
/// [`i16`] that's guaranteed to be a specific value
pub type UnitI16<const VAL: i16> = RangedI16<VAL, VAL>;
/// [`i32`] that's guaranteed to be a specific value
pub type UnitI32<const VAL: i32> = RangedI32<VAL, VAL>;
/// [`i64`] that's guaranteed to be a specific value
pub type UnitI64<const VAL: i64> = RangedI64<VAL, VAL>;
/// [`i128`] that's guaranteed to be a specific value
pub type UnitI128<const VAL: i128> = RangedI128<VAL, VAL>;

/// [`u8`] that's guaranteed to be a specific value
pub type UnitU8<const VAL: i8> = RangedU8<VAL, VAL>;
/// [`u16`] that's guaranteed to be a specific value
pub type UnitU16<const VAL: i16> = RangedU16<VAL, VAL>;
/// [`u32`] that's guaranteed to be a specific value
pub type UnitU32<const VAL: i32> = RangedU32<VAL, VAL>;
/// [`u64`] that's guaranteed to be a specific value
pub type UnitU64<const VAL: i64> = RangedU64<VAL, VAL>;
/// [`u128`] that's guaranteed to be a specific value
pub type UnitU128<const VAL: i128> = RangedU128<VAL, VAL>;

/// [`i8`] that's guaranteed to be a specific non-zero value
pub type UnitNonZeroI8<const VAL: i8> = RangedNonZeroI8<VAL, VAL>;
/// [`i16`] that's guaranteed to be a specific non-zero value
pub type UnitNonZeroI16<const VAL: i16> = RangedNonZeroI16<VAL, VAL>;
/// [`i32`] that's guaranteed to be a specific non-zero value
pub type UnitNonZeroI32<const VAL: i32> = RangedNonZeroI32<VAL, VAL>;
/// [`i64`] that's guaranteed to be a specific non-zero value
pub type UnitNonZeroI64<const VAL: i64> = RangedNonZeroI64<VAL, VAL>;
/// [`i128`] that's guaranteed to be a specific non-zero value
pub type UnitNonZeroI128<const VAL: i128> = RangedNonZeroI128<VAL, VAL>;

/// [`u8`] that's guaranteed to be a specific non-zero value
pub type UnitNonZeroU8<const VAL: i8> = RangedNonZeroU8<VAL, VAL>;
/// [`u16`] that's guaranteed to be a specific non-zero value
pub type UnitNonZeroU16<const VAL: i16> = RangedNonZeroU16<VAL, VAL>;
/// [`u32`] that's guaranteed to be a specific non-zero value
pub type UnitNonZeroU32<const VAL: i32> = RangedNonZeroU32<VAL, VAL>;
/// [`u64`] that's guaranteed to be a specific non-zero value
pub type UnitNonZeroU64<const VAL: i64> = RangedNonZeroU64<VAL, VAL>;
/// [`u128`] that's guaranteed to be a specific non-zero value
pub type UnitNonZeroU128<const VAL: i128> = RangedNonZeroU128<VAL, VAL>;

macro_rules! impl_default {
    ($type:ident, $p:ty $(,)?) => {
        impl<const VAL: $p> Default for $type<VAL> {
            fn default() -> Self {
                Self::new::<VAL>()
            }
        }

        impl<const VAL: $p> Extend<$type<VAL>> for $type<VAL> {
            fn extend<T: IntoIterator<Item = Self>>(&mut self, iter: T) {
                iter.into_iter().for_each(drop);
            }
        }

        impl<const VAL: $p> FromIterator<$type<VAL>> for $type<VAL> {
            fn from_iter<I: IntoIterator<Item = $type<VAL>>>(iter: I) -> Self {
                iter.into_iter().for_each(drop);
                Self::default()
            }
        }

        impl<const VAL: $p> From<()> for $type<VAL> {
            fn from((): ()) -> Self {
                Self::default()
            }
        }

        impl<const VAL: $p> From<$type<VAL>> for () {
            fn from(_: $type<VAL>) {}
        }
    };
}

impl_default!(UnitI8, i8);
impl_default!(UnitI16, i16);
impl_default!(UnitI32, i32);
impl_default!(UnitI64, i64);
impl_default!(UnitI128, i128);

impl_default!(UnitU8, u8);
impl_default!(UnitU16, u16);
impl_default!(UnitU32, u32);
impl_default!(UnitU64, u64);
impl_default!(UnitU128, u128);

impl_default!(UnitNonZeroI8, i8);
impl_default!(UnitNonZeroI16, i16);
impl_default!(UnitNonZeroI32, i32);
impl_default!(UnitNonZeroI64, i64);
impl_default!(UnitNonZeroI128, i128);

impl_default!(UnitNonZeroU8, u8);
impl_default!(UnitNonZeroU16, u16);
impl_default!(UnitNonZeroU32, u32);
impl_default!(UnitNonZeroU64, u64);
impl_default!(UnitNonZeroU128, u128);
