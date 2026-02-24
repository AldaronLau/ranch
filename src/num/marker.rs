use crate::range::Range;

macro_rules! marker_range {
    ($name:ident, $p:ty) => {
        #[doc = concat!("Marker type range of [`", stringify!($p), "`]")]
        #[derive(
            Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default,
        )]
        pub struct $name<const MIN: $p, const MAX: $p>;

        impl<const MIN: $p, const MAX: $p> Range<$p> for $name<MIN, MAX> {
            const MAX: $p = MAX;
            const MIN: $p = MIN;
        }
    };
}

marker_range!(RangeU8, u8);
marker_range!(RangeU16, u16);
marker_range!(RangeU32, u32);
marker_range!(RangeU64, u64);
marker_range!(RangeU128, u128);
marker_range!(RangeI8, i8);
marker_range!(RangeI16, i16);
marker_range!(RangeI32, i32);
marker_range!(RangeI64, i64);
marker_range!(RangeI128, i128);
