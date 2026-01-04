#![allow(unsafe_code)]

use core::num::NonZero;

use as_repr::AsRepr;

use super::*;

// unsafe: `repr(transparent)` is `repr(i8)`
unsafe impl<const MIN: i8, const MAX: i8> AsRepr<i8> for RangedI8<MIN, MAX> {}

// unsafe: `repr(transparent)` is `repr(i16)`
unsafe impl<const MIN: i16, const MAX: i16> AsRepr<i16>
    for RangedI16<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(i32)`
unsafe impl<const MIN: i32, const MAX: i32> AsRepr<i32>
    for RangedI32<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(i64)`
unsafe impl<const MIN: i64, const MAX: i64> AsRepr<i64>
    for RangedI64<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(i128)`
unsafe impl<const MIN: i128, const MAX: i128> AsRepr<i128>
    for RangedI128<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(u8)`
unsafe impl<const MIN: u8, const MAX: u8> AsRepr<u8> for RangedU8<MIN, MAX> {}

// unsafe: `repr(transparent)` is `repr(u16)`
unsafe impl<const MIN: u16, const MAX: u16> AsRepr<u16>
    for RangedU16<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(u32)`
unsafe impl<const MIN: u32, const MAX: u32> AsRepr<u32>
    for RangedU32<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(u64)`
unsafe impl<const MIN: u64, const MAX: u64> AsRepr<u64>
    for RangedU64<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(u128)`
unsafe impl<const MIN: u128, const MAX: u128> AsRepr<u128>
    for RangedU128<MIN, MAX>
{
}

//

// unsafe: `repr(transparent)` is `repr(NonZero<u8>)`
unsafe impl<const MIN: u8, const MAX: u8> AsRepr<NonZero<u8>>
    for RangedNonZeroU8<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(NonZero<u16>)`
unsafe impl<const MIN: u16, const MAX: u16> AsRepr<NonZero<u16>>
    for RangedNonZeroU16<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(NonZero<u32>)`
unsafe impl<const MIN: u32, const MAX: u32> AsRepr<NonZero<u32>>
    for RangedNonZeroU32<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(NonZero<u64>)`
unsafe impl<const MIN: u64, const MAX: u64> AsRepr<NonZero<u64>>
    for RangedNonZeroU64<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(NonZero<u128>)`
unsafe impl<const MIN: u128, const MAX: u128> AsRepr<NonZero<u128>>
    for RangedNonZeroU128<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(NonZero<i8>)`
unsafe impl<const MIN: i8, const MAX: i8> AsRepr<NonZero<i8>>
    for RangedNonZeroI8<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(NonZero<i16>)`
unsafe impl<const MIN: i16, const MAX: i16> AsRepr<NonZero<i16>>
    for RangedNonZeroI16<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(NonZero<i32>)`
unsafe impl<const MIN: i32, const MAX: i32> AsRepr<NonZero<i32>>
    for RangedNonZeroI32<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(NonZero<i64>)`
unsafe impl<const MIN: i64, const MAX: i64> AsRepr<NonZero<i64>>
    for RangedNonZeroI64<MIN, MAX>
{
}

// unsafe: `repr(transparent)` is `repr(NonZero<i128>)`
unsafe impl<const MIN: i128, const MAX: i128> AsRepr<NonZero<i128>>
    for RangedNonZeroI128<MIN, MAX>
{
}
