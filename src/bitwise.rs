//! Aliases for bitwise ranges (integers with arbitrary bit widths)

use crate::*;

/// 1-bit unsigned integer
pub type U1 = RangedU8<0, { 2u8.pow(1) - 1 }>;
/// 2-bit unsigned integer
pub type U2 = RangedU8<0, { 2u8.pow(2) - 1 }>;
/// 3-bit unsigned integer
pub type U3 = RangedU8<0, { 2u8.pow(3) - 1 }>;
/// 4-bit unsigned integer
pub type U4 = RangedU8<0, { 2u8.pow(4) - 1 }>;
/// 5-bit unsigned integer
pub type U5 = RangedU8<0, { 2u8.pow(5) - 1 }>;
/// 6-bit unsigned integer
pub type U6 = RangedU8<0, { 2u8.pow(6) - 1 }>;
/// 7-bit unsigned integer
pub type U7 = RangedU8<0, { 2u8.pow(7) - 1 }>;
/// 8-bit unsigned integer
pub type U8 = RangedU8<0, { u8::MAX }>;
/// 9-bit unsigned integer
pub type U9 = RangedU16<0, { 2u16.pow(9) - 1 }>;
/// 10-bit unsigned integer
pub type U10 = RangedU16<0, { 2u16.pow(10) - 1 }>;
/// 11-bit unsigned integer
pub type U11 = RangedU16<0, { 2u16.pow(11) - 1 }>;
/// 12-bit unsigned integer
pub type U12 = RangedU16<0, { 2u16.pow(12) - 1 }>;
/// 13-bit unsigned integer
pub type U13 = RangedU16<0, { 2u16.pow(13) - 1 }>;
/// 14-bit unsigned integer
pub type U14 = RangedU16<0, { 2u16.pow(14) - 1 }>;
/// 15-bit unsigned integer
pub type U15 = RangedU16<0, { 2u16.pow(15) - 1 }>;
/// 16-bit unsigned integer
pub type U16 = RangedU16<0, { u16::MAX }>;
/// 17-bit unsigned integer
pub type U17 = RangedU32<0, { 2u32.pow(17) - 1 }>;
/// 18-bit unsigned integer
pub type U18 = RangedU32<0, { 2u32.pow(18) - 1 }>;
/// 19-bit unsigned integer
pub type U19 = RangedU32<0, { 2u32.pow(19) - 1 }>;
/// 20-bit unsigned integer
pub type U20 = RangedU32<0, { 2u32.pow(20) - 1 }>;
/// 21-bit unsigned integer
pub type U21 = RangedU32<0, { 2u32.pow(21) - 1 }>;
/// 22-bit unsigned integer
pub type U22 = RangedU32<0, { 2u32.pow(22) - 1 }>;
/// 23-bit unsigned integer
pub type U23 = RangedU32<0, { 2u32.pow(23) - 1 }>;
/// 24-bit unsigned integer
pub type U24 = RangedU32<0, { 2u32.pow(24) - 1 }>;
/// 25-bit unsigned integer
pub type U25 = RangedU32<0, { 2u32.pow(25) - 1 }>;
/// 26-bit unsigned integer
pub type U26 = RangedU32<0, { 2u32.pow(26) - 1 }>;
/// 27-bit unsigned integer
pub type U27 = RangedU32<0, { 2u32.pow(27) - 1 }>;
/// 28-bit unsigned integer
pub type U28 = RangedU32<0, { 2u32.pow(28) - 1 }>;
/// 29-bit unsigned integer
pub type U29 = RangedU32<0, { 2u32.pow(29) - 1 }>;
/// 30-bit unsigned integer
pub type U30 = RangedU32<0, { 2u32.pow(30) - 1 }>;
/// 31-bit unsigned integer
pub type U31 = RangedU32<0, { 2u32.pow(31) - 1 }>;
/// 32-bit unsigned integer
pub type U32 = RangedU32<0, { u32::MAX }>;
/// 33-bit unsigned integer
pub type U33 = RangedU64<0, { 2u64.pow(33) - 1 }>;
/// 34-bit unsigned integer
pub type U34 = RangedU64<0, { 2u64.pow(34) - 1 }>;
/// 35-bit unsigned integer
pub type U35 = RangedU64<0, { 2u64.pow(35) - 1 }>;
/// 36-bit unsigned integer
pub type U36 = RangedU64<0, { 2u64.pow(36) - 1 }>;
/// 37-bit unsigned integer
pub type U37 = RangedU64<0, { 2u64.pow(37) - 1 }>;
/// 38-bit unsigned integer
pub type U38 = RangedU64<0, { 2u64.pow(38) - 1 }>;
/// 39-bit unsigned integer
pub type U39 = RangedU64<0, { 2u64.pow(39) - 1 }>;
/// 40-bit unsigned integer
pub type U40 = RangedU64<0, { 2u64.pow(40) - 1 }>;
/// 41-bit unsigned integer
pub type U41 = RangedU64<0, { 2u64.pow(41) - 1 }>;
/// 42-bit unsigned integer
pub type U42 = RangedU64<0, { 2u64.pow(42) - 1 }>;
/// 43-bit unsigned integer
pub type U43 = RangedU64<0, { 2u64.pow(43) - 1 }>;
/// 44-bit unsigned integer
pub type U44 = RangedU64<0, { 2u64.pow(44) - 1 }>;
/// 45-bit unsigned integer
pub type U45 = RangedU64<0, { 2u64.pow(45) - 1 }>;
/// 46-bit unsigned integer
pub type U46 = RangedU64<0, { 2u64.pow(46) - 1 }>;
/// 47-bit unsigned integer
pub type U47 = RangedU64<0, { 2u64.pow(47) - 1 }>;
/// 48-bit unsigned integer
pub type U48 = RangedU64<0, { 2u64.pow(48) - 1 }>;
/// 49-bit unsigned integer
pub type U49 = RangedU64<0, { 2u64.pow(49) - 1 }>;
/// 50-bit unsigned integer
pub type U50 = RangedU64<0, { 2u64.pow(50) - 1 }>;
/// 51-bit unsigned integer
pub type U51 = RangedU64<0, { 2u64.pow(51) - 1 }>;
/// 52-bit unsigned integer
pub type U52 = RangedU64<0, { 2u64.pow(52) - 1 }>;
/// 53-bit unsigned integer
pub type U53 = RangedU64<0, { 2u64.pow(53) - 1 }>;
/// 54-bit unsigned integer
pub type U54 = RangedU64<0, { 2u64.pow(54) - 1 }>;
/// 55-bit unsigned integer
pub type U55 = RangedU64<0, { 2u64.pow(55) - 1 }>;
/// 56-bit unsigned integer
pub type U56 = RangedU64<0, { 2u64.pow(56) - 1 }>;
/// 57-bit unsigned integer
pub type U57 = RangedU64<0, { 2u64.pow(57) - 1 }>;
/// 58-bit unsigned integer
pub type U58 = RangedU64<0, { 2u64.pow(58) - 1 }>;
/// 59-bit unsigned integer
pub type U59 = RangedU64<0, { 2u64.pow(59) - 1 }>;
/// 60-bit unsigned integer
pub type U60 = RangedU64<0, { 2u64.pow(60) - 1 }>;
/// 61-bit unsigned integer
pub type U61 = RangedU64<0, { 2u64.pow(61) - 1 }>;
/// 62-bit unsigned integer
pub type U62 = RangedU64<0, { 2u64.pow(62) - 1 }>;
/// 63-bit unsigned integer
pub type U63 = RangedU64<0, { 2u64.pow(63) - 1 }>;
/// 64-bit unsigned integer
pub type U64 = RangedU64<0, { u64::MAX }>;
/// 65-bit unsigned integer
pub type U65 = RangedU128<0, { 2u128.pow(65) - 1 }>;
/// 66-bit unsigned integer
pub type U66 = RangedU128<0, { 2u128.pow(66) - 1 }>;
/// 67-bit unsigned integer
pub type U67 = RangedU128<0, { 2u128.pow(67) - 1 }>;
/// 68-bit unsigned integer
pub type U68 = RangedU128<0, { 2u128.pow(68) - 1 }>;
/// 69-bit unsigned integer
pub type U69 = RangedU128<0, { 2u128.pow(69) - 1 }>;
/// 70-bit unsigned integer
pub type U70 = RangedU128<0, { 2u128.pow(70) - 1 }>;
/// 71-bit unsigned integer
pub type U71 = RangedU128<0, { 2u128.pow(71) - 1 }>;
/// 72-bit unsigned integer
pub type U72 = RangedU128<0, { 2u128.pow(72) - 1 }>;
/// 73-bit unsigned integer
pub type U73 = RangedU128<0, { 2u128.pow(73) - 1 }>;
/// 74-bit unsigned integer
pub type U74 = RangedU128<0, { 2u128.pow(74) - 1 }>;
/// 75-bit unsigned integer
pub type U75 = RangedU128<0, { 2u128.pow(75) - 1 }>;
/// 76-bit unsigned integer
pub type U76 = RangedU128<0, { 2u128.pow(76) - 1 }>;
/// 77-bit unsigned integer
pub type U77 = RangedU128<0, { 2u128.pow(77) - 1 }>;
/// 78-bit unsigned integer
pub type U78 = RangedU128<0, { 2u128.pow(78) - 1 }>;
/// 79-bit unsigned integer
pub type U79 = RangedU128<0, { 2u128.pow(79) - 1 }>;
/// 80-bit unsigned integer
pub type U80 = RangedU128<0, { 2u128.pow(80) - 1 }>;
/// 81-bit unsigned integer
pub type U81 = RangedU128<0, { 2u128.pow(81) - 1 }>;
/// 82-bit unsigned integer
pub type U82 = RangedU128<0, { 2u128.pow(82) - 1 }>;
/// 83-bit unsigned integer
pub type U83 = RangedU128<0, { 2u128.pow(83) - 1 }>;
/// 84-bit unsigned integer
pub type U84 = RangedU128<0, { 2u128.pow(84) - 1 }>;
/// 85-bit unsigned integer
pub type U85 = RangedU128<0, { 2u128.pow(85) - 1 }>;
/// 86-bit unsigned integer
pub type U86 = RangedU128<0, { 2u128.pow(86) - 1 }>;
/// 87-bit unsigned integer
pub type U87 = RangedU128<0, { 2u128.pow(87) - 1 }>;
/// 88-bit unsigned integer
pub type U88 = RangedU128<0, { 2u128.pow(88) - 1 }>;
/// 89-bit unsigned integer
pub type U89 = RangedU128<0, { 2u128.pow(89) - 1 }>;
/// 90-bit unsigned integer
pub type U90 = RangedU128<0, { 2u128.pow(90) - 1 }>;
/// 91-bit unsigned integer
pub type U91 = RangedU128<0, { 2u128.pow(91) - 1 }>;
/// 92-bit unsigned integer
pub type U92 = RangedU128<0, { 2u128.pow(92) - 1 }>;
/// 93-bit unsigned integer
pub type U93 = RangedU128<0, { 2u128.pow(93) - 1 }>;
/// 94-bit unsigned integer
pub type U94 = RangedU128<0, { 2u128.pow(94) - 1 }>;
/// 95-bit unsigned integer
pub type U95 = RangedU128<0, { 2u128.pow(95) - 1 }>;
/// 96-bit unsigned integer
pub type U96 = RangedU128<0, { 2u128.pow(96) - 1 }>;
/// 97-bit unsigned integer
pub type U97 = RangedU128<0, { 2u128.pow(97) - 1 }>;
/// 98-bit unsigned integer
pub type U98 = RangedU128<0, { 2u128.pow(98) - 1 }>;
/// 99-bit unsigned integer
pub type U99 = RangedU128<0, { 2u128.pow(99) - 1 }>;
/// 100-bit unsigned integer
pub type U100 = RangedU128<0, { 2u128.pow(100) - 1 }>;
/// 101-bit unsigned integer
pub type U101 = RangedU128<0, { 2u128.pow(101) - 1 }>;
/// 102-bit unsigned integer
pub type U102 = RangedU128<0, { 2u128.pow(102) - 1 }>;
/// 103-bit unsigned integer
pub type U103 = RangedU128<0, { 2u128.pow(103) - 1 }>;
/// 104-bit unsigned integer
pub type U104 = RangedU128<0, { 2u128.pow(104) - 1 }>;
/// 105-bit unsigned integer
pub type U105 = RangedU128<0, { 2u128.pow(105) - 1 }>;
/// 106-bit unsigned integer
pub type U106 = RangedU128<0, { 2u128.pow(106) - 1 }>;
/// 107-bit unsigned integer
pub type U107 = RangedU128<0, { 2u128.pow(107) - 1 }>;
/// 108-bit unsigned integer
pub type U108 = RangedU128<0, { 2u128.pow(108) - 1 }>;
/// 109-bit unsigned integer
pub type U109 = RangedU128<0, { 2u128.pow(109) - 1 }>;
/// 110-bit unsigned integer
pub type U110 = RangedU128<0, { 2u128.pow(110) - 1 }>;
/// 111-bit unsigned integer
pub type U111 = RangedU128<0, { 2u128.pow(111) - 1 }>;
/// 112-bit unsigned integer
pub type U112 = RangedU128<0, { 2u128.pow(112) - 1 }>;
/// 113-bit unsigned integer
pub type U113 = RangedU128<0, { 2u128.pow(113) - 1 }>;
/// 114-bit unsigned integer
pub type U114 = RangedU128<0, { 2u128.pow(114) - 1 }>;
/// 115-bit unsigned integer
pub type U115 = RangedU128<0, { 2u128.pow(115) - 1 }>;
/// 116-bit unsigned integer
pub type U116 = RangedU128<0, { 2u128.pow(116) - 1 }>;
/// 117-bit unsigned integer
pub type U117 = RangedU128<0, { 2u128.pow(117) - 1 }>;
/// 118-bit unsigned integer
pub type U118 = RangedU128<0, { 2u128.pow(118) - 1 }>;
/// 119-bit unsigned integer
pub type U119 = RangedU128<0, { 2u128.pow(119) - 1 }>;
/// 120-bit unsigned integer
pub type U120 = RangedU128<0, { 2u128.pow(120) - 1 }>;
/// 121-bit unsigned integer
pub type U121 = RangedU128<0, { 2u128.pow(121) - 1 }>;
/// 122-bit unsigned integer
pub type U122 = RangedU128<0, { 2u128.pow(122) - 1 }>;
/// 123-bit unsigned integer
pub type U123 = RangedU128<0, { 2u128.pow(123) - 1 }>;
/// 124-bit unsigned integer
pub type U124 = RangedU128<0, { 2u128.pow(124) - 1 }>;
/// 125-bit unsigned integer
pub type U125 = RangedU128<0, { 2u128.pow(125) - 1 }>;
/// 126-bit unsigned integer
pub type U126 = RangedU128<0, { 2u128.pow(126) - 1 }>;
/// 127-bit unsigned integer
pub type U127 = RangedU128<0, { 2u128.pow(127) - 1 }>;
/// 128-bit unsigned integer
pub type U128 = RangedU128<0, { u128::MAX }>;

/// 1-bit signed integer
pub type I1 = RangedI8<{ -2i8.pow(0) }, { 2i8.pow(0) - 1 }>;
/// 2-bit signed integer
pub type I2 = RangedI8<{ -2i8.pow(1) }, { 2i8.pow(1) - 1 }>;
/// 3-bit signed integer
pub type I3 = RangedI8<{ -2i8.pow(2) }, { 2i8.pow(2) - 1 }>;
/// 4-bit signed integer
pub type I4 = RangedI8<{ -2i8.pow(3) }, { 2i8.pow(3) - 1 }>;
/// 5-bit signed integer
pub type I5 = RangedI8<{ -2i8.pow(4) }, { 2i8.pow(4) - 1 }>;
/// 6-bit signed integer
pub type I6 = RangedI8<{ -2i8.pow(5) }, { 2i8.pow(5) - 1 }>;
/// 7-bit signed integer
pub type I7 = RangedI8<{ -2i8.pow(6) }, { 2i8.pow(6) - 1 }>;
/// 8-bit signed integer
pub type I8 = RangedI8<{ i8::MIN }, { i8::MAX }>;
/// 9-bit signed integer
pub type I9 = RangedI16<{ -2i16.pow(8) }, { 2i16.pow(8) - 1 }>;
/// 10-bit signed integer
pub type I10 = RangedI16<{ -2i16.pow(9) }, { 2i16.pow(9) - 1 }>;
/// 11-bit signed integer
pub type I11 = RangedI16<{ -2i16.pow(10) }, { 2i16.pow(10) - 1 }>;
/// 12-bit signed integer
pub type I12 = RangedI16<{ -2i16.pow(11) }, { 2i16.pow(11) - 1 }>;
/// 13-bit signed integer
pub type I13 = RangedI16<{ -2i16.pow(12) }, { 2i16.pow(12) - 1 }>;
/// 14-bit signed integer
pub type I14 = RangedI16<{ -2i16.pow(13) }, { 2i16.pow(13) - 1 }>;
/// 15-bit signed integer
pub type I15 = RangedI16<{ -2i16.pow(14) }, { 2i16.pow(14) - 1 }>;
/// 16-bit signed integer
pub type I16 = RangedI16<{ i16::MIN }, { i16::MAX }>;
/// 17-bit signed integer
pub type I17 = RangedI32<{ -2i32.pow(16) }, { 2i32.pow(16) - 1 }>;
/// 18-bit signed integer
pub type I18 = RangedI32<{ -2i32.pow(17) }, { 2i32.pow(17) - 1 }>;
/// 19-bit signed integer
pub type I19 = RangedI32<{ -2i32.pow(18) }, { 2i32.pow(18) - 1 }>;
/// 20-bit signed integer
pub type I20 = RangedI32<{ -2i32.pow(19) }, { 2i32.pow(19) - 1 }>;
/// 21-bit signed integer
pub type I21 = RangedI32<{ -2i32.pow(20) }, { 2i32.pow(20) - 1 }>;
/// 22-bit signed integer
pub type I22 = RangedI32<{ -2i32.pow(21) }, { 2i32.pow(21) - 1 }>;
/// 23-bit signed integer
pub type I23 = RangedI32<{ -2i32.pow(22) }, { 2i32.pow(22) - 1 }>;
/// 24-bit signed integer
pub type I24 = RangedI32<{ -2i32.pow(23) }, { 2i32.pow(23) - 1 }>;
/// 25-bit signed integer
pub type I25 = RangedI32<{ -2i32.pow(24) }, { 2i32.pow(24) - 1 }>;
/// 26-bit signed integer
pub type I26 = RangedI32<{ -2i32.pow(25) }, { 2i32.pow(25) - 1 }>;
/// 27-bit signed integer
pub type I27 = RangedI32<{ -2i32.pow(26) }, { 2i32.pow(26) - 1 }>;
/// 28-bit signed integer
pub type I28 = RangedI32<{ -2i32.pow(27) }, { 2i32.pow(27) - 1 }>;
/// 29-bit signed integer
pub type I29 = RangedI32<{ -2i32.pow(28) }, { 2i32.pow(28) - 1 }>;
/// 30-bit signed integer
pub type I30 = RangedI32<{ -2i32.pow(29) }, { 2i32.pow(29) - 1 }>;
/// 31-bit signed integer
pub type I31 = RangedI32<{ -2i32.pow(30) }, { 2i32.pow(30) - 1 }>;
/// 32-bit signed integer
pub type I32 = RangedI32<{ i32::MIN }, { i32::MAX }>;
/// 33-bit signed integer
pub type I33 = RangedI32<{ -2i32.pow(32) }, { 2i32.pow(32) - 1 }>;
/// 34-bit signed integer
pub type I34 = RangedI32<{ -2i32.pow(33) }, { 2i32.pow(33) - 1 }>;
/// 35-bit signed integer
pub type I35 = RangedI32<{ -2i32.pow(34) }, { 2i32.pow(34) - 1 }>;
/// 36-bit signed integer
pub type I36 = RangedI32<{ -2i32.pow(35) }, { 2i32.pow(35) - 1 }>;
/// 37-bit signed integer
pub type I37 = RangedI32<{ -2i32.pow(36) }, { 2i32.pow(36) - 1 }>;
/// 38-bit signed integer
pub type I38 = RangedI32<{ -2i32.pow(37) }, { 2i32.pow(37) - 1 }>;
/// 39-bit signed integer
pub type I39 = RangedI32<{ -2i32.pow(38) }, { 2i32.pow(38) - 1 }>;
/// 40-bit signed integer
pub type I40 = RangedI64<{ -2i64.pow(39) }, { 2i64.pow(39) - 1 }>;
/// 41-bit signed integer
pub type I41 = RangedI32<{ -2i32.pow(40) }, { 2i32.pow(40) - 1 }>;
/// 42-bit signed integer
pub type I42 = RangedI32<{ -2i32.pow(41) }, { 2i32.pow(41) - 1 }>;
/// 43-bit signed integer
pub type I43 = RangedI32<{ -2i32.pow(42) }, { 2i32.pow(42) - 1 }>;
/// 44-bit signed integer
pub type I44 = RangedI32<{ -2i32.pow(43) }, { 2i32.pow(43) - 1 }>;
/// 45-bit signed integer
pub type I45 = RangedI32<{ -2i32.pow(44) }, { 2i32.pow(44) - 1 }>;
/// 46-bit signed integer
pub type I46 = RangedI32<{ -2i32.pow(45) }, { 2i32.pow(45) - 1 }>;
/// 47-bit signed integer
pub type I47 = RangedI32<{ -2i32.pow(46) }, { 2i32.pow(46) - 1 }>;
/// 48-bit signed integer
pub type I48 = RangedI64<{ -2i64.pow(47) }, { 2i64.pow(47) - 1 }>;
/// 49-bit signed integer
pub type I49 = RangedI32<{ -2i32.pow(48) }, { 2i32.pow(48) - 1 }>;
/// 50-bit signed integer
pub type I50 = RangedI32<{ -2i32.pow(49) }, { 2i32.pow(49) - 1 }>;
/// 51-bit signed integer
pub type I51 = RangedI32<{ -2i32.pow(50) }, { 2i32.pow(50) - 1 }>;
/// 52-bit signed integer
pub type I52 = RangedI32<{ -2i32.pow(51) }, { 2i32.pow(51) - 1 }>;
/// 53-bit signed integer
pub type I53 = RangedI32<{ -2i32.pow(52) }, { 2i32.pow(52) - 1 }>;
/// 54-bit signed integer
pub type I54 = RangedI32<{ -2i32.pow(53) }, { 2i32.pow(53) - 1 }>;
/// 55-bit signed integer
pub type I55 = RangedI32<{ -2i32.pow(54) }, { 2i32.pow(54) - 1 }>;
/// 56-bit signed integer
pub type I56 = RangedI64<{ -2i64.pow(55) }, { 2i64.pow(55) - 1 }>;
/// 57-bit signed integer
pub type I57 = RangedI32<{ -2i32.pow(56) }, { 2i32.pow(56) - 1 }>;
/// 58-bit signed integer
pub type I58 = RangedI32<{ -2i32.pow(57) }, { 2i32.pow(57) - 1 }>;
/// 59-bit signed integer
pub type I59 = RangedI32<{ -2i32.pow(58) }, { 2i32.pow(58) - 1 }>;
/// 60-bit signed integer
pub type I60 = RangedI32<{ -2i32.pow(59) }, { 2i32.pow(59) - 1 }>;
/// 61-bit signed integer
pub type I61 = RangedI32<{ -2i32.pow(60) }, { 2i32.pow(60) - 1 }>;
/// 62-bit signed integer
pub type I62 = RangedI32<{ -2i32.pow(61) }, { 2i32.pow(61) - 1 }>;
/// 63-bit signed integer
pub type I63 = RangedI32<{ -2i32.pow(62) }, { 2i32.pow(62) - 1 }>;
/// 64-bit signed integer
pub type I64 = RangedI64<{ i64::MIN }, { i64::MAX }>;
/// 65-bit signed integer
pub type I65 = RangedI32<{ -2i32.pow(64) }, { 2i32.pow(64) - 1 }>;
/// 66-bit signed integer
pub type I66 = RangedI32<{ -2i32.pow(65) }, { 2i32.pow(65) - 1 }>;
/// 67-bit signed integer
pub type I67 = RangedI32<{ -2i32.pow(66) }, { 2i32.pow(66) - 1 }>;
/// 68-bit signed integer
pub type I68 = RangedI32<{ -2i32.pow(67) }, { 2i32.pow(67) - 1 }>;
/// 69-bit signed integer
pub type I69 = RangedI32<{ -2i32.pow(68) }, { 2i32.pow(68) - 1 }>;
/// 70-bit signed integer
pub type I70 = RangedI32<{ -2i32.pow(69) }, { 2i32.pow(69) - 1 }>;
/// 71-bit signed integer
pub type I71 = RangedI32<{ -2i32.pow(70) }, { 2i32.pow(70) - 1 }>;
/// 72-bit signed integer
pub type I72 = RangedI32<{ -2i32.pow(71) }, { 2i32.pow(71) - 1 }>;
/// 73-bit signed integer
pub type I73 = RangedI32<{ -2i32.pow(72) }, { 2i32.pow(72) - 1 }>;
/// 74-bit signed integer
pub type I74 = RangedI32<{ -2i32.pow(73) }, { 2i32.pow(73) - 1 }>;
/// 75-bit signed integer
pub type I75 = RangedI32<{ -2i32.pow(74) }, { 2i32.pow(74) - 1 }>;
/// 76-bit signed integer
pub type I76 = RangedI32<{ -2i32.pow(75) }, { 2i32.pow(75) - 1 }>;
/// 77-bit signed integer
pub type I77 = RangedI32<{ -2i32.pow(76) }, { 2i32.pow(76) - 1 }>;
/// 78-bit signed integer
pub type I78 = RangedI32<{ -2i32.pow(77) }, { 2i32.pow(77) - 1 }>;
/// 79-bit signed integer
pub type I79 = RangedI32<{ -2i32.pow(78) }, { 2i32.pow(78) - 1 }>;
/// 80-bit signed integer
pub type I80 = RangedI128<{ -2i128.pow(79) }, { 2i128.pow(79) - 1 }>;
/// 81-bit signed integer
pub type I81 = RangedI32<{ -2i32.pow(80) }, { 2i32.pow(80) - 1 }>;
/// 82-bit signed integer
pub type I82 = RangedI32<{ -2i32.pow(81) }, { 2i32.pow(81) - 1 }>;
/// 83-bit signed integer
pub type I83 = RangedI32<{ -2i32.pow(82) }, { 2i32.pow(82) - 1 }>;
/// 84-bit signed integer
pub type I84 = RangedI32<{ -2i32.pow(83) }, { 2i32.pow(83) - 1 }>;
/// 85-bit signed integer
pub type I85 = RangedI32<{ -2i32.pow(84) }, { 2i32.pow(84) - 1 }>;
/// 86-bit signed integer
pub type I86 = RangedI32<{ -2i32.pow(85) }, { 2i32.pow(85) - 1 }>;
/// 87-bit signed integer
pub type I87 = RangedI32<{ -2i32.pow(86) }, { 2i32.pow(86) - 1 }>;
/// 88-bit signed integer
pub type I88 = RangedI32<{ -2i32.pow(87) }, { 2i32.pow(87) - 1 }>;
/// 89-bit signed integer
pub type I89 = RangedI32<{ -2i32.pow(88) }, { 2i32.pow(88) - 1 }>;
/// 90-bit signed integer
pub type I90 = RangedI32<{ -2i32.pow(89) }, { 2i32.pow(89) - 1 }>;
/// 91-bit signed integer
pub type I91 = RangedI32<{ -2i32.pow(90) }, { 2i32.pow(90) - 1 }>;
/// 92-bit signed integer
pub type I92 = RangedI32<{ -2i32.pow(91) }, { 2i32.pow(91) - 1 }>;
/// 93-bit signed integer
pub type I93 = RangedI32<{ -2i32.pow(92) }, { 2i32.pow(92) - 1 }>;
/// 94-bit signed integer
pub type I94 = RangedI32<{ -2i32.pow(93) }, { 2i32.pow(93) - 1 }>;
/// 95-bit signed integer
pub type I95 = RangedI32<{ -2i32.pow(94) }, { 2i32.pow(94) - 1 }>;
/// 96-bit signed integer
pub type I96 = RangedI128<{ -2i128.pow(95) }, { 2i128.pow(95) - 1 }>;
/// 97-bit signed integer
pub type I97 = RangedI32<{ -2i32.pow(96) }, { 2i32.pow(96) - 1 }>;
/// 98-bit signed integer
pub type I98 = RangedI32<{ -2i32.pow(97) }, { 2i32.pow(97) - 1 }>;
/// 99-bit signed integer
pub type I99 = RangedI32<{ -2i32.pow(98) }, { 2i32.pow(98) - 1 }>;
/// 100-bit signed integer
pub type I100 = RangedI32<{ -2i32.pow(99) }, { 2i32.pow(99) - 1 }>;
/// 101-bit signed integer
pub type I101 = RangedI32<{ -2i32.pow(100) }, { 2i32.pow(100) - 1 }>;
/// 102-bit signed integer
pub type I102 = RangedI32<{ -2i32.pow(101) }, { 2i32.pow(101) - 1 }>;
/// 103-bit signed integer
pub type I103 = RangedI32<{ -2i32.pow(102) }, { 2i32.pow(102) - 1 }>;
/// 104-bit signed integer
pub type I104 = RangedI32<{ -2i32.pow(103) }, { 2i32.pow(103) - 1 }>;
/// 105-bit signed integer
pub type I105 = RangedI32<{ -2i32.pow(104) }, { 2i32.pow(104) - 1 }>;
/// 106-bit signed integer
pub type I106 = RangedI32<{ -2i32.pow(105) }, { 2i32.pow(105) - 1 }>;
/// 107-bit signed integer
pub type I107 = RangedI32<{ -2i32.pow(106) }, { 2i32.pow(106) - 1 }>;
/// 108-bit signed integer
pub type I108 = RangedI32<{ -2i32.pow(107) }, { 2i32.pow(107) - 1 }>;
/// 109-bit signed integer
pub type I109 = RangedI32<{ -2i32.pow(108) }, { 2i32.pow(108) - 1 }>;
/// 110-bit signed integer
pub type I110 = RangedI32<{ -2i32.pow(109) }, { 2i32.pow(109) - 1 }>;
/// 111-bit signed integer
pub type I111 = RangedI32<{ -2i32.pow(110) }, { 2i32.pow(110) - 1 }>;
/// 112-bit signed integer
pub type I112 = RangedI128<{ -2i128.pow(111) }, { 2i128.pow(111) - 1 }>;
/// 113-bit signed integer
pub type I113 = RangedI32<{ -2i32.pow(112) }, { 2i32.pow(112) - 1 }>;
/// 114-bit signed integer
pub type I114 = RangedI32<{ -2i32.pow(113) }, { 2i32.pow(113) - 1 }>;
/// 115-bit signed integer
pub type I115 = RangedI32<{ -2i32.pow(114) }, { 2i32.pow(114) - 1 }>;
/// 116-bit signed integer
pub type I116 = RangedI32<{ -2i32.pow(115) }, { 2i32.pow(115) - 1 }>;
/// 117-bit signed integer
pub type I117 = RangedI32<{ -2i32.pow(116) }, { 2i32.pow(116) - 1 }>;
/// 118-bit signed integer
pub type I118 = RangedI32<{ -2i32.pow(117) }, { 2i32.pow(117) - 1 }>;
/// 119-bit signed integer
pub type I119 = RangedI32<{ -2i32.pow(118) }, { 2i32.pow(118) - 1 }>;
/// 120-bit signed integer
pub type I120 = RangedI32<{ -2i32.pow(119) }, { 2i32.pow(119) - 1 }>;
/// 121-bit signed integer
pub type I121 = RangedI32<{ -2i32.pow(120) }, { 2i32.pow(120) - 1 }>;
/// 122-bit signed integer
pub type I122 = RangedI32<{ -2i32.pow(121) }, { 2i32.pow(121) - 1 }>;
/// 123-bit signed integer
pub type I123 = RangedI32<{ -2i32.pow(122) }, { 2i32.pow(122) - 1 }>;
/// 124-bit signed integer
pub type I124 = RangedI32<{ -2i32.pow(123) }, { 2i32.pow(123) - 1 }>;
/// 125-bit signed integer
pub type I125 = RangedI32<{ -2i32.pow(124) }, { 2i32.pow(124) - 1 }>;
/// 126-bit signed integer
pub type I126 = RangedI32<{ -2i32.pow(125) }, { 2i32.pow(125) - 1 }>;
/// 127-bit signed integer
pub type I127 = RangedI32<{ -2i32.pow(126) }, { 2i32.pow(126) - 1 }>;
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
