#![allow(unsafe_code)]
#![allow(dead_code)] // FIXME

use core::{mem, ptr};

use as_repr::AsRepr;

use crate::{
    num::rangeable_primitive::RangeablePrimitive, range::MultiRange, *,
};

trait Primitive {}

impl<T> Primitive for T where T: RangeablePrimitive<ZeroablePrimitive = T> {}

/// Turn representation into a primitive
///
/// # Safety
///
///  - `T` must be a primitive type (`u*` or `i*`)
///  - `Repr` must also be a primitive type
pub unsafe trait AsPrimitive<T>:
    Copy + Clone + AsRepr<Self::Repr> + MultiRange
{
    type Repr: Copy + Clone;
}

macro_rules! as_primitive {
    ($type:ident, $p:ty) => {
        unsafe impl<const MIN: $p, const MAX: $p, T> AsPrimitive<T>
            for $type<MIN, MAX>
        where
            T: Primitive,
        {
            type Repr = $p;
        }
    };
}

as_primitive!(RangedU8, u8);
as_primitive!(RangedU16, u16);
as_primitive!(RangedU32, u32);
as_primitive!(RangedU64, u64);
as_primitive!(RangedU128, u128);
as_primitive!(RangedI8, i8);
as_primitive!(RangedI16, i16);
as_primitive!(RangedI32, i32);
as_primitive!(RangedI64, i64);
as_primitive!(RangedI128, i128);
as_primitive!(RangedNonZeroU8, u8);
as_primitive!(RangedNonZeroU16, u16);
as_primitive!(RangedNonZeroU32, u32);
as_primitive!(RangedNonZeroU64, u64);
as_primitive!(RangedNonZeroU128, u128);
as_primitive!(RangedNonZeroI8, i8);
as_primitive!(RangedNonZeroI16, i16);
as_primitive!(RangedNonZeroI32, i32);
as_primitive!(RangedNonZeroI64, i64);
as_primitive!(RangedNonZeroI128, i128);

pub(crate) const fn as_primitive_shrinking<T, V>(value: V) -> T
where
    V: AsPrimitive<T>,
    T: Copy + Clone,
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
    T: Copy + Clone,
{
    const {
        if size_of::<V::Repr>() > size_of::<T>() {
            panic!("ranged type cannot be shrunk to primitive")
        }
    }

    as_primitive(value)
}

/// This should act like `as`, keeping the value the same, removing any extra
/// bits
pub(crate) const fn as_primitive<T, V>(value: V) -> T
where
    V: AsPrimitive<T>,
    T: Copy + Clone,
{
    #[repr(u32)]
    enum Size {
        Byte = 1,
        Half = 2,
        Word = 4,
        Long = 8,
        Quad = 16,
    }

    let in_size = const {
        match size_of::<V::Repr>() {
            1 => Size::Byte,
            2 => Size::Half,
            4 => Size::Word,
            8 => Size::Long,
            16 => Size::Quad,
            _ => panic!("invalid size"),
        }
    };
    let out_size = const {
        match size_of::<T>() {
            1 => Size::Byte,
            2 => Size::Half,
            4 => Size::Word,
            8 => Size::Long,
            16 => Size::Quad,
            _ => panic!("invalid size"),
        }
    };

    let mut value = as_repr::as_repr(value);

    // Convert to little endian
    if cfg!(target_endian = "big") {
        value = unsafe { endian_swizzle(value) };
    }

    let mut primitive: T = unsafe { mem::zeroed() };

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
