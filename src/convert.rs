use core::num::{NonZero, TryFromIntError};

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

macro_rules! impl_ranged_nonzero_conversion {
    ($type:ident, $p:ty, $r:ident $(,)?) => {
        impl<const MIN: $p, const MAX: $p> TryFrom<$p> for $type<MIN, MAX> {
            type Error = range::Error;

            fn try_from(primitive: $p) -> range::Result<Self> {
                $r::<MIN, MAX>::with_primitive(primitive)?
                    .to_ranged_nonzero()
                    .ok_or(Self::Error::Zero)
            }
        }

        impl<const MIN: $p, const MAX: $p> TryFrom<$r<MIN, MAX>>
            for $type<MIN, MAX>
        {
            type Error = TryFromIntError;

            fn try_from(ranged: $r<MIN, MAX>) -> Result<Self, Self::Error> {
                ranged.to_ranged_nonzero().ok_or_else(try_from_int_err)
            }
        }

        impl<const MIN: $p, const MAX: $p> From<$type<MIN, MAX>> for $p {
            fn from(ranged: $type<MIN, MAX>) -> Self {
                ranged.get()
            }
        }

        impl<const MIN: $p, const MAX: $p> From<$type<MIN, MAX>>
            for $r<MIN, MAX>
        {
            fn from(ranged: $type<MIN, MAX>) -> Self {
                ranged.to_ranged()
            }
        }
    };
}

macro_rules! impl_unsigned_nonzero_conversion {
    ($type:ident, $p:ty $(,)?) => {
        impl From<NonZero<$p>> for $type<1, { <$p>::MAX }> {
            fn from(non_zero: NonZero<$p>) -> Self {
                Self::from_unchecked(non_zero.get())
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

                $nonzero::from_unchecked(value)
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
                    Some(value) => Some($nonzero::from_unchecked(value)),
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

impl_ranged_nonzero_conversion!(RangedNonZeroI8, i8, RangedI8);
impl_ranged_nonzero_conversion!(RangedNonZeroI16, i16, RangedI16);
impl_ranged_nonzero_conversion!(RangedNonZeroI32, i32, RangedI32);
impl_ranged_nonzero_conversion!(RangedNonZeroI64, i64, RangedI64);
impl_ranged_nonzero_conversion!(RangedNonZeroI128, i128, RangedI128);
impl_ranged_nonzero_conversion!(RangedNonZeroU8, u8, RangedU8);
impl_ranged_nonzero_conversion!(RangedNonZeroU16, u16, RangedU16);
impl_ranged_nonzero_conversion!(RangedNonZeroU32, u32, RangedU32);
impl_ranged_nonzero_conversion!(RangedNonZeroU64, u64, RangedU64);
impl_ranged_nonzero_conversion!(RangedNonZeroU128, u128, RangedU128);

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

fn try_from_int_err() -> TryFromIntError {
    NonZero::try_from(0u32).unwrap_err()
}
