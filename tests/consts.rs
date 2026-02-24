use core::num::NonZero;

use ranch::*;

#[test]
fn consts() {
    assert_eq!(bitwise::I8::MIN, i8::MIN);
    assert_eq!(bitwise::I16::MIN, i16::MIN);
    assert_eq!(bitwise::I32::MIN, i32::MIN);
    assert_eq!(bitwise::I64::MIN, i64::MIN);
    assert_eq!(bitwise::I128::MIN, i128::MIN);

    assert_eq!(bitwise::U8::MIN, u8::MIN);
    assert_eq!(bitwise::U16::MIN, u16::MIN);
    assert_eq!(bitwise::U32::MIN, u32::MIN);
    assert_eq!(bitwise::U64::MIN, u64::MIN);
    assert_eq!(bitwise::U128::MIN, u128::MIN);

    assert_eq!(bitwise::I8::MAX, i8::MAX);
    assert_eq!(bitwise::I16::MAX, i16::MAX);
    assert_eq!(bitwise::I32::MAX, i32::MAX);
    assert_eq!(bitwise::I64::MAX, i64::MAX);
    assert_eq!(bitwise::I128::MAX, i128::MAX);

    assert_eq!(bitwise::U8::MAX, u8::MAX);
    assert_eq!(bitwise::U16::MAX, u16::MAX);
    assert_eq!(bitwise::U32::MAX, u32::MAX);
    assert_eq!(bitwise::U64::MAX, u64::MAX);
    assert_eq!(bitwise::U128::MAX, u128::MAX);

    assert_eq!(bitwise::I8::BITS, i8::BITS);
    assert_eq!(bitwise::I16::BITS, i16::BITS);
    assert_eq!(bitwise::I32::BITS, i32::BITS);
    assert_eq!(bitwise::I64::BITS, i64::BITS);
    assert_eq!(bitwise::I128::BITS, i128::BITS);

    assert_eq!(bitwise::U8::BITS, u8::BITS);
    assert_eq!(bitwise::U16::BITS, u16::BITS);
    assert_eq!(bitwise::U32::BITS, u32::BITS);
    assert_eq!(bitwise::U64::BITS, u64::BITS);
    assert_eq!(bitwise::U128::BITS, u128::BITS);

    // non-zero
    assert_eq!(bitwise::NonZeroI8::MIN, NonZero::<i8>::MIN);
    assert_eq!(bitwise::NonZeroI16::MIN, NonZero::<i16>::MIN);
    assert_eq!(bitwise::NonZeroI32::MIN, NonZero::<i32>::MIN);
    assert_eq!(bitwise::NonZeroI64::MIN, NonZero::<i64>::MIN);
    assert_eq!(bitwise::NonZeroI128::MIN, NonZero::<i128>::MIN);

    assert_eq!(bitwise::NonZeroU8::MIN, NonZero::<u8>::MIN);
    assert_eq!(bitwise::NonZeroU16::MIN, NonZero::<u16>::MIN);
    assert_eq!(bitwise::NonZeroU32::MIN, NonZero::<u32>::MIN);
    assert_eq!(bitwise::NonZeroU64::MIN, NonZero::<u64>::MIN);
    assert_eq!(bitwise::NonZeroU128::MIN, NonZero::<u128>::MIN);

    assert_eq!(bitwise::NonZeroI8::MAX, NonZero::<i8>::MAX);
    assert_eq!(bitwise::NonZeroI16::MAX, NonZero::<i16>::MAX);
    assert_eq!(bitwise::NonZeroI32::MAX, NonZero::<i32>::MAX);
    assert_eq!(bitwise::NonZeroI64::MAX, NonZero::<i64>::MAX);
    assert_eq!(bitwise::NonZeroI128::MAX, NonZero::<i128>::MAX);

    assert_eq!(bitwise::NonZeroU8::MAX, NonZero::<u8>::MAX);
    assert_eq!(bitwise::NonZeroU16::MAX, NonZero::<u16>::MAX);
    assert_eq!(bitwise::NonZeroU32::MAX, NonZero::<u32>::MAX);
    assert_eq!(bitwise::NonZeroU64::MAX, NonZero::<u64>::MAX);
    assert_eq!(bitwise::NonZeroU128::MAX, NonZero::<u128>::MAX);

    assert_eq!(bitwise::NonZeroI8::BITS, NonZero::<i8>::BITS);
    assert_eq!(bitwise::NonZeroI16::BITS, NonZero::<i16>::BITS);
    assert_eq!(bitwise::NonZeroI32::BITS, NonZero::<i32>::BITS);
    assert_eq!(bitwise::NonZeroI64::BITS, NonZero::<i64>::BITS);
    assert_eq!(bitwise::NonZeroI128::BITS, NonZero::<i128>::BITS);

    assert_eq!(bitwise::NonZeroU8::BITS, NonZero::<u8>::BITS);
    assert_eq!(bitwise::NonZeroU16::BITS, NonZero::<u16>::BITS);
    assert_eq!(bitwise::NonZeroU32::BITS, NonZero::<u32>::BITS);
    assert_eq!(bitwise::NonZeroU64::BITS, NonZero::<u64>::BITS);
    assert_eq!(bitwise::NonZeroU128::BITS, NonZero::<u128>::BITS);
}
