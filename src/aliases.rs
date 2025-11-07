//! Convenience aliases for the most common non-standard integer widths

#![expect(non_camel_case_types)]

use crate::*;

/// 1 bit unsigned integer
pub type u1 = RangedU8<0, { 2u8.pow(1) - 1 }>;
/// 2 bit unsigned integer
pub type u2 = RangedU8<0, { 2u8.pow(2) - 1 }>;
/// 3 bit unsigned integer
pub type u3 = RangedU8<0, { 2u8.pow(3) - 1 }>;
/// 4 bit unsigned integer
pub type u4 = RangedU8<0, { 2u8.pow(4) - 1 }>;
/// 5 bit unsigned integer
pub type u5 = RangedU8<0, { 2u8.pow(5) - 1 }>;
/// 6 bit unsigned integer
pub type u6 = RangedU8<0, { 2u8.pow(6) - 1 }>;
/// 7 bit unsigned integer
pub type u7 = RangedU8<0, { 2u8.pow(7) - 1 }>;

/// 10 bit unsigned integer
pub type u10 = RangedU16<0, { 2u16.pow(10) - 1 }>;
/// 12 bit unsigned integer
pub type u12 = RangedU16<0, { 2u16.pow(12) - 1 }>;
/// 14 bit unsigned integer
pub type u14 = RangedU16<0, { 2u16.pow(14) - 1 }>;

/// 20 bit unsigned integer
pub type u20 = RangedU32<0, { 2u32.pow(20) - 1 }>;
/// 24 bit unsigned integer
pub type u24 = RangedU32<0, { 2u32.pow(24) - 1 }>;
/// 28 bit unsigned integer
pub type u28 = RangedU32<0, { 2u32.pow(28) - 1 }>;

/// 40 bit unsigned integer
pub type u40 = RangedU64<0, { 2u64.pow(40) - 1 }>;
/// 48 bit unsigned integer
pub type u48 = RangedU64<0, { 2u64.pow(48) - 1 }>;
/// 56 bit unsigned integer
pub type u56 = RangedU64<0, { 2u64.pow(56) - 1 }>;

/// 80 bit unsigned integer
pub type u80 = RangedU128<0, { 2u128.pow(80) - 1 }>;
/// 96 bit unsigned integer
pub type u96 = RangedU128<0, { 2u128.pow(96) - 1 }>;
/// 112 bit unsigned integer
pub type u112 = RangedU128<0, { 2u128.pow(112) - 1 }>;

/// 1 bit signed integer
pub type i1 = RangedI8<{ -2i8.pow(0) }, { 2i8.pow(0) - 1 }>;
/// 2 bit signed integer
pub type i2 = RangedI8<{ -2i8.pow(1) }, { 2i8.pow(1) - 1 }>;
/// 3 bit signed integer
pub type i3 = RangedI8<{ -2i8.pow(2) }, { 2i8.pow(2) - 1 }>;
/// 4 bit signed integer
pub type i4 = RangedI8<{ -2i8.pow(3) }, { 2i8.pow(3) - 1 }>;
/// 5 bit signed integer
pub type i5 = RangedI8<{ -2i8.pow(4) }, { 2i8.pow(4) - 1 }>;
/// 6 bit signed integer
pub type i6 = RangedI8<{ -2i8.pow(5) }, { 2i8.pow(5) - 1 }>;
/// 7 bit signed integer
pub type i7 = RangedI8<{ -2i8.pow(6) }, { 2i8.pow(6) - 1 }>;

/// 10 bit signed integer
pub type i10 = RangedI16<{ -2i16.pow(9) }, { 2i16.pow(9) - 1 }>;
/// 12 bit signed integer
pub type i12 = RangedI16<{ -2i16.pow(11) }, { 2i16.pow(11) - 1 }>;
/// 14 bit signed integer
pub type i14 = RangedI16<{ -2i16.pow(13) }, { 2i16.pow(13) - 1 }>;

/// 20 bit signed integer
pub type i20 = RangedI32<{ -2i32.pow(19) }, { 2i32.pow(19) - 1 }>;
/// 24 bit signed integer
pub type i24 = RangedI32<{ -2i32.pow(23) }, { 2i32.pow(23) - 1 }>;
/// 28 bit signed integer
pub type i28 = RangedI32<{ -2i32.pow(27) }, { 2i32.pow(27) - 1 }>;

/// 40 bit signed integer
pub type i40 = RangedI64<{ -2i64.pow(39) }, { 2i64.pow(39) - 1 }>;
/// 48 bit signed integer
pub type i48 = RangedI64<{ -2i64.pow(47) }, { 2i64.pow(47) - 1 }>;
/// 56 bit signed integer
pub type i56 = RangedI64<{ -2i64.pow(55) }, { 2i64.pow(55) - 1 }>;

/// 80 bit signed integer
pub type i80 = RangedI128<{ -2i128.pow(79) }, { 2i128.pow(79) - 1 }>;
/// 96 bit signed integer
pub type i96 = RangedI128<{ -2i128.pow(95) }, { 2i128.pow(95) - 1 }>;
/// 112 bit signed integer
pub type i112 = RangedI128<{ -2i128.pow(111) }, { 2i128.pow(111) - 1 }>;
