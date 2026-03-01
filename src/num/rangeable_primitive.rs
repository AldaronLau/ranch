use core::{hash::Hash, num::NonZero};

use crate::multirange::Rangeable;

pub trait RangeablePrimitive: Hash + Sized + Rangeable {
    const BITS: u32;

    type ZeroablePrimitive: Sized + Rangeable;
}

macro_rules! rangeable_primitive {
    ($p:ty) => { rangeable_primitive!($p, $p); };
    ($p:ty, $zp:ty) => {
        impl RangeablePrimitive for $p {
            const BITS: u32 = Self::BITS;

            type ZeroablePrimitive = $zp;
        }
    };
}

rangeable_primitive!(u8);
rangeable_primitive!(u16);
rangeable_primitive!(u32);
rangeable_primitive!(u64);
rangeable_primitive!(u128);
rangeable_primitive!(i8);
rangeable_primitive!(i16);
rangeable_primitive!(i32);
rangeable_primitive!(i64);
rangeable_primitive!(i128);

rangeable_primitive!(NonZero<u8>, u8);
rangeable_primitive!(NonZero<u16>, u16);
rangeable_primitive!(NonZero<u32>, u32);
rangeable_primitive!(NonZero<u64>, u64);
rangeable_primitive!(NonZero<u128>, u128);
rangeable_primitive!(NonZero<i8>, i8);
rangeable_primitive!(NonZero<i16>, i16);
rangeable_primitive!(NonZero<i32>, i32);
rangeable_primitive!(NonZero<i64>, i64);
rangeable_primitive!(NonZero<i128>, i128);
