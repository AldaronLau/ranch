//! Aliases for unit (single value ranged) integers
//!
//! Like the [`prim@unit`] type, these types have exactly one possible value.
//! However, these types are guaranteed to be represented as the underlying
//! integer.
//!
//! Unit types can be constructed with [`Default::default()`] or their `new()`
//! method if `const` is required.  Conversions between these unit types and
//! [`prim@unit`] are also provided.

use crate::*;

macro_rules! unit {
    ($type:ident, $nonzero:ident, $ranged:ident, $rnz:ident, $p:ty $(,)?) => {
        #[doc = concat!("[`", stringify!($p), "`]")]
        /// that's guaranteed to be a specific value
        pub type $type<const VAL: $p> = $ranged<VAL, VAL>;

        #[doc = concat!("[`", stringify!($p), "`]")]
        /// that's guaranteed to be a specific non-zero value
        pub type $nonzero<const VAL: $p> = $rnz<VAL, VAL>;

        unit!($type, $p);
        unit!($nonzero, $p);
    };

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

unit!(UnitI8, UnitNonZeroI8, RangedI8, RangedNonZeroI8, i8);
unit!(UnitI16, UnitNonZeroI16, RangedI16, RangedNonZeroI16, i16);
unit!(UnitI32, UnitNonZeroI32, RangedI32, RangedNonZeroI32, i32);
unit!(UnitI64, UnitNonZeroI64, RangedI64, RangedNonZeroI64, i64);
unit!(
    UnitI128,
    UnitNonZeroI128,
    RangedI128,
    RangedNonZeroI128,
    i128,
);

unit!(UnitU8, UnitNonZeroU8, RangedU8, RangedNonZeroU8, u8);
unit!(UnitU16, UnitNonZeroU16, RangedU16, RangedNonZeroU16, u16);
unit!(UnitU32, UnitNonZeroU32, RangedU32, RangedNonZeroU32, u32);
unit!(UnitU64, UnitNonZeroU64, RangedU64, RangedNonZeroU64, u64);
unit!(
    UnitU128,
    UnitNonZeroU128,
    RangedU128,
    RangedNonZeroU128,
    u128,
);
