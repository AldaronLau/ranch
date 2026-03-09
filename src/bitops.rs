use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl,
    ShlAssign, Shr, ShrAssign,
};

use as_repr::AsRepr;

use crate::{
    bitwise::*,
    cast::as_primitive::{self, AsPrimitive},
    from_repr::FromRepr,
    shl::DowncastShl,
    *,
};

macro_rules! bitops_impl {
    ($unsigned:ident, $signed:ident, $u:ty, $s:ty) => {
        impl<const MIN: $s, const MAX: $s> $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
        {
            /// Bitwise NOT.
            ///
            /// ```rust
            /// # use ranch::bitwise::I12;
            /// assert_eq!(I12::new::<0>().bitnot(), I12::new::<-1>());
            /// assert_eq!(I12::new::<-1>().bitnot(), I12::new::<0>());
            /// assert_eq!(I12::new::<-2048>().bitnot(), I12::new::<2047>());
            /// assert_eq!(I12::new::<2047>().bitnot(), I12::new::<-2048>());
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitnot(self) -> Self {
                Self::from_unchecked(!self.get()).clear_invalid_bits()
            }

            /// Bitwise mask (AND).
            ///
            /// ```rust
            /// # use ranch::bitwise::{I12, I6};
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitmask::<0b1110, I6>(),
            ///     I6::new::<0b1010>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitmask::<0b1110, I12>(),
            ///     I12::new::<0b1010>(),
            /// );
            /// ```
            ///
            /// Fails to compile if you try to expand the type:
            ///
            /// ```rust,compile_fail,E0080
            /// # use ranch::bitwise::{I12, I6};
            /// assert_eq!(
            ///     I6::new::<0b1011>().bitmask::<0b1110, I12>(),
            ///     I12::new::<0b1010>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitmask<const N: $s, T>(self) -> T
            where
                T: FromRepr
                    + BitwiseSigned<<T as FromRepr>::Repr>
                    + AsPrimitive<$s>,
                Self: AsPrimitive<<T as FromRepr>::Repr>,
            {
                const {
                    if N > as_primitive::as_primitive_expanding(T::MAX)
                        || N < as_primitive::as_primitive_expanding(T::MIN)
                    {
                        panic!("Mask must fit within bounds of output range");
                    }
                }

                let masked = self.bitand::<N>();
                let scaled: <T as FromRepr>::Repr =
                    as_primitive::as_primitive_shrinking(masked);

                from_repr::from_repr(scaled)
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
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitand<const N: $s>(self) -> Self {
                self.bitand_ranged(Self::new::<N>())
            }

            /// Bitwise AND another ranged signed int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{I5, I10, I12};
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitand_ranged(I5::new::<0b1110>()),
            ///     I12::new::<0b1010>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitand_ranged(I10::new::<0b1110>()),
            ///     I12::new::<0b1010>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitand_ranged(I12::new::<0b1110>()),
            ///     I12::new::<0b1010>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitand_ranged<R>(self, ranged: R) -> Self
            where
                R: BitwiseSigned<$s>,
            {
                const {
                    if as_primitive::as_primitive_expanding(R::MAX)
                        > Self::MAX.get()
                    {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self::from_unchecked(
                    self.get() & as_primitive::as_primitive_expanding(ranged),
                )
                .clear_invalid_bits()
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
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitor<const N: $s>(self) -> Self {
                self.bitor_ranged(Self::new::<N>())
            }

            /// Bitwise OR another ranged signed int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{I5, I10, I12};
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitor_ranged(I5::new::<0b1110>()),
            ///     I12::new::<0b1111>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitor_ranged(I10::new::<0b1110>()),
            ///     I12::new::<0b1111>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitor_ranged(I12::new::<0b1110>()),
            ///     I12::new::<0b1111>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitor_ranged<R>(self, ranged: R) -> Self
            where
                R: BitwiseSigned<$s>,
            {
                const {
                    if as_primitive::as_primitive_expanding(R::MAX)
                        > Self::MAX.get()
                    {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self::from_unchecked(
                    self.get() | as_primitive::as_primitive_expanding(ranged),
                )
                .clear_invalid_bits()
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
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitxor<const N: $s>(self) -> Self {
                self.bitxor_ranged(Self::new::<N>())
            }

            /// Bitwise XOR another ranged unsigned int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{I5, I10, I12};
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitxor_ranged(I5::new::<0b1110>()),
            ///     I12::new::<0b0101>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitxor_ranged(I10::new::<0b1110>()),
            ///     I12::new::<0b0101>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011>().bitxor_ranged(I12::new::<0b1110>()),
            ///     I12::new::<0b0101>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitxor_ranged<R>(self, ranged: R) -> Self
            where
                R: BitwiseSigned<$s>,
            {
                const {
                    if as_primitive::as_primitive_expanding(R::MAX)
                        > Self::MAX.get()
                    {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self::from_unchecked(
                    self.get() ^ as_primitive::as_primitive_expanding(ranged),
                )
                .clear_invalid_bits()
            }

            /// Bitwise shift left.
            ///
            /// Returns `None` if `rhs` is greater than or equal to `N` for
            /// type `I{N}`.
            ///
            /// ```rust
            /// # use ranch::bitwise::I12;
            /// assert_eq!(
            ///     I12::new::<0b1011>().checked_shl(4).unwrap(),
            ///     I12::new::<0b1011_0000>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_shl(
                self,
                rhs: impl AsRepr<u32>,
            ) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);

                if rhs >= Self::USED_BITS {
                    return None;
                }

                let Some(value) = self.get().checked_shl(rhs) else {
                    return None;
                };

                Some(Self::from_unchecked(value).clear_invalid_bits())
            }

            /// Expanding bitwise shift left.
            ///
            /// ```rust
            /// # use ranch::bitwise::{I7, I13};
            /// assert_eq!(
            ///     I7::new::<0b10_1011>().expanding_shl::<6, I13>(),
            ///     I13::new::<0b1010_1100_0000>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn expanding_shl<const N: u32, T>(self) -> T
            where
                <T as FromRepr>::Repr: DowncastShl,
                T: FromRepr + BitwiseSigned<<T as FromRepr>::Repr>,
                Self: AsPrimitive<<T as FromRepr>::Repr>,
            {
                const {
                    if T::USED_BITS != Self::USED_BITS + N {
                        panic!("bit size plus shift must equal result bit size")
                    }
                }

                let scaled: <T as FromRepr>::Repr =
                    as_primitive::as_primitive_expanding(self);
                let shifted: <T as FromRepr>::Repr =
                    shl::downcast_shl::<N, <T as FromRepr>::Repr>(scaled);

                from_repr::from_repr(shifted)
            }

            /// Shrinking bitwise shift right.
            ///
            /// ```rust
            /// # use ranch::bitwise::{I7, I13};
            /// assert_eq!(
            ///     I13::new::<0b1010_1100_0000>().shrinking_shr::<6, I7>(),
            ///     I7::new::<0b10_1011>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn shrinking_shr<const N: u32, T>(self) -> T
            where
                T: FromRepr + BitwiseSigned<<T as FromRepr>::Repr>,
                Self: AsPrimitive<<T as FromRepr>::Repr>,
            {
                const {
                    if T::USED_BITS != Self::USED_BITS - N {
                        panic!(
                            "bit size minus shift must equal result bit size"
                        )
                    }
                }

                let repr: $s = as_repr::as_repr(self);
                let shifted: $s = shr::downcast_shr_signed::<N, _>(repr);
                let wrapped = Self::from_unchecked(shifted);
                let scaled: <T as FromRepr>::Repr =
                    as_primitive::as_primitive_shrinking(wrapped);

                from_repr::from_repr(scaled)
            }

            /// Bitwise shift right.
            ///
            /// Returns `None` if `rhs` is greater than or equal to `N` for
            /// type `I{N}`.
            ///
            /// ```rust
            /// # use ranch::bitwise::I12;
            /// assert_eq!(
            ///     I12::new::<0b1011_0000>().checked_shr(4).unwrap(),
            ///     I12::new::<0b1011>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_shr(
                self,
                rhs: impl AsRepr<u32>,
            ) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);

                if rhs >= Self::USED_BITS {
                    return None;
                }

                let Some(value) = self.get().checked_shr(rhs) else {
                    return None;
                };

                Some(Self::from_unchecked(value).clear_invalid_bits())
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
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn shl<const N: u32>(self) -> Self {
                const {
                    if N >= Self::USED_BITS {
                        panic!("cannot shift left more than size - 1 in bits");
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
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn shr<const N: u32>(self) -> Self {
                const {
                    if N >= Self::USED_BITS {
                        panic!("cannot shift right more than size - 1 in bits");
                    }
                }

                match self.checked_shr(N) {
                    Some(value) => value,
                    None => unreachable!(),
                }
            }

            /// Bitwise shift left.
            ///
            /// ```rust
            /// # use ranch::{bitwise::I12, unit::{UnitU32, UnitU8}};
            /// assert_eq!(
            ///     I12::new::<0b1011>()
            ///         .shl_ranged(UnitU32::<4>::default()),
            ///     I12::new::<0b1011_0000>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011>()
            ///         .shl_ranged(UnitU8::<4>::default()),
            ///     I12::new::<0b1011_0000>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn shl_ranged<R>(self, ranged: R) -> Self
            where
                R: AsPrimitive<u32>,
            {
                const {
                    as_primitive::as_primitive_expanding(R::MIN);

                    if as_primitive::as_primitive_expanding(R::MAX)
                        >= Self::USED_BITS
                    {
                        panic!("cannot shift left more than size - 1 in bits");
                    }
                }

                match self
                    .checked_shl(as_primitive::as_primitive_expanding(ranged))
                {
                    Some(value) => value,
                    None => unreachable!(),
                }
            }

            /// Bitwise shift right.
            ///
            /// ```rust
            /// # use ranch::{bitwise::I12, unit::{UnitU32, UnitU8}};
            /// assert_eq!(
            ///     I12::new::<0b1011_0000>()
            ///         .shr_ranged(UnitU32::<4>::default()),
            ///     I12::new::<0b1011>(),
            /// );
            /// assert_eq!(
            ///     I12::new::<0b1011_0000>()
            ///         .shr_ranged(UnitU8::<4>::default()),
            ///     I12::new::<0b1011>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn shr_ranged<R>(self, ranged: R) -> Self
            where
                R: AsPrimitive<u32>,
            {
                const {
                    as_primitive::as_primitive_expanding(R::MIN);

                    if as_primitive::as_primitive_expanding(R::MAX)
                        >= Self::USED_BITS
                    {
                        panic!("cannot shift right more than size - 1 in bits");
                    }
                }

                match self
                    .checked_shr(as_primitive::as_primitive_expanding(ranged))
                {
                    Some(value) => value,
                    None => unreachable!(),
                }
            }

            const fn clear_invalid_bits(self) -> Self {
                let unused_bits = const { Self::BITS - Self::USED_BITS };

                Self::from_unchecked((self.get() << unused_bits) >> unused_bits)
            }
        }

        impl<const MIN: $u, const MAX: $u> $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
        {
            /// Bitwise NOT.
            ///
            /// ```rust
            /// # use ranch::bitwise::U12;
            /// assert_eq!(U12::new::<0>().bitnot(), U12::new::<4095>());
            /// assert_eq!(U12::new::<4095>().bitnot(), U12::new::<0>());
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitnot(self) -> Self {
                Self::from_unchecked(!self.get()).clear_invalid_bits()
            }

            /// Bitwise mask (AND).
            ///
            /// ```rust
            /// # use ranch::bitwise::{U12, U4};
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitmask::<0b1110, U4>(),
            ///     U4::new::<0b1010>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitmask::<0b1110, U12>(),
            ///     U12::new::<0b1010>(),
            /// );
            /// ```
            ///
            /// Fails to compile if you try to expand the type:
            ///
            /// ```rust,compile_fail,E0080
            /// # use ranch::bitwise::{U12, U4};
            /// assert_eq!(
            ///     U4::new::<0b1011>().bitmask::<0b1110, U12>(),
            ///     U12::new::<0b1010>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitmask<const N: $u, T>(self) -> T
            where
                T: FromRepr
                    + BitwiseUnsigned<<T as FromRepr>::Repr>
                    + AsPrimitive<$u>,
                Self: AsPrimitive<<T as FromRepr>::Repr>,
            {
                const {
                    if N > as_primitive::as_primitive_expanding(T::MAX)
                        || N < as_primitive::as_primitive_expanding(T::MIN)
                    {
                        panic!("Mask must fit within bounds of output range");
                    }
                }

                let masked = self.bitand::<N>();
                let scaled: <T as FromRepr>::Repr =
                    as_primitive::as_primitive_shrinking(masked);

                from_repr::from_repr(scaled)
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
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitand<const N: $u>(self) -> Self {
                self.bitand_ranged(Self::new::<N>())
            }

            /// Bitwise AND another ranged unsigned int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{U4, U10, U12};
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitand_ranged(U4::new::<0b1110>()),
            ///     U12::new::<0b1010>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitand_ranged(U10::new::<0b1110>()),
            ///     U12::new::<0b1010>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitand_ranged(U12::new::<0b1110>()),
            ///     U12::new::<0b1010>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitand_ranged<R>(self, ranged: R) -> Self
            where
                R: BitwiseUnsigned<$u>,
            {
                const {
                    if as_primitive::as_primitive_expanding(R::MAX)
                        > Self::MAX.get()
                    {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self::from_unchecked(
                    self.get() & as_primitive::as_primitive_expanding(ranged),
                )
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
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitor<const N: $u>(self) -> Self {
                self.bitor_ranged(Self::new::<N>())
            }

            /// Bitwise OR another ranged unsigned int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{U4, U10, U12};
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitor_ranged(U4::new::<0b1110>()),
            ///     U12::new::<0b1111>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitor_ranged(U10::new::<0b1110>()),
            ///     U12::new::<0b1111>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitor_ranged(U12::new::<0b1110>()),
            ///     U12::new::<0b1111>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitor_ranged<R>(self, ranged: R) -> Self
            where
                R: BitwiseUnsigned<$u>,
            {
                const {
                    if as_primitive::as_primitive_expanding(R::MAX)
                        > Self::MAX.get()
                    {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self::from_unchecked(
                    self.get() | as_primitive::as_primitive_expanding(ranged),
                )
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
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitxor<const N: $u>(self) -> Self {
                self.bitxor_ranged(Self::new::<N>())
            }

            /// Bitwise XOR another ranged unsigned int.
            ///
            /// ```rust
            /// # use ranch::bitwise::{U4, U10, U12};
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitxor_ranged(U4::new::<0b1110>()),
            ///     U12::new::<0b0101>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitxor_ranged(U10::new::<0b1110>()),
            ///     U12::new::<0b0101>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011>().bitxor_ranged(U12::new::<0b1110>()),
            ///     U12::new::<0b0101>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn bitxor_ranged<R>(self, ranged: R) -> Self
            where
                R: BitwiseUnsigned<$u>,
            {
                const {
                    if as_primitive::as_primitive_expanding(R::MAX)
                        > Self::MAX.get()
                    {
                        panic!("cannot bitwise AND with a larger type")
                    }
                }

                Self::from_unchecked(
                    self.get() ^ as_primitive::as_primitive_expanding(ranged),
                )
            }

            /// Bitwise shift left.
            ///
            /// Returns `None` if `rhs` is greater than or equal to `N` for
            /// type `U{N}`.
            ///
            /// ```rust
            /// # use ranch::bitwise::U12;
            /// assert_eq!(
            ///     U12::new::<0b1011>().checked_shl(4).unwrap(),
            ///     U12::new::<0b1011_0000>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_shl(
                self,
                rhs: impl AsRepr<u32>,
            ) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);

                if rhs >= Self::USED_BITS {
                    return None;
                }

                let Some(value) = self.get().checked_shl(rhs) else {
                    return None;
                };

                Some(Self::from_unchecked(value).clear_invalid_bits())
            }

            /// Bitwise shift right.
            ///
            /// Returns `None` if `rhs` is greater than or equal to `N` for
            /// type `U{N}`.
            ///
            /// ```rust
            /// # use ranch::bitwise::U12;
            /// assert_eq!(
            ///     U12::new::<0b1011_0000>().checked_shr(4).unwrap(),
            ///     U12::new::<0b1011>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_shr(
                self,
                rhs: impl AsRepr<u32>,
            ) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);

                if rhs >= Self::USED_BITS {
                    return None;
                }

                let Some(value) = self.get().checked_shr(rhs) else {
                    return None;
                };

                Some(Self::from_unchecked(value).clear_invalid_bits())
            }

            /// Expanding bitwise shift left.
            ///
            /// ```rust
            /// # use ranch::bitwise::{U6, U12};
            /// assert_eq!(
            ///     U6::new::<0b10_1011>().expanding_shl::<6, U12>(),
            ///     U12::new::<0b1010_1100_0000>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn expanding_shl<const N: u32, T>(self) -> T
            where
                <T as FromRepr>::Repr: DowncastShl,
                T: FromRepr + BitwiseUnsigned<<T as FromRepr>::Repr>,
                Self: AsPrimitive<<T as FromRepr>::Repr>,
            {
                const {
                    if T::USED_BITS != Self::USED_BITS + N {
                        panic!("bit size plus shift must equal result bit size")
                    }
                }

                let scaled: <T as FromRepr>::Repr =
                    as_primitive::as_primitive_expanding(self);
                let shifted: <T as FromRepr>::Repr =
                    shl::downcast_shl::<N, <T as FromRepr>::Repr>(scaled);

                from_repr::from_repr(shifted)
            }

            /// Shrinking bitwise shift right.
            ///
            /// ```rust
            /// # use ranch::bitwise::{U6, U12};
            /// assert_eq!(
            ///     U12::new::<0b1010_1100_0000>().shrinking_shr::<6, U6>(),
            ///     U6::new::<0b10_1011>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn shrinking_shr<const N: u32, T>(self) -> T
            where
                T: FromRepr + BitwiseUnsigned<<T as FromRepr>::Repr>,
                Self: AsPrimitive<<T as FromRepr>::Repr>,
            {
                const {
                    if T::USED_BITS != Self::USED_BITS - N {
                        panic!(
                            "bit size minus shift must equal result bit size"
                        )
                    }
                }

                let repr: $u = as_repr::as_repr(self);
                let shifted: $u = shr::downcast_shr_unsigned::<N, _>(repr);
                let wrapped = Self::from_unchecked(shifted);
                let scaled: <T as FromRepr>::Repr =
                    as_primitive::as_primitive_shrinking(wrapped);

                from_repr::from_repr(scaled)
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
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn shl<const N: u32>(self) -> Self {
                const {
                    if N >= Self::USED_BITS {
                        panic!("cannot shift left more than size - 1 in bits");
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
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn shr<const N: u32>(self) -> Self {
                const {
                    if N >= Self::USED_BITS {
                        panic!("cannot shift right more than size - 1 in bits");
                    }
                }

                match self.checked_shr(N) {
                    Some(value) => value,
                    None => unreachable!(),
                }
            }

            /// Bitwise shift left.
            ///
            /// ```rust
            /// # use ranch::{bitwise::U12, unit::{UnitU32, UnitU8}};
            /// assert_eq!(
            ///     U12::new::<0b1011>()
            ///         .shl_ranged(UnitU32::<4>::default()),
            ///     U12::new::<0b1011_0000>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011>()
            ///         .shl_ranged(UnitU8::<4>::default()),
            ///     U12::new::<0b1011_0000>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn shl_ranged<R>(self, ranged: R) -> Self
            where
                R: AsPrimitive<u32>,
            {
                const {
                    as_primitive::as_primitive_expanding(R::MIN);

                    if as_primitive::as_primitive_expanding(R::MAX)
                        >= Self::USED_BITS
                    {
                        panic!("cannot shift left more than size - 1 in bits");
                    }
                }

                match self
                    .checked_shl(as_primitive::as_primitive_expanding(ranged))
                {
                    Some(value) => value,
                    None => unreachable!(),
                }
            }

            /// Bitwise shift right.
            ///
            /// ```rust
            /// # use ranch::{bitwise::U12, unit::{UnitU32, UnitU8}};
            /// assert_eq!(
            ///     U12::new::<0b1011_0000>()
            ///         .shr_ranged(UnitU32::<4>::default()),
            ///     U12::new::<0b1011>(),
            /// );
            /// assert_eq!(
            ///     U12::new::<0b1011_0000>()
            ///         .shr_ranged(UnitU8::<4>::default()),
            ///     U12::new::<0b1011>(),
            /// );
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn shr_ranged<R>(self, ranged: R) -> Self
            where
                R: AsPrimitive<u32>,
            {
                const {
                    as_primitive::as_primitive_expanding(R::MIN);

                    if as_primitive::as_primitive_expanding(R::MAX)
                        >= Self::USED_BITS
                    {
                        panic!("cannot shift right more than size - 1 in bits");
                    }
                }

                match self
                    .checked_shr(as_primitive::as_primitive_expanding(ranged))
                {
                    Some(value) => value,
                    None => unreachable!(),
                }
            }

            const fn clear_invalid_bits(self) -> Self {
                let unused_bits = const { Self::BITS - Self::USED_BITS };

                Self::from_unchecked((self.get() << unused_bits) >> unused_bits)
            }
        }

        impl<const MIN: $s, const MAX: $s> Not for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
        {
            type Output = Self;

            fn not(self) -> Self::Output {
                self.bitnot()
            }
        }

        impl<const MIN: $s, const MAX: $s> BitAnd for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
        {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                self.bitand_ranged(rhs)
            }
        }

        impl<const MIN: $s, const MAX: $s> BitOr for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
        {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                self.bitor_ranged(rhs)
            }
        }

        impl<const MIN: $s, const MAX: $s> BitXor for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
        {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self {
                self.bitxor_ranged(rhs)
            }
        }

        impl<const MIN: $s, const MAX: $s, T> Shl<T> for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
            T: AsRepr<u32>,
        {
            type Output = Self;

            fn shl(self, rhs: T) -> Self::Output {
                let Some(value) = self.checked_shl(rhs) else {
                    let used_bits = Self::USED_BITS;

                    panic!("cannot shift left more than {used_bits} - 1 bits");
                };

                value
            }
        }

        impl<const MIN: $s, const MAX: $s, T> Shr<T> for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
            T: AsRepr<u32>,
        {
            type Output = Self;

            fn shr(self, rhs: T) -> Self::Output {
                let Some(value) = self.checked_shr(rhs) else {
                    let used_bits = Self::USED_BITS;

                    panic!("cannot shift right more than {used_bits} - 1 bits");
                };

                value
            }
        }

        impl<const MIN: $u, const MAX: $u> Not for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
        {
            type Output = Self;

            fn not(self) -> Self::Output {
                self.bitnot()
            }
        }

        impl<const MIN: $u, const MAX: $u> BitAnd for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
        {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                self.bitand_ranged(rhs)
            }
        }

        impl<const MIN: $u, const MAX: $u> BitOr for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
        {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                self.bitor_ranged(rhs)
            }
        }

        impl<const MIN: $u, const MAX: $u> BitXor for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
        {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self {
                self.bitxor_ranged(rhs)
            }
        }

        impl<const MIN: $u, const MAX: $u, T> Shl<T> for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
            T: AsRepr<u32>,
        {
            type Output = Self;

            fn shl(self, rhs: T) -> Self::Output {
                let Some(value) = self.checked_shl(rhs) else {
                    let used_bits = Self::USED_BITS;

                    panic!("cannot shift left more than {used_bits} - 1 bits");
                };

                value
            }
        }

        impl<const MIN: $u, const MAX: $u, T> Shr<T> for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
            T: AsRepr<u32>,
        {
            type Output = Self;

            fn shr(self, rhs: T) -> Self::Output {
                let Some(value) = self.checked_shr(rhs) else {
                    let used_bits = Self::USED_BITS;

                    panic!("cannot shift right more than {used_bits} - 1 bits");
                };

                value
            }
        }

        impl<const MIN: $s, const MAX: $s> BitXorAssign for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
        {
            fn bitxor_assign(&mut self, other: Self) {
                *self = *self ^ other;
            }
        }

        impl<const MIN: $s, const MAX: $s> BitOrAssign for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
        {
            fn bitor_assign(&mut self, other: Self) {
                *self = *self | other;
            }
        }

        impl<const MIN: $s, const MAX: $s> BitAndAssign for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
        {
            fn bitand_assign(&mut self, other: Self) {
                *self = *self & other;
            }
        }

        impl<const MIN: $s, const MAX: $s, T> ShlAssign<T> for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
            T: AsRepr<u32>,
        {
            fn shl_assign(&mut self, other: T) {
                *self = *self << other;
            }
        }

        impl<const MIN: $s, const MAX: $s, T> ShrAssign<T> for $signed<MIN, MAX>
        where
            Self: BitwiseSigned<$s>,
            T: AsRepr<u32>,
        {
            fn shr_assign(&mut self, other: T) {
                *self = *self >> other;
            }
        }

        impl<const MIN: $u, const MAX: $u> BitXorAssign for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
        {
            fn bitxor_assign(&mut self, other: Self) {
                *self = *self ^ other;
            }
        }

        impl<const MIN: $u, const MAX: $u> BitOrAssign for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
        {
            fn bitor_assign(&mut self, other: Self) {
                *self = *self | other;
            }
        }

        impl<const MIN: $u, const MAX: $u> BitAndAssign for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
        {
            fn bitand_assign(&mut self, other: Self) {
                *self = *self & other;
            }
        }

        impl<const MIN: $u, const MAX: $u, T> ShlAssign<T>
            for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
            T: AsRepr<u32>,
        {
            fn shl_assign(&mut self, other: T) {
                *self = *self << other;
            }
        }

        impl<const MIN: $u, const MAX: $u, T> ShrAssign<T>
            for $unsigned<MIN, MAX>
        where
            Self: BitwiseUnsigned<$u>,
            T: AsRepr<u32>,
        {
            fn shr_assign(&mut self, other: T) {
                *self = *self >> other;
            }
        }
    };
}

macro_rules! bitops {
    ($u:ty, $s:ty, $unsigned:ty, $signed:ty, $bits:literal) => {
        impl<T> BitwiseSigned<T> for $signed
        where
            $signed: AsPrimitive<T>,
        {
            const USED_BITS: u32 = $bits;
        }

        impl<T> BitwiseUnsigned<T> for $unsigned
        where
            $unsigned: AsPrimitive<T>,
        {
            const USED_BITS: u32 = $bits;
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

bitops_impl!(RangedU8, RangedI8, u8, i8);
bitops_impl!(RangedU16, RangedI16, u16, i16);
bitops_impl!(RangedU32, RangedI32, u32, i32);
bitops_impl!(RangedU64, RangedI64, u64, i64);
bitops_impl!(RangedU128, RangedI128, u128, i128);

pub trait BitwiseUnsigned<T>: AsPrimitive<T> {
    const USED_BITS: u32;
}

pub trait BitwiseSigned<T>: AsPrimitive<T> {
    const USED_BITS: u32;
}
