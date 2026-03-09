use core::{hash::Hash, num::NonZero};

use crate::{cmp::Cmp, multirange::Rangeable};

pub trait RangeablePrimitive: Hash + Sized + Rangeable {
    const BITS: u32;
    const SIGNED: bool;

    type ZeroablePrimitive: Sized + Rangeable + Cmp;
}

macro_rules! rangeable_primitive {
    ($p:ty, $signed:literal) => { rangeable_primitive!($p, $p, $signed); };
    ($p:ty, $zp:ty, $signed:literal) => {
        impl RangeablePrimitive for $p {
            const BITS: u32 = Self::BITS;
            const SIGNED: bool = $signed;

            type ZeroablePrimitive = $zp;
        }
    };
}

rangeable_primitive!(u8, false);
rangeable_primitive!(u16, false);
rangeable_primitive!(u32, false);
rangeable_primitive!(u64, false);
rangeable_primitive!(u128, false);
rangeable_primitive!(i8, true);
rangeable_primitive!(i16, true);
rangeable_primitive!(i32, true);
rangeable_primitive!(i64, true);
rangeable_primitive!(i128, true);

rangeable_primitive!(NonZero<u8>, u8, false);
rangeable_primitive!(NonZero<u16>, u16, false);
rangeable_primitive!(NonZero<u32>, u32, false);
rangeable_primitive!(NonZero<u64>, u64, false);
rangeable_primitive!(NonZero<u128>, u128, false);
rangeable_primitive!(NonZero<i8>, i8, true);
rangeable_primitive!(NonZero<i16>, i16, true);
rangeable_primitive!(NonZero<i32>, i32, true);
rangeable_primitive!(NonZero<i64>, i64, true);
rangeable_primitive!(NonZero<i128>, i128, true);
