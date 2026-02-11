#![allow(unsafe_code)]

use core::{mem, ptr};

use as_repr::AsRepr;

use crate::{range::MultiRange, *};

macro_rules! ranged_scale_to {
    ($nonzero:ident, $ranged:ident, $repr:ty, $scaled:ty) => {
        ranged_scale_to!($nonzero, $repr, $scaled);
        ranged_scale_to!($ranged, $repr, $scaled);
    };

    ($ranged:ident, $repr:ty, $scaled:ty) => {
        unsafe impl<const MIN: $repr, const MAX: $repr> RangedScaleTo<$scaled>
            for $ranged<MIN, MAX>
        {
            type Repr = $repr;
        }
    };
}

ranged_scale_to!(RangedNonZeroI8, RangedI8, i8, i8);
ranged_scale_to!(RangedNonZeroI8, RangedI8, i8, i16);
ranged_scale_to!(RangedNonZeroI8, RangedI8, i8, i32);
ranged_scale_to!(RangedNonZeroI8, RangedI8, i8, i64);
ranged_scale_to!(RangedNonZeroI8, RangedI8, i8, i128);

ranged_scale_to!(RangedNonZeroU8, RangedU8, u8, u8);
ranged_scale_to!(RangedNonZeroU8, RangedU8, u8, u16);
ranged_scale_to!(RangedNonZeroU8, RangedU8, u8, u32);
ranged_scale_to!(RangedNonZeroU8, RangedU8, u8, u64);
ranged_scale_to!(RangedNonZeroU8, RangedU8, u8, u128);

ranged_scale_to!(RangedNonZeroI16, RangedI16, i16, i16);
ranged_scale_to!(RangedNonZeroI16, RangedI16, i16, i32);
ranged_scale_to!(RangedNonZeroI16, RangedI16, i16, i64);
ranged_scale_to!(RangedNonZeroI16, RangedI16, i16, i128);

ranged_scale_to!(RangedNonZeroU16, RangedU16, u16, u16);
ranged_scale_to!(RangedNonZeroU16, RangedU16, u16, u32);
ranged_scale_to!(RangedNonZeroU16, RangedU16, u16, u64);
ranged_scale_to!(RangedNonZeroU16, RangedU16, u16, u128);

ranged_scale_to!(RangedNonZeroI32, RangedI32, i32, i32);
ranged_scale_to!(RangedNonZeroI32, RangedI32, i32, i64);
ranged_scale_to!(RangedNonZeroI32, RangedI32, i32, i128);

ranged_scale_to!(RangedNonZeroU32, RangedU32, u32, u32);
ranged_scale_to!(RangedNonZeroU32, RangedU32, u32, u64);
ranged_scale_to!(RangedNonZeroU32, RangedU32, u32, u128);

ranged_scale_to!(RangedNonZeroI64, RangedI64, i64, i64);
ranged_scale_to!(RangedNonZeroI64, RangedI64, i64, i128);

ranged_scale_to!(RangedNonZeroU64, RangedU64, u64, u64);
ranged_scale_to!(RangedNonZeroU64, RangedU64, u64, u128);

ranged_scale_to!(RangedNonZeroI128, RangedI128, i128, i128);
ranged_scale_to!(RangedNonZeroU128, RangedU128, u128, u128);

/// # Safety
///
///  - `Self` should be a ranged integer type
///  - `Repr` should be either a primitive integer or `R` for `AsRepr<R>`
pub unsafe trait RangedScaleTo<T>:
    AsRepr<Self::Repr> + MultiRange + Copy + Clone + Sized
{
    type Repr: Copy + Clone;
}

pub(crate) const fn ranged_scale_to<I, T>(input: I) -> T
where
    I: RangedScaleTo<T>,
    T: Copy + Clone,
{
    // Convert to little endian
    let primitive = if cfg!(target_endian = "big") {
        unsafe { endian_swizzle(as_repr::as_repr(input)) }
    } else {
        as_repr::as_repr(input)
    };
    let mut output = unsafe { mem::zeroed() };
    let out: *mut T = &mut output;

    // Copy little endian to output (may not fill the whole thing)
    unsafe { ptr::copy_nonoverlapping(&primitive, out.cast(), 1) };

    // Convert to native endian
    if cfg!(target_endian = "big") {
        unsafe { endian_swizzle(output) }
    } else {
        output
    }
}

const unsafe fn endian_swizzle<T>(input: T) -> T
where
    T: Copy + Clone,
{
    let mut output = unsafe { mem::zeroed() };
    let mut i = 0;

    loop {
        if i == size_of::<T>() {
            break;
        }

        let j = (size_of::<T>() - 1) - i;
        let input: *const T = &input;
        let output: *mut T = &mut output;
        let input: *const u8 = input.cast();
        let output: *mut u8 = output.cast();
        let input = unsafe { input.add(i) };
        let output = unsafe { output.add(j) };

        unsafe { ptr::copy_nonoverlapping(input, output, 1) };

        i += 1;
    }

    output
}
