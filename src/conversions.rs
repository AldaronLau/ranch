//! Constant `#[repr(T)]` conversions

#![allow(unsafe_code)]

use core::mem::{self, ManuallyDrop};

/// Trait that allows usage with [`as_repr()`]
///
/// # Safety
///
///  - For this trait to be safe to implement, it must be `#[repr(T)]`
pub unsafe trait AsRepr<T> {}

// unsafe: all types `#[repr]` themselves
unsafe impl<T> AsRepr<T> for T {}

/// Convert a type implementing [`AsRepr`] to `T`
pub const fn as_repr<T>(value: impl AsRepr<T>) -> T {
    let value = ManuallyDrop::new(value);

    // safety: not calling drop allows us to move the data with a "copy"
    unsafe { mem::transmute_copy(&value) }
}
