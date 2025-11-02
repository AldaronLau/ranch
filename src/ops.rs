use core::{
    num::NonZero,
    ops::{Add, Div, Mul, Sub},
};

use as_repr::AsRepr;

use super::*;

pub trait OpsAsRepr<T>: AsRepr<T> {}

impl OpsAsRepr<i8> for i8 {}
impl OpsAsRepr<i16> for i16 {}
impl OpsAsRepr<i32> for i32 {}
impl OpsAsRepr<i64> for i64 {}
impl OpsAsRepr<i128> for i128 {}
impl OpsAsRepr<u8> for u8 {}
impl OpsAsRepr<u16> for u16 {}
impl OpsAsRepr<u32> for u32 {}
impl OpsAsRepr<u64> for u64 {}
impl OpsAsRepr<u128> for u128 {}

impl<const MIN: i8, const MAX: i8> OpsAsRepr<i8> for RangedI8<MIN, MAX> {}
//impl<const MIN: i16, const MAX: i16> OpsAsRepr<i16> for RangedI16<MIN, MAX>
// {} impl<const MIN: i32, const MAX: i32> OpsAsRepr<i32> for RangedI32<MIN,
// MAX> {} impl<const MIN: i64, const MAX: i64> OpsAsRepr<i64> for
// RangedI64<MIN, MAX> {} impl<const MIN: i128, const MAX: i128> OpsAsRepr<i128>
// for RangedI128<MIN, MAX> {}
impl<const MIN: u8, const MAX: u8> OpsAsRepr<u8> for RangedU8<MIN, MAX> {}
impl<const MIN: u16, const MAX: u16> OpsAsRepr<u16> for RangedU16<MIN, MAX> {}
impl<const MIN: u32, const MAX: u32> OpsAsRepr<u32> for RangedU32<MIN, MAX> {}
impl<const MIN: u64, const MAX: u64> OpsAsRepr<u64> for RangedU64<MIN, MAX> {}
impl<const MIN: u128, const MAX: u128> OpsAsRepr<u128>
    for RangedU128<MIN, MAX>
{
}

macro_rules! impl_ops {
    ($type:ident, $p:ty $(,)?) => {
        impl<T, const MIN: $p, const MAX: $p> Add<T> for $type<MIN, MAX>
        where
            T: OpsAsRepr<$p>,
        {
            type Output = Self;

            fn add(self, other: T) -> Self {
                self.checked_add(other).unwrap()
            }
        }

        impl<T, const MIN: $p, const MAX: $p> Sub<T> for $type<MIN, MAX>
        where
            T: OpsAsRepr<$p>,
        {
            type Output = Self;

            fn sub(self, other: T) -> Self {
                self.checked_sub(other).unwrap()
            }
        }

        impl<T, const MIN: $p, const MAX: $p> Mul<T> for $type<MIN, MAX>
        where
            T: OpsAsRepr<$p>,
        {
            type Output = Self;

            fn mul(self, other: T) -> Self {
                self.checked_mul(other).unwrap()
            }
        }

        impl<T, const MIN: $p, const MAX: $p> Div<T> for $type<MIN, MAX>
        where
            T: OpsAsRepr<$p>,
        {
            type Output = Quotient<Self>;

            fn div(self, other: T) -> Quotient<Self> {
                self.checked_div(other).unwrap()
            }
        }

        impl<const MIN: $p, const MAX: $p> Add<NonZero<$p>>
            for $type<MIN, MAX>
        {
            type Output = Self;

            fn add(self, other: NonZero<$p>) -> Self {
                self.checked_add(other.get()).unwrap()
            }
        }

        impl<const MIN: $p, const MAX: $p> Sub<NonZero<$p>>
            for $type<MIN, MAX>
        {
            type Output = Self;

            fn sub(self, other: NonZero<$p>) -> Self {
                self.checked_sub(other.get()).unwrap()
            }
        }

        impl<const MIN: $p, const MAX: $p> Mul<NonZero<$p>>
            for $type<MIN, MAX>
        {
            type Output = Self;

            fn mul(self, other: NonZero<$p>) -> Self {
                self.checked_mul(other.get()).unwrap()
            }
        }

        impl<const MIN: $p, const MAX: $p> Div<NonZero<$p>>
            for $type<MIN, MAX>
        {
            type Output = Self;

            fn div(self, other: NonZero<$p>) -> Self {
                match self.checked_div(other.get()).unwrap() {
                    Quotient::Nan => unreachable!(),
                    Quotient::Number(number) => number,
                }
            }
        }
    };
}

impl_ops!(RangedI8, i8);

impl_ops!(RangedU8, u8);
impl_ops!(RangedU16, u16);
impl_ops!(RangedU32, u32);
impl_ops!(RangedU64, u64);
impl_ops!(RangedU128, u128);
