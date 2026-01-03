//! Aliases for bitwise ranges (integers with arbitrary bit widths)

use core::num::NonZero;

use crate::*;

/// 1-bit unsigned non-zero integer
pub type NonZeroU1 = RangedNonZeroU8<1, { u8_two_pow(1) - 1 }>;
/// 2-bit unsigned non-zero integer
pub type NonZeroU2 = RangedNonZeroU8<1, { u8_two_pow(2) - 1 }>;
/// 3-bit unsigned non-zero integer
pub type NonZeroU3 = RangedNonZeroU8<1, { u8_two_pow(3) - 1 }>;
/// 4-bit unsigned non-zero integer
pub type NonZeroU4 = RangedNonZeroU8<1, { u8_two_pow(4) - 1 }>;
/// 5-bit unsigned non-zero integer
pub type NonZeroU5 = RangedNonZeroU8<1, { u8_two_pow(5) - 1 }>;
/// 6-bit unsigned non-zero integer
pub type NonZeroU6 = RangedNonZeroU8<1, { u8_two_pow(6) - 1 }>;
/// 7-bit unsigned non-zero integer
pub type NonZeroU7 = RangedNonZeroU8<1, { u8_two_pow(7) - 1 }>;
/// 8-bit unsigned non-zero integer
pub type NonZeroU8 = RangedNonZeroU8<1, { u8::MAX }>;
/// 9-bit unsigned non-zero integer
pub type NonZeroU9 = RangedNonZeroU16<1, { u16_two_pow(9) - 1 }>;
/// 10-bit unsigned non-zero integer
pub type NonZeroU10 = RangedNonZeroU16<1, { u16_two_pow(10) - 1 }>;
/// 11-bit unsigned non-zero integer
pub type NonZeroU11 = RangedNonZeroU16<1, { u16_two_pow(11) - 1 }>;
/// 12-bit unsigned non-zero integer
pub type NonZeroU12 = RangedNonZeroU16<1, { u16_two_pow(12) - 1 }>;
/// 13-bit unsigned non-zero integer
pub type NonZeroU13 = RangedNonZeroU16<1, { u16_two_pow(13) - 1 }>;
/// 14-bit unsigned non-zero integer
pub type NonZeroU14 = RangedNonZeroU16<1, { u16_two_pow(14) - 1 }>;
/// 15-bit unsigned non-zero integer
pub type NonZeroU15 = RangedNonZeroU16<1, { u16_two_pow(15) - 1 }>;
/// 16-bit unsigned non-zero integer
pub type NonZeroU16 = RangedNonZeroU16<1, { u16::MAX }>;
/// 17-bit unsigned non-zero integer
pub type NonZeroU17 = RangedNonZeroU32<1, { u32_two_pow(17) - 1 }>;
/// 18-bit unsigned non-zero integer
pub type NonZeroU18 = RangedNonZeroU32<1, { u32_two_pow(18) - 1 }>;
/// 19-bit unsigned non-zero integer
pub type NonZeroU19 = RangedNonZeroU32<1, { u32_two_pow(19) - 1 }>;
/// 20-bit unsigned non-zero integer
pub type NonZeroU20 = RangedNonZeroU32<1, { u32_two_pow(20) - 1 }>;
/// 21-bit unsigned non-zero integer
pub type NonZeroU21 = RangedNonZeroU32<1, { u32_two_pow(21) - 1 }>;
/// 22-bit unsigned non-zero integer
pub type NonZeroU22 = RangedNonZeroU32<1, { u32_two_pow(22) - 1 }>;
/// 23-bit unsigned non-zero integer
pub type NonZeroU23 = RangedNonZeroU32<1, { u32_two_pow(23) - 1 }>;
/// 24-bit unsigned non-zero integer
pub type NonZeroU24 = RangedNonZeroU32<1, { u32_two_pow(24) - 1 }>;
/// 25-bit unsigned non-zero integer
pub type NonZeroU25 = RangedNonZeroU32<1, { u32_two_pow(25) - 1 }>;
/// 26-bit unsigned non-zero integer
pub type NonZeroU26 = RangedNonZeroU32<1, { u32_two_pow(26) - 1 }>;
/// 27-bit unsigned non-zero integer
pub type NonZeroU27 = RangedNonZeroU32<1, { u32_two_pow(27) - 1 }>;
/// 28-bit unsigned non-zero integer
pub type NonZeroU28 = RangedNonZeroU32<1, { u32_two_pow(28) - 1 }>;
/// 29-bit unsigned non-zero integer
pub type NonZeroU29 = RangedNonZeroU32<1, { u32_two_pow(29) - 1 }>;
/// 30-bit unsigned non-zero integer
pub type NonZeroU30 = RangedNonZeroU32<1, { u32_two_pow(30) - 1 }>;
/// 31-bit unsigned non-zero integer
pub type NonZeroU31 = RangedNonZeroU32<1, { u32_two_pow(31) - 1 }>;
/// 32-bit unsigned non-zero integer
pub type NonZeroU32 = RangedNonZeroU32<1, { u32::MAX }>;
/// 33-bit unsigned non-zero integer
pub type NonZeroU33 = RangedNonZeroU64<1, { u64_two_pow(33) - 1 }>;
/// 34-bit unsigned non-zero integer
pub type NonZeroU34 = RangedNonZeroU64<1, { u64_two_pow(34) - 1 }>;
/// 35-bit unsigned non-zero integer
pub type NonZeroU35 = RangedNonZeroU64<1, { u64_two_pow(35) - 1 }>;
/// 36-bit unsigned non-zero integer
pub type NonZeroU36 = RangedNonZeroU64<1, { u64_two_pow(36) - 1 }>;
/// 37-bit unsigned non-zero integer
pub type NonZeroU37 = RangedNonZeroU64<1, { u64_two_pow(37) - 1 }>;
/// 38-bit unsigned non-zero integer
pub type NonZeroU38 = RangedNonZeroU64<1, { u64_two_pow(38) - 1 }>;
/// 39-bit unsigned non-zero integer
pub type NonZeroU39 = RangedNonZeroU64<1, { u64_two_pow(39) - 1 }>;
/// 40-bit unsigned non-zero integer
pub type NonZeroU40 = RangedNonZeroU64<1, { u64_two_pow(40) - 1 }>;
/// 41-bit unsigned non-zero integer
pub type NonZeroU41 = RangedNonZeroU64<1, { u64_two_pow(41) - 1 }>;
/// 42-bit unsigned non-zero integer
pub type NonZeroU42 = RangedNonZeroU64<1, { u64_two_pow(42) - 1 }>;
/// 43-bit unsigned non-zero integer
pub type NonZeroU43 = RangedNonZeroU64<1, { u64_two_pow(43) - 1 }>;
/// 44-bit unsigned non-zero integer
pub type NonZeroU44 = RangedNonZeroU64<1, { u64_two_pow(44) - 1 }>;
/// 45-bit unsigned non-zero integer
pub type NonZeroU45 = RangedNonZeroU64<1, { u64_two_pow(45) - 1 }>;
/// 46-bit unsigned non-zero integer
pub type NonZeroU46 = RangedNonZeroU64<1, { u64_two_pow(46) - 1 }>;
/// 47-bit unsigned non-zero integer
pub type NonZeroU47 = RangedNonZeroU64<1, { u64_two_pow(47) - 1 }>;
/// 48-bit unsigned non-zero integer
pub type NonZeroU48 = RangedNonZeroU64<1, { u64_two_pow(48) - 1 }>;
/// 49-bit unsigned non-zero integer
pub type NonZeroU49 = RangedNonZeroU64<1, { u64_two_pow(49) - 1 }>;
/// 50-bit unsigned non-zero integer
pub type NonZeroU50 = RangedNonZeroU64<1, { u64_two_pow(50) - 1 }>;
/// 51-bit unsigned non-zero integer
pub type NonZeroU51 = RangedNonZeroU64<1, { u64_two_pow(51) - 1 }>;
/// 52-bit unsigned non-zero integer
pub type NonZeroU52 = RangedNonZeroU64<1, { u64_two_pow(52) - 1 }>;
/// 53-bit unsigned non-zero integer
pub type NonZeroU53 = RangedNonZeroU64<1, { u64_two_pow(53) - 1 }>;
/// 54-bit unsigned non-zero integer
pub type NonZeroU54 = RangedNonZeroU64<1, { u64_two_pow(54) - 1 }>;
/// 55-bit unsigned non-zero integer
pub type NonZeroU55 = RangedNonZeroU64<1, { u64_two_pow(55) - 1 }>;
/// 56-bit unsigned non-zero integer
pub type NonZeroU56 = RangedNonZeroU64<1, { u64_two_pow(56) - 1 }>;
/// 57-bit unsigned non-zero integer
pub type NonZeroU57 = RangedNonZeroU64<1, { u64_two_pow(57) - 1 }>;
/// 58-bit unsigned non-zero integer
pub type NonZeroU58 = RangedNonZeroU64<1, { u64_two_pow(58) - 1 }>;
/// 59-bit unsigned non-zero integer
pub type NonZeroU59 = RangedNonZeroU64<1, { u64_two_pow(59) - 1 }>;
/// 60-bit unsigned non-zero integer
pub type NonZeroU60 = RangedNonZeroU64<1, { u64_two_pow(60) - 1 }>;
/// 61-bit unsigned non-zero integer
pub type NonZeroU61 = RangedNonZeroU64<1, { u64_two_pow(61) - 1 }>;
/// 62-bit unsigned non-zero integer
pub type NonZeroU62 = RangedNonZeroU64<1, { u64_two_pow(62) - 1 }>;
/// 63-bit unsigned non-zero integer
pub type NonZeroU63 = RangedNonZeroU64<1, { u64_two_pow(63) - 1 }>;
/// 64-bit unsigned non-zero integer
pub type NonZeroU64 = RangedNonZeroU64<1, { u64::MAX }>;
/// 65-bit unsigned non-zero integer
pub type NonZeroU65 = RangedNonZeroU128<1, { u128_two_pow(65) - 1 }>;
/// 66-bit unsigned non-zero integer
pub type NonZeroU66 = RangedNonZeroU128<1, { u128_two_pow(66) - 1 }>;
/// 67-bit unsigned non-zero integer
pub type NonZeroU67 = RangedNonZeroU128<1, { u128_two_pow(67) - 1 }>;
/// 68-bit unsigned non-zero integer
pub type NonZeroU68 = RangedNonZeroU128<1, { u128_two_pow(68) - 1 }>;
/// 69-bit unsigned non-zero integer
pub type NonZeroU69 = RangedNonZeroU128<1, { u128_two_pow(69) - 1 }>;
/// 70-bit unsigned non-zero integer
pub type NonZeroU70 = RangedNonZeroU128<1, { u128_two_pow(70) - 1 }>;
/// 71-bit unsigned non-zero integer
pub type NonZeroU71 = RangedNonZeroU128<1, { u128_two_pow(71) - 1 }>;
/// 72-bit unsigned non-zero integer
pub type NonZeroU72 = RangedNonZeroU128<1, { u128_two_pow(72) - 1 }>;
/// 73-bit unsigned non-zero integer
pub type NonZeroU73 = RangedNonZeroU128<1, { u128_two_pow(73) - 1 }>;
/// 74-bit unsigned non-zero integer
pub type NonZeroU74 = RangedNonZeroU128<1, { u128_two_pow(74) - 1 }>;
/// 75-bit unsigned non-zero integer
pub type NonZeroU75 = RangedNonZeroU128<1, { u128_two_pow(75) - 1 }>;
/// 76-bit unsigned non-zero integer
pub type NonZeroU76 = RangedNonZeroU128<1, { u128_two_pow(76) - 1 }>;
/// 77-bit unsigned non-zero integer
pub type NonZeroU77 = RangedNonZeroU128<1, { u128_two_pow(77) - 1 }>;
/// 78-bit unsigned non-zero integer
pub type NonZeroU78 = RangedNonZeroU128<1, { u128_two_pow(78) - 1 }>;
/// 79-bit unsigned non-zero integer
pub type NonZeroU79 = RangedNonZeroU128<1, { u128_two_pow(79) - 1 }>;
/// 80-bit unsigned non-zero integer
pub type NonZeroU80 = RangedNonZeroU128<1, { u128_two_pow(80) - 1 }>;
/// 81-bit unsigned non-zero integer
pub type NonZeroU81 = RangedNonZeroU128<1, { u128_two_pow(81) - 1 }>;
/// 82-bit unsigned non-zero integer
pub type NonZeroU82 = RangedNonZeroU128<1, { u128_two_pow(82) - 1 }>;
/// 83-bit unsigned non-zero integer
pub type NonZeroU83 = RangedNonZeroU128<1, { u128_two_pow(83) - 1 }>;
/// 84-bit unsigned non-zero integer
pub type NonZeroU84 = RangedNonZeroU128<1, { u128_two_pow(84) - 1 }>;
/// 85-bit unsigned non-zero integer
pub type NonZeroU85 = RangedNonZeroU128<1, { u128_two_pow(85) - 1 }>;
/// 86-bit unsigned non-zero integer
pub type NonZeroU86 = RangedNonZeroU128<1, { u128_two_pow(86) - 1 }>;
/// 87-bit unsigned non-zero integer
pub type NonZeroU87 = RangedNonZeroU128<1, { u128_two_pow(87) - 1 }>;
/// 88-bit unsigned non-zero integer
pub type NonZeroU88 = RangedNonZeroU128<1, { u128_two_pow(88) - 1 }>;
/// 89-bit unsigned non-zero integer
pub type NonZeroU89 = RangedNonZeroU128<1, { u128_two_pow(89) - 1 }>;
/// 90-bit unsigned non-zero integer
pub type NonZeroU90 = RangedNonZeroU128<1, { u128_two_pow(90) - 1 }>;
/// 91-bit unsigned non-zero integer
pub type NonZeroU91 = RangedNonZeroU128<1, { u128_two_pow(91) - 1 }>;
/// 92-bit unsigned non-zero integer
pub type NonZeroU92 = RangedNonZeroU128<1, { u128_two_pow(92) - 1 }>;
/// 93-bit unsigned non-zero integer
pub type NonZeroU93 = RangedNonZeroU128<1, { u128_two_pow(93) - 1 }>;
/// 94-bit unsigned non-zero integer
pub type NonZeroU94 = RangedNonZeroU128<1, { u128_two_pow(94) - 1 }>;
/// 95-bit unsigned non-zero integer
pub type NonZeroU95 = RangedNonZeroU128<1, { u128_two_pow(95) - 1 }>;
/// 96-bit unsigned non-zero integer
pub type NonZeroU96 = RangedNonZeroU128<1, { u128_two_pow(96) - 1 }>;
/// 97-bit unsigned non-zero integer
pub type NonZeroU97 = RangedNonZeroU128<1, { u128_two_pow(97) - 1 }>;
/// 98-bit unsigned non-zero integer
pub type NonZeroU98 = RangedNonZeroU128<1, { u128_two_pow(98) - 1 }>;
/// 99-bit unsigned non-zero integer
pub type NonZeroU99 = RangedNonZeroU128<1, { u128_two_pow(99) - 1 }>;
/// 100-bit unsigned non-zero integer
pub type NonZeroU100 = RangedNonZeroU128<1, { u128_two_pow(100) - 1 }>;
/// 101-bit unsigned non-zero integer
pub type NonZeroU101 = RangedNonZeroU128<1, { u128_two_pow(101) - 1 }>;
/// 102-bit unsigned non-zero integer
pub type NonZeroU102 = RangedNonZeroU128<1, { u128_two_pow(102) - 1 }>;
/// 103-bit unsigned non-zero integer
pub type NonZeroU103 = RangedNonZeroU128<1, { u128_two_pow(103) - 1 }>;
/// 104-bit unsigned non-zero integer
pub type NonZeroU104 = RangedNonZeroU128<1, { u128_two_pow(104) - 1 }>;
/// 105-bit unsigned non-zero integer
pub type NonZeroU105 = RangedNonZeroU128<1, { u128_two_pow(105) - 1 }>;
/// 106-bit unsigned non-zero integer
pub type NonZeroU106 = RangedNonZeroU128<1, { u128_two_pow(106) - 1 }>;
/// 107-bit unsigned non-zero integer
pub type NonZeroU107 = RangedNonZeroU128<1, { u128_two_pow(107) - 1 }>;
/// 108-bit unsigned non-zero integer
pub type NonZeroU108 = RangedNonZeroU128<1, { u128_two_pow(108) - 1 }>;
/// 109-bit unsigned non-zero integer
pub type NonZeroU109 = RangedNonZeroU128<1, { u128_two_pow(109) - 1 }>;
/// 110-bit unsigned non-zero integer
pub type NonZeroU110 = RangedNonZeroU128<1, { u128_two_pow(110) - 1 }>;
/// 111-bit unsigned non-zero integer
pub type NonZeroU111 = RangedNonZeroU128<1, { u128_two_pow(111) - 1 }>;
/// 112-bit unsigned non-zero integer
pub type NonZeroU112 = RangedNonZeroU128<1, { u128_two_pow(112) - 1 }>;
/// 113-bit unsigned non-zero integer
pub type NonZeroU113 = RangedNonZeroU128<1, { u128_two_pow(113) - 1 }>;
/// 114-bit unsigned non-zero integer
pub type NonZeroU114 = RangedNonZeroU128<1, { u128_two_pow(114) - 1 }>;
/// 115-bit unsigned non-zero integer
pub type NonZeroU115 = RangedNonZeroU128<1, { u128_two_pow(115) - 1 }>;
/// 116-bit unsigned non-zero integer
pub type NonZeroU116 = RangedNonZeroU128<1, { u128_two_pow(116) - 1 }>;
/// 117-bit unsigned non-zero integer
pub type NonZeroU117 = RangedNonZeroU128<1, { u128_two_pow(117) - 1 }>;
/// 118-bit unsigned non-zero integer
pub type NonZeroU118 = RangedNonZeroU128<1, { u128_two_pow(118) - 1 }>;
/// 119-bit unsigned non-zero integer
pub type NonZeroU119 = RangedNonZeroU128<1, { u128_two_pow(119) - 1 }>;
/// 120-bit unsigned non-zero integer
pub type NonZeroU120 = RangedNonZeroU128<1, { u128_two_pow(120) - 1 }>;
/// 121-bit unsigned non-zero integer
pub type NonZeroU121 = RangedNonZeroU128<1, { u128_two_pow(121) - 1 }>;
/// 122-bit unsigned non-zero integer
pub type NonZeroU122 = RangedNonZeroU128<1, { u128_two_pow(122) - 1 }>;
/// 123-bit unsigned non-zero integer
pub type NonZeroU123 = RangedNonZeroU128<1, { u128_two_pow(123) - 1 }>;
/// 124-bit unsigned non-zero integer
pub type NonZeroU124 = RangedNonZeroU128<1, { u128_two_pow(124) - 1 }>;
/// 125-bit unsigned non-zero integer
pub type NonZeroU125 = RangedNonZeroU128<1, { u128_two_pow(125) - 1 }>;
/// 126-bit unsigned non-zero integer
pub type NonZeroU126 = RangedNonZeroU128<1, { u128_two_pow(126) - 1 }>;
/// 127-bit unsigned non-zero integer
pub type NonZeroU127 = RangedNonZeroU128<1, { u128_two_pow(127) - 1 }>;
/// 128-bit unsigned non-zero integer
pub type NonZeroU128 = RangedNonZeroU128<1, { u128::MAX }>;

/// 1-bit signed non-zero integer
pub type NonZeroI1 = RangedNonZeroI8<{ -i8_two_pow(0) }, { i8_two_pow(0) }>;
/// 2-bit signed non-zero integer
pub type NonZeroI2 = RangedNonZeroI8<{ -i8_two_pow(1) }, { i8_two_pow(1) - 1 }>;
/// 3-bit signed non-zero integer
pub type NonZeroI3 = RangedNonZeroI8<{ -i8_two_pow(2) }, { i8_two_pow(2) - 1 }>;
/// 4-bit signed non-zero integer
pub type NonZeroI4 = RangedNonZeroI8<{ -i8_two_pow(3) }, { i8_two_pow(3) - 1 }>;
/// 5-bit signed non-zero integer
pub type NonZeroI5 = RangedNonZeroI8<{ -i8_two_pow(4) }, { i8_two_pow(4) - 1 }>;
/// 6-bit signed non-zero integer
pub type NonZeroI6 = RangedNonZeroI8<{ -i8_two_pow(5) }, { i8_two_pow(5) - 1 }>;
/// 7-bit signed non-zero integer
pub type NonZeroI7 = RangedNonZeroI8<{ -i8_two_pow(6) }, { i8_two_pow(6) - 1 }>;
/// 8-bit signed non-zero integer
pub type NonZeroI8 = RangedNonZeroI8<{ i8::MIN }, { i8::MAX }>;
/// 9-bit signed non-zero integer
pub type NonZeroI9 = RangedNonZeroI16<{ -i16_two_pow(8) }, { i16_two_pow(8) - 1 }>;
/// 10-bit signed non-zero integer
pub type NonZeroI10 = RangedNonZeroI16<{ -i16_two_pow(9) }, { i16_two_pow(9) - 1 }>;
/// 11-bit signed non-zero integer
pub type NonZeroI11 = RangedNonZeroI16<{ -i16_two_pow(10) }, { i16_two_pow(10) - 1 }>;
/// 12-bit signed non-zero integer
pub type NonZeroI12 = RangedNonZeroI16<{ -i16_two_pow(11) }, { i16_two_pow(11) - 1 }>;
/// 13-bit signed non-zero integer
pub type NonZeroI13 = RangedNonZeroI16<{ -i16_two_pow(12) }, { i16_two_pow(12) - 1 }>;
/// 14-bit signed non-zero integer
pub type NonZeroI14 = RangedNonZeroI16<{ -i16_two_pow(13) }, { i16_two_pow(13) - 1 }>;
/// 15-bit signed non-zero integer
pub type NonZeroI15 = RangedNonZeroI16<{ -i16_two_pow(14) }, { i16_two_pow(14) - 1 }>;
/// 16-bit signed non-zero integer
pub type NonZeroI16 = RangedNonZeroI16<{ i16::MIN }, { i16::MAX }>;
/// 17-bit signed non-zero integer
pub type NonZeroI17 = RangedNonZeroI32<{ -i32_two_pow(16) }, { i32_two_pow(16) - 1 }>;
/// 18-bit signed non-zero integer
pub type NonZeroI18 = RangedNonZeroI32<{ -i32_two_pow(17) }, { i32_two_pow(17) - 1 }>;
/// 19-bit signed non-zero integer
pub type NonZeroI19 = RangedNonZeroI32<{ -i32_two_pow(18) }, { i32_two_pow(18) - 1 }>;
/// 20-bit signed non-zero integer
pub type NonZeroI20 = RangedNonZeroI32<{ -i32_two_pow(19) }, { i32_two_pow(19) - 1 }>;
/// 21-bit signed non-zero integer
pub type NonZeroI21 = RangedNonZeroI32<{ -i32_two_pow(20) }, { i32_two_pow(20) - 1 }>;
/// 22-bit signed non-zero integer
pub type NonZeroI22 = RangedNonZeroI32<{ -i32_two_pow(21) }, { i32_two_pow(21) - 1 }>;
/// 23-bit signed non-zero integer
pub type NonZeroI23 = RangedNonZeroI32<{ -i32_two_pow(22) }, { i32_two_pow(22) - 1 }>;
/// 24-bit signed non-zero integer
pub type NonZeroI24 = RangedNonZeroI32<{ -i32_two_pow(23) }, { i32_two_pow(23) - 1 }>;
/// 25-bit signed non-zero integer
pub type NonZeroI25 = RangedNonZeroI32<{ -i32_two_pow(24) }, { i32_two_pow(24) - 1 }>;
/// 26-bit signed non-zero integer
pub type NonZeroI26 = RangedNonZeroI32<{ -i32_two_pow(25) }, { i32_two_pow(25) - 1 }>;
/// 27-bit signed non-zero integer
pub type NonZeroI27 = RangedNonZeroI32<{ -i32_two_pow(26) }, { i32_two_pow(26) - 1 }>;
/// 28-bit signed non-zero integer
pub type NonZeroI28 = RangedNonZeroI32<{ -i32_two_pow(27) }, { i32_two_pow(27) - 1 }>;
/// 29-bit signed non-zero integer
pub type NonZeroI29 = RangedNonZeroI32<{ -i32_two_pow(28) }, { i32_two_pow(28) - 1 }>;
/// 30-bit signed non-zero integer
pub type NonZeroI30 = RangedNonZeroI32<{ -i32_two_pow(29) }, { i32_two_pow(29) - 1 }>;
/// 31-bit signed non-zero integer
pub type NonZeroI31 = RangedNonZeroI32<{ -i32_two_pow(30) }, { i32_two_pow(30) - 1 }>;
/// 32-bit signed non-zero integer
pub type NonZeroI32 = RangedNonZeroI32<{ i32::MIN }, { i32::MAX }>;
/// 33-bit signed non-zero integer
pub type NonZeroI33 = RangedNonZeroI64<{ -i64_two_pow(32) }, { i64_two_pow(32) - 1 }>;
/// 34-bit signed non-zero integer
pub type NonZeroI34 = RangedNonZeroI64<{ -i64_two_pow(33) }, { i64_two_pow(33) - 1 }>;
/// 35-bit signed non-zero integer
pub type NonZeroI35 = RangedNonZeroI64<{ -i64_two_pow(34) }, { i64_two_pow(34) - 1 }>;
/// 36-bit signed non-zero integer
pub type NonZeroI36 = RangedNonZeroI64<{ -i64_two_pow(35) }, { i64_two_pow(35) - 1 }>;
/// 37-bit signed non-zero integer
pub type NonZeroI37 = RangedNonZeroI64<{ -i64_two_pow(36) }, { i64_two_pow(36) - 1 }>;
/// 38-bit signed non-zero integer
pub type NonZeroI38 = RangedNonZeroI64<{ -i64_two_pow(37) }, { i64_two_pow(37) - 1 }>;
/// 39-bit signed non-zero integer
pub type NonZeroI39 = RangedNonZeroI64<{ -i64_two_pow(38) }, { i64_two_pow(38) - 1 }>;
/// 40-bit signed non-zero integer
pub type NonZeroI40 = RangedNonZeroI64<{ -i64_two_pow(39) }, { i64_two_pow(39) - 1 }>;
/// 41-bit signed non-zero integer
pub type NonZeroI41 = RangedNonZeroI64<{ -i64_two_pow(40) }, { i64_two_pow(40) - 1 }>;
/// 42-bit signed non-zero integer
pub type NonZeroI42 = RangedNonZeroI64<{ -i64_two_pow(41) }, { i64_two_pow(41) - 1 }>;
/// 43-bit signed non-zero integer
pub type NonZeroI43 = RangedNonZeroI64<{ -i64_two_pow(42) }, { i64_two_pow(42) - 1 }>;
/// 44-bit signed non-zero integer
pub type NonZeroI44 = RangedNonZeroI64<{ -i64_two_pow(43) }, { i64_two_pow(43) - 1 }>;
/// 45-bit signed non-zero integer
pub type NonZeroI45 = RangedNonZeroI64<{ -i64_two_pow(44) }, { i64_two_pow(44) - 1 }>;
/// 46-bit signed non-zero integer
pub type NonZeroI46 = RangedNonZeroI64<{ -i64_two_pow(45) }, { i64_two_pow(45) - 1 }>;
/// 47-bit signed non-zero integer
pub type NonZeroI47 = RangedNonZeroI64<{ -i64_two_pow(46) }, { i64_two_pow(46) - 1 }>;
/// 48-bit signed non-zero integer
pub type NonZeroI48 = RangedNonZeroI64<{ -i64_two_pow(47) }, { i64_two_pow(47) - 1 }>;
/// 49-bit signed non-zero integer
pub type NonZeroI49 = RangedNonZeroI64<{ -i64_two_pow(48) }, { i64_two_pow(48) - 1 }>;
/// 50-bit signed non-zero integer
pub type NonZeroI50 = RangedNonZeroI64<{ -i64_two_pow(49) }, { i64_two_pow(49) - 1 }>;
/// 51-bit signed non-zero integer
pub type NonZeroI51 = RangedNonZeroI64<{ -i64_two_pow(50) }, { i64_two_pow(50) - 1 }>;
/// 52-bit signed non-zero integer
pub type NonZeroI52 = RangedNonZeroI64<{ -i64_two_pow(51) }, { i64_two_pow(51) - 1 }>;
/// 53-bit signed non-zero integer
pub type NonZeroI53 = RangedNonZeroI64<{ -i64_two_pow(52) }, { i64_two_pow(52) - 1 }>;
/// 54-bit signed non-zero integer
pub type NonZeroI54 = RangedNonZeroI64<{ -i64_two_pow(53) }, { i64_two_pow(53) - 1 }>;
/// 55-bit signed non-zero integer
pub type NonZeroI55 = RangedNonZeroI64<{ -i64_two_pow(54) }, { i64_two_pow(54) - 1 }>;
/// 56-bit signed non-zero integer
pub type NonZeroI56 = RangedNonZeroI64<{ -i64_two_pow(55) }, { i64_two_pow(55) - 1 }>;
/// 57-bit signed non-zero integer
pub type NonZeroI57 = RangedNonZeroI64<{ -i64_two_pow(56) }, { i64_two_pow(56) - 1 }>;
/// 58-bit signed non-zero integer
pub type NonZeroI58 = RangedNonZeroI64<{ -i64_two_pow(57) }, { i64_two_pow(57) - 1 }>;
/// 59-bit signed non-zero integer
pub type NonZeroI59 = RangedNonZeroI64<{ -i64_two_pow(58) }, { i64_two_pow(58) - 1 }>;
/// 60-bit signed non-zero integer
pub type NonZeroI60 = RangedNonZeroI64<{ -i64_two_pow(59) }, { i64_two_pow(59) - 1 }>;
/// 61-bit signed non-zero integer
pub type NonZeroI61 = RangedNonZeroI64<{ -i64_two_pow(60) }, { i64_two_pow(60) - 1 }>;
/// 62-bit signed non-zero integer
pub type NonZeroI62 = RangedNonZeroI64<{ -i64_two_pow(61) }, { i64_two_pow(61) - 1 }>;
/// 63-bit signed non-zero integer
pub type NonZeroI63 = RangedNonZeroI64<{ -i64_two_pow(62) }, { i64_two_pow(62) - 1 }>;
/// 64-bit signed non-zero integer
pub type NonZeroI64 = RangedNonZeroI64<{ i64::MIN }, { i64::MAX }>;
/// 65-bit signed non-zero integer
pub type NonZeroI65 = RangedNonZeroI128<{ -i128_two_pow(64) }, { i128_two_pow(64) - 1 }>;
/// 66-bit signed non-zero integer
pub type NonZeroI66 = RangedNonZeroI128<{ -i128_two_pow(65) }, { i128_two_pow(65) - 1 }>;
/// 67-bit signed non-zero integer
pub type NonZeroI67 = RangedNonZeroI128<{ -i128_two_pow(66) }, { i128_two_pow(66) - 1 }>;
/// 68-bit signed non-zero integer
pub type NonZeroI68 = RangedNonZeroI128<{ -i128_two_pow(67) }, { i128_two_pow(67) - 1 }>;
/// 69-bit signed non-zero integer
pub type NonZeroI69 = RangedNonZeroI128<{ -i128_two_pow(68) }, { i128_two_pow(68) - 1 }>;
/// 70-bit signed non-zero integer
pub type NonZeroI70 = RangedNonZeroI128<{ -i128_two_pow(69) }, { i128_two_pow(69) - 1 }>;
/// 71-bit signed non-zero integer
pub type NonZeroI71 = RangedNonZeroI128<{ -i128_two_pow(70) }, { i128_two_pow(70) - 1 }>;
/// 72-bit signed non-zero integer
pub type NonZeroI72 = RangedNonZeroI128<{ -i128_two_pow(71) }, { i128_two_pow(71) - 1 }>;
/// 73-bit signed non-zero integer
pub type NonZeroI73 = RangedNonZeroI128<{ -i128_two_pow(72) }, { i128_two_pow(72) - 1 }>;
/// 74-bit signed non-zero integer
pub type NonZeroI74 = RangedNonZeroI128<{ -i128_two_pow(73) }, { i128_two_pow(73) - 1 }>;
/// 75-bit signed non-zero integer
pub type NonZeroI75 = RangedNonZeroI128<{ -i128_two_pow(74) }, { i128_two_pow(74) - 1 }>;
/// 76-bit signed non-zero integer
pub type NonZeroI76 = RangedNonZeroI128<{ -i128_two_pow(75) }, { i128_two_pow(75) - 1 }>;
/// 77-bit signed non-zero integer
pub type NonZeroI77 = RangedNonZeroI128<{ -i128_two_pow(76) }, { i128_two_pow(76) - 1 }>;
/// 78-bit signed non-zero integer
pub type NonZeroI78 = RangedNonZeroI128<{ -i128_two_pow(77) }, { i128_two_pow(77) - 1 }>;
/// 79-bit signed non-zero integer
pub type NonZeroI79 = RangedNonZeroI128<{ -i128_two_pow(78) }, { i128_two_pow(78) - 1 }>;
/// 80-bit signed non-zero integer
pub type NonZeroI80 = RangedNonZeroI128<{ -i128_two_pow(79) }, { i128_two_pow(79) - 1 }>;
/// 81-bit signed non-zero integer
pub type NonZeroI81 = RangedNonZeroI128<{ -i128_two_pow(80) }, { i128_two_pow(80) - 1 }>;
/// 82-bit signed non-zero integer
pub type NonZeroI82 = RangedNonZeroI128<{ -i128_two_pow(81) }, { i128_two_pow(81) - 1 }>;
/// 83-bit signed non-zero integer
pub type NonZeroI83 = RangedNonZeroI128<{ -i128_two_pow(82) }, { i128_two_pow(82) - 1 }>;
/// 84-bit signed non-zero integer
pub type NonZeroI84 = RangedNonZeroI128<{ -i128_two_pow(83) }, { i128_two_pow(83) - 1 }>;
/// 85-bit signed non-zero integer
pub type NonZeroI85 = RangedNonZeroI128<{ -i128_two_pow(84) }, { i128_two_pow(84) - 1 }>;
/// 86-bit signed non-zero integer
pub type NonZeroI86 = RangedNonZeroI128<{ -i128_two_pow(85) }, { i128_two_pow(85) - 1 }>;
/// 87-bit signed non-zero integer
pub type NonZeroI87 = RangedNonZeroI128<{ -i128_two_pow(86) }, { i128_two_pow(86) - 1 }>;
/// 88-bit signed non-zero integer
pub type NonZeroI88 = RangedNonZeroI128<{ -i128_two_pow(87) }, { i128_two_pow(87) - 1 }>;
/// 89-bit signed non-zero integer
pub type NonZeroI89 = RangedNonZeroI128<{ -i128_two_pow(88) }, { i128_two_pow(88) - 1 }>;
/// 90-bit signed non-zero integer
pub type NonZeroI90 = RangedNonZeroI128<{ -i128_two_pow(89) }, { i128_two_pow(89) - 1 }>;
/// 91-bit signed non-zero integer
pub type NonZeroI91 = RangedNonZeroI128<{ -i128_two_pow(90) }, { i128_two_pow(90) - 1 }>;
/// 92-bit signed non-zero integer
pub type NonZeroI92 = RangedNonZeroI128<{ -i128_two_pow(91) }, { i128_two_pow(91) - 1 }>;
/// 93-bit signed non-zero integer
pub type NonZeroI93 = RangedNonZeroI128<{ -i128_two_pow(92) }, { i128_two_pow(92) - 1 }>;
/// 94-bit signed non-zero integer
pub type NonZeroI94 = RangedNonZeroI128<{ -i128_two_pow(93) }, { i128_two_pow(93) - 1 }>;
/// 95-bit signed non-zero integer
pub type NonZeroI95 = RangedNonZeroI128<{ -i128_two_pow(94) }, { i128_two_pow(94) - 1 }>;
/// 96-bit signed non-zero integer
pub type NonZeroI96 = RangedNonZeroI128<{ -i128_two_pow(95) }, { i128_two_pow(95) - 1 }>;
/// 97-bit signed non-zero integer
pub type NonZeroI97 = RangedNonZeroI128<{ -i128_two_pow(96) }, { i128_two_pow(96) - 1 }>;
/// 98-bit signed non-zero integer
pub type NonZeroI98 = RangedNonZeroI128<{ -i128_two_pow(97) }, { i128_two_pow(97) - 1 }>;
/// 99-bit signed non-zero integer
pub type NonZeroI99 = RangedNonZeroI128<{ -i128_two_pow(98) }, { i128_two_pow(98) - 1 }>;
/// 100-bit signed non-zero integer
pub type NonZeroI100 = RangedNonZeroI128<{ -i128_two_pow(99) }, { i128_two_pow(99) - 1 }>;
/// 101-bit signed non-zero integer
pub type NonZeroI101 = RangedNonZeroI128<{ -i128_two_pow(100) }, { i128_two_pow(100) - 1 }>;
/// 102-bit signed non-zero integer
pub type NonZeroI102 = RangedNonZeroI128<{ -i128_two_pow(101) }, { i128_two_pow(101) - 1 }>;
/// 103-bit signed non-zero integer
pub type NonZeroI103 = RangedNonZeroI128<{ -i128_two_pow(102) }, { i128_two_pow(102) - 1 }>;
/// 104-bit signed non-zero integer
pub type NonZeroI104 = RangedNonZeroI128<{ -i128_two_pow(103) }, { i128_two_pow(103) - 1 }>;
/// 105-bit signed non-zero integer
pub type NonZeroI105 = RangedNonZeroI128<{ -i128_two_pow(104) }, { i128_two_pow(104) - 1 }>;
/// 106-bit signed non-zero integer
pub type NonZeroI106 = RangedNonZeroI128<{ -i128_two_pow(105) }, { i128_two_pow(105) - 1 }>;
/// 107-bit signed non-zero integer
pub type NonZeroI107 = RangedNonZeroI128<{ -i128_two_pow(106) }, { i128_two_pow(106) - 1 }>;
/// 108-bit signed non-zero integer
pub type NonZeroI108 = RangedNonZeroI128<{ -i128_two_pow(107) }, { i128_two_pow(107) - 1 }>;
/// 109-bit signed non-zero integer
pub type NonZeroI109 = RangedNonZeroI128<{ -i128_two_pow(108) }, { i128_two_pow(108) - 1 }>;
/// 110-bit signed non-zero integer
pub type NonZeroI110 = RangedNonZeroI128<{ -i128_two_pow(109) }, { i128_two_pow(109) - 1 }>;
/// 111-bit signed non-zero integer
pub type NonZeroI111 = RangedNonZeroI128<{ -i128_two_pow(110) }, { i128_two_pow(110) - 1 }>;
/// 112-bit signed non-zero integer
pub type NonZeroI112 = RangedNonZeroI128<{ -i128_two_pow(111) }, { i128_two_pow(111) - 1 }>;
/// 113-bit signed non-zero integer
pub type NonZeroI113 = RangedNonZeroI128<{ -i128_two_pow(112) }, { i128_two_pow(112) - 1 }>;
/// 114-bit signed non-zero integer
pub type NonZeroI114 = RangedNonZeroI128<{ -i128_two_pow(113) }, { i128_two_pow(113) - 1 }>;
/// 115-bit signed non-zero integer
pub type NonZeroI115 = RangedNonZeroI128<{ -i128_two_pow(114) }, { i128_two_pow(114) - 1 }>;
/// 116-bit signed non-zero integer
pub type NonZeroI116 = RangedNonZeroI128<{ -i128_two_pow(115) }, { i128_two_pow(115) - 1 }>;
/// 117-bit signed non-zero integer
pub type NonZeroI117 = RangedNonZeroI128<{ -i128_two_pow(116) }, { i128_two_pow(116) - 1 }>;
/// 118-bit signed non-zero integer
pub type NonZeroI118 = RangedNonZeroI128<{ -i128_two_pow(117) }, { i128_two_pow(117) - 1 }>;
/// 119-bit signed non-zero integer
pub type NonZeroI119 = RangedNonZeroI128<{ -i128_two_pow(118) }, { i128_two_pow(118) - 1 }>;
/// 120-bit signed non-zero integer
pub type NonZeroI120 = RangedNonZeroI128<{ -i128_two_pow(119) }, { i128_two_pow(119) - 1 }>;
/// 121-bit signed non-zero integer
pub type NonZeroI121 = RangedNonZeroI128<{ -i128_two_pow(120) }, { i128_two_pow(120) - 1 }>;
/// 122-bit signed non-zero integer
pub type NonZeroI122 = RangedNonZeroI128<{ -i128_two_pow(121) }, { i128_two_pow(121) - 1 }>;
/// 123-bit signed non-zero integer
pub type NonZeroI123 = RangedNonZeroI128<{ -i128_two_pow(122) }, { i128_two_pow(122) - 1 }>;
/// 124-bit signed non-zero integer
pub type NonZeroI124 = RangedNonZeroI128<{ -i128_two_pow(123) }, { i128_two_pow(123) - 1 }>;
/// 125-bit signed non-zero integer
pub type NonZeroI125 = RangedNonZeroI128<{ -i128_two_pow(124) }, { i128_two_pow(124) - 1 }>;
/// 126-bit signed non-zero integer
pub type NonZeroI126 = RangedNonZeroI128<{ -i128_two_pow(125) }, { i128_two_pow(125) - 1 }>;
/// 127-bit signed non-zero integer
pub type NonZeroI127 = RangedNonZeroI128<{ -i128_two_pow(126) }, { i128_two_pow(126) - 1 }>;
/// 128-bit signed non-zero integer
pub type NonZeroI128 = RangedNonZeroI128<{ i128::MIN }, { i128::MAX }>;

/// 1-bit unsigned integer
pub type U1 = RangedU8<0, { u8_two_pow(1) - 1 }>;
/// 2-bit unsigned integer
pub type U2 = RangedU8<0, { u8_two_pow(2) - 1 }>;
/// 3-bit unsigned integer
pub type U3 = RangedU8<0, { u8_two_pow(3) - 1 }>;
/// 4-bit unsigned integer
pub type U4 = RangedU8<0, { u8_two_pow(4) - 1 }>;
/// 5-bit unsigned integer
pub type U5 = RangedU8<0, { u8_two_pow(5) - 1 }>;
/// 6-bit unsigned integer
pub type U6 = RangedU8<0, { u8_two_pow(6) - 1 }>;
/// 7-bit unsigned integer
pub type U7 = RangedU8<0, { u8_two_pow(7) - 1 }>;
/// 8-bit unsigned integer
pub type U8 = RangedU8<0, { u8::MAX }>;
/// 9-bit unsigned integer
pub type U9 = RangedU16<0, { u16_two_pow(9) - 1 }>;
/// 10-bit unsigned integer
pub type U10 = RangedU16<0, { u16_two_pow(10) - 1 }>;
/// 11-bit unsigned integer
pub type U11 = RangedU16<0, { u16_two_pow(11) - 1 }>;
/// 12-bit unsigned integer
pub type U12 = RangedU16<0, { u16_two_pow(12) - 1 }>;
/// 13-bit unsigned integer
pub type U13 = RangedU16<0, { u16_two_pow(13) - 1 }>;
/// 14-bit unsigned integer
pub type U14 = RangedU16<0, { u16_two_pow(14) - 1 }>;
/// 15-bit unsigned integer
pub type U15 = RangedU16<0, { u16_two_pow(15) - 1 }>;
/// 16-bit unsigned integer
pub type U16 = RangedU16<0, { u16::MAX }>;
/// 17-bit unsigned integer
pub type U17 = RangedU32<0, { u32_two_pow(17) - 1 }>;
/// 18-bit unsigned integer
pub type U18 = RangedU32<0, { u32_two_pow(18) - 1 }>;
/// 19-bit unsigned integer
pub type U19 = RangedU32<0, { u32_two_pow(19) - 1 }>;
/// 20-bit unsigned integer
pub type U20 = RangedU32<0, { u32_two_pow(20) - 1 }>;
/// 21-bit unsigned integer
pub type U21 = RangedU32<0, { u32_two_pow(21) - 1 }>;
/// 22-bit unsigned integer
pub type U22 = RangedU32<0, { u32_two_pow(22) - 1 }>;
/// 23-bit unsigned integer
pub type U23 = RangedU32<0, { u32_two_pow(23) - 1 }>;
/// 24-bit unsigned integer
pub type U24 = RangedU32<0, { u32_two_pow(24) - 1 }>;
/// 25-bit unsigned integer
pub type U25 = RangedU32<0, { u32_two_pow(25) - 1 }>;
/// 26-bit unsigned integer
pub type U26 = RangedU32<0, { u32_two_pow(26) - 1 }>;
/// 27-bit unsigned integer
pub type U27 = RangedU32<0, { u32_two_pow(27) - 1 }>;
/// 28-bit unsigned integer
pub type U28 = RangedU32<0, { u32_two_pow(28) - 1 }>;
/// 29-bit unsigned integer
pub type U29 = RangedU32<0, { u32_two_pow(29) - 1 }>;
/// 30-bit unsigned integer
pub type U30 = RangedU32<0, { u32_two_pow(30) - 1 }>;
/// 31-bit unsigned integer
pub type U31 = RangedU32<0, { u32_two_pow(31) - 1 }>;
/// 32-bit unsigned integer
pub type U32 = RangedU32<0, { u32::MAX }>;
/// 33-bit unsigned integer
pub type U33 = RangedU64<0, { u64_two_pow(33) - 1 }>;
/// 34-bit unsigned integer
pub type U34 = RangedU64<0, { u64_two_pow(34) - 1 }>;
/// 35-bit unsigned integer
pub type U35 = RangedU64<0, { u64_two_pow(35) - 1 }>;
/// 36-bit unsigned integer
pub type U36 = RangedU64<0, { u64_two_pow(36) - 1 }>;
/// 37-bit unsigned integer
pub type U37 = RangedU64<0, { u64_two_pow(37) - 1 }>;
/// 38-bit unsigned integer
pub type U38 = RangedU64<0, { u64_two_pow(38) - 1 }>;
/// 39-bit unsigned integer
pub type U39 = RangedU64<0, { u64_two_pow(39) - 1 }>;
/// 40-bit unsigned integer
pub type U40 = RangedU64<0, { u64_two_pow(40) - 1 }>;
/// 41-bit unsigned integer
pub type U41 = RangedU64<0, { u64_two_pow(41) - 1 }>;
/// 42-bit unsigned integer
pub type U42 = RangedU64<0, { u64_two_pow(42) - 1 }>;
/// 43-bit unsigned integer
pub type U43 = RangedU64<0, { u64_two_pow(43) - 1 }>;
/// 44-bit unsigned integer
pub type U44 = RangedU64<0, { u64_two_pow(44) - 1 }>;
/// 45-bit unsigned integer
pub type U45 = RangedU64<0, { u64_two_pow(45) - 1 }>;
/// 46-bit unsigned integer
pub type U46 = RangedU64<0, { u64_two_pow(46) - 1 }>;
/// 47-bit unsigned integer
pub type U47 = RangedU64<0, { u64_two_pow(47) - 1 }>;
/// 48-bit unsigned integer
pub type U48 = RangedU64<0, { u64_two_pow(48) - 1 }>;
/// 49-bit unsigned integer
pub type U49 = RangedU64<0, { u64_two_pow(49) - 1 }>;
/// 50-bit unsigned integer
pub type U50 = RangedU64<0, { u64_two_pow(50) - 1 }>;
/// 51-bit unsigned integer
pub type U51 = RangedU64<0, { u64_two_pow(51) - 1 }>;
/// 52-bit unsigned integer
pub type U52 = RangedU64<0, { u64_two_pow(52) - 1 }>;
/// 53-bit unsigned integer
pub type U53 = RangedU64<0, { u64_two_pow(53) - 1 }>;
/// 54-bit unsigned integer
pub type U54 = RangedU64<0, { u64_two_pow(54) - 1 }>;
/// 55-bit unsigned integer
pub type U55 = RangedU64<0, { u64_two_pow(55) - 1 }>;
/// 56-bit unsigned integer
pub type U56 = RangedU64<0, { u64_two_pow(56) - 1 }>;
/// 57-bit unsigned integer
pub type U57 = RangedU64<0, { u64_two_pow(57) - 1 }>;
/// 58-bit unsigned integer
pub type U58 = RangedU64<0, { u64_two_pow(58) - 1 }>;
/// 59-bit unsigned integer
pub type U59 = RangedU64<0, { u64_two_pow(59) - 1 }>;
/// 60-bit unsigned integer
pub type U60 = RangedU64<0, { u64_two_pow(60) - 1 }>;
/// 61-bit unsigned integer
pub type U61 = RangedU64<0, { u64_two_pow(61) - 1 }>;
/// 62-bit unsigned integer
pub type U62 = RangedU64<0, { u64_two_pow(62) - 1 }>;
/// 63-bit unsigned integer
pub type U63 = RangedU64<0, { u64_two_pow(63) - 1 }>;
/// 64-bit unsigned integer
pub type U64 = RangedU64<0, { u64::MAX }>;
/// 65-bit unsigned integer
pub type U65 = RangedU128<0, { u128_two_pow(65) - 1 }>;
/// 66-bit unsigned integer
pub type U66 = RangedU128<0, { u128_two_pow(66) - 1 }>;
/// 67-bit unsigned integer
pub type U67 = RangedU128<0, { u128_two_pow(67) - 1 }>;
/// 68-bit unsigned integer
pub type U68 = RangedU128<0, { u128_two_pow(68) - 1 }>;
/// 69-bit unsigned integer
pub type U69 = RangedU128<0, { u128_two_pow(69) - 1 }>;
/// 70-bit unsigned integer
pub type U70 = RangedU128<0, { u128_two_pow(70) - 1 }>;
/// 71-bit unsigned integer
pub type U71 = RangedU128<0, { u128_two_pow(71) - 1 }>;
/// 72-bit unsigned integer
pub type U72 = RangedU128<0, { u128_two_pow(72) - 1 }>;
/// 73-bit unsigned integer
pub type U73 = RangedU128<0, { u128_two_pow(73) - 1 }>;
/// 74-bit unsigned integer
pub type U74 = RangedU128<0, { u128_two_pow(74) - 1 }>;
/// 75-bit unsigned integer
pub type U75 = RangedU128<0, { u128_two_pow(75) - 1 }>;
/// 76-bit unsigned integer
pub type U76 = RangedU128<0, { u128_two_pow(76) - 1 }>;
/// 77-bit unsigned integer
pub type U77 = RangedU128<0, { u128_two_pow(77) - 1 }>;
/// 78-bit unsigned integer
pub type U78 = RangedU128<0, { u128_two_pow(78) - 1 }>;
/// 79-bit unsigned integer
pub type U79 = RangedU128<0, { u128_two_pow(79) - 1 }>;
/// 80-bit unsigned integer
pub type U80 = RangedU128<0, { u128_two_pow(80) - 1 }>;
/// 81-bit unsigned integer
pub type U81 = RangedU128<0, { u128_two_pow(81) - 1 }>;
/// 82-bit unsigned integer
pub type U82 = RangedU128<0, { u128_two_pow(82) - 1 }>;
/// 83-bit unsigned integer
pub type U83 = RangedU128<0, { u128_two_pow(83) - 1 }>;
/// 84-bit unsigned integer
pub type U84 = RangedU128<0, { u128_two_pow(84) - 1 }>;
/// 85-bit unsigned integer
pub type U85 = RangedU128<0, { u128_two_pow(85) - 1 }>;
/// 86-bit unsigned integer
pub type U86 = RangedU128<0, { u128_two_pow(86) - 1 }>;
/// 87-bit unsigned integer
pub type U87 = RangedU128<0, { u128_two_pow(87) - 1 }>;
/// 88-bit unsigned integer
pub type U88 = RangedU128<0, { u128_two_pow(88) - 1 }>;
/// 89-bit unsigned integer
pub type U89 = RangedU128<0, { u128_two_pow(89) - 1 }>;
/// 90-bit unsigned integer
pub type U90 = RangedU128<0, { u128_two_pow(90) - 1 }>;
/// 91-bit unsigned integer
pub type U91 = RangedU128<0, { u128_two_pow(91) - 1 }>;
/// 92-bit unsigned integer
pub type U92 = RangedU128<0, { u128_two_pow(92) - 1 }>;
/// 93-bit unsigned integer
pub type U93 = RangedU128<0, { u128_two_pow(93) - 1 }>;
/// 94-bit unsigned integer
pub type U94 = RangedU128<0, { u128_two_pow(94) - 1 }>;
/// 95-bit unsigned integer
pub type U95 = RangedU128<0, { u128_two_pow(95) - 1 }>;
/// 96-bit unsigned integer
pub type U96 = RangedU128<0, { u128_two_pow(96) - 1 }>;
/// 97-bit unsigned integer
pub type U97 = RangedU128<0, { u128_two_pow(97) - 1 }>;
/// 98-bit unsigned integer
pub type U98 = RangedU128<0, { u128_two_pow(98) - 1 }>;
/// 99-bit unsigned integer
pub type U99 = RangedU128<0, { u128_two_pow(99) - 1 }>;
/// 100-bit unsigned integer
pub type U100 = RangedU128<0, { u128_two_pow(100) - 1 }>;
/// 101-bit unsigned integer
pub type U101 = RangedU128<0, { u128_two_pow(101) - 1 }>;
/// 102-bit unsigned integer
pub type U102 = RangedU128<0, { u128_two_pow(102) - 1 }>;
/// 103-bit unsigned integer
pub type U103 = RangedU128<0, { u128_two_pow(103) - 1 }>;
/// 104-bit unsigned integer
pub type U104 = RangedU128<0, { u128_two_pow(104) - 1 }>;
/// 105-bit unsigned integer
pub type U105 = RangedU128<0, { u128_two_pow(105) - 1 }>;
/// 106-bit unsigned integer
pub type U106 = RangedU128<0, { u128_two_pow(106) - 1 }>;
/// 107-bit unsigned integer
pub type U107 = RangedU128<0, { u128_two_pow(107) - 1 }>;
/// 108-bit unsigned integer
pub type U108 = RangedU128<0, { u128_two_pow(108) - 1 }>;
/// 109-bit unsigned integer
pub type U109 = RangedU128<0, { u128_two_pow(109) - 1 }>;
/// 110-bit unsigned integer
pub type U110 = RangedU128<0, { u128_two_pow(110) - 1 }>;
/// 111-bit unsigned integer
pub type U111 = RangedU128<0, { u128_two_pow(111) - 1 }>;
/// 112-bit unsigned integer
pub type U112 = RangedU128<0, { u128_two_pow(112) - 1 }>;
/// 113-bit unsigned integer
pub type U113 = RangedU128<0, { u128_two_pow(113) - 1 }>;
/// 114-bit unsigned integer
pub type U114 = RangedU128<0, { u128_two_pow(114) - 1 }>;
/// 115-bit unsigned integer
pub type U115 = RangedU128<0, { u128_two_pow(115) - 1 }>;
/// 116-bit unsigned integer
pub type U116 = RangedU128<0, { u128_two_pow(116) - 1 }>;
/// 117-bit unsigned integer
pub type U117 = RangedU128<0, { u128_two_pow(117) - 1 }>;
/// 118-bit unsigned integer
pub type U118 = RangedU128<0, { u128_two_pow(118) - 1 }>;
/// 119-bit unsigned integer
pub type U119 = RangedU128<0, { u128_two_pow(119) - 1 }>;
/// 120-bit unsigned integer
pub type U120 = RangedU128<0, { u128_two_pow(120) - 1 }>;
/// 121-bit unsigned integer
pub type U121 = RangedU128<0, { u128_two_pow(121) - 1 }>;
/// 122-bit unsigned integer
pub type U122 = RangedU128<0, { u128_two_pow(122) - 1 }>;
/// 123-bit unsigned integer
pub type U123 = RangedU128<0, { u128_two_pow(123) - 1 }>;
/// 124-bit unsigned integer
pub type U124 = RangedU128<0, { u128_two_pow(124) - 1 }>;
/// 125-bit unsigned integer
pub type U125 = RangedU128<0, { u128_two_pow(125) - 1 }>;
/// 126-bit unsigned integer
pub type U126 = RangedU128<0, { u128_two_pow(126) - 1 }>;
/// 127-bit unsigned integer
pub type U127 = RangedU128<0, { u128_two_pow(127) - 1 }>;
/// 128-bit unsigned integer
pub type U128 = RangedU128<0, { u128::MAX }>;

/// 1-bit signed integer
pub type I1 = RangedI8<{ -i8_two_pow(0) }, { i8_two_pow(0) - 1 }>;
/// 2-bit signed integer
pub type I2 = RangedI8<{ -i8_two_pow(1) }, { i8_two_pow(1) - 1 }>;
/// 3-bit signed integer
pub type I3 = RangedI8<{ -i8_two_pow(2) }, { i8_two_pow(2) - 1 }>;
/// 4-bit signed integer
pub type I4 = RangedI8<{ -i8_two_pow(3) }, { i8_two_pow(3) - 1 }>;
/// 5-bit signed integer
pub type I5 = RangedI8<{ -i8_two_pow(4) }, { i8_two_pow(4) - 1 }>;
/// 6-bit signed integer
pub type I6 = RangedI8<{ -i8_two_pow(5) }, { i8_two_pow(5) - 1 }>;
/// 7-bit signed integer
pub type I7 = RangedI8<{ -i8_two_pow(6) }, { i8_two_pow(6) - 1 }>;
/// 8-bit signed integer
pub type I8 = RangedI8<{ i8::MIN }, { i8::MAX }>;
/// 9-bit signed integer
pub type I9 = RangedI16<{ -i16_two_pow(8) }, { i16_two_pow(8) - 1 }>;
/// 10-bit signed integer
pub type I10 = RangedI16<{ -i16_two_pow(9) }, { i16_two_pow(9) - 1 }>;
/// 11-bit signed integer
pub type I11 = RangedI16<{ -i16_two_pow(10) }, { i16_two_pow(10) - 1 }>;
/// 12-bit signed integer
pub type I12 = RangedI16<{ -i16_two_pow(11) }, { i16_two_pow(11) - 1 }>;
/// 13-bit signed integer
pub type I13 = RangedI16<{ -i16_two_pow(12) }, { i16_two_pow(12) - 1 }>;
/// 14-bit signed integer
pub type I14 = RangedI16<{ -i16_two_pow(13) }, { i16_two_pow(13) - 1 }>;
/// 15-bit signed integer
pub type I15 = RangedI16<{ -i16_two_pow(14) }, { i16_two_pow(14) - 1 }>;
/// 16-bit signed integer
pub type I16 = RangedI16<{ i16::MIN }, { i16::MAX }>;
/// 17-bit signed integer
pub type I17 = RangedI32<{ -i32_two_pow(16) }, { i32_two_pow(16) - 1 }>;
/// 18-bit signed integer
pub type I18 = RangedI32<{ -i32_two_pow(17) }, { i32_two_pow(17) - 1 }>;
/// 19-bit signed integer
pub type I19 = RangedI32<{ -i32_two_pow(18) }, { i32_two_pow(18) - 1 }>;
/// 20-bit signed integer
pub type I20 = RangedI32<{ -i32_two_pow(19) }, { i32_two_pow(19) - 1 }>;
/// 21-bit signed integer
pub type I21 = RangedI32<{ -i32_two_pow(20) }, { i32_two_pow(20) - 1 }>;
/// 22-bit signed integer
pub type I22 = RangedI32<{ -i32_two_pow(21) }, { i32_two_pow(21) - 1 }>;
/// 23-bit signed integer
pub type I23 = RangedI32<{ -i32_two_pow(22) }, { i32_two_pow(22) - 1 }>;
/// 24-bit signed integer
pub type I24 = RangedI32<{ -i32_two_pow(23) }, { i32_two_pow(23) - 1 }>;
/// 25-bit signed integer
pub type I25 = RangedI32<{ -i32_two_pow(24) }, { i32_two_pow(24) - 1 }>;
/// 26-bit signed integer
pub type I26 = RangedI32<{ -i32_two_pow(25) }, { i32_two_pow(25) - 1 }>;
/// 27-bit signed integer
pub type I27 = RangedI32<{ -i32_two_pow(26) }, { i32_two_pow(26) - 1 }>;
/// 28-bit signed integer
pub type I28 = RangedI32<{ -i32_two_pow(27) }, { i32_two_pow(27) - 1 }>;
/// 29-bit signed integer
pub type I29 = RangedI32<{ -i32_two_pow(28) }, { i32_two_pow(28) - 1 }>;
/// 30-bit signed integer
pub type I30 = RangedI32<{ -i32_two_pow(29) }, { i32_two_pow(29) - 1 }>;
/// 31-bit signed integer
pub type I31 = RangedI32<{ -i32_two_pow(30) }, { i32_two_pow(30) - 1 }>;
/// 32-bit signed integer
pub type I32 = RangedI32<{ i32::MIN }, { i32::MAX }>;
/// 33-bit signed integer
pub type I33 = RangedI64<{ -i64_two_pow(32) }, { i64_two_pow(32) - 1 }>;
/// 34-bit signed integer
pub type I34 = RangedI64<{ -i64_two_pow(33) }, { i64_two_pow(33) - 1 }>;
/// 35-bit signed integer
pub type I35 = RangedI64<{ -i64_two_pow(34) }, { i64_two_pow(34) - 1 }>;
/// 36-bit signed integer
pub type I36 = RangedI64<{ -i64_two_pow(35) }, { i64_two_pow(35) - 1 }>;
/// 37-bit signed integer
pub type I37 = RangedI64<{ -i64_two_pow(36) }, { i64_two_pow(36) - 1 }>;
/// 38-bit signed integer
pub type I38 = RangedI64<{ -i64_two_pow(37) }, { i64_two_pow(37) - 1 }>;
/// 39-bit signed integer
pub type I39 = RangedI64<{ -i64_two_pow(38) }, { i64_two_pow(38) - 1 }>;
/// 40-bit signed integer
pub type I40 = RangedI64<{ -i64_two_pow(39) }, { i64_two_pow(39) - 1 }>;
/// 41-bit signed integer
pub type I41 = RangedI64<{ -i64_two_pow(40) }, { i64_two_pow(40) - 1 }>;
/// 42-bit signed integer
pub type I42 = RangedI64<{ -i64_two_pow(41) }, { i64_two_pow(41) - 1 }>;
/// 43-bit signed integer
pub type I43 = RangedI64<{ -i64_two_pow(42) }, { i64_two_pow(42) - 1 }>;
/// 44-bit signed integer
pub type I44 = RangedI64<{ -i64_two_pow(43) }, { i64_two_pow(43) - 1 }>;
/// 45-bit signed integer
pub type I45 = RangedI64<{ -i64_two_pow(44) }, { i64_two_pow(44) - 1 }>;
/// 46-bit signed integer
pub type I46 = RangedI64<{ -i64_two_pow(45) }, { i64_two_pow(45) - 1 }>;
/// 47-bit signed integer
pub type I47 = RangedI64<{ -i64_two_pow(46) }, { i64_two_pow(46) - 1 }>;
/// 48-bit signed integer
pub type I48 = RangedI64<{ -i64_two_pow(47) }, { i64_two_pow(47) - 1 }>;
/// 49-bit signed integer
pub type I49 = RangedI64<{ -i64_two_pow(48) }, { i64_two_pow(48) - 1 }>;
/// 50-bit signed integer
pub type I50 = RangedI64<{ -i64_two_pow(49) }, { i64_two_pow(49) - 1 }>;
/// 51-bit signed integer
pub type I51 = RangedI64<{ -i64_two_pow(50) }, { i64_two_pow(50) - 1 }>;
/// 52-bit signed integer
pub type I52 = RangedI64<{ -i64_two_pow(51) }, { i64_two_pow(51) - 1 }>;
/// 53-bit signed integer
pub type I53 = RangedI64<{ -i64_two_pow(52) }, { i64_two_pow(52) - 1 }>;
/// 54-bit signed integer
pub type I54 = RangedI64<{ -i64_two_pow(53) }, { i64_two_pow(53) - 1 }>;
/// 55-bit signed integer
pub type I55 = RangedI64<{ -i64_two_pow(54) }, { i64_two_pow(54) - 1 }>;
/// 56-bit signed integer
pub type I56 = RangedI64<{ -i64_two_pow(55) }, { i64_two_pow(55) - 1 }>;
/// 57-bit signed integer
pub type I57 = RangedI64<{ -i64_two_pow(56) }, { i64_two_pow(56) - 1 }>;
/// 58-bit signed integer
pub type I58 = RangedI64<{ -i64_two_pow(57) }, { i64_two_pow(57) - 1 }>;
/// 59-bit signed integer
pub type I59 = RangedI64<{ -i64_two_pow(58) }, { i64_two_pow(58) - 1 }>;
/// 60-bit signed integer
pub type I60 = RangedI64<{ -i64_two_pow(59) }, { i64_two_pow(59) - 1 }>;
/// 61-bit signed integer
pub type I61 = RangedI64<{ -i64_two_pow(60) }, { i64_two_pow(60) - 1 }>;
/// 62-bit signed integer
pub type I62 = RangedI64<{ -i64_two_pow(61) }, { i64_two_pow(61) - 1 }>;
/// 63-bit signed integer
pub type I63 = RangedI64<{ -i64_two_pow(62) }, { i64_two_pow(62) - 1 }>;
/// 64-bit signed integer
pub type I64 = RangedI64<{ i64::MIN }, { i64::MAX }>;
/// 65-bit signed integer
pub type I65 = RangedI128<{ -i128_two_pow(64) }, { i128_two_pow(64) - 1 }>;
/// 66-bit signed integer
pub type I66 = RangedI128<{ -i128_two_pow(65) }, { i128_two_pow(65) - 1 }>;
/// 67-bit signed integer
pub type I67 = RangedI128<{ -i128_two_pow(66) }, { i128_two_pow(66) - 1 }>;
/// 68-bit signed integer
pub type I68 = RangedI128<{ -i128_two_pow(67) }, { i128_two_pow(67) - 1 }>;
/// 69-bit signed integer
pub type I69 = RangedI128<{ -i128_two_pow(68) }, { i128_two_pow(68) - 1 }>;
/// 70-bit signed integer
pub type I70 = RangedI128<{ -i128_two_pow(69) }, { i128_two_pow(69) - 1 }>;
/// 71-bit signed integer
pub type I71 = RangedI128<{ -i128_two_pow(70) }, { i128_two_pow(70) - 1 }>;
/// 72-bit signed integer
pub type I72 = RangedI128<{ -i128_two_pow(71) }, { i128_two_pow(71) - 1 }>;
/// 73-bit signed integer
pub type I73 = RangedI128<{ -i128_two_pow(72) }, { i128_two_pow(72) - 1 }>;
/// 74-bit signed integer
pub type I74 = RangedI128<{ -i128_two_pow(73) }, { i128_two_pow(73) - 1 }>;
/// 75-bit signed integer
pub type I75 = RangedI128<{ -i128_two_pow(74) }, { i128_two_pow(74) - 1 }>;
/// 76-bit signed integer
pub type I76 = RangedI128<{ -i128_two_pow(75) }, { i128_two_pow(75) - 1 }>;
/// 77-bit signed integer
pub type I77 = RangedI128<{ -i128_two_pow(76) }, { i128_two_pow(76) - 1 }>;
/// 78-bit signed integer
pub type I78 = RangedI128<{ -i128_two_pow(77) }, { i128_two_pow(77) - 1 }>;
/// 79-bit signed integer
pub type I79 = RangedI128<{ -i128_two_pow(78) }, { i128_two_pow(78) - 1 }>;
/// 80-bit signed integer
pub type I80 = RangedI128<{ -i128_two_pow(79) }, { i128_two_pow(79) - 1 }>;
/// 81-bit signed integer
pub type I81 = RangedI128<{ -i128_two_pow(80) }, { i128_two_pow(80) - 1 }>;
/// 82-bit signed integer
pub type I82 = RangedI128<{ -i128_two_pow(81) }, { i128_two_pow(81) - 1 }>;
/// 83-bit signed integer
pub type I83 = RangedI128<{ -i128_two_pow(82) }, { i128_two_pow(82) - 1 }>;
/// 84-bit signed integer
pub type I84 = RangedI128<{ -i128_two_pow(83) }, { i128_two_pow(83) - 1 }>;
/// 85-bit signed integer
pub type I85 = RangedI128<{ -i128_two_pow(84) }, { i128_two_pow(84) - 1 }>;
/// 86-bit signed integer
pub type I86 = RangedI128<{ -i128_two_pow(85) }, { i128_two_pow(85) - 1 }>;
/// 87-bit signed integer
pub type I87 = RangedI128<{ -i128_two_pow(86) }, { i128_two_pow(86) - 1 }>;
/// 88-bit signed integer
pub type I88 = RangedI128<{ -i128_two_pow(87) }, { i128_two_pow(87) - 1 }>;
/// 89-bit signed integer
pub type I89 = RangedI128<{ -i128_two_pow(88) }, { i128_two_pow(88) - 1 }>;
/// 90-bit signed integer
pub type I90 = RangedI128<{ -i128_two_pow(89) }, { i128_two_pow(89) - 1 }>;
/// 91-bit signed integer
pub type I91 = RangedI128<{ -i128_two_pow(90) }, { i128_two_pow(90) - 1 }>;
/// 92-bit signed integer
pub type I92 = RangedI128<{ -i128_two_pow(91) }, { i128_two_pow(91) - 1 }>;
/// 93-bit signed integer
pub type I93 = RangedI128<{ -i128_two_pow(92) }, { i128_two_pow(92) - 1 }>;
/// 94-bit signed integer
pub type I94 = RangedI128<{ -i128_two_pow(93) }, { i128_two_pow(93) - 1 }>;
/// 95-bit signed integer
pub type I95 = RangedI128<{ -i128_two_pow(94) }, { i128_two_pow(94) - 1 }>;
/// 96-bit signed integer
pub type I96 = RangedI128<{ -i128_two_pow(95) }, { i128_two_pow(95) - 1 }>;
/// 97-bit signed integer
pub type I97 = RangedI128<{ -i128_two_pow(96) }, { i128_two_pow(96) - 1 }>;
/// 98-bit signed integer
pub type I98 = RangedI128<{ -i128_two_pow(97) }, { i128_two_pow(97) - 1 }>;
/// 99-bit signed integer
pub type I99 = RangedI128<{ -i128_two_pow(98) }, { i128_two_pow(98) - 1 }>;
/// 100-bit signed integer
pub type I100 = RangedI128<{ -i128_two_pow(99) }, { i128_two_pow(99) - 1 }>;
/// 101-bit signed integer
pub type I101 = RangedI128<{ -i128_two_pow(100) }, { i128_two_pow(100) - 1 }>;
/// 102-bit signed integer
pub type I102 = RangedI128<{ -i128_two_pow(101) }, { i128_two_pow(101) - 1 }>;
/// 103-bit signed integer
pub type I103 = RangedI128<{ -i128_two_pow(102) }, { i128_two_pow(102) - 1 }>;
/// 104-bit signed integer
pub type I104 = RangedI128<{ -i128_two_pow(103) }, { i128_two_pow(103) - 1 }>;
/// 105-bit signed integer
pub type I105 = RangedI128<{ -i128_two_pow(104) }, { i128_two_pow(104) - 1 }>;
/// 106-bit signed integer
pub type I106 = RangedI128<{ -i128_two_pow(105) }, { i128_two_pow(105) - 1 }>;
/// 107-bit signed integer
pub type I107 = RangedI128<{ -i128_two_pow(106) }, { i128_two_pow(106) - 1 }>;
/// 108-bit signed integer
pub type I108 = RangedI128<{ -i128_two_pow(107) }, { i128_two_pow(107) - 1 }>;
/// 109-bit signed integer
pub type I109 = RangedI128<{ -i128_two_pow(108) }, { i128_two_pow(108) - 1 }>;
/// 110-bit signed integer
pub type I110 = RangedI128<{ -i128_two_pow(109) }, { i128_two_pow(109) - 1 }>;
/// 111-bit signed integer
pub type I111 = RangedI128<{ -i128_two_pow(110) }, { i128_two_pow(110) - 1 }>;
/// 112-bit signed integer
pub type I112 = RangedI128<{ -i128_two_pow(111) }, { i128_two_pow(111) - 1 }>;
/// 113-bit signed integer
pub type I113 = RangedI128<{ -i128_two_pow(112) }, { i128_two_pow(112) - 1 }>;
/// 114-bit signed integer
pub type I114 = RangedI128<{ -i128_two_pow(113) }, { i128_two_pow(113) - 1 }>;
/// 115-bit signed integer
pub type I115 = RangedI128<{ -i128_two_pow(114) }, { i128_two_pow(114) - 1 }>;
/// 116-bit signed integer
pub type I116 = RangedI128<{ -i128_two_pow(115) }, { i128_two_pow(115) - 1 }>;
/// 117-bit signed integer
pub type I117 = RangedI128<{ -i128_two_pow(116) }, { i128_two_pow(116) - 1 }>;
/// 118-bit signed integer
pub type I118 = RangedI128<{ -i128_two_pow(117) }, { i128_two_pow(117) - 1 }>;
/// 119-bit signed integer
pub type I119 = RangedI128<{ -i128_two_pow(118) }, { i128_two_pow(118) - 1 }>;
/// 120-bit signed integer
pub type I120 = RangedI128<{ -i128_two_pow(119) }, { i128_two_pow(119) - 1 }>;
/// 121-bit signed integer
pub type I121 = RangedI128<{ -i128_two_pow(120) }, { i128_two_pow(120) - 1 }>;
/// 122-bit signed integer
pub type I122 = RangedI128<{ -i128_two_pow(121) }, { i128_two_pow(121) - 1 }>;
/// 123-bit signed integer
pub type I123 = RangedI128<{ -i128_two_pow(122) }, { i128_two_pow(122) - 1 }>;
/// 124-bit signed integer
pub type I124 = RangedI128<{ -i128_two_pow(123) }, { i128_two_pow(123) - 1 }>;
/// 125-bit signed integer
pub type I125 = RangedI128<{ -i128_two_pow(124) }, { i128_two_pow(124) - 1 }>;
/// 126-bit signed integer
pub type I126 = RangedI128<{ -i128_two_pow(125) }, { i128_two_pow(125) - 1 }>;
/// 127-bit signed integer
pub type I127 = RangedI128<{ -i128_two_pow(126) }, { i128_two_pow(126) - 1 }>;
/// 128-bit signed integer
pub type I128 = RangedI128<{ i128::MIN }, { i128::MAX }>;

macro_rules! from_primitive {
    ($ranged:ty, $primitive:ty, $name:ident, $with:ident) => {
        impl $ranged {
            /// Create a new ranged integer.
            ///
            /// Uses the full range, so it will never be out of bounds.
            ///
            /// # See Also
            ///
            ///  - [`Self::new()`] - compile-time bounds checking constructor
            #[doc = concat!(" - [`Self::", stringify!($with), "()`] - run-time bounds checking constructor")]
            ///
            /// ```rust
            #[doc = concat!("# use ranch::bitwise::", stringify!($ranged), ";")]
            #[doc = concat!(
                "assert_eq!(",
                stringify!($ranged),
                "::",
                stringify!($name),
                "(42).get(), 42);",
            )]
            /// ```
            #[must_use]
            pub fn $name(value: $primitive) -> Self {
                Self(value)
            }
        }
    }
}

from_primitive!(U8, u8, from_u8, with_u8);
from_primitive!(U16, u16, from_u16, with_u16);
from_primitive!(U32, u32, from_u32, with_u32);
from_primitive!(U64, u64, from_u64, with_u64);
from_primitive!(U128, u128, from_u128, with_u128);

from_primitive!(I8, i8, from_i8, with_i8);
from_primitive!(I16, i16, from_i16, with_i16);
from_primitive!(I32, i32, from_i32, with_i32);
from_primitive!(I64, i64, from_i64, with_i64);
from_primitive!(I128, i128, from_i128, with_i128);

macro_rules! from_nonzero {
    ($nonzero:ty, $primitive:ty) => { //, $ranged:ident
        impl $nonzero {
            /// Convert from [`NonZero`].
            ///
            /// ```rust
            /// # use std::num::NonZero;
            #[doc = concat!("# use ranch::bitwise::", stringify!($nonzero), ";")]
            /// assert_eq!(
            #[doc = concat!("    ", stringify!($nonzero), "::from_nonzero(NonZero::new(42).unwrap()),")]
            #[doc = concat!("    ", stringify!($nonzero), "::new::<42>(),")]
            /// );
            /// ```
            pub const fn from_nonzero(nonzero: NonZero<$primitive>) -> Self {
                Self(nonzero)
            }
        }
    }
}

from_nonzero!(NonZeroU8, u8);
from_nonzero!(NonZeroU16, u16);
from_nonzero!(NonZeroU32, u32);
from_nonzero!(NonZeroU64, u64);
from_nonzero!(NonZeroU128, u128);

from_nonzero!(NonZeroI8, i8);
from_nonzero!(NonZeroI16, i16);
from_nonzero!(NonZeroI32, i32);
from_nonzero!(NonZeroI64, i64);
from_nonzero!(NonZeroI128, i128);

const fn u8_two_pow(power: u32) -> u8 {
    1u8 << power
}

const fn u16_two_pow(power: u32) -> u16 {
    1u16 << power
}

const fn u32_two_pow(power: u32) -> u32 {
    1u32 << power
}

const fn u64_two_pow(power: u32) -> u64 {
    1u64 << power
}

const fn u128_two_pow(power: u32) -> u128 {
    1u128 << power
}

const fn i8_two_pow(power: u32) -> i8 {
    1i8 << power
}

const fn i16_two_pow(power: u32) -> i16 {
    1i16 << power
}

const fn i32_two_pow(power: u32) -> i32 {
    1i32 << power
}

const fn i64_two_pow(power: u32) -> i64 {
    1i64 << power
}

const fn i128_two_pow(power: u32) -> i128 {
    1i128 << power
}
