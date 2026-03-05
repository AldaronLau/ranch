#![allow(unsafe_code)]

use core::cmp::Ordering;

/// Trait to compare two types
///
/// # Safety
///
///  - `T` must be a primitive `i*` or `u*`
pub unsafe trait Cmp: Copy + Clone {
    const SIGNED: bool;
    const ZERO: Self;
    const ONE: Self;
    const MINUS_ONE: Option<Self>;
}

macro_rules! cmp {
    ($p:ty, $signed:literal, $minus_one:expr) => {
        unsafe impl Cmp for $p {
            const MINUS_ONE: Option<$p> = $minus_one;
            const ONE: $p = 1;
            const SIGNED: bool = $signed;
            const ZERO: $p = 0;
        }
    };
}

cmp!(i8, true, Some(-1));
cmp!(i16, true, Some(-1));
cmp!(i32, true, Some(-1));
cmp!(i64, true, Some(-1));
cmp!(i128, true, Some(-1));
cmp!(u8, true, None);
cmp!(u16, true, None);
cmp!(u32, true, None);
cmp!(u64, true, None);
cmp!(u128, true, None);

pub(crate) const fn ordering<C>(a: C, b: C) -> Ordering
where
    C: Cmp,
{
    const {
        if !matches!(size_of::<C>(), 1 | 2 | 4 | 8 | 16) {
            panic!("invalid size");
        }
    }

    if C::SIGNED {
        match size_of::<C>() {
            1 => {
                let a: *const C = &a;
                let a: *const i8 = a.cast();
                let b: *const C = &b;
                let b: *const i8 = b.cast();

                match unsafe { ((*a) < (*b), (*a) > (*b)) } {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (_, _) => Ordering::Equal,
                }
            }
            2 => {
                let a: *const C = &a;
                let a: *const i16 = a.cast();
                let b: *const C = &b;
                let b: *const i16 = b.cast();

                match unsafe { ((*a) < (*b), (*a) > (*b)) } {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (_, _) => Ordering::Equal,
                }
            }
            4 => {
                let a: *const C = &a;
                let a: *const i32 = a.cast();
                let b: *const C = &b;
                let b: *const i32 = b.cast();

                match unsafe { ((*a) < (*b), (*a) > (*b)) } {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (_, _) => Ordering::Equal,
                }
            }
            8 => {
                let a: *const C = &a;
                let a: *const i64 = a.cast();
                let b: *const C = &b;
                let b: *const i64 = b.cast();

                match unsafe { ((*a) < (*b), (*a) > (*b)) } {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (_, _) => Ordering::Equal,
                }
            }
            16 => {
                let a: *const C = &a;
                let a: *const i128 = a.cast();
                let b: *const C = &b;
                let b: *const i128 = b.cast();

                match unsafe { ((*a) < (*b), (*a) > (*b)) } {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (_, _) => Ordering::Equal,
                }
            }
            _ => unreachable!(),
        }
    } else {
        match size_of::<C>() {
            1 => {
                let a: *const C = &a;
                let a: *const u8 = a.cast();
                let b: *const C = &b;
                let b: *const u8 = b.cast();

                match unsafe { ((*a) < (*b), (*a) > (*b)) } {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (_, _) => Ordering::Equal,
                }
            }
            2 => {
                let a: *const C = &a;
                let a: *const u16 = a.cast();
                let b: *const C = &b;
                let b: *const u16 = b.cast();

                match unsafe { ((*a) < (*b), (*a) > (*b)) } {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (_, _) => Ordering::Equal,
                }
            }
            4 => {
                let a: *const C = &a;
                let a: *const u32 = a.cast();
                let b: *const C = &b;
                let b: *const u32 = b.cast();

                match unsafe { ((*a) < (*b), (*a) > (*b)) } {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (_, _) => Ordering::Equal,
                }
            }
            8 => {
                let a: *const C = &a;
                let a: *const u64 = a.cast();
                let b: *const C = &b;
                let b: *const u64 = b.cast();

                match unsafe { ((*a) < (*b), (*a) > (*b)) } {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (_, _) => Ordering::Equal,
                }
            }
            16 => {
                let a: *const C = &a;
                let a: *const u128 = a.cast();
                let b: *const C = &b;
                let b: *const u128 = b.cast();

                match unsafe { ((*a) < (*b), (*a) > (*b)) } {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (_, _) => Ordering::Equal,
                }
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) const fn gt<C>(a: C, b: C) -> bool
where
    C: Cmp,
{
    ordering(a, b).is_gt()
}

pub(crate) const fn lt<C>(a: C, b: C) -> bool
where
    C: Cmp,
{
    ordering(a, b).is_lt()
}

pub(crate) const fn is_zero<C>(a: C) -> bool
where
    C: Cmp,
{
    ordering(a, C::ZERO).is_eq()
}

pub(crate) const fn is_one<C>(a: C) -> bool
where
    C: Cmp,
{
    ordering(a, C::ONE).is_eq()
}

pub(crate) const fn is_minus_one<C>(a: C) -> bool
where
    C: Cmp,
{
    let Some(minus_one) = C::MINUS_ONE else {
        return false;
    };

    ordering(a, minus_one).is_eq()
}
