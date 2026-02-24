#![allow(unsafe_code)]

use core::{any::Any, mem};

pub unsafe trait DowncastShl: Copy + Clone + Any {}

unsafe impl DowncastShl for u8 {}
unsafe impl DowncastShl for u16 {}
unsafe impl DowncastShl for u32 {}
unsafe impl DowncastShl for u64 {}
unsafe impl DowncastShl for u128 {}

unsafe impl DowncastShl for i8 {}
unsafe impl DowncastShl for i16 {}
unsafe impl DowncastShl for i32 {}
unsafe impl DowncastShl for i64 {}
unsafe impl DowncastShl for i128 {}

pub(crate) const fn downcast_shl<const N: u32, T>(value: T) -> T
where
    T: DowncastShl,
{
    match size_of::<T>() {
        1 => {
            let value: u8 = unsafe { mem::transmute_copy(&value) };
            let value = value << N;

            unsafe { mem::transmute_copy(&value) }
        }
        2 => {
            let value: u16 = unsafe { mem::transmute_copy(&value) };
            let value = value << N;

            unsafe { mem::transmute_copy(&value) }
        }
        4 => {
            let value: u32 = unsafe { mem::transmute_copy(&value) };
            let value = value << N;

            unsafe { mem::transmute_copy(&value) }
        }
        8 => {
            let value: u64 = unsafe { mem::transmute_copy(&value) };
            let value = value << N;

            unsafe { mem::transmute_copy(&value) }
        }
        16 => {
            let value: u128 = unsafe { mem::transmute_copy(&value) };
            let value = value << N;

            unsafe { mem::transmute_copy(&value) }
        }
        _ => unreachable!(),
    }
}
