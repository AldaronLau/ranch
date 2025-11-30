use core::num::NonZero;

use super::*;

macro_rules! impl_ranged_conversion {
    ($type:ident, $p:ty $(,)?) => {
        impl From<$p> for $type<{ <$p>::MIN }, { <$p>::MAX }> {
            fn from(primitive: $p) -> Self {
                Self(primitive)
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

macro_rules! impl_try_from {
    (($type:ident, $p:ty), $(($t:ident, $q:ty)),* $(,)?) => {
        $(
            impl<
                const IN_MIN: $p,
                const IN_MAX: $p,
                const OUT_MIN: $q,
                const OUT_MAX: $q,
            >
                TryFrom<$type::<IN_MIN, IN_MAX>> for $t::<OUT_MIN, OUT_MAX>
            {
                type Error = Error;

                fn try_from(ranged: $type<IN_MIN, IN_MAX>) -> Result<Self> {
                    Self::new(<$q>::from(ranged.get()))
                }
            }

            impl<
                const IN_MIN: $q,
                const IN_MAX: $q,
                const OUT_MIN: $p,
                const OUT_MAX: $p,
            >
                TryFrom<$t::<IN_MIN, IN_MAX>> for $type::<OUT_MIN, OUT_MAX>
            {
                type Error = Error;

                #[allow(unused_comparisons)]
                fn try_from(ranged: $t<IN_MIN, IN_MAX>) -> Result<Self> {
                    let value = ranged.get();

                    Self::new(<$p>::try_from(value).map_err(|_| {
                        if value < 0 {
                            Error::NegOverflow
                        } else {
                            Error::PosOverflow
                        }
                    })?)
                }
            }
        )*
    };
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

impl_try_from!(
    (RangedU8, u8),
    (RangedU16, u16),
    (RangedU32, u32),
    (RangedU64, u64),
    (RangedU128, u128),
);
impl_try_from!(
    (RangedU16, u16),
    (RangedU32, u32),
    (RangedU64, u64),
    (RangedU128, u128),
);
impl_try_from!((RangedU32, u32), (RangedU64, u64), (RangedU128, u128));
impl_try_from!((RangedU64, u64), (RangedU128, u128));

impl_try_from!(
    (RangedI8, i8),
    (RangedI16, i16),
    (RangedI32, i32),
    (RangedI64, i64),
    (RangedI128, i128),
);
impl_try_from!(
    (RangedI16, i16),
    (RangedI32, i32),
    (RangedI64, i64),
    (RangedI128, i128),
);
impl_try_from!((RangedI32, i32), (RangedI64, i64), (RangedI128, i128));
impl_try_from!((RangedI64, i64), (RangedI128, i128));

impl<const MIN: u8, const MAX: u8> RangedU8<MIN, MAX> {
    /// Convert to [`RangedU8`].
    ///
    /// ```rust
    /// # use ranch::{RangedU8};
    /// let ranged_u8 = RangedU8::<0, 2>::new_const::<1>();
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
    /// let ranged_u8 = RangedU8::<0, 2>::new_const::<1>();
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
    /// let ranged_u8 = RangedU8::<0, 2>::new_const::<1>();
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
    /// let ranged_u8 = RangedU8::<0, 2>::new_const::<1>();
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
    /// let ranged_u8 = RangedU8::<0, 2>::new_const::<1>();
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
    /// let ranged_u16 = RangedU16::<0, 2>::new_const::<1>();
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
    /// let ranged_u16 = RangedU16::<0, 2>::new_const::<1>();
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
    /// let ranged_u16 = RangedU16::<0, 2>::new_const::<1>();
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
    /// let ranged_u16 = RangedU16::<0, 2>::new_const::<1>();
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
    /// let ranged_u16 = RangedU16::<0, 2>::new_const::<1>();
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
    /// let ranged_u32 = RangedU32::<0, 2>::new_const::<1>();
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
    /// let ranged_u32 = RangedU32::<0, 2>::new_const::<1>();
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
    /// let ranged_u32 = RangedU32::<0, 2>::new_const::<1>();
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
    /// let ranged_u32 = RangedU32::<0, 2>::new_const::<1>();
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
    /// let ranged_u32 = RangedU32::<0, 2>::new_const::<1>();
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
    /// let ranged_u64 = RangedU64::<0, 2>::new_const::<1>();
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
    /// let ranged_u64 = RangedU64::<0, 2>::new_const::<1>();
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
    /// let ranged_u64 = RangedU64::<0, 2>::new_const::<1>();
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
    /// let ranged_u64 = RangedU64::<0, 2>::new_const::<1>();
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
    /// let ranged_u64 = RangedU64::<0, 2>::new_const::<1>();
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
    /// let ranged_u128 = RangedU128::<0, 2>::new_const::<1>();
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
    /// let ranged_u128 = RangedU128::<0, 2>::new_const::<1>();
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
    /// let ranged_u128 = RangedU128::<0, 2>::new_const::<1>();
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
    /// let ranged_u128 = RangedU128::<0, 2>::new_const::<1>();
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
    /// let ranged_u128 = RangedU128::<0, 2>::new_const::<1>();
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
