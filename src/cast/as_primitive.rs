#![allow(unsafe_code)]

use core::{mem, num::NonZero, ptr};

use as_repr::AsRepr;

use super::as_repr_primitive::{self, Size};
use crate::{
    multirange::{MultiRange, Ranged},
    num::rangeable_primitive::RangeablePrimitive,
};

pub trait Primitive: RangeablePrimitive<ZeroablePrimitive = Self> {
    const ZERO: Self;
}

impl<T> Primitive for T
where
    T: RangeablePrimitive<ZeroablePrimitive = T>,
{
    const ZERO: Self = unsafe { mem::zeroed() };
}

/// Turn representation into a primitive
///
/// # Safety
///
///  - `T` must be a primitive type (`u*` or `i*`)
///  - `Repr` must also be a primitive type
pub unsafe trait AsPrimitive<T>:
    Copy + Clone + AsRepr<Self::Repr> + MultiRange
{
    const SIGNED: bool;

    type Repr: Primitive;
}

macro_rules! as_primitive {
    ($p:ty, $signed:literal) => {
        unsafe impl<T, R> AsPrimitive<T> for Ranged<$p, R>
        where
            T: Primitive,
            R: MultiRange<$p>,
        {
            type Repr = $p;

            const SIGNED: bool = $signed;
        }

        unsafe impl<T, R> AsPrimitive<T> for Ranged<NonZero<$p>, R>
        where
            T: Primitive,
            R: MultiRange<$p>,
        {
            type Repr = $p;

            const SIGNED: bool = $signed;
        }
    };
}

as_primitive!(u8, false);
as_primitive!(u16, false);
as_primitive!(u32, false);
as_primitive!(u64, false);
as_primitive!(u128, false);
as_primitive!(i8, true);
as_primitive!(i16, true);
as_primitive!(i32, true);
as_primitive!(i64, true);
as_primitive!(i128, true);

pub(crate) const fn as_primitive_shrinking<T, V>(value: V) -> T
where
    V: AsPrimitive<T>,
    T: Primitive,
{
    const {
        if size_of::<T>() > size_of::<V::Repr>() {
            panic!("ranged type cannot be expanded to primitive")
        }
    }

    as_primitive(value)
}

pub(crate) const fn as_primitive_expanding<T, V>(value: V) -> T
where
    V: AsPrimitive<T>,
    T: Primitive,
{
    const {
        if size_of::<V::Repr>() > size_of::<T>() {
            panic!("ranged type cannot be shrunk to primitive")
        }
    }

    as_primitive(value)
}

/// This should act like `as`, keeping the value the same, removing any extra
/// bits.
///
/// If the sign would change by casting, panics.
pub(crate) const fn as_primitive<T, V>(value: V) -> T
where
    V: AsPrimitive<T>,
    T: Primitive,
{
    let in_size = const { as_repr_primitive::size::<V::Repr>() };
    let out_size = const { as_repr_primitive::size::<T>() };

    let mut value = as_repr::as_repr(value);
    let negative = unsafe { is_negative(value) };

    // Convert to little endian
    if cfg!(target_endian = "big") {
        value = unsafe { endian_swizzle(value) };
    }

    let mut primitive: T = unsafe { mem::zeroed() };

    if negative {
        primitive = unsafe { not(primitive) };
    }

    if out_size as u32 > in_size as u32 {
        let primitive: *mut T = &mut primitive;

        // Copy the input into the output, leaving rest as zeros
        unsafe { ptr::copy_nonoverlapping(&value, primitive.cast(), 1) };
    } else {
        let value: *const V::Repr = &value;

        // Copy part (or all) of the input into the output
        unsafe { ptr::copy_nonoverlapping(value.cast(), &mut primitive, 1) };
    }

    // Convert to native endian
    if cfg!(target_endian = "big") {
        primitive = unsafe { endian_swizzle(primitive) };
    }

    // Check if sign changed
    if unsafe { is_negative(primitive) } ^ negative {
        panic!("cannot change sign during cast")
    }

    primitive
}

/// Toggle the endianness of T
///
/// # Safety
///
///  - `T` must be a primitive value
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

/// Flip all the bits.
///
/// # Safety
///
///  - `T` must be a primitive value
const unsafe fn not<T>(input: T) -> T
where
    T: Copy + Clone,
{
    const {
        if !matches!(size_of::<T>(), 1 | 2 | 4 | 8 | 16) {
            panic!("invalid size");
        }
    }

    let mut output: T = input;

    match size_of::<T>() {
        1 => {
            let output: *mut T = &mut output;
            let output: *mut u8 = output.cast();

            unsafe { (*output) = !(*output) };
        }
        2 => {
            let output: *mut T = &mut output;
            let output: *mut u16 = output.cast();

            unsafe { (*output) = !(*output) };
        }
        4 => {
            let output: *mut T = &mut output;
            let output: *mut u32 = output.cast();

            unsafe { (*output) = !(*output) };
        }
        8 => {
            let output: *mut T = &mut output;
            let output: *mut u64 = output.cast();

            unsafe { (*output) = !(*output) };
        }
        16 => {
            let output: *mut T = &mut output;
            let output: *mut u128 = output.cast();

            unsafe { (*output) = !(*output) };
        }
        _ => unreachable!(),
    }

    output
}

/// Return if a value is negative.
///
/// # Safety
///
///  - `T` must be a primitive value (since `RangeablePrimitive` and `Primitive`
///    are safe traits that could theoretically break this invariant, but
///    shouldn't, this function must be marked unsafe)
const unsafe fn is_negative<T>(input: T) -> bool
where
    T: Primitive,
{
    const {
        if !matches!(size_of::<T>(), 1 | 2 | 4 | 8 | 16) {
            panic!("invalid size");
        }
    }

    if const { !T::SIGNED } {
        return false;
    }

    match size_of::<T>() {
        1 => {
            let input: *const T = &input;
            let input: *const i8 = input.cast();

            unsafe { (*input).is_negative() }
        }
        2 => {
            let input: *const T = &input;
            let input: *const i16 = input.cast();

            unsafe { (*input).is_negative() }
        }
        4 => {
            let input: *const T = &input;
            let input: *const i32 = input.cast();

            unsafe { (*input).is_negative() }
        }
        8 => {
            let input: *const T = &input;
            let input: *const i64 = input.cast();

            unsafe { (*input).is_negative() }
        }
        16 => {
            let input: *const T = &input;
            let input: *const i128 = input.cast();

            unsafe { (*input).is_negative() }
        }
        _ => unreachable!(),
    }
}
