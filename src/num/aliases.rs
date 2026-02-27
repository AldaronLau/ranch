use core::num::NonZero;

use crate::{multirange::*, range::*};

macro_rules! ranged_nonzero_alias {
    ($name:ident, $range:ident, $p:ty) => {
        #[doc = concat!("[`", stringify!($p), "`]")]
        /// not to equal zero with a specified minimum and maximum value
        pub type $name<const MIN: $p, const MAX: $p> =
            Ranged<NonZero<$p>, $range<MIN, MAX>>;
    };
}

macro_rules! ranged_alias {
    ($name:ident, $range:ident, $p:ty) => {
        #[doc = concat!("[`", stringify!($p), "`]")]
        /// with a specified minimum and maximum value
        pub type $name<const MIN: $p, const MAX: $p> =
            Ranged<$p, $range<MIN, MAX>>;
    };
}

ranged_alias!(RangedU8, RangeU8, u8);
ranged_alias!(RangedU16, RangeU16, u16);
ranged_alias!(RangedU32, RangeU32, u32);
ranged_alias!(RangedU64, RangeU64, u64);
ranged_alias!(RangedU128, RangeU128, u128);

ranged_alias!(RangedI8, RangeI8, i8);
ranged_alias!(RangedI16, RangeI16, i16);
ranged_alias!(RangedI32, RangeI32, i32);
ranged_alias!(RangedI64, RangeI64, i64);
ranged_alias!(RangedI128, RangeI128, i128);

ranged_nonzero_alias!(RangedNonZeroU8, RangeU8, u8);
ranged_nonzero_alias!(RangedNonZeroU16, RangeU16, u16);
ranged_nonzero_alias!(RangedNonZeroU32, RangeU32, u32);
ranged_nonzero_alias!(RangedNonZeroU64, RangeU64, u64);
ranged_nonzero_alias!(RangedNonZeroU128, RangeU128, u128);

ranged_nonzero_alias!(RangedNonZeroI8, RangeI8, i8);
ranged_nonzero_alias!(RangedNonZeroI16, RangeI16, i16);
ranged_nonzero_alias!(RangedNonZeroI32, RangeI32, i32);
ranged_nonzero_alias!(RangedNonZeroI64, RangeI64, i64);
ranged_nonzero_alias!(RangedNonZeroI128, RangeI128, i128);
