use core::num::NonZero;

use super::*;

macro_rules! impl_ranged_conversion {
    ($type:ident, $p:ty $(,)?) => {
        impl<const MIN: $p, const MAX: $p> TryFrom<$p> for $type<MIN, MAX> {
            type Error = Error;

            fn try_from(primitive: $p) -> Result<Self, Self::Error> {
                Self::with_primitive(primitive)
            }
        }

        impl<const MIN: $p, const MAX: $p> From<$type<MIN, MAX>> for $p {
            fn from(ranged: $type<MIN, MAX>) -> Self {
                ranged.get()
            }
        }
    };
}

macro_rules! impl_unsigned_nonzero_conversion {
    ($type:ident, $p:ty $(,)?) => {
        impl From<NonZero<$p>> for $type<1, { <$p>::MAX }> {
            fn from(non_zero: NonZero<$p>) -> Self {
                Self(non_zero.get())
            }
        }

        impl From<$type<1, { <$p>::MAX }>> for NonZero<$p> {
            fn from(ranged: $type<1, { <$p>::MAX }>) -> Self {
                // saturate if there's a bug and a value out of range
                NonZero::new(ranged.get()).unwrap_or(NonZero::<$p>::MIN)
            }
        }
    };
}

macro_rules! impl_signed_nonzero_conversion {
    ($type:ident, $p:ty $(,)?) => {
        impl From<$type<1, { <$p>::MAX }>> for NonZero<$p> {
            fn from(ranged: $type<1, { <$p>::MAX }>) -> Self {
                // saturate if there's a bug and a value out of range
                NonZero::new(ranged.get()).unwrap_or(NonZero::<$p>::MIN)
            }
        }
    };
}

macro_rules! impl_nonzero_from_ranged {
    ($type:ident, $p:ty, $nonzero:ident $(,)?) => {
        impl<const MIN: $p, const MAX: $p> $nonzero::<MIN, MAX> {
            #[doc = concat!("Convert from [`", stringify!($type), "`].")]
            ///
            /// Won't compile if the range contains zero.  If you need to check
            /// at runtime for zero instead of at compile-time, try using
            #[doc = concat!("[`", stringify!($type), "::to_ranged_nonzero()`].")]
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            /// assert_eq!(
            #[doc = concat!("    ", stringify!($nonzero), "::from_ranged(", stringify!($type), "::<1, 100>::new::<42>()),")]
            #[doc = concat!("    ", stringify!($nonzero), "::<1, 100>::new::<42>(),")]
            /// );
            /// ```
            ///
            /// ```rust,compile_fail
            #[doc = concat!("# use ranch::{", stringify!($type), "};")]
            ///
            #[doc = concat!(stringify!($nonzero), "::from_ranged(", stringify!($type), "::<0, 100>::new::<42>())")]
            /// ```
            pub const fn from_ranged(ranged: $type::<MIN, MAX>) -> Self {
                // `MAX` comparison only needed for signed numbers
                #[allow(unused_comparisons)]
                const {
                    if MIN <= 0 && MAX >= 0 {
                        panic!("input range can't contain zero")
                    }
                }

                let Some(value) = NonZero::new(ranged.get()) else {
                    unreachable!()
                };

                $nonzero(value)
            }
        }

        impl<const MIN: $p, const MAX: $p> $type::<MIN, MAX> {
            #[doc = concat!("Convert from [`", stringify!($nonzero), "`],")]
            /// optionally expanding the range.
            ///
            /// If you don't need to change the range (range neither includes zero nor needs to be
            /// expanded), try using
            #[doc = concat!("[`", stringify!($nonzero), "::from_ranged()`].")]
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let ranged = ", stringify!($type), "::<0, 2>::new::<1>();")]
            #[doc = concat!("let expanded: ", stringify!($nonzero), "<1, 4> =")]
            ///     ranged.to_ranged_nonzero().unwrap();
            ///
            /// assert_eq!(expanded.get(), ranged.get());
            /// ```
            pub const fn to_ranged_nonzero<
                const OUT_MIN: $p,
                const OUT_MAX: $p,
            >(self) -> Option<$nonzero::<OUT_MIN, OUT_MAX>>
            {
                const {
                    if OUT_MIN > MIN && MIN != 0 && (OUT_MIN - 1) != 0 {
                        panic!(
                            "minimum must be lower or match or exclude zero",
                        );
                    }

                    if OUT_MAX < MAX && MAX != 0 && (OUT_MAX + 1) != 0 {
                        panic!(
                            "maximum must be higher or match or exclude zero",
                        );
                    }

                    if OUT_MIN == 0 {
                        panic!("minimum of a non-zero number cannot be zero");
                    }

                    if OUT_MAX == 0 {
                        panic!("maximum of a non-zero number cannot be zero");
                    }
                }

                match NonZero::new(self.get()) {
                    Some(value) => Some($nonzero(value)),
                    None => None,
                }
            }
        }
    }
}

impl_ranged_conversion!(RangedI8, i8);
impl_ranged_conversion!(RangedI16, i16);
impl_ranged_conversion!(RangedI32, i32);
impl_ranged_conversion!(RangedI64, i64);
impl_ranged_conversion!(RangedI128, i128);
impl_ranged_conversion!(RangedU8, u8);
impl_ranged_conversion!(RangedU16, u16);
impl_ranged_conversion!(RangedU32, u32);
impl_ranged_conversion!(RangedU64, u64);
impl_ranged_conversion!(RangedU128, u128);

impl_signed_nonzero_conversion!(RangedI8, i8);
impl_signed_nonzero_conversion!(RangedI16, i16);
impl_signed_nonzero_conversion!(RangedI32, i32);
impl_signed_nonzero_conversion!(RangedI64, i64);
impl_signed_nonzero_conversion!(RangedI128, i128);
impl_unsigned_nonzero_conversion!(RangedU8, u8);
impl_unsigned_nonzero_conversion!(RangedU16, u16);
impl_unsigned_nonzero_conversion!(RangedU32, u32);
impl_unsigned_nonzero_conversion!(RangedU64, u64);
impl_unsigned_nonzero_conversion!(RangedU128, u128);

impl_nonzero_from_ranged!(RangedI8, i8, RangedNonZeroI8);
impl_nonzero_from_ranged!(RangedI16, i16, RangedNonZeroI16);
impl_nonzero_from_ranged!(RangedI32, i32, RangedNonZeroI32);
impl_nonzero_from_ranged!(RangedI64, i64, RangedNonZeroI64);
impl_nonzero_from_ranged!(RangedI128, i128, RangedNonZeroI128);
impl_nonzero_from_ranged!(RangedU8, u8, RangedNonZeroU8);
impl_nonzero_from_ranged!(RangedU16, u16, RangedNonZeroU16);
impl_nonzero_from_ranged!(RangedU32, u32, RangedNonZeroU32);
impl_nonzero_from_ranged!(RangedU64, u64, RangedNonZeroU64);
impl_nonzero_from_ranged!(RangedU128, u128, RangedNonZeroU128);

impl<const MIN: u8, const MAX: u8> RangedU8<MIN, MAX> {
    /// Convert to [`RangedU8`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8};
    /// let ranged_u8 = RangedU8::<0, 2>::new::<1>();
    /// let expanded_u8: RangedU8<0, 4> = ranged_u8.to_ranged_u8();
    ///
    /// assert_eq!(expanded_u8.get(), ranged_u8.get().into());
    /// ```
    pub const fn to_ranged_u8<const OUT_MIN: u8, const OUT_MAX: u8>(
        self,
    ) -> RangedU8<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU8(self.get())
    }

    /// Convert to [`RangedU16`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedU16};
    /// let ranged_u8 = RangedU8::<0, 2>::new::<1>();
    /// let ranged_u16: RangedU16<0, 2> = ranged_u8.to_ranged_u16();
    ///
    /// assert_eq!(ranged_u16.get(), ranged_u8.get().into());
    /// ```
    pub const fn to_ranged_u16<const OUT_MIN: u16, const OUT_MAX: u16>(
        self,
    ) -> RangedU16<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as u16 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as u16 {
                panic!("maximum must be higher or match");
            }
        }

        RangedU16(self.get() as u16)
    }

    /// Convert to [`RangedU32`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedU32};
    /// let ranged_u8 = RangedU8::<0, 2>::new::<1>();
    /// let ranged_u32: RangedU32<0, 2> = ranged_u8.to_ranged_u32();
    ///
    /// assert_eq!(ranged_u32.get(), ranged_u8.get().into());
    /// ```
    pub const fn to_ranged_u32<const OUT_MIN: u32, const OUT_MAX: u32>(
        self,
    ) -> RangedU32<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as u32 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as u32 {
                panic!("maximum must be higher or match");
            }
        }

        RangedU32(self.get() as u32)
    }

    /// Convert to [`RangedU64`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedU64};
    /// let ranged_u8 = RangedU8::<0, 2>::new::<1>();
    /// let ranged_u64: RangedU64<0, 2> = ranged_u8.to_ranged_u64();
    ///
    /// assert_eq!(ranged_u64.get(), ranged_u8.get().into());
    /// ```
    pub const fn to_ranged_u64<const OUT_MIN: u64, const OUT_MAX: u64>(
        self,
    ) -> RangedU64<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as u64 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as u64 {
                panic!("maximum must be higher or match");
            }
        }

        RangedU64(self.get() as u64)
    }

    /// Convert to [`RangedU128`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedU128};
    /// let ranged_u8 = RangedU8::<0, 2>::new::<1>();
    /// let ranged_u128: RangedU128<0, 2> = ranged_u8.to_ranged_u128();
    ///
    /// assert_eq!(ranged_u128.get(), ranged_u8.get().into());
    /// ```
    pub const fn to_ranged_u128<const OUT_MIN: u128, const OUT_MAX: u128>(
        self,
    ) -> RangedU128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as u128 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as u128 {
                panic!("maximum must be higher or match");
            }
        }

        RangedU128(self.get() as u128)
    }
}

impl<const MIN: u16, const MAX: u16> RangedU16<MIN, MAX> {
    /// Convert to [`RangedU8`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedU16};
    /// let ranged_u16 = RangedU16::<0, 2>::new::<1>();
    /// let ranged_u8: RangedU8<0, 2> = ranged_u16.to_ranged_u8();
    ///
    /// assert_eq!(ranged_u16.get(), ranged_u8.get().into());
    /// ```
    pub const fn to_ranged_u8<const OUT_MIN: u8, const OUT_MAX: u8>(
        self,
    ) -> RangedU8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u16) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u16) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU8(self.get() as u8)
    }

    /// Convert to [`RangedU16`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16};
    /// let ranged_u16 = RangedU16::<0, 2>::new::<1>();
    /// let expanded_u16: RangedU16<0, 4> = ranged_u16.to_ranged_u16();
    ///
    /// assert_eq!(expanded_u16.get(), ranged_u16.get().into());
    /// ```
    pub const fn to_ranged_u16<const OUT_MIN: u16, const OUT_MAX: u16>(
        self,
    ) -> RangedU16<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU16(self.get())
    }

    /// Convert to [`RangedU32`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16, RangedU32};
    /// let ranged_u16 = RangedU16::<0, 2>::new::<1>();
    /// let ranged_u32: RangedU32<0, 2> = ranged_u16.to_ranged_u32();
    ///
    /// assert_eq!(ranged_u32.get(), ranged_u16.get().into());
    /// ```
    pub const fn to_ranged_u32<const OUT_MIN: u32, const OUT_MAX: u32>(
        self,
    ) -> RangedU32<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as u32 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as u32 {
                panic!("maximum must be higher or match");
            }
        }

        RangedU32(self.get() as u32)
    }

    /// Convert to [`RangedU64`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16, RangedU64};
    /// let ranged_u16 = RangedU16::<0, 2>::new::<1>();
    /// let ranged_u64: RangedU64<0, 2> = ranged_u16.to_ranged_u64();
    ///
    /// assert_eq!(ranged_u64.get(), ranged_u16.get().into());
    /// ```
    pub const fn to_ranged_u64<const OUT_MIN: u64, const OUT_MAX: u64>(
        self,
    ) -> RangedU64<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as u64 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as u64 {
                panic!("maximum must be higher or match");
            }
        }

        RangedU64(self.get() as u64)
    }

    /// Convert to [`RangedU128`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16, RangedU128};
    /// let ranged_u16 = RangedU16::<0, 2>::new::<1>();
    /// let ranged_u128: RangedU128<0, 2> = ranged_u16.to_ranged_u128();
    ///
    /// assert_eq!(ranged_u128.get(), ranged_u16.get().into());
    /// ```
    pub const fn to_ranged_u128<const OUT_MIN: u128, const OUT_MAX: u128>(
        self,
    ) -> RangedU128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as u128 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as u128 {
                panic!("maximum must be higher or match");
            }
        }

        RangedU128(self.get() as u128)
    }
}

impl<const MIN: u32, const MAX: u32> RangedU32<MIN, MAX> {
    /// Convert to [`RangedU8`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedU32};
    /// let ranged_u32 = RangedU32::<0, 2>::new::<1>();
    /// let ranged_u8: RangedU8<0, 2> = ranged_u32.to_ranged_u8();
    ///
    /// assert_eq!(ranged_u32.get(), ranged_u8.get().into());
    /// ```
    pub const fn to_ranged_u8<const OUT_MIN: u8, const OUT_MAX: u8>(
        self,
    ) -> RangedU8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u32) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u32) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU8(self.get() as u8)
    }

    /// Convert to [`RangedU16`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16, RangedU32};
    /// let ranged_u32 = RangedU32::<0, 2>::new::<1>();
    /// let ranged_u16: RangedU16<0, 2> = ranged_u32.to_ranged_u16();
    ///
    /// assert_eq!(ranged_u32.get(), ranged_u16.get().into());
    /// ```
    pub const fn to_ranged_u16<const OUT_MIN: u16, const OUT_MAX: u16>(
        self,
    ) -> RangedU16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u32) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u32) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU16(self.get() as u16)
    }

    /// Convert to [`RangedU32`].
    ///
    /// ```rust
    /// # use ranch::{RangedU32};
    /// let ranged_u32 = RangedU32::<0, 2>::new::<1>();
    /// let expanded_u32: RangedU32<0, 4> = ranged_u32.to_ranged_u32();
    ///
    /// assert_eq!(expanded_u32.get(), ranged_u32.get().into());
    /// ```
    pub const fn to_ranged_u32<const OUT_MIN: u32, const OUT_MAX: u32>(
        self,
    ) -> RangedU32<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU32(self.get())
    }

    /// Convert to [`RangedU64`].
    ///
    /// ```rust
    /// # use ranch::{RangedU32, RangedU64};
    /// let ranged_u32 = RangedU32::<0, 2>::new::<1>();
    /// let ranged_u64: RangedU64<0, 2> = ranged_u32.to_ranged_u64();
    ///
    /// assert_eq!(ranged_u64.get(), ranged_u32.get().into());
    /// ```
    pub const fn to_ranged_u64<const OUT_MIN: u64, const OUT_MAX: u64>(
        self,
    ) -> RangedU64<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as u64 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as u64 {
                panic!("maximum must be higher or match");
            }
        }

        RangedU64(self.get() as u64)
    }

    /// Convert to [`RangedU128`].
    ///
    /// ```rust
    /// # use ranch::{RangedU32, RangedU128};
    /// let ranged_u32 = RangedU32::<0, 2>::new::<1>();
    /// let ranged_u128: RangedU128<0, 2> = ranged_u32.to_ranged_u128();
    ///
    /// assert_eq!(ranged_u128.get(), ranged_u32.get().into());
    /// ```
    pub const fn to_ranged_u128<const OUT_MIN: u128, const OUT_MAX: u128>(
        self,
    ) -> RangedU128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as u128 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as u128 {
                panic!("maximum must be higher or match");
            }
        }

        RangedU128(self.get() as u128)
    }
}

impl<const MIN: u64, const MAX: u64> RangedU64<MIN, MAX> {
    /// Convert to [`RangedU8`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedU64};
    /// let ranged_u64 = RangedU64::<0, 2>::new::<1>();
    /// let ranged_u8: RangedU8<0, 2> = ranged_u64.to_ranged_u8();
    ///
    /// assert_eq!(ranged_u64.get(), ranged_u8.get().into());
    /// ```
    pub const fn to_ranged_u8<const OUT_MIN: u8, const OUT_MAX: u8>(
        self,
    ) -> RangedU8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u64) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u64) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU8(self.get() as u8)
    }

    /// Convert to [`RangedU16`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16, RangedU64};
    /// let ranged_u64 = RangedU64::<0, 2>::new::<1>();
    /// let ranged_u16: RangedU16<0, 2> = ranged_u64.to_ranged_u16();
    ///
    /// assert_eq!(ranged_u64.get(), ranged_u16.get().into());
    /// ```
    pub const fn to_ranged_u16<const OUT_MIN: u16, const OUT_MAX: u16>(
        self,
    ) -> RangedU16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u64) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u64) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU16(self.get() as u16)
    }

    /// Convert to [`RangedU32`].
    ///
    /// ```rust
    /// # use ranch::{RangedU32, RangedU64};
    /// let ranged_u64 = RangedU64::<0, 2>::new::<1>();
    /// let ranged_u32: RangedU32<0, 2> = ranged_u64.to_ranged_u32();
    ///
    /// assert_eq!(ranged_u64.get(), ranged_u32.get().into());
    /// ```
    pub const fn to_ranged_u32<const OUT_MIN: u32, const OUT_MAX: u32>(
        self,
    ) -> RangedU32<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u64) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u64) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU32(self.get() as u32)
    }

    /// Convert to [`RangedU64`].
    ///
    /// ```rust
    /// # use ranch::{RangedU64};
    /// let ranged_u64 = RangedU64::<0, 2>::new::<1>();
    /// let expanded_u64: RangedU64<0, 4> = ranged_u64.to_ranged_u64();
    ///
    /// assert_eq!(expanded_u64.get(), ranged_u64.get().into());
    /// ```
    pub const fn to_ranged_u64<const OUT_MIN: u64, const OUT_MAX: u64>(
        self,
    ) -> RangedU64<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU64(self.get())
    }

    /// Convert to [`RangedU128`].
    ///
    /// ```rust
    /// # use ranch::{RangedU64, RangedU128};
    /// let ranged_u64 = RangedU64::<0, 2>::new::<1>();
    /// let ranged_u128: RangedU128<0, 2> = ranged_u64.to_ranged_u128();
    ///
    /// assert_eq!(ranged_u128.get(), ranged_u64.get().into());
    /// ```
    pub const fn to_ranged_u128<const OUT_MIN: u128, const OUT_MAX: u128>(
        self,
    ) -> RangedU128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as u128 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as u128 {
                panic!("maximum must be higher or match");
            }
        }

        RangedU128(self.get() as u128)
    }
}

impl<const MIN: u128, const MAX: u128> RangedU128<MIN, MAX> {
    /// Convert to [`RangedU8`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedU128};
    /// let ranged_u128 = RangedU128::<0, 2>::new::<1>();
    /// let ranged_u8: RangedU8<0, 2> = ranged_u128.to_ranged_u8();
    ///
    /// assert_eq!(ranged_u128.get(), ranged_u8.get().into());
    /// ```
    pub const fn to_ranged_u8<const OUT_MIN: u8, const OUT_MAX: u8>(
        self,
    ) -> RangedU8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u128) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u128) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU8(self.get() as u8)
    }

    /// Convert to [`RangedU16`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16, RangedU128};
    /// let ranged_u128 = RangedU128::<0, 2>::new::<1>();
    /// let ranged_u16: RangedU16<0, 2> = ranged_u128.to_ranged_u16();
    ///
    /// assert_eq!(ranged_u128.get(), ranged_u16.get().into());
    /// ```
    pub const fn to_ranged_u16<const OUT_MIN: u16, const OUT_MAX: u16>(
        self,
    ) -> RangedU16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u128) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u128) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU16(self.get() as u16)
    }

    /// Convert to [`RangedU32`].
    ///
    /// ```rust
    /// # use ranch::{RangedU32, RangedU128};
    /// let ranged_u128 = RangedU128::<0, 2>::new::<1>();
    /// let ranged_u32: RangedU32<0, 2> = ranged_u128.to_ranged_u32();
    ///
    /// assert_eq!(ranged_u128.get(), ranged_u32.get().into());
    /// ```
    pub const fn to_ranged_u32<const OUT_MIN: u32, const OUT_MAX: u32>(
        self,
    ) -> RangedU32<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u128) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u128) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU32(self.get() as u32)
    }

    /// Convert to [`RangedU64`].
    ///
    /// ```rust
    /// # use ranch::{RangedU64, RangedU128};
    /// let ranged_u128 = RangedU128::<0, 2>::new::<1>();
    /// let ranged_u64: RangedU64<0, 2> = ranged_u128.to_ranged_u64();
    ///
    /// assert_eq!(ranged_u128.get(), ranged_u64.get().into());
    /// ```
    pub const fn to_ranged_u64<const OUT_MIN: u64, const OUT_MAX: u64>(
        self,
    ) -> RangedU64<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u128) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u128) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU64(self.get() as u64)
    }

    /// Convert to [`RangedU128`].
    ///
    /// ```rust
    /// # use ranch::{RangedU128};
    /// let ranged_u128 = RangedU128::<0, 2>::new::<1>();
    /// let expanded_u128: RangedU128<0, 4> = ranged_u128.to_ranged_u128();
    ///
    /// assert_eq!(expanded_u128.get(), ranged_u128.get().into());
    /// ```
    pub const fn to_ranged_u128<const OUT_MIN: u128, const OUT_MAX: u128>(
        self,
    ) -> RangedU128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedU128(self.get())
    }
}

impl<const MIN: i8, const MAX: i8> RangedI8<MIN, MAX> {
    /// Convert to [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8};
    /// let ranged_i8 = RangedI8::<0, 2>::new::<1>();
    /// let expanded_i8: RangedI8<0, 4> = ranged_i8.to_ranged_i8();
    ///
    /// assert_eq!(expanded_i8.get(), ranged_i8.get().into());
    /// ```
    pub const fn to_ranged_i8<const OUT_MIN: i8, const OUT_MAX: i8>(
        self,
    ) -> RangedI8<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI8(self.get())
    }

    /// Convert to [`RangedI16`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedI16};
    /// let ranged_i8 = RangedI8::<0, 2>::new::<1>();
    /// let ranged_i16: RangedI16<0, 2> = ranged_i8.to_ranged_i16();
    ///
    /// assert_eq!(ranged_i16.get(), ranged_i8.get().into());
    /// ```
    pub const fn to_ranged_i16<const OUT_MIN: i16, const OUT_MAX: i16>(
        self,
    ) -> RangedI16<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as i16 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as i16 {
                panic!("maximum must be higher or match");
            }
        }

        RangedI16(self.get() as i16)
    }

    /// Convert to [`RangedI32`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedI32};
    /// let ranged_i8 = RangedI8::<0, 2>::new::<1>();
    /// let ranged_i32: RangedI32<0, 2> = ranged_i8.to_ranged_i32();
    ///
    /// assert_eq!(ranged_i32.get(), ranged_i8.get().into());
    /// ```
    pub const fn to_ranged_i32<const OUT_MIN: i32, const OUT_MAX: i32>(
        self,
    ) -> RangedI32<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as i32 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as i32 {
                panic!("maximum must be higher or match");
            }
        }

        RangedI32(self.get() as i32)
    }

    /// Convert to [`RangedI64`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedI64};
    /// let ranged_i8 = RangedI8::<0, 2>::new::<1>();
    /// let ranged_i64: RangedI64<0, 2> = ranged_i8.to_ranged_i64();
    ///
    /// assert_eq!(ranged_i64.get(), ranged_i8.get().into());
    /// ```
    pub const fn to_ranged_i64<const OUT_MIN: i64, const OUT_MAX: i64>(
        self,
    ) -> RangedI64<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as i64 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as i64 {
                panic!("maximum must be higher or match");
            }
        }

        RangedI64(self.get() as i64)
    }

    /// Convert to [`RangedI128`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedI128};
    /// let ranged_i8 = RangedI8::<0, 2>::new::<1>();
    /// let ranged_i128: RangedI128<0, 2> = ranged_i8.to_ranged_i128();
    ///
    /// assert_eq!(ranged_i128.get(), ranged_i8.get().into());
    /// ```
    pub const fn to_ranged_i128<const OUT_MIN: i128, const OUT_MAX: i128>(
        self,
    ) -> RangedI128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as i128 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as i128 {
                panic!("maximum must be higher or match");
            }
        }

        RangedI128(self.get() as i128)
    }
}

impl<const MIN: i16, const MAX: i16> RangedI16<MIN, MAX> {
    /// Convert to [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedI16};
    /// let ranged_i16 = RangedI16::<0, 2>::new::<1>();
    /// let ranged_i8: RangedI8<0, 2> = ranged_i16.to_ranged_i8();
    ///
    /// assert_eq!(ranged_i16.get(), ranged_i8.get().into());
    /// ```
    pub const fn to_ranged_i8<const OUT_MIN: i8, const OUT_MAX: i8>(
        self,
    ) -> RangedI8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i16) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i16) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI8(self.get() as i8)
    }

    /// Convert to [`RangedI16`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16};
    /// let ranged_i16 = RangedI16::<0, 2>::new::<1>();
    /// let expanded_i16: RangedI16<0, 4> = ranged_i16.to_ranged_i16();
    ///
    /// assert_eq!(expanded_i16.get(), ranged_i16.get().into());
    /// ```
    pub const fn to_ranged_i16<const OUT_MIN: i16, const OUT_MAX: i16>(
        self,
    ) -> RangedI16<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI16(self.get())
    }

    /// Convert to [`RangedI32`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16, RangedI32};
    /// let ranged_i16 = RangedI16::<0, 2>::new::<1>();
    /// let ranged_i32: RangedI32<0, 2> = ranged_i16.to_ranged_i32();
    ///
    /// assert_eq!(ranged_i32.get(), ranged_i16.get().into());
    /// ```
    pub const fn to_ranged_i32<const OUT_MIN: i32, const OUT_MAX: i32>(
        self,
    ) -> RangedI32<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as i32 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as i32 {
                panic!("maximum must be higher or match");
            }
        }

        RangedI32(self.get() as i32)
    }

    /// Convert to [`RangedI64`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16, RangedI64};
    /// let ranged_i16 = RangedI16::<0, 2>::new::<1>();
    /// let ranged_i64: RangedI64<0, 2> = ranged_i16.to_ranged_i64();
    ///
    /// assert_eq!(ranged_i64.get(), ranged_i16.get().into());
    /// ```
    pub const fn to_ranged_i64<const OUT_MIN: i64, const OUT_MAX: i64>(
        self,
    ) -> RangedI64<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as i64 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as i64 {
                panic!("maximum must be higher or match");
            }
        }

        RangedI64(self.get() as i64)
    }

    /// Convert to [`RangedI128`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16, RangedI128};
    /// let ranged_i16 = RangedI16::<0, 2>::new::<1>();
    /// let ranged_i128: RangedI128<0, 2> = ranged_i16.to_ranged_i128();
    ///
    /// assert_eq!(ranged_i128.get(), ranged_i16.get().into());
    /// ```
    pub const fn to_ranged_i128<const OUT_MIN: i128, const OUT_MAX: i128>(
        self,
    ) -> RangedI128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as i128 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as i128 {
                panic!("maximum must be higher or match");
            }
        }

        RangedI128(self.get() as i128)
    }
}

impl<const MIN: i32, const MAX: i32> RangedI32<MIN, MAX> {
    /// Convert to [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedI32};
    /// let ranged_i32 = RangedI32::<0, 2>::new::<1>();
    /// let ranged_i8: RangedI8<0, 2> = ranged_i32.to_ranged_i8();
    ///
    /// assert_eq!(ranged_i32.get(), ranged_i8.get().into());
    /// ```
    pub const fn to_ranged_i8<const OUT_MIN: i8, const OUT_MAX: i8>(
        self,
    ) -> RangedI8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i32) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i32) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI8(self.get() as i8)
    }

    /// Convert to [`RangedI16`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16, RangedI32};
    /// let ranged_i32 = RangedI32::<0, 2>::new::<1>();
    /// let ranged_i16: RangedI16<0, 2> = ranged_i32.to_ranged_i16();
    ///
    /// assert_eq!(ranged_i32.get(), ranged_i16.get().into());
    /// ```
    pub const fn to_ranged_i16<const OUT_MIN: i16, const OUT_MAX: i16>(
        self,
    ) -> RangedI16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i32) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i32) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI16(self.get() as i16)
    }

    /// Convert to [`RangedI32`].
    ///
    /// ```rust
    /// # use ranch::{RangedI32};
    /// let ranged_i32 = RangedI32::<0, 2>::new::<1>();
    /// let expanded_i32: RangedI32<0, 4> = ranged_i32.to_ranged_i32();
    ///
    /// assert_eq!(expanded_i32.get(), ranged_i32.get().into());
    /// ```
    pub const fn to_ranged_i32<const OUT_MIN: i32, const OUT_MAX: i32>(
        self,
    ) -> RangedI32<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI32(self.get())
    }

    /// Convert to [`RangedI64`].
    ///
    /// ```rust
    /// # use ranch::{RangedI32, RangedI64};
    /// let ranged_i32 = RangedI32::<0, 2>::new::<1>();
    /// let ranged_i64: RangedI64<0, 2> = ranged_i32.to_ranged_i64();
    ///
    /// assert_eq!(ranged_i64.get(), ranged_i32.get().into());
    /// ```
    pub const fn to_ranged_i64<const OUT_MIN: i64, const OUT_MAX: i64>(
        self,
    ) -> RangedI64<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as i64 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as i64 {
                panic!("maximum must be higher or match");
            }
        }

        RangedI64(self.get() as i64)
    }

    /// Convert to [`RangedI128`].
    ///
    /// ```rust
    /// # use ranch::{RangedI32, RangedI128};
    /// let ranged_i32 = RangedI32::<0, 2>::new::<1>();
    /// let ranged_i128: RangedI128<0, 2> = ranged_i32.to_ranged_i128();
    ///
    /// assert_eq!(ranged_i128.get(), ranged_i32.get().into());
    /// ```
    pub const fn to_ranged_i128<const OUT_MIN: i128, const OUT_MAX: i128>(
        self,
    ) -> RangedI128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as i128 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as i128 {
                panic!("maximum must be higher or match");
            }
        }

        RangedI128(self.get() as i128)
    }
}

impl<const MIN: i64, const MAX: i64> RangedI64<MIN, MAX> {
    /// Convert to [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedI64};
    /// let ranged_i64 = RangedI64::<0, 2>::new::<1>();
    /// let ranged_i8: RangedI8<0, 2> = ranged_i64.to_ranged_i8();
    ///
    /// assert_eq!(ranged_i64.get(), ranged_i8.get().into());
    /// ```
    pub const fn to_ranged_i8<const OUT_MIN: i8, const OUT_MAX: i8>(
        self,
    ) -> RangedI8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i64) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i64) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI8(self.get() as i8)
    }

    /// Convert to [`RangedI16`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16, RangedI64};
    /// let ranged_i64 = RangedI64::<0, 2>::new::<1>();
    /// let ranged_i16: RangedI16<0, 2> = ranged_i64.to_ranged_i16();
    ///
    /// assert_eq!(ranged_i64.get(), ranged_i16.get().into());
    /// ```
    pub const fn to_ranged_i16<const OUT_MIN: i16, const OUT_MAX: i16>(
        self,
    ) -> RangedI16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i64) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i64) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI16(self.get() as i16)
    }

    /// Convert to [`RangedI32`].
    ///
    /// ```rust
    /// # use ranch::{RangedI32, RangedI64};
    /// let ranged_i64 = RangedI64::<0, 2>::new::<1>();
    /// let ranged_i32: RangedI32<0, 2> = ranged_i64.to_ranged_i32();
    ///
    /// assert_eq!(ranged_i64.get(), ranged_i32.get().into());
    /// ```
    pub const fn to_ranged_i32<const OUT_MIN: i32, const OUT_MAX: i32>(
        self,
    ) -> RangedI32<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i64) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i64) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI32(self.get() as i32)
    }

    /// Convert to [`RangedI64`].
    ///
    /// ```rust
    /// # use ranch::{RangedI64};
    /// let ranged_i64 = RangedI64::<0, 2>::new::<1>();
    /// let expanded_i64: RangedI64<0, 4> = ranged_i64.to_ranged_i64();
    ///
    /// assert_eq!(expanded_i64.get(), ranged_i64.get().into());
    /// ```
    pub const fn to_ranged_i64<const OUT_MIN: i64, const OUT_MAX: i64>(
        self,
    ) -> RangedI64<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI64(self.get())
    }

    /// Convert to [`RangedI128`].
    ///
    /// ```rust
    /// # use ranch::{RangedI64, RangedI128};
    /// let ranged_i64 = RangedI64::<0, 2>::new::<1>();
    /// let ranged_i128: RangedI128<0, 2> = ranged_i64.to_ranged_i128();
    ///
    /// assert_eq!(ranged_i128.get(), ranged_i64.get().into());
    /// ```
    pub const fn to_ranged_i128<const OUT_MIN: i128, const OUT_MAX: i128>(
        self,
    ) -> RangedI128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN as i128 {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX as i128 {
                panic!("maximum must be higher or match");
            }
        }

        RangedI128(self.get() as i128)
    }
}

impl<const MIN: i128, const MAX: i128> RangedI128<MIN, MAX> {
    /// Convert to [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedI128};
    /// let ranged_i128 = RangedI128::<0, 2>::new::<1>();
    /// let ranged_i8: RangedI8<0, 2> = ranged_i128.to_ranged_i8();
    ///
    /// assert_eq!(ranged_i128.get(), ranged_i8.get().into());
    /// ```
    pub const fn to_ranged_i8<const OUT_MIN: i8, const OUT_MAX: i8>(
        self,
    ) -> RangedI8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i128) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i128) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI8(self.get() as i8)
    }

    /// Convert to [`RangedI16`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16, RangedI128};
    /// let ranged_i128 = RangedI128::<0, 2>::new::<1>();
    /// let ranged_i16: RangedI16<0, 2> = ranged_i128.to_ranged_i16();
    ///
    /// assert_eq!(ranged_i128.get(), ranged_i16.get().into());
    /// ```
    pub const fn to_ranged_i16<const OUT_MIN: i16, const OUT_MAX: i16>(
        self,
    ) -> RangedI16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i128) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i128) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI16(self.get() as i16)
    }

    /// Convert to [`RangedI32`].
    ///
    /// ```rust
    /// # use ranch::{RangedI32, RangedI128};
    /// let ranged_i128 = RangedI128::<0, 2>::new::<1>();
    /// let ranged_i32: RangedI32<0, 2> = ranged_i128.to_ranged_i32();
    ///
    /// assert_eq!(ranged_i128.get(), ranged_i32.get().into());
    /// ```
    pub const fn to_ranged_i32<const OUT_MIN: i32, const OUT_MAX: i32>(
        self,
    ) -> RangedI32<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i128) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i128) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI32(self.get() as i32)
    }

    /// Convert to [`RangedI64`].
    ///
    /// ```rust
    /// # use ranch::{RangedI64, RangedI128};
    /// let ranged_i128 = RangedI128::<0, 2>::new::<1>();
    /// let ranged_i64: RangedI64<0, 2> = ranged_i128.to_ranged_i64();
    ///
    /// assert_eq!(ranged_i128.get(), ranged_i64.get().into());
    /// ```
    pub const fn to_ranged_i64<const OUT_MIN: i64, const OUT_MAX: i64>(
        self,
    ) -> RangedI64<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i128) > MIN {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i128) < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI64(self.get() as i64)
    }

    /// Convert to [`RangedI128`].
    ///
    /// ```rust
    /// # use ranch::{RangedI128};
    /// let ranged_i128 = RangedI128::<0, 2>::new::<1>();
    /// let expanded_i128: RangedI128<0, 4> = ranged_i128.to_ranged_i128();
    ///
    /// assert_eq!(expanded_i128.get(), ranged_i128.get().into());
    /// ```
    pub const fn to_ranged_i128<const OUT_MIN: i128, const OUT_MAX: i128>(
        self,
    ) -> RangedI128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > MIN {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < MAX {
                panic!("maximum must be higher or match");
            }
        }

        RangedI128(self.get())
    }
}

impl<const MIN: i8, const MAX: i8> RangedI8<MIN, MAX> {
    /// Convert to [`RangedU8`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedI8};
    /// let ranged_i8 = RangedI8::<0, 2>::new::<1>();
    /// let expanded_u8: RangedU8<0, 4> = ranged_i8.to_ranged_u8();
    ///
    /// assert_eq!(Ok(expanded_u8.get()), ranged_i8.get().try_into());
    /// ```
    pub const fn to_ranged_u8<const OUT_MIN: u8, const OUT_MAX: u8>(
        self,
    ) -> RangedU8<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > i8_to_u8(MIN) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i8_to_u8(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU8(self.get() as _)
    }

    /// Convert to [`RangedU16`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16, RangedI8};
    /// let ranged_i8 = RangedI8::<0, 2>::new::<1>();
    /// let expanded_u16: RangedU16<0, 4> = ranged_i8.to_ranged_u16();
    ///
    /// assert_eq!(Ok(expanded_u16.get()), ranged_i8.get().try_into());
    /// ```
    pub const fn to_ranged_u16<const OUT_MIN: u16, const OUT_MAX: u16>(
        self,
    ) -> RangedU16<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > i16_to_u16(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i16_to_u16(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU16(self.get() as _)
    }

    /// Convert to [`RangedU32`].
    ///
    /// ```rust
    /// # use ranch::{RangedU32, RangedI8};
    /// let ranged_i8 = RangedI8::<0, 2>::new::<1>();
    /// let expanded_u32: RangedU32<0, 4> = ranged_i8.to_ranged_u32();
    ///
    /// assert_eq!(Ok(expanded_u32.get()), ranged_i8.get().try_into());
    /// ```
    pub const fn to_ranged_u32<const OUT_MIN: u32, const OUT_MAX: u32>(
        self,
    ) -> RangedU32<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > i32_to_u32(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i32_to_u32(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU32(self.get() as _)
    }

    /// Convert to [`RangedU64`].
    ///
    /// ```rust
    /// # use ranch::{RangedU64, RangedI8};
    /// let ranged_i8 = RangedI8::<0, 2>::new::<1>();
    /// let expanded_u64: RangedU64<0, 4> = ranged_i8.to_ranged_u64();
    ///
    /// assert_eq!(Ok(expanded_u64.get()), ranged_i8.get().try_into());
    /// ```
    pub const fn to_ranged_u64<const OUT_MIN: u64, const OUT_MAX: u64>(
        self,
    ) -> RangedU64<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > i64_to_u64(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i64_to_u64(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU64(self.get() as _)
    }

    /// Convert to [`RangedU128`].
    ///
    /// ```rust
    /// # use ranch::{RangedU128, RangedI8};
    /// let ranged_i8 = RangedI8::<0, 2>::new::<1>();
    /// let expanded_u128: RangedU128<0, 4> = ranged_i8.to_ranged_u128();
    ///
    /// assert_eq!(Ok(expanded_u128.get()), ranged_i8.get().try_into());
    /// ```
    pub const fn to_ranged_u128<const OUT_MIN: u128, const OUT_MAX: u128>(
        self,
    ) -> RangedU128<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > i128_to_u128(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i128_to_u128(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU128(self.get() as _)
    }
}

impl<const MIN: i16, const MAX: i16> RangedI16<MIN, MAX> {
    /// Convert to [`RangedU8`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedI16};
    /// let ranged_i16 = RangedI16::<0, 2>::new::<1>();
    /// let expanded_u8: RangedU8<0, 4> = ranged_i16.to_ranged_u8();
    ///
    /// assert_eq!(Ok(expanded_u8.get()), ranged_i16.get().try_into());
    /// ```
    pub const fn to_ranged_u8<const OUT_MIN: u8, const OUT_MAX: u8>(
        self,
    ) -> RangedU8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u16) > i16_to_u16(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u16) < i16_to_u16(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU8(self.get() as _)
    }

    /// Convert to [`RangedU16`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16, RangedI16};
    /// let ranged_i16 = RangedI16::<0, 2>::new::<1>();
    /// let expanded_u16: RangedU16<0, 4> = ranged_i16.to_ranged_u16();
    ///
    /// assert_eq!(Ok(expanded_u16.get()), ranged_i16.get().try_into());
    /// ```
    pub const fn to_ranged_u16<const OUT_MIN: u16, const OUT_MAX: u16>(
        self,
    ) -> RangedU16<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > i16_to_u16(MIN) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i16_to_u16(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU16(self.get() as _)
    }

    /// Convert to [`RangedU32`].
    ///
    /// ```rust
    /// # use ranch::{RangedU32, RangedI16};
    /// let ranged_i16 = RangedI16::<0, 2>::new::<1>();
    /// let expanded_u32: RangedU32<0, 4> = ranged_i16.to_ranged_u32();
    ///
    /// assert_eq!(Ok(expanded_u32.get()), ranged_i16.get().try_into());
    /// ```
    pub const fn to_ranged_u32<const OUT_MIN: u32, const OUT_MAX: u32>(
        self,
    ) -> RangedU32<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > i32_to_u32(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i32_to_u32(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU32(self.get() as _)
    }

    /// Convert to [`RangedU64`].
    ///
    /// ```rust
    /// # use ranch::{RangedU64, RangedI16};
    /// let ranged_i16 = RangedI16::<0, 2>::new::<1>();
    /// let expanded_u64: RangedU64<0, 4> = ranged_i16.to_ranged_u64();
    ///
    /// assert_eq!(Ok(expanded_u64.get()), ranged_i16.get().try_into());
    /// ```
    pub const fn to_ranged_u64<const OUT_MIN: u64, const OUT_MAX: u64>(
        self,
    ) -> RangedU64<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > i64_to_u64(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i64_to_u64(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU64(self.get() as _)
    }

    /// Convert to [`RangedU128`].
    ///
    /// ```rust
    /// # use ranch::{RangedU128, RangedI16};
    /// let ranged_i16 = RangedI16::<0, 2>::new::<1>();
    /// let expanded_u128: RangedU128<0, 4> = ranged_i16.to_ranged_u128();
    ///
    /// assert_eq!(Ok(expanded_u128.get()), ranged_i16.get().try_into());
    /// ```
    pub const fn to_ranged_u128<const OUT_MIN: u128, const OUT_MAX: u128>(
        self,
    ) -> RangedU128<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > i128_to_u128(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i128_to_u128(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU128(self.get() as _)
    }
}

impl<const MIN: i32, const MAX: i32> RangedI32<MIN, MAX> {
    /// Convert to [`RangedU8`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedI32};
    /// let ranged_i32 = RangedI32::<0, 2>::new::<1>();
    /// let expanded_u8: RangedU8<0, 4> = ranged_i32.to_ranged_u8();
    ///
    /// assert_eq!(Ok(expanded_u8.get()), ranged_i32.get().try_into());
    /// ```
    pub const fn to_ranged_u8<const OUT_MIN: u8, const OUT_MAX: u8>(
        self,
    ) -> RangedU8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u32) > i32_to_u32(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u32) < i32_to_u32(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU8(self.get() as _)
    }

    /// Convert to [`RangedU16`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16, RangedI32};
    /// let ranged_i32 = RangedI32::<0, 2>::new::<1>();
    /// let expanded_u16: RangedU16<0, 4> = ranged_i32.to_ranged_u16();
    ///
    /// assert_eq!(Ok(expanded_u16.get()), ranged_i32.get().try_into());
    /// ```
    pub const fn to_ranged_u16<const OUT_MIN: u16, const OUT_MAX: u16>(
        self,
    ) -> RangedU16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u32) > i32_to_u32(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u32) < i32_to_u32(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU16(self.get() as _)
    }

    /// Convert to [`RangedU32`].
    ///
    /// ```rust
    /// # use ranch::{RangedU32, RangedI32};
    /// let ranged_i32 = RangedI32::<0, 2>::new::<1>();
    /// let expanded_u32: RangedU32<0, 4> = ranged_i32.to_ranged_u32();
    ///
    /// assert_eq!(Ok(expanded_u32.get()), ranged_i32.get().try_into());
    /// ```
    pub const fn to_ranged_u32<const OUT_MIN: u32, const OUT_MAX: u32>(
        self,
    ) -> RangedU32<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > i32_to_u32(MIN) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i32_to_u32(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU32(self.get() as _)
    }

    /// Convert to [`RangedU64`].
    ///
    /// ```rust
    /// # use ranch::{RangedU64, RangedI32};
    /// let ranged_i32 = RangedI32::<0, 2>::new::<1>();
    /// let expanded_u64: RangedU64<0, 4> = ranged_i32.to_ranged_u64();
    ///
    /// assert_eq!(Ok(expanded_u64.get()), ranged_i32.get().try_into());
    /// ```
    pub const fn to_ranged_u64<const OUT_MIN: u64, const OUT_MAX: u64>(
        self,
    ) -> RangedU64<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > i64_to_u64(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i64_to_u64(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU64(self.get() as _)
    }

    /// Convert to [`RangedU128`].
    ///
    /// ```rust
    /// # use ranch::{RangedU128, RangedI32};
    /// let ranged_i32 = RangedI32::<0, 2>::new::<1>();
    /// let expanded_u128: RangedU128<0, 4> = ranged_i32.to_ranged_u128();
    ///
    /// assert_eq!(Ok(expanded_u128.get()), ranged_i32.get().try_into());
    /// ```
    pub const fn to_ranged_u128<const OUT_MIN: u128, const OUT_MAX: u128>(
        self,
    ) -> RangedU128<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > i128_to_u128(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i128_to_u128(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU128(self.get() as _)
    }
}

impl<const MIN: i64, const MAX: i64> RangedI64<MIN, MAX> {
    /// Convert to [`RangedU8`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedI64};
    /// let ranged_i64 = RangedI64::<0, 2>::new::<1>();
    /// let expanded_u8: RangedU8<0, 4> = ranged_i64.to_ranged_u8();
    ///
    /// assert_eq!(Ok(expanded_u8.get()), ranged_i64.get().try_into());
    /// ```
    pub const fn to_ranged_u8<const OUT_MIN: u8, const OUT_MAX: u8>(
        self,
    ) -> RangedU8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u64) > i64_to_u64(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u64) < i64_to_u64(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU8(self.get() as _)
    }

    /// Convert to [`RangedU16`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16, RangedI64};
    /// let ranged_i64 = RangedI64::<0, 2>::new::<1>();
    /// let expanded_u16: RangedU16<0, 4> = ranged_i64.to_ranged_u16();
    ///
    /// assert_eq!(Ok(expanded_u16.get()), ranged_i64.get().try_into());
    /// ```
    pub const fn to_ranged_u16<const OUT_MIN: u16, const OUT_MAX: u16>(
        self,
    ) -> RangedU16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u64) > i64_to_u64(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u64) < i64_to_u64(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU16(self.get() as _)
    }

    /// Convert to [`RangedU32`].
    ///
    /// ```rust
    /// # use ranch::{RangedU32, RangedI64};
    /// let ranged_i64 = RangedI64::<0, 2>::new::<1>();
    /// let expanded_u32: RangedU32<0, 4> = ranged_i64.to_ranged_u32();
    ///
    /// assert_eq!(Ok(expanded_u32.get()), ranged_i64.get().try_into());
    /// ```
    pub const fn to_ranged_u32<const OUT_MIN: u32, const OUT_MAX: u32>(
        self,
    ) -> RangedU32<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u64) > i64_to_u64(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u64) < i64_to_u64(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU32(self.get() as _)
    }

    /// Convert to [`RangedU64`].
    ///
    /// ```rust
    /// # use ranch::{RangedU64, RangedI64};
    /// let ranged_i64 = RangedI64::<0, 2>::new::<1>();
    /// let expanded_u64: RangedU64<0, 4> = ranged_i64.to_ranged_u64();
    ///
    /// assert_eq!(Ok(expanded_u64.get()), ranged_i64.get().try_into());
    /// ```
    pub const fn to_ranged_u64<const OUT_MIN: u64, const OUT_MAX: u64>(
        self,
    ) -> RangedU64<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > i64_to_u64(MIN) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i64_to_u64(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU64(self.get() as _)
    }

    /// Convert to [`RangedU128`].
    ///
    /// ```rust
    /// # use ranch::{RangedU128, RangedI64};
    /// let ranged_i64 = RangedI64::<0, 2>::new::<1>();
    /// let expanded_u128: RangedU128<0, 4> = ranged_i64.to_ranged_u128();
    ///
    /// assert_eq!(Ok(expanded_u128.get()), ranged_i64.get().try_into());
    /// ```
    pub const fn to_ranged_u128<const OUT_MIN: u128, const OUT_MAX: u128>(
        self,
    ) -> RangedU128<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > i128_to_u128(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i128_to_u128(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU128(self.get() as _)
    }
}

impl<const MIN: i128, const MAX: i128> RangedI128<MIN, MAX> {
    /// Convert to [`RangedU8`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedI128};
    /// let ranged_i128 = RangedI128::<0, 2>::new::<1>();
    /// let expanded_u8: RangedU8<0, 4> = ranged_i128.to_ranged_u8();
    ///
    /// assert_eq!(Ok(expanded_u8.get()), ranged_i128.get().try_into());
    /// ```
    pub const fn to_ranged_u8<const OUT_MIN: u8, const OUT_MAX: u8>(
        self,
    ) -> RangedU8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u128) > i128_to_u128(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u128) < i128_to_u128(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU8(self.get() as _)
    }

    /// Convert to [`RangedU16`].
    ///
    /// ```rust
    /// # use ranch::{RangedU16, RangedI128};
    /// let ranged_i128 = RangedI128::<0, 2>::new::<1>();
    /// let expanded_u16: RangedU16<0, 4> = ranged_i128.to_ranged_u16();
    ///
    /// assert_eq!(Ok(expanded_u16.get()), ranged_i128.get().try_into());
    /// ```
    pub const fn to_ranged_u16<const OUT_MIN: u16, const OUT_MAX: u16>(
        self,
    ) -> RangedU16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u128) > i128_to_u128(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u128) < i128_to_u128(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU16(self.get() as _)
    }

    /// Convert to [`RangedU32`].
    ///
    /// ```rust
    /// # use ranch::{RangedU32, RangedI128};
    /// let ranged_i128 = RangedI128::<0, 2>::new::<1>();
    /// let expanded_u32: RangedU32<0, 4> = ranged_i128.to_ranged_u32();
    ///
    /// assert_eq!(Ok(expanded_u32.get()), ranged_i128.get().try_into());
    /// ```
    pub const fn to_ranged_u32<const OUT_MIN: u32, const OUT_MAX: u32>(
        self,
    ) -> RangedU32<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u128) > i128_to_u128(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u128) < i128_to_u128(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU32(self.get() as _)
    }

    /// Convert to [`RangedU64`].
    ///
    /// ```rust
    /// # use ranch::{RangedU64, RangedI128};
    /// let ranged_i128 = RangedI128::<0, 2>::new::<1>();
    /// let expanded_u64: RangedU64<0, 4> = ranged_i128.to_ranged_u64();
    ///
    /// assert_eq!(Ok(expanded_u64.get()), ranged_i128.get().try_into());
    /// ```
    pub const fn to_ranged_u64<const OUT_MIN: u64, const OUT_MAX: u64>(
        self,
    ) -> RangedU64<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as u128) > i128_to_u128(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as u128) < i128_to_u128(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU64(self.get() as _)
    }

    /// Convert to [`RangedU128`].
    ///
    /// ```rust
    /// # use ranch::{RangedU128, RangedI128};
    /// let ranged_i128 = RangedI128::<0, 2>::new::<1>();
    /// let expanded_u128: RangedU128<0, 4> = ranged_i128.to_ranged_u128();
    ///
    /// assert_eq!(Ok(expanded_u128.get()), ranged_i128.get().try_into());
    /// ```
    pub const fn to_ranged_u128<const OUT_MIN: u128, const OUT_MAX: u128>(
        self,
    ) -> RangedU128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > i128_to_u128(MIN) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < i128_to_u128(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedU128(self.get() as _)
    }
}

impl<const MIN: u8, const MAX: u8> RangedU8<MIN, MAX> {
    /// Convert to [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedU8};
    /// let ranged_u8 = RangedU8::<0, 2>::new::<1>();
    /// let expanded_i8: RangedI8<0, 4> = ranged_u8.to_ranged_i8();
    ///
    /// assert_eq!(Ok(expanded_i8.get()), ranged_u8.get().try_into());
    /// ```
    pub const fn to_ranged_i8<const OUT_MIN: i8, const OUT_MAX: i8>(
        self,
    ) -> RangedI8<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > u8_to_i8(MIN) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u8_to_i8(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI8(self.get() as _)
    }

    /// Convert to [`RangedI16`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16, RangedU8};
    /// let ranged_u8 = RangedU8::<0, 2>::new::<1>();
    /// let expanded_i16: RangedI16<0, 4> = ranged_u8.to_ranged_i16();
    ///
    /// assert_eq!(Ok(expanded_i16.get()), ranged_u8.get().try_into());
    /// ```
    pub const fn to_ranged_i16<const OUT_MIN: i16, const OUT_MAX: i16>(
        self,
    ) -> RangedI16<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > u16_to_i16(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u16_to_i16(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI16(self.get() as _)
    }

    /// Convert to [`RangedI32`].
    ///
    /// ```rust
    /// # use ranch::{RangedI32, RangedU8};
    /// let ranged_u8 = RangedU8::<0, 2>::new::<1>();
    /// let expanded_i32: RangedI32<0, 4> = ranged_u8.to_ranged_i32();
    ///
    /// assert_eq!(Ok(expanded_i32.get()), ranged_u8.get().try_into());
    /// ```
    pub const fn to_ranged_i32<const OUT_MIN: i32, const OUT_MAX: i32>(
        self,
    ) -> RangedI32<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > u32_to_i32(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u32_to_i32(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI32(self.get() as _)
    }

    /// Convert to [`RangedI64`].
    ///
    /// ```rust
    /// # use ranch::{RangedI64, RangedU8};
    /// let ranged_u8 = RangedU8::<0, 2>::new::<1>();
    /// let expanded_i64: RangedI64<0, 4> = ranged_u8.to_ranged_i64();
    ///
    /// assert_eq!(Ok(expanded_i64.get()), ranged_u8.get().try_into());
    /// ```
    pub const fn to_ranged_i64<const OUT_MIN: i64, const OUT_MAX: i64>(
        self,
    ) -> RangedI64<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > u64_to_i64(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u64_to_i64(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI64(self.get() as _)
    }

    /// Convert to [`RangedI128`].
    ///
    /// ```rust
    /// # use ranch::{RangedI128, RangedU8};
    /// let ranged_u8 = RangedU8::<0, 2>::new::<1>();
    /// let expanded_i128: RangedI128<0, 4> = ranged_u8.to_ranged_i128();
    ///
    /// assert_eq!(Ok(expanded_i128.get()), ranged_u8.get().try_into());
    /// ```
    pub const fn to_ranged_i128<const OUT_MIN: i128, const OUT_MAX: i128>(
        self,
    ) -> RangedI128<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > u128_to_i128(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u128_to_i128(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI128(self.get() as _)
    }
}

impl<const MIN: u16, const MAX: u16> RangedU16<MIN, MAX> {
    /// Convert to [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedU16};
    /// let ranged_u16 = RangedU16::<0, 2>::new::<1>();
    /// let expanded_i8: RangedI8<0, 4> = ranged_u16.to_ranged_i8();
    ///
    /// assert_eq!(Ok(expanded_i8.get()), ranged_u16.get().try_into());
    /// ```
    pub const fn to_ranged_i8<const OUT_MIN: i8, const OUT_MAX: i8>(
        self,
    ) -> RangedI8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i16) > u16_to_i16(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i16) < u16_to_i16(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI8(self.get() as _)
    }

    /// Convert to [`RangedI16`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16, RangedU16};
    /// let ranged_u16 = RangedU16::<0, 2>::new::<1>();
    /// let expanded_i16: RangedI16<0, 4> = ranged_u16.to_ranged_i16();
    ///
    /// assert_eq!(Ok(expanded_i16.get()), ranged_u16.get().try_into());
    /// ```
    pub const fn to_ranged_i16<const OUT_MIN: i16, const OUT_MAX: i16>(
        self,
    ) -> RangedI16<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > u16_to_i16(MIN) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u16_to_i16(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI16(self.get() as _)
    }

    /// Convert to [`RangedI32`].
    ///
    /// ```rust
    /// # use ranch::{RangedI32, RangedU16};
    /// let ranged_u16 = RangedU16::<0, 2>::new::<1>();
    /// let expanded_i32: RangedI32<0, 4> = ranged_u16.to_ranged_i32();
    ///
    /// assert_eq!(Ok(expanded_i32.get()), ranged_u16.get().try_into());
    /// ```
    pub const fn to_ranged_i32<const OUT_MIN: i32, const OUT_MAX: i32>(
        self,
    ) -> RangedI32<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > u32_to_i32(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u32_to_i32(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI32(self.get() as _)
    }

    /// Convert to [`RangedI64`].
    ///
    /// ```rust
    /// # use ranch::{RangedI64, RangedU16};
    /// let ranged_u16 = RangedU16::<0, 2>::new::<1>();
    /// let expanded_i64: RangedI64<0, 4> = ranged_u16.to_ranged_i64();
    ///
    /// assert_eq!(Ok(expanded_i64.get()), ranged_u16.get().try_into());
    /// ```
    pub const fn to_ranged_i64<const OUT_MIN: i64, const OUT_MAX: i64>(
        self,
    ) -> RangedI64<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > u64_to_i64(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u64_to_i64(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI64(self.get() as _)
    }

    /// Convert to [`RangedI128`].
    ///
    /// ```rust
    /// # use ranch::{RangedI128, RangedU16};
    /// let ranged_u16 = RangedU16::<0, 2>::new::<1>();
    /// let expanded_i128: RangedI128<0, 4> = ranged_u16.to_ranged_i128();
    ///
    /// assert_eq!(Ok(expanded_i128.get()), ranged_u16.get().try_into());
    /// ```
    pub const fn to_ranged_i128<const OUT_MIN: i128, const OUT_MAX: i128>(
        self,
    ) -> RangedI128<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > u128_to_i128(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u128_to_i128(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI128(self.get() as _)
    }
}

impl<const MIN: u32, const MAX: u32> RangedU32<MIN, MAX> {
    /// Convert to [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedU32};
    /// let ranged_u32 = RangedU32::<0, 2>::new::<1>();
    /// let expanded_i8: RangedI8<0, 4> = ranged_u32.to_ranged_i8();
    ///
    /// assert_eq!(Ok(expanded_i8.get()), ranged_u32.get().try_into());
    /// ```
    pub const fn to_ranged_i8<const OUT_MIN: i8, const OUT_MAX: i8>(
        self,
    ) -> RangedI8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i32) > u32_to_i32(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i32) < u32_to_i32(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI8(self.get() as _)
    }

    /// Convert to [`RangedI16`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16, RangedU32};
    /// let ranged_u32 = RangedU32::<0, 2>::new::<1>();
    /// let expanded_i16: RangedI16<0, 4> = ranged_u32.to_ranged_i16();
    ///
    /// assert_eq!(Ok(expanded_i16.get()), ranged_u32.get().try_into());
    /// ```
    pub const fn to_ranged_i16<const OUT_MIN: i16, const OUT_MAX: i16>(
        self,
    ) -> RangedI16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i32) > u32_to_i32(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i32) < u32_to_i32(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI16(self.get() as _)
    }

    /// Convert to [`RangedI32`].
    ///
    /// ```rust
    /// # use ranch::{RangedI32, RangedU32};
    /// let ranged_u32 = RangedU32::<0, 2>::new::<1>();
    /// let expanded_i32: RangedI32<0, 4> = ranged_u32.to_ranged_i32();
    ///
    /// assert_eq!(Ok(expanded_i32.get()), ranged_u32.get().try_into());
    /// ```
    pub const fn to_ranged_i32<const OUT_MIN: i32, const OUT_MAX: i32>(
        self,
    ) -> RangedI32<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > u32_to_i32(MIN) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u32_to_i32(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI32(self.get() as _)
    }

    /// Convert to [`RangedI64`].
    ///
    /// ```rust
    /// # use ranch::{RangedI64, RangedU32};
    /// let ranged_u32 = RangedU32::<0, 2>::new::<1>();
    /// let expanded_i64: RangedI64<0, 4> = ranged_u32.to_ranged_i64();
    ///
    /// assert_eq!(Ok(expanded_i64.get()), ranged_u32.get().try_into());
    /// ```
    pub const fn to_ranged_i64<const OUT_MIN: i64, const OUT_MAX: i64>(
        self,
    ) -> RangedI64<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > u64_to_i64(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u64_to_i64(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI64(self.get() as _)
    }

    /// Convert to [`RangedI128`].
    ///
    /// ```rust
    /// # use ranch::{RangedI128, RangedU32};
    /// let ranged_u32 = RangedU32::<0, 2>::new::<1>();
    /// let expanded_i128: RangedI128<0, 4> = ranged_u32.to_ranged_i128();
    ///
    /// assert_eq!(Ok(expanded_i128.get()), ranged_u32.get().try_into());
    /// ```
    pub const fn to_ranged_i128<const OUT_MIN: i128, const OUT_MAX: i128>(
        self,
    ) -> RangedI128<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > u128_to_i128(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u128_to_i128(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI128(self.get() as _)
    }
}

impl<const MIN: u64, const MAX: u64> RangedU64<MIN, MAX> {
    /// Convert to [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedU64};
    /// let ranged_u64 = RangedU64::<0, 2>::new::<1>();
    /// let expanded_i8: RangedI8<0, 4> = ranged_u64.to_ranged_i8();
    ///
    /// assert_eq!(Ok(expanded_i8.get()), ranged_u64.get().try_into());
    /// ```
    pub const fn to_ranged_i8<const OUT_MIN: i8, const OUT_MAX: i8>(
        self,
    ) -> RangedI8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i64) > u64_to_i64(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i64) < u64_to_i64(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI8(self.get() as _)
    }

    /// Convert to [`RangedI16`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16, RangedU64};
    /// let ranged_u64 = RangedU64::<0, 2>::new::<1>();
    /// let expanded_i16: RangedI16<0, 4> = ranged_u64.to_ranged_i16();
    ///
    /// assert_eq!(Ok(expanded_i16.get()), ranged_u64.get().try_into());
    /// ```
    pub const fn to_ranged_i16<const OUT_MIN: i16, const OUT_MAX: i16>(
        self,
    ) -> RangedI16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i64) > u64_to_i64(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i64) < u64_to_i64(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI16(self.get() as _)
    }

    /// Convert to [`RangedI32`].
    ///
    /// ```rust
    /// # use ranch::{RangedI32, RangedU64};
    /// let ranged_u64 = RangedU64::<0, 2>::new::<1>();
    /// let expanded_i32: RangedI32<0, 4> = ranged_u64.to_ranged_i32();
    ///
    /// assert_eq!(Ok(expanded_i32.get()), ranged_u64.get().try_into());
    /// ```
    pub const fn to_ranged_i32<const OUT_MIN: i32, const OUT_MAX: i32>(
        self,
    ) -> RangedI32<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i64) > u64_to_i64(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i64) < u64_to_i64(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI32(self.get() as _)
    }

    /// Convert to [`RangedI64`].
    ///
    /// ```rust
    /// # use ranch::{RangedI64, RangedU64};
    /// let ranged_u64 = RangedU64::<0, 2>::new::<1>();
    /// let expanded_i64: RangedI64<0, 4> = ranged_u64.to_ranged_i64();
    ///
    /// assert_eq!(Ok(expanded_i64.get()), ranged_u64.get().try_into());
    /// ```
    pub const fn to_ranged_i64<const OUT_MIN: i64, const OUT_MAX: i64>(
        self,
    ) -> RangedI64<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > u64_to_i64(MIN) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u64_to_i64(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI64(self.get() as _)
    }

    /// Convert to [`RangedI128`].
    ///
    /// ```rust
    /// # use ranch::{RangedI128, RangedU64};
    /// let ranged_u64 = RangedU64::<0, 2>::new::<1>();
    /// let expanded_i128: RangedI128<0, 4> = ranged_u64.to_ranged_i128();
    ///
    /// assert_eq!(Ok(expanded_i128.get()), ranged_u64.get().try_into());
    /// ```
    pub const fn to_ranged_i128<const OUT_MIN: i128, const OUT_MAX: i128>(
        self,
    ) -> RangedI128<OUT_MIN, OUT_MAX> {
        const {
            let (min, max) = (MIN as _, MAX as _);

            if OUT_MIN > u128_to_i128(min) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u128_to_i128(max) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI128(self.get() as _)
    }
}

impl<const MIN: u128, const MAX: u128> RangedU128<MIN, MAX> {
    /// Convert to [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedU128};
    /// let ranged_u128 = RangedU128::<0, 2>::new::<1>();
    /// let expanded_i8: RangedI8<0, 4> = ranged_u128.to_ranged_i8();
    ///
    /// assert_eq!(Ok(expanded_i8.get()), ranged_u128.get().try_into());
    /// ```
    pub const fn to_ranged_i8<const OUT_MIN: i8, const OUT_MAX: i8>(
        self,
    ) -> RangedI8<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i128) > u128_to_i128(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i128) < u128_to_i128(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI8(self.get() as _)
    }

    /// Convert to [`RangedI16`].
    ///
    /// ```rust
    /// # use ranch::{RangedI16, RangedU128};
    /// let ranged_u128 = RangedU128::<0, 2>::new::<1>();
    /// let expanded_i16: RangedI16<0, 4> = ranged_u128.to_ranged_i16();
    ///
    /// assert_eq!(Ok(expanded_i16.get()), ranged_u128.get().try_into());
    /// ```
    pub const fn to_ranged_i16<const OUT_MIN: i16, const OUT_MAX: i16>(
        self,
    ) -> RangedI16<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i128) > u128_to_i128(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i128) < u128_to_i128(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI16(self.get() as _)
    }

    /// Convert to [`RangedI32`].
    ///
    /// ```rust
    /// # use ranch::{RangedI32, RangedU128};
    /// let ranged_u128 = RangedU128::<0, 2>::new::<1>();
    /// let expanded_i32: RangedI32<0, 4> = ranged_u128.to_ranged_i32();
    ///
    /// assert_eq!(Ok(expanded_i32.get()), ranged_u128.get().try_into());
    /// ```
    pub const fn to_ranged_i32<const OUT_MIN: i32, const OUT_MAX: i32>(
        self,
    ) -> RangedI32<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i128) > u128_to_i128(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i128) < u128_to_i128(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI32(self.get() as _)
    }

    /// Convert to [`RangedI64`].
    ///
    /// ```rust
    /// # use ranch::{RangedI64, RangedU128};
    /// let ranged_u128 = RangedU128::<0, 2>::new::<1>();
    /// let expanded_i64: RangedI64<0, 4> = ranged_u128.to_ranged_i64();
    ///
    /// assert_eq!(Ok(expanded_i64.get()), ranged_u128.get().try_into());
    /// ```
    pub const fn to_ranged_i64<const OUT_MIN: i64, const OUT_MAX: i64>(
        self,
    ) -> RangedI64<OUT_MIN, OUT_MAX> {
        const {
            if (OUT_MIN as i128) > u128_to_i128(MIN) {
                panic!("minimum must be lower or match");
            }

            if (OUT_MAX as i128) < u128_to_i128(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI64(self.get() as _)
    }

    /// Convert to [`RangedI128`].
    ///
    /// ```rust
    /// # use ranch::{RangedI128, RangedU128};
    /// let ranged_u128 = RangedU128::<0, 2>::new::<1>();
    /// let expanded_i128: RangedI128<0, 4> = ranged_u128.to_ranged_i128();
    ///
    /// assert_eq!(Ok(expanded_i128.get()), ranged_u128.get().try_into());
    /// ```
    pub const fn to_ranged_i128<const OUT_MIN: i128, const OUT_MAX: i128>(
        self,
    ) -> RangedI128<OUT_MIN, OUT_MAX> {
        const {
            if OUT_MIN > u128_to_i128(MIN) {
                panic!("minimum must be lower or match");
            }

            if OUT_MAX < u128_to_i128(MAX) {
                panic!("maximum must be higher or match");
            }
        }

        RangedI128(self.get() as _)
    }
}

impl<const MIN: i8, const MAX: i8> RangedI8<MIN, MAX> {
    fn with_primitive(value: i8) -> Result<Self> {
        Self::with_i8(value)
    }
}

impl<const MIN: u8, const MAX: u8> RangedU8<MIN, MAX> {
    fn with_primitive(value: u8) -> Result<Self> {
        Self::with_u8(value)
    }
}

impl<const MIN: i16, const MAX: i16> RangedI16<MIN, MAX> {
    fn with_primitive(value: i16) -> Result<Self> {
        Self::with_i16(value)
    }
}

impl<const MIN: u16, const MAX: u16> RangedU16<MIN, MAX> {
    fn with_primitive(value: u16) -> Result<Self> {
        Self::with_u16(value)
    }
}

impl<const MIN: i32, const MAX: i32> RangedI32<MIN, MAX> {
    fn with_primitive(value: i32) -> Result<Self> {
        Self::with_i32(value)
    }
}

impl<const MIN: u32, const MAX: u32> RangedU32<MIN, MAX> {
    fn with_primitive(value: u32) -> Result<Self> {
        Self::with_u32(value)
    }
}

impl<const MIN: i64, const MAX: i64> RangedI64<MIN, MAX> {
    fn with_primitive(value: i64) -> Result<Self> {
        Self::with_i64(value)
    }
}

impl<const MIN: u64, const MAX: u64> RangedU64<MIN, MAX> {
    fn with_primitive(value: u64) -> Result<Self> {
        Self::with_u64(value)
    }
}

impl<const MIN: i128, const MAX: i128> RangedI128<MIN, MAX> {
    fn with_primitive(value: i128) -> Result<Self> {
        Self::with_i128(value)
    }
}

impl<const MIN: u128, const MAX: u128> RangedU128<MIN, MAX> {
    fn with_primitive(value: u128) -> Result<Self> {
        Self::with_u128(value)
    }
}

const fn i8_to_u8(value: i8) -> u8 {
    if value < 0 {
        panic!("minimum must be lower or match");
    }

    value as _
}

const fn i16_to_u16(value: i16) -> u16 {
    if value < 0 {
        panic!("minimum must be lower or match");
    }

    value as _
}

const fn i32_to_u32(value: i32) -> u32 {
    if value < 0 {
        panic!("minimum must be lower or match");
    }

    value as _
}

const fn i64_to_u64(value: i64) -> u64 {
    if value < 0 {
        panic!("minimum must be lower or match");
    }

    value as _
}

const fn i128_to_u128(value: i128) -> u128 {
    if value < 0 {
        panic!("minimum must be lower or match");
    }

    value as _
}

const fn u8_to_i8(value: u8) -> i8 {
    if value > i8::MAX as _ {
        panic!("maximum must be higher or match");
    }

    value as _
}

const fn u16_to_i16(value: u16) -> i16 {
    if value > i16::MAX as _ {
        panic!("maximum must be higher or match");
    }

    value as _
}

const fn u32_to_i32(value: u32) -> i32 {
    if value > i32::MAX as _ {
        panic!("maximum must be higher or match");
    }

    value as _
}

const fn u64_to_i64(value: u64) -> i64 {
    if value > i64::MAX as _ {
        panic!("maximum must be higher or match");
    }

    value as _
}

const fn u128_to_i128(value: u128) -> i128 {
    if value > i128::MAX as _ {
        panic!("maximum must be higher or match");
    }

    value as _
}
