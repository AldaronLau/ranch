#![allow(unsafe_code)]

use core::{any::Any, mem};

/// # Safety
///
///  - Must be able to cast to one of the unsigned primitive integers and back
///    without introducing UB
pub(crate) unsafe trait DowncastShrUnsigned:
    Copy + Clone + Any
{
}

/// # Safety
///
///  - Must be able to cast to one of the signed primitive integers and back
///    without introducing UB
pub(crate) unsafe trait DowncastShrSigned:
    Copy + Clone + Any
{
}

unsafe impl DowncastShrUnsigned for u8 {}
unsafe impl DowncastShrUnsigned for u16 {}
unsafe impl DowncastShrUnsigned for u32 {}
unsafe impl DowncastShrUnsigned for u64 {}
unsafe impl DowncastShrUnsigned for u128 {}

unsafe impl DowncastShrSigned for i8 {}
unsafe impl DowncastShrSigned for i16 {}
unsafe impl DowncastShrSigned for i32 {}
unsafe impl DowncastShrSigned for i64 {}
unsafe impl DowncastShrSigned for i128 {}

pub(crate) const fn downcast_shr_unsigned<const N: u32, T>(value: T) -> T
where
    T: DowncastShrUnsigned,
{
    match size_of::<T>() {
        1 => {
            let value: u8 = unsafe { mem::transmute_copy(&value) };
            let value = value >> N;

            unsafe { mem::transmute_copy(&value) }
        }
        2 => {
            let value: u16 = unsafe { mem::transmute_copy(&value) };
            let value = value >> N;

            unsafe { mem::transmute_copy(&value) }
        }
        4 => {
            let value: u32 = unsafe { mem::transmute_copy(&value) };
            let value = value >> N;

            unsafe { mem::transmute_copy(&value) }
        }
        8 => {
            let value: u64 = unsafe { mem::transmute_copy(&value) };
            let value = value >> N;

            unsafe { mem::transmute_copy(&value) }
        }
        16 => {
            let value: u128 = unsafe { mem::transmute_copy(&value) };
            let value = value >> N;

            unsafe { mem::transmute_copy(&value) }
        }
        _ => unreachable!(),
    }
}

pub(crate) const fn downcast_shr_signed<const N: u32, T>(value: T) -> T
where
    T: DowncastShrSigned,
{
    match size_of::<T>() {
        1 => {
            let value: i8 = unsafe { mem::transmute_copy(&value) };
            let value = value >> N;

            unsafe { mem::transmute_copy(&value) }
        }
        2 => {
            let value: i16 = unsafe { mem::transmute_copy(&value) };
            let value = value >> N;

            unsafe { mem::transmute_copy(&value) }
        }
        4 => {
            let value: i32 = unsafe { mem::transmute_copy(&value) };
            let value = value >> N;

            unsafe { mem::transmute_copy(&value) }
        }
        8 => {
            let value: i64 = unsafe { mem::transmute_copy(&value) };
            let value = value >> N;

            unsafe { mem::transmute_copy(&value) }
        }
        16 => {
            let value: i128 = unsafe { mem::transmute_copy(&value) };
            let value = value >> N;

            unsafe { mem::transmute_copy(&value) }
        }
        _ => unreachable!(),
    }
}
