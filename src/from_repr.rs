#![allow(unsafe_code)]

use core::mem::{self, ManuallyDrop};

use as_repr::AsRepr;

use crate::{cast::as_primitive::Primitive, *};

/// # Safety
///
///  - Must be able to transmute from representation type without UB
pub unsafe trait FromRepr: AsRepr<Self::Repr> + Copy + Clone {
    type Repr: Primitive;
}

unsafe impl<const MIN: u8, const MAX: u8> FromRepr for RangedU8<MIN, MAX> {
    type Repr = u8;
}
unsafe impl<const MIN: u16, const MAX: u16> FromRepr for RangedU16<MIN, MAX> {
    type Repr = u16;
}
unsafe impl<const MIN: u32, const MAX: u32> FromRepr for RangedU32<MIN, MAX> {
    type Repr = u32;
}
unsafe impl<const MIN: u64, const MAX: u64> FromRepr for RangedU64<MIN, MAX> {
    type Repr = u64;
}
unsafe impl<const MIN: u128, const MAX: u128> FromRepr
    for RangedU128<MIN, MAX>
{
    type Repr = u128;
}

unsafe impl<const MIN: i8, const MAX: i8> FromRepr for RangedI8<MIN, MAX> {
    type Repr = i8;
}
unsafe impl<const MIN: i16, const MAX: i16> FromRepr for RangedI16<MIN, MAX> {
    type Repr = i16;
}
unsafe impl<const MIN: i32, const MAX: i32> FromRepr for RangedI32<MIN, MAX> {
    type Repr = i32;
}
unsafe impl<const MIN: i64, const MAX: i64> FromRepr for RangedI64<MIN, MAX> {
    type Repr = i64;
}
unsafe impl<const MIN: i128, const MAX: i128> FromRepr
    for RangedI128<MIN, MAX>
{
    type Repr = i128;
}

pub(crate) const fn from_repr<T>(value: T::Repr) -> T
where
    T: FromRepr,
{
    let value = ManuallyDrop::new(value);

    // safety: not calling drop allows us to move the data with a "copy"
    unsafe { mem::transmute_copy(&value) }
}
