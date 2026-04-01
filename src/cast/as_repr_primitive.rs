// FIXME: Move to as_repr crate as `AsReprIntPrimitive`

//! The trait [`AsReprPrimitive`] (`AsReprIntPrimitive`) makes certain const
//! functions available, that aren't provided with the basic [`AsRepr`].

#![allow(unsafe_code)]

use core::{cmp::Ordering, num::NonZero};

use as_repr::AsRepr;

use super::as_primitive::Primitive;
use crate::{
    multirange::{MultiRange, Ranged},
    num::rangeable_primitive::RangeablePrimitive,
};

#[repr(u32)]
pub(crate) enum Size {
    Byte = 1,
    Half = 2,
    Word = 4,
    Long = 8,
    Quad = 16,
}

/// Has a representation as a primitive
pub trait AsReprPrimitive: AsRepr<Self::Repr> + Copy {
    type Repr: Primitive;

    const SIGNED: bool;
}

impl<T> AsReprPrimitive for T
where
    T: Primitive,
{
    type Repr = T;

    const SIGNED: bool = T::SIGNED;
}

// Implement for `Ranged`
impl<T, R> AsReprPrimitive for Ranged<T, R>
where
    T: RangeablePrimitive,
    R: MultiRange<T::Repr>,
{
    type Repr = T::Repr;

    const SIGNED: bool = T::SIGNED;
}

macro_rules! nonzero {
    ($p:ty) => {
        impl AsReprPrimitive for NonZero<$p> {
            type Repr = $p;

            const SIGNED: bool = <$p as AsReprPrimitive>::SIGNED;
        }
    };
}

nonzero!(i8);
nonzero!(i16);
nonzero!(i32);
nonzero!(i64);
nonzero!(i128);
nonzero!(u8);
nonzero!(u16);
nonzero!(u32);
nonzero!(u64);
nonzero!(u128);

pub const fn shl<const N: u32, T>(value: T) -> T::Repr
where
    T: AsReprPrimitive,
{
    // Valid for ints only
    let mut value = as_repr::as_repr(value);
    let size = const { size::<T>() };
    let ptr: *mut T = &mut value;

    unsafe {
        match size {
            Size::Byte => *ptr.cast::<u8>() <<= N,
            Size::Half => *ptr.cast::<u16>() <<= N,
            Size::Word => *ptr.cast::<u32>() <<= N,
            Size::Long => *ptr.cast::<u64>() <<= N,
            Size::Quad => *ptr.cast::<u128>() <<= N,
        }
    }

    value
}

pub const fn shr<const N: u32, T>(value: T) -> T::Repr
where
    T: AsReprPrimitive,
{
    // Valid for ints only
    let mut value = as_repr::as_repr(value);
    let size = const { size::<T>() };
    let ptr: *mut T = &mut value;

    if T::SIGNED {
        unsafe {
            match size {
                Size::Byte => *ptr.cast::<i8>() >>= N,
                Size::Half => *ptr.cast::<i16>() >>= N,
                Size::Word => *ptr.cast::<i32>() >>= N,
                Size::Long => *ptr.cast::<i64>() >>= N,
                Size::Quad => *ptr.cast::<i128>() >>= N,
            }
        }
    } else {
        unsafe {
            match size {
                Size::Byte => *ptr.cast::<u8>() >>= N,
                Size::Half => *ptr.cast::<u16>() >>= N,
                Size::Word => *ptr.cast::<u32>() >>= N,
                Size::Long => *ptr.cast::<u64>() >>= N,
                Size::Quad => *ptr.cast::<u128>() >>= N,
            }
        }
    }

    value
}

pub const fn ordering<T>(a: T, b: T) -> Ordering
where
    T: AsReprPrimitive,
{
    // Valid for ints only
    const ORDERING: [[Ordering; 2]; 2] = [
        [Ordering::Equal, Ordering::Greater],
        [Ordering::Less, Ordering::Equal],
    ];

    const unsafe fn ordering_unsigned<T>(
        a: *const T,
        b: *const T,
    ) -> (bool, bool) {
        match const { size::<T>() } {
            Size::Byte => {
                let (a, b) = unsafe { (*a.cast::<u8>(), *b.cast::<u8>()) };
                (a < b, a > b)
            }
            Size::Half => {
                let (a, b) = unsafe { (*a.cast::<u16>(), *b.cast::<u16>()) };
                (a < b, a > b)
            }
            Size::Word => {
                let (a, b) = unsafe { (*a.cast::<u32>(), *b.cast::<u32>()) };
                (a < b, a > b)
            }
            Size::Long => {
                let (a, b) = unsafe { (*a.cast::<u64>(), *b.cast::<u64>()) };
                (a < b, a > b)
            }
            Size::Quad => {
                let (a, b) = unsafe { (*a.cast::<u128>(), *b.cast::<u128>()) };
                (a < b, a > b)
            }
        }
    }

    const unsafe fn ordering_signed<T>(
        a: *const T,
        b: *const T,
    ) -> (bool, bool) {
        match const { size::<T>() } {
            Size::Byte => {
                let (a, b) = unsafe { (*a.cast::<i8>(), *b.cast::<i8>()) };
                (a < b, a > b)
            }
            Size::Half => {
                let (a, b) = unsafe { (*a.cast::<i16>(), *b.cast::<i16>()) };
                (a < b, a > b)
            }
            Size::Word => {
                let (a, b) = unsafe { (*a.cast::<i32>(), *b.cast::<i32>()) };
                (a < b, a > b)
            }
            Size::Long => {
                let (a, b) = unsafe { (*a.cast::<i64>(), *b.cast::<i64>()) };
                (a < b, a > b)
            }
            Size::Quad => {
                let (a, b) = unsafe { (*a.cast::<i128>(), *b.cast::<i128>()) };
                (a < b, a > b)
            }
        }
    }

    // Valid for ints only
    let a: *const T = &a;
    let b: *const T = &b;
    let (less, greater) = if T::SIGNED {
        unsafe { ordering_signed(a, b) }
    } else {
        unsafe { ordering_unsigned(a, b) }
    };

    ORDERING[less as usize][greater as usize]
}

pub const fn is_nonzero<T>(a: T) -> bool
where
    T: AsReprPrimitive,
{
    // Valid for ints / floats
    ordering(a, <T::Repr>::ZERO).is_ne()
}

pub const fn is_zero<T>(a: T) -> bool
where
    T: AsReprPrimitive,
{
    // Valid for ints / floats
    ordering(a, <T::Repr>::ZERO).is_eq()
}

pub const fn is_one<T>(a: T) -> bool
where
    T: AsReprPrimitive,
{
    // Valid for ints only
    let one = const { wrapping_add_one(<T::Repr>::ZERO) };

    ordering(a, one).is_eq()
}

pub const fn is_minus_one<T>(a: T) -> bool
where
    T: AsReprPrimitive,
{
    // Valid for ints only
    let minus_one = const { wrapping_sub_one(<T::Repr>::ZERO) };

    T::SIGNED && ordering(a, minus_one).is_eq()
}

pub const fn is_negative<T>(a: T) -> bool
where
    T: AsReprPrimitive,
{
    if const { !T::SIGNED } {
        return false;
    }

    let ptr: *const T = &a;

    unsafe {
        match const { size::<T>() } {
            Size::Byte => (*ptr.cast::<i8>()).is_negative(),
            Size::Half => (*ptr.cast::<i16>()).is_negative(),
            Size::Word => (*ptr.cast::<i32>()).is_negative(),
            Size::Long => (*ptr.cast::<i64>()).is_negative(),
            Size::Quad => (*ptr.cast::<i128>()).is_negative(),
        }
    }
}

pub const fn is_positive<T>(a: T) -> bool
where
    T: AsReprPrimitive,
{
    if const { !T::SIGNED } {
        return is_nonzero(a);
    }

    let ptr: *const T = &a;

    unsafe {
        match const { size::<T>() } {
            Size::Byte => (*ptr.cast::<i8>()).is_positive(),
            Size::Half => (*ptr.cast::<i16>()).is_positive(),
            Size::Word => (*ptr.cast::<i32>()).is_positive(),
            Size::Long => (*ptr.cast::<i64>()).is_positive(),
            Size::Quad => (*ptr.cast::<i128>()).is_positive(),
        }
    }
}

pub const fn gt<T>(a: T, b: T) -> bool
where
    T: AsReprPrimitive,
{
    ordering(a, b).is_gt()
}

pub const fn ge<T>(a: T, b: T) -> bool
where
    T: AsReprPrimitive,
{
    ordering(a, b).is_ge()
}

pub const fn lt<T>(a: T, b: T) -> bool
where
    T: AsReprPrimitive,
{
    ordering(a, b).is_lt()
}

pub const fn le<T>(a: T, b: T) -> bool
where
    T: AsReprPrimitive,
{
    ordering(a, b).is_le()
}

pub const fn min<T>(a: T, b: T) -> T
where
    T: AsReprPrimitive,
{
    if ordering(a, b).is_le() { a } else { b }
}

pub const fn max<T>(a: T, b: T) -> T
where
    T: AsReprPrimitive,
{
    if ordering(a, b).is_le() { b } else { a }
}

pub(crate) const fn wrapping_add_one<T>(mut t: T) -> T
where
    T: Primitive,
{
    // Valid for ints only
    let size = const { size::<T>() };
    let ptr: *mut T = &mut t;

    unsafe {
        match size {
            Size::Byte => (*ptr.cast::<u8>()).wrapping_add(1),
            Size::Half => (*ptr.cast::<u16>()).wrapping_add(1),
            Size::Word => (*ptr.cast::<u32>()).wrapping_add(1),
            Size::Long => (*ptr.cast::<u64>()).wrapping_add(1),
            Size::Quad => (*ptr.cast::<u128>()).wrapping_add(1),
        }
    }
}

pub(crate) const fn wrapping_sub_one<T>(mut t: T) -> T
where
    T: Primitive,
{
    // Valid for ints only
    let size = const { size::<T>() };
    let ptr: *mut T = &mut t;

    unsafe {
        match size {
            Size::Byte => (*ptr.cast::<u8>()).wrapping_sub(1),
            Size::Half => (*ptr.cast::<u16>()).wrapping_sub(1),
            Size::Word => (*ptr.cast::<u32>()).wrapping_sub(1),
            Size::Long => (*ptr.cast::<u64>()).wrapping_sub(1),
            Size::Quad => (*ptr.cast::<u128>()).wrapping_sub(1),
        }
    }
}

pub(crate) const fn size<T>() -> Size {
    match size_of::<T>() {
        1 => Size::Byte,
        2 => Size::Half,
        4 => Size::Word,
        8 => Size::Long,
        16 => Size::Quad,
        _ => panic!("invalid size"),
    }
}
