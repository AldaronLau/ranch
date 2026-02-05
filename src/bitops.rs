use core::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

use as_repr::AsRepr;

use crate::{bitwise::*, range::Range};

macro_rules! bitops {
    ($u:ty, $s:ty, $unsigned:ty, $signed:ty, $bits:literal) => {
        impl $signed {
            /// Bitwise NOT.
            ///
            /// ```rust
            /// # use ranch::bitwise::I12;
            /// assert_eq!(I12::new::<0>().bitnot(), I12::new::<-1>());
            /// assert_eq!(I12::new::<-1>().bitnot(), I12::new::<0>());
            /// assert_eq!(I12::new::<-2048>().bitnot(), I12::new::<2047>());
            /// assert_eq!(I12::new::<2047>().bitnot(), I12::new::<-2048>());
            /// ```
            pub const fn bitnot(self) -> Self {
                Self(!self.get()).clear_invalid_bits()
            }

            /// Bitwise AND.
            ///
            /// ```rust
            /// # use ranch::bitwise::I12;
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitand::<0b1110>(),
            ///     I12::new::<0b1010>(),
            /// );
            /// ```
            pub const fn bitand<const N: $s>(self) -> Self {
                self.bitand_ranged(Self::new::<N>())
            }

            /// Bitwise AND another ranged signed int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{I10, I12};
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitand_ranged(I10::new::<0b1110>()),
            ///     I12::new::<0b1010>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitand_ranged(I12::new::<0b1110>()),
            ///     I12::new::<0b1010>(),
            /// );
            /// ```
            pub const fn bitand_ranged<R>(self, ranged: R) -> Self
            where
                R: RangedSigned<$s>,
            {
                const {
                    if as_repr::as_repr(R::MAX) > Self::MAX.get() {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self(self.get() & as_repr::as_repr(ranged)).clear_invalid_bits()
            }

            /// Bitwise OR.
            ///
            /// ```rust
            /// # use ranch::bitwise::I12;
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitor::<0b1110>(),
            ///     I12::new::<0b1111>(),
            /// );
            /// ```
            pub const fn bitor<const N: $s>(self) -> Self {
                self.bitor_ranged(Self::new::<N>())
            }

            /// Bitwise OR another ranged signed int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{I10, I12};
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitor_ranged(I10::new::<0b1110>()),
            ///     I12::new::<0b1111>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitor_ranged(I12::new::<0b1110>()),
            ///     I12::new::<0b1111>(),
            /// );
            /// ```
            pub const fn bitor_ranged<R>(self, ranged: R) -> Self
            where
                R: RangedSigned<$s>,
            {
                const {
                    if as_repr::as_repr(R::MAX) > Self::MAX.get() {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self(self.get() | as_repr::as_repr(ranged)).clear_invalid_bits()
            }

            /// Bitwise XOR.
            ///
            /// ```rust
            /// # use ranch::bitwise::I12;
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitxor::<0b1110>(),
            ///     I12::new::<0b0101>(),
            /// );
            /// ```
            pub const fn bitxor<const N: $s>(self) -> Self {
                self.bitxor_ranged(Self::new::<N>())
            }

            /// Bitwise XOR another ranged unsigned int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{I10, I12};
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitxor_ranged(I10::new::<0b1110>()),
            ///     I12::new::<0b0101>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitxor_ranged(I12::new::<0b1110>()),
            ///     I12::new::<0b0101>(),
            /// );
            /// ```
            pub const fn bitxor_ranged<R>(self, ranged: R) -> Self
            where
                R: RangedSigned<$s>,
            {
                const {
                    if as_repr::as_repr(R::MAX) > Self::MAX.get() {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self(self.get() ^ as_repr::as_repr(ranged)).clear_invalid_bits()
            }

            /// Bitwise shift left.
            ///
            /// Returns `None` if `rhs` is greater than or equal to
            #[doc = concat!(stringify!($bits), ".")]
            ///
            /// ```rust
            /// # use ranch::bitwise::I12;
            /// assert_eq!(
            ///     I12::new::<0b1011>().checked_shl(4).unwrap(),
            ///     I12::new::<0b1011_0000>(),
            /// );
            /// ```
            pub const fn checked_shl(
                self,
                rhs: impl AsRepr<u32>,
            ) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);

                if rhs >= $bits {
                    return None;
                }

                let Some(value) = self.get().checked_shl(rhs) else {
                    return None;
                };

                Some(Self(value).clear_invalid_bits())
            }

            /// Bitwise shift right.
            ///
            /// Returns `None` if `rhs` is greater than or equal to
            ///
            /// ```rust
            /// # use ranch::bitwise::I12;
            /// assert_eq!(
            ///     I12::new::<0b1011_0000>().checked_shr(4).unwrap(),
            ///     I12::new::<0b1011>(),
            /// );
            /// ```
            #[doc = concat!(stringify!($bits), ".")]
            pub const fn checked_shr(
                self,
                rhs: impl AsRepr<u32>,
            ) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);

                if rhs >= $bits {
                    return None;
                }

                let Some(value) = self.get().checked_shr(rhs) else {
                    return None;
                };

                Some(Self(value).clear_invalid_bits())
            }

            /// Bitwise shift left.
            ///
            /// ```rust
            /// # use ranch::bitwise::I12;
            /// assert_eq!(
            ///     I12::new::<0b1011>().shl::<4>(),
            ///     I12::new::<0b1011_0000>(),
            /// );
            /// ```
            pub const fn shl<const N: u32>(self) -> Self {
                const {
                    if N >= $bits {
                        panic!(concat!(
                            "cannot shift left more than ",
                            stringify!($bits),
                            " bits.",
                        ));
                    }
                }

                match self.checked_shl(N) {
                    Some(value) => value,
                    None => unreachable!(),
                }
            }

            /// Bitwise shift right.
            ///
            /// ```rust
            /// # use ranch::bitwise::I12;
            /// assert_eq!(
            ///     I12::new::<0b1011_0000>().shr::<4>(),
            ///     I12::new::<0b1011>(),
            /// );
            /// ```
            pub const fn shr<const N: u32>(self) -> Self {
                const {
                    if N >= $bits {
                        panic!(concat!(
                            "cannot shift right more than ",
                            stringify!($bits),
                            " bits.",
                        ));
                    }
                }

                match self.checked_shr(N) {
                    Some(value) => value,
                    None => unreachable!(),
                }
            }

            const fn clear_invalid_bits(self) -> Self {
                let unused_bits = const { Self::BITS - $bits };

                Self((self.get() << unused_bits) >> unused_bits)
            }
        }

        impl RangedSigned<$s> for $signed {}

        impl Not for $signed {
            type Output = $signed;

            fn not(self) -> Self::Output {
                self.bitnot()
            }
        }

        impl BitAnd for $signed {
            type Output = $signed;

            fn bitand(self, rhs: Self) -> Self {
                self.bitand_ranged(rhs)
            }
        }

        impl BitOr for $signed {
            type Output = $signed;

            fn bitor(self, rhs: Self) -> Self {
                self.bitor_ranged(rhs)
            }
        }

        impl BitXor for $signed {
            type Output = $signed;

            fn bitxor(self, rhs: Self) -> Self {
                self.bitxor_ranged(rhs)
            }
        }

        impl<T> Shl<T> for $signed
        where
            T: AsRepr<u32>,
        {
            type Output = Self;

            fn shl(self, rhs: T) -> Self::Output {
                let Some(value) = self.checked_shl(rhs) else {
                    panic!(concat!(
                        "cannot shift left more than ",
                        stringify!($bits),
                        " bits.",
                    ));
                };

                value
            }
        }

        impl<T> Shr<T> for $signed
        where
            T: AsRepr<u32>,
        {
            type Output = Self;

            fn shr(self, rhs: T) -> Self::Output {
                let Some(value) = self.checked_shr(rhs) else {
                    panic!(concat!(
                        "cannot shift right more than ",
                        stringify!($bits),
                        " bits.",
                    ));
                };

                value
            }
        }

        impl $unsigned {
            /// Bitwise NOT.
            ///
            /// ```rust
            /// # use ranch::bitwise::U12;
            /// assert_eq!(U12::new::<0>().bitnot(), U12::new::<4095>());
            /// assert_eq!(U12::new::<4095>().bitnot(), U12::new::<0>());
            /// ```
            pub const fn bitnot(self) -> Self {
                Self(!self.get()).clear_invalid_bits()
            }

            /// Bitwise AND.
            ///
            /// ```rust
            /// # use ranch::bitwise::U12;
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitand::<0b1110>(),
            ///     U12::new::<0b1010>(),
            /// );
            /// ```
            pub const fn bitand<const N: $u>(self) -> Self {
                self.bitand_ranged(Self::new::<N>())
            }

            /// Bitwise AND another ranged unsigned int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{U10, U12};
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitand_ranged(U10::new::<0b1110>()),
            ///     U12::new::<0b1010>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitand_ranged(U12::new::<0b1110>()),
            ///     U12::new::<0b1010>(),
            /// );
            /// ```
            pub const fn bitand_ranged<R>(self, ranged: R) -> Self
            where
                R: RangedUnsigned<$u>,
            {
                const {
                    if as_repr::as_repr(R::MAX) > Self::MAX.get() {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self(self.get() & as_repr::as_repr(ranged))
            }

            /// Bitwise OR.
            ///
            /// ```rust
            /// # use ranch::bitwise::U12;
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitor::<0b1110>(),
            ///     U12::new::<0b1111>(),
            /// );
            /// ```
            pub const fn bitor<const N: $u>(self) -> Self {
                self.bitor_ranged(Self::new::<N>())
            }

            /// Bitwise OR another ranged unsigned int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{U10, U12};
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitor_ranged(U10::new::<0b1110>()),
            ///     U12::new::<0b1111>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitor_ranged(U12::new::<0b1110>()),
            ///     U12::new::<0b1111>(),
            /// );
            /// ```
            pub const fn bitor_ranged<R>(self, ranged: R) -> Self
            where
                R: RangedUnsigned<$u>,
            {
                const {
                    if as_repr::as_repr(R::MAX) > Self::MAX.get() {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self(self.get() | as_repr::as_repr(ranged))
            }

            /// Bitwise XOR.
            ///
            /// ```rust
            /// # use ranch::bitwise::U12;
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitxor::<0b1110>(),
            ///     U12::new::<0b0101>(),
            /// );
            /// ```
            pub const fn bitxor<const N: $u>(self) -> Self {
                self.bitxor_ranged(Self::new::<N>())
            }

            /// Bitwise XOR another ranged unsigned int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{U10, U12};
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitxor_ranged(U10::new::<0b1110>()),
            ///     U12::new::<0b0101>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitxor_ranged(U12::new::<0b1110>()),
            ///     U12::new::<0b0101>(),
            /// );
            /// ```
            pub const fn bitxor_ranged<R>(self, ranged: R) -> Self
            where
                R: RangedUnsigned<$u>,
            {
                const {
                    if as_repr::as_repr(R::MAX) > Self::MAX.get() {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self(self.get() ^ as_repr::as_repr(ranged))
            }

            /// Bitwise shift left.
            ///
            /// Returns `None` if `rhs` is greater than or equal to
            #[doc = concat!(stringify!($bits), ".")]
            ///
            /// ```rust
            /// # use ranch::bitwise::U12;
            /// assert_eq!(
            ///     U12::new::<0b1011>().checked_shl(4).unwrap(),
            ///     U12::new::<0b1011_0000>(),
            /// );
            /// ```
            pub const fn checked_shl(
                self,
                rhs: impl AsRepr<u32>,
            ) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);

                if rhs >= $bits {
                    return None;
                }

                let Some(value) = self.get().checked_shl(rhs) else {
                    return None;
                };

                Some(Self(value).clear_invalid_bits())
            }

            /// Bitwise shift right.
            ///
            /// Returns `None` if `rhs` is greater than or equal to
            ///
            /// ```rust
            /// # use ranch::bitwise::U12;
            /// assert_eq!(
            ///     U12::new::<0b1011_0000>().checked_shr(4).unwrap(),
            ///     U12::new::<0b1011>(),
            /// );
            /// ```
            #[doc = concat!(stringify!($bits), ".")]
            pub const fn checked_shr(
                self,
                rhs: impl AsRepr<u32>,
            ) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);

                if rhs >= $bits {
                    return None;
                }

                let Some(value) = self.get().checked_shr(rhs) else {
                    return None;
                };

                Some(Self(value).clear_invalid_bits())
            }

            /// Bitwise shift left.
            ///
            /// ```rust
            /// # use ranch::bitwise::U12;
            /// assert_eq!(
            ///     U12::new::<0b1011>().shl::<4>(),
            ///     U12::new::<0b1011_0000>(),
            /// );
            /// ```
            pub const fn shl<const N: u32>(self) -> Self {
                const {
                    if N >= $bits {
                        panic!(concat!(
                            "cannot shift left more than ",
                            stringify!($bits),
                            " bits.",
                        ));
                    }
                }

                match self.checked_shl(N) {
                    Some(value) => value,
                    None => unreachable!(),
                }
            }

            /// Bitwise shift right.
            ///
            /// ```rust
            /// # use ranch::bitwise::U12;
            /// assert_eq!(
            ///     U12::new::<0b1011_0000>().shr::<4>(),
            ///     U12::new::<0b1011>(),
            /// );
            /// ```
            pub const fn shr<const N: u32>(self) -> Self {
                const {
                    if N >= $bits {
                        panic!(concat!(
                            "cannot shift right more than ",
                            stringify!($bits),
                            " bits.",
                        ));
                    }
                }

                match self.checked_shr(N) {
                    Some(value) => value,
                    None => unreachable!(),
                }
            }

            const fn clear_invalid_bits(self) -> Self {
                self.bitand::<{ Self::MAX.get() }>()
            }
        }

        impl RangedUnsigned<$u> for $unsigned {}

        impl Not for $unsigned {
            type Output = $unsigned;

            fn not(self) -> Self::Output {
                self.bitnot()
            }
        }

        impl BitAnd for $unsigned {
            type Output = $unsigned;

            fn bitand(self, rhs: Self) -> Self {
                self.bitand_ranged(rhs)
            }
        }

        impl BitOr for $unsigned {
            type Output = $unsigned;

            fn bitor(self, rhs: Self) -> Self {
                self.bitor_ranged(rhs)
            }
        }

        impl BitXor for $unsigned {
            type Output = $unsigned;

            fn bitxor(self, rhs: Self) -> Self {
                self.bitxor_ranged(rhs)
            }
        }

        impl<T> Shl<T> for $unsigned
        where
            T: AsRepr<u32>,
        {
            type Output = Self;

            fn shl(self, rhs: T) -> Self::Output {
                let Some(value) = self.checked_shl(rhs) else {
                    panic!(concat!(
                        "cannot shift left more than ",
                        stringify!($bits),
                        " bits.",
                    ));
                };

                value
            }
        }

        impl<T> Shr<T> for $unsigned
        where
            T: AsRepr<u32>,
        {
            type Output = Self;

            fn shr(self, rhs: T) -> Self::Output {
                let Some(value) = self.checked_shr(rhs) else {
                    panic!(concat!(
                        "cannot shift right more than ",
                        stringify!($bits),
                        " bits.",
                    ));
                };

                value
            }
        }
    };
}

bitops!(u8, i8, U1, I1, 1);
bitops!(u8, i8, U2, I2, 2);
bitops!(u8, i8, U3, I3, 3);
bitops!(u8, i8, U4, I4, 4);
bitops!(u8, i8, U5, I5, 5);
bitops!(u8, i8, U6, I6, 6);
bitops!(u8, i8, U7, I7, 7);
bitops!(u8, i8, U8, I8, 8);
bitops!(u16, i16, U9, I9, 9);
bitops!(u16, i16, U10, I10, 10);
bitops!(u16, i16, U11, I11, 11);
bitops!(u16, i16, U12, I12, 12);
bitops!(u16, i16, U13, I13, 13);
bitops!(u16, i16, U14, I14, 14);
bitops!(u16, i16, U15, I15, 15);
bitops!(u16, i16, U16, I16, 16);
bitops!(u32, i32, U17, I17, 17);
bitops!(u32, i32, U18, I18, 18);
bitops!(u32, i32, U19, I19, 19);
bitops!(u32, i32, U20, I20, 20);
bitops!(u32, i32, U21, I21, 21);
bitops!(u32, i32, U22, I22, 22);
bitops!(u32, i32, U23, I23, 23);
bitops!(u32, i32, U24, I24, 24);
bitops!(u32, i32, U25, I25, 25);
bitops!(u32, i32, U26, I26, 26);
bitops!(u32, i32, U27, I27, 27);
bitops!(u32, i32, U28, I28, 28);
bitops!(u32, i32, U29, I29, 29);
bitops!(u32, i32, U30, I30, 30);
bitops!(u32, i32, U31, I31, 31);
bitops!(u32, i32, U32, I32, 32);
bitops!(u64, i64, U33, I33, 33);
bitops!(u64, i64, U34, I34, 34);
bitops!(u64, i64, U35, I35, 35);
bitops!(u64, i64, U36, I36, 36);
bitops!(u64, i64, U37, I37, 37);
bitops!(u64, i64, U38, I38, 38);
bitops!(u64, i64, U39, I39, 39);
bitops!(u64, i64, U40, I40, 40);
bitops!(u64, i64, U41, I41, 41);
bitops!(u64, i64, U42, I42, 42);
bitops!(u64, i64, U43, I43, 43);
bitops!(u64, i64, U44, I44, 44);
bitops!(u64, i64, U45, I45, 45);
bitops!(u64, i64, U46, I46, 46);
bitops!(u64, i64, U47, I47, 47);
bitops!(u64, i64, U48, I48, 48);
bitops!(u64, i64, U49, I49, 49);
bitops!(u64, i64, U50, I50, 50);
bitops!(u64, i64, U51, I51, 51);
bitops!(u64, i64, U52, I52, 52);
bitops!(u64, i64, U53, I53, 53);
bitops!(u64, i64, U54, I54, 54);
bitops!(u64, i64, U55, I55, 55);
bitops!(u64, i64, U56, I56, 56);
bitops!(u64, i64, U57, I57, 57);
bitops!(u64, i64, U58, I58, 58);
bitops!(u64, i64, U59, I59, 59);
bitops!(u64, i64, U60, I60, 60);
bitops!(u64, i64, U61, I61, 61);
bitops!(u64, i64, U62, I62, 62);
bitops!(u64, i64, U63, I63, 63);
bitops!(u64, i64, U64, I64, 64);
bitops!(u128, i128, U65, I65, 65);
bitops!(u128, i128, U66, I66, 66);
bitops!(u128, i128, U67, I67, 67);
bitops!(u128, i128, U68, I68, 68);
bitops!(u128, i128, U69, I69, 69);
bitops!(u128, i128, U70, I70, 70);
bitops!(u128, i128, U71, I71, 71);
bitops!(u128, i128, U72, I72, 72);
bitops!(u128, i128, U73, I73, 73);
bitops!(u128, i128, U74, I74, 74);
bitops!(u128, i128, U75, I75, 75);
bitops!(u128, i128, U76, I76, 76);
bitops!(u128, i128, U77, I77, 77);
bitops!(u128, i128, U78, I78, 78);
bitops!(u128, i128, U79, I79, 79);
bitops!(u128, i128, U80, I80, 80);
bitops!(u128, i128, U81, I81, 81);
bitops!(u128, i128, U82, I82, 82);
bitops!(u128, i128, U83, I83, 83);
bitops!(u128, i128, U84, I84, 84);
bitops!(u128, i128, U85, I85, 85);
bitops!(u128, i128, U86, I86, 86);
bitops!(u128, i128, U87, I87, 87);
bitops!(u128, i128, U88, I88, 88);
bitops!(u128, i128, U89, I89, 89);
bitops!(u128, i128, U90, I90, 90);
bitops!(u128, i128, U91, I91, 91);
bitops!(u128, i128, U92, I92, 92);
bitops!(u128, i128, U93, I93, 93);
bitops!(u128, i128, U94, I94, 94);
bitops!(u128, i128, U95, I95, 95);
bitops!(u128, i128, U96, I96, 96);
bitops!(u128, i128, U97, I97, 97);
bitops!(u128, i128, U98, I98, 98);
bitops!(u128, i128, U99, I99, 99);
bitops!(u128, i128, U100, I100, 100);
bitops!(u128, i128, U101, I101, 101);
bitops!(u128, i128, U102, I102, 102);
bitops!(u128, i128, U103, I103, 103);
bitops!(u128, i128, U104, I104, 104);
bitops!(u128, i128, U105, I105, 105);
bitops!(u128, i128, U106, I106, 106);
bitops!(u128, i128, U107, I107, 107);
bitops!(u128, i128, U108, I108, 108);
bitops!(u128, i128, U109, I109, 109);
bitops!(u128, i128, U110, I110, 110);
bitops!(u128, i128, U111, I111, 111);
bitops!(u128, i128, U112, I112, 112);
bitops!(u128, i128, U113, I113, 113);
bitops!(u128, i128, U114, I114, 114);
bitops!(u128, i128, U115, I115, 115);
bitops!(u128, i128, U116, I116, 116);
bitops!(u128, i128, U117, I117, 117);
bitops!(u128, i128, U118, I118, 118);
bitops!(u128, i128, U119, I119, 119);
bitops!(u128, i128, U120, I120, 120);
bitops!(u128, i128, U121, I121, 121);
bitops!(u128, i128, U122, I122, 122);
bitops!(u128, i128, U123, I123, 123);
bitops!(u128, i128, U124, I124, 124);
bitops!(u128, i128, U125, I125, 125);
bitops!(u128, i128, U126, I126, 126);
bitops!(u128, i128, U127, I127, 127);
bitops!(u128, i128, U128, I128, 128);

pub trait RangedUnsigned<T>: AsRepr<T> + Range + Sized {}

pub trait RangedSigned<T>: AsRepr<T> + Range + Sized {}
