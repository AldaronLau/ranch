//! Compile-time type operations on ranges
//!
//! This is incomplete, and currently only supports addition and subtraction.
//!
//! ```rust
//! # use ranch::{types::*, range::*};
//! assert_eq!(<RangeI32::<-100, 100> as OpsI32>::Add::<50>::MIN, -50);
//! assert_eq!(<RangeI32::<-100, 100> as OpsI32>::Add::<50>::MAX, 150);
//!
//! assert_eq!(<RangeI32::<-100, 100> as OpsI32>::Sub::<50>::MIN, -150);
//! assert_eq!(<RangeI32::<-100, 100> as OpsI32>::Sub::<50>::MAX, 50);
//! ```

mod sealed {
    use super::*;

    pub trait Sealed {}

    impl<const MIN: u8, const MAX: u8> Sealed for RangeU8<MIN, MAX> {}
    impl<const MIN: u16, const MAX: u16> Sealed for RangeU16<MIN, MAX> {}
    impl<const MIN: u32, const MAX: u32> Sealed for RangeU32<MIN, MAX> {}
    impl<const MIN: u64, const MAX: u64> Sealed for RangeU64<MIN, MAX> {}
    impl<const MIN: u128, const MAX: u128> Sealed for RangeU128<MIN, MAX> {}
    impl<const MIN: i8, const MAX: i8> Sealed for RangeI8<MIN, MAX> {}
    impl<const MIN: i16, const MAX: i16> Sealed for RangeI16<MIN, MAX> {}
    impl<const MIN: i32, const MAX: i32> Sealed for RangeI32<MIN, MAX> {}
    impl<const MIN: i64, const MAX: i64> Sealed for RangeI64<MIN, MAX> {}
    impl<const MIN: i128, const MAX: i128> Sealed for RangeI128<MIN, MAX> {}
}

use crate::range::*;

macro_rules! types {
    ($p:ty, $range:ident, $ops:ident, $add:ident, $sub:ident $(,)?) => {
        /// Marker type for range addition
        #[repr(transparent)]
        #[derive(
            Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default,
        )]
        pub struct $add<const N: $p, T>(T);

        impl<const N: $p, T> Range<$p> for $add<N, T>
        where
            T: Range<$p>,
        {
            const MAX: $p = const { T::MAX + N };
            const MIN: $p = const { T::MIN + N };
        }

        /// Marker type for range subtraction
        #[repr(transparent)]
        #[derive(
            Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default,
        )]
        pub struct $sub<const N: $p, T>(T);

        impl<const N: $p, T> Range<$p> for $sub<N, T>
        where
            T: Range<$p>,
        {
            const MAX: $p = const { T::MAX - N };
            const MIN: $p = const { T::MIN - N };
        }

        /// Compile-time type operations on range
        pub trait $ops: sealed::Sealed {
            /// Range adding a constant number to this range
            type Add<const N: $p>;
            /// Range subtracting a constant number from this range
            type Sub<const N: $p>;
        }

        impl<const MIN: $p, const MAX: $p> $ops for $range<MIN, MAX> {
            type Add<const N: $p> = $add<N, Self>;
            type Sub<const N: $p> = $sub<N, Self>;
        }
    };
}

types!(u8, RangeU8, OpsU8, AddU8, SubU8);
types!(u16, RangeU16, OpsU16, AddU16, SubU16);
types!(u32, RangeU32, OpsU32, AddU32, SubU32);
types!(u64, RangeU64, OpsU64, AddU64, SubU64);
types!(u128, RangeU128, OpsU128, AddU128, SubU128);

types!(i8, RangeI8, OpsI8, AddI8, SubI8);
types!(i16, RangeI16, OpsI16, AddI16, SubI16);
types!(i32, RangeI32, OpsI32, AddI32, SubI32);
types!(i64, RangeI64, OpsI64, AddI64, SubI64);
types!(i128, RangeI128, OpsI128, AddI128, SubI128);
