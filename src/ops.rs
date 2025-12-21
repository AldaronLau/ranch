use core::{
    num::NonZero,
    ops::{Add, Div, Mul, Sub},
};

use as_repr::AsRepr;

use super::*;

pub trait OpsAsRepr<T>: AsRepr<T> {}

impl OpsAsRepr<i8> for i8 {}
impl OpsAsRepr<i16> for i16 {}
impl OpsAsRepr<i32> for i32 {}
impl OpsAsRepr<i64> for i64 {}
impl OpsAsRepr<i128> for i128 {}
impl OpsAsRepr<u8> for u8 {}
impl OpsAsRepr<u16> for u16 {}
impl OpsAsRepr<u32> for u32 {}
impl OpsAsRepr<u64> for u64 {}
impl OpsAsRepr<u128> for u128 {}

impl<const MIN: i8, const MAX: i8> OpsAsRepr<i8> for RangedI8<MIN, MAX> {}
impl<const MIN: i16, const MAX: i16> OpsAsRepr<i16> for RangedI16<MIN, MAX> {}
impl<const MIN: i32, const MAX: i32> OpsAsRepr<i32> for RangedI32<MIN, MAX> {}
impl<const MIN: i64, const MAX: i64> OpsAsRepr<i64> for RangedI64<MIN, MAX> {}
impl<const MIN: i128, const MAX: i128> OpsAsRepr<i128>
    for RangedI128<MIN, MAX>
{
}
impl<const MIN: u8, const MAX: u8> OpsAsRepr<u8> for RangedU8<MIN, MAX> {}
impl<const MIN: u16, const MAX: u16> OpsAsRepr<u16> for RangedU16<MIN, MAX> {}
impl<const MIN: u32, const MAX: u32> OpsAsRepr<u32> for RangedU32<MIN, MAX> {}
impl<const MIN: u64, const MAX: u64> OpsAsRepr<u64> for RangedU64<MIN, MAX> {}
impl<const MIN: u128, const MAX: u128> OpsAsRepr<u128>
    for RangedU128<MIN, MAX>
{
}

macro_rules! impl_ops {
    ($type:ident, $p:ty $(,)?) => {
        impl<T, const MIN: $p, const MAX: $p> Add<T> for $type<MIN, MAX>
        where
            T: OpsAsRepr<$p>,
        {
            type Output = Self;

            fn add(self, other: T) -> Self {
                self.checked_add(other).unwrap()
            }
        }

        impl<T, const MIN: $p, const MAX: $p> Sub<T> for $type<MIN, MAX>
        where
            T: OpsAsRepr<$p>,
        {
            type Output = Self;

            fn sub(self, other: T) -> Self {
                self.checked_sub(other).unwrap()
            }
        }

        impl<T, const MIN: $p, const MAX: $p> Mul<T> for $type<MIN, MAX>
        where
            T: OpsAsRepr<$p>,
        {
            type Output = Self;

            fn mul(self, other: T) -> Self {
                self.checked_mul(other).unwrap()
            }
        }

        impl<T, const MIN: $p, const MAX: $p> Div<T> for $type<MIN, MAX>
        where
            T: OpsAsRepr<$p>,
        {
            type Output = Quotient<Self>;

            fn div(self, other: T) -> Quotient<Self> {
                self.checked_div(other).unwrap()
            }
        }

        impl<const MIN: $p, const MAX: $p> Add<NonZero<$p>>
            for $type<MIN, MAX>
        {
            type Output = Self;

            fn add(self, other: NonZero<$p>) -> Self {
                self.checked_add(other.get()).unwrap()
            }
        }

        impl<const MIN: $p, const MAX: $p> Sub<NonZero<$p>>
            for $type<MIN, MAX>
        {
            type Output = Self;

            fn sub(self, other: NonZero<$p>) -> Self {
                self.checked_sub(other.get()).unwrap()
            }
        }

        impl<const MIN: $p, const MAX: $p> Mul<NonZero<$p>>
            for $type<MIN, MAX>
        {
            type Output = Self;

            fn mul(self, other: NonZero<$p>) -> Self {
                self.checked_mul(other.get()).unwrap()
            }
        }

        impl<const MIN: $p, const MAX: $p> Div<NonZero<$p>>
            for $type<MIN, MAX>
        {
            type Output = Self;

            fn div(self, other: NonZero<$p>) -> Self {
                match self.checked_div(other.get()).unwrap() {
                    Quotient::Nan => unreachable!(),
                    Quotient::Number(number) => number,
                }
            }
        }

        impl<const MIN: $p, const MAX: $p> $type<MIN, MAX> {
            /// Add a number to `self`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<15, 85>::new::<16>();")]
            #[doc = concat!("let output = ", stringify!($type), "::<38, 108>::new::<39>();")]
            /// assert_eq!(a.add::<23, _, _>(), output);
            /// ```
            #[must_use]
            pub const fn add<
                const RHS: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                let rhs = const { $type::<RHS, RHS>::new::<RHS>() };

                self.ranged_add(rhs)
            }

            /// Subtract a number from `self`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<38, 108>::new::<39>();")]
            #[doc = concat!("let output = ", stringify!($type), "::<15, 85>::new::<16>();")]
            /// assert_eq!(a.sub::<23, _, _>(), output);
            /// ```
            #[must_use]
            pub const fn sub<
                const RHS: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                let rhs = const { $type::<RHS, RHS>::new::<RHS>() };

                self.ranged_sub(rhs)
            }

            /// Multiply a number to `self`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<23, 42>::new::<30>();")]
            #[doc = concat!("let output = ", stringify!($type), "::<46, 84>::new::<60>();")]
            /// assert_eq!(a.mul::<2, _, _>(), output);
            /// ```
            #[must_use]
            pub const fn mul<
                const RHS: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                let rhs = const { $type::<RHS, RHS>::new::<RHS>() };

                self.ranged_mul(rhs)
            }

            /// Divide `self` by a number.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<46, 84>::new::<60>();")]
            #[doc = concat!("let output = ", stringify!($type), "::<23, 42>::new::<30>();")]
            /// assert_eq!(a.div::<2, _, _>(), output);
            /// ```
            #[must_use]
            pub const fn div<
                const RHS: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                let rhs = const { $type::<RHS, RHS>::new::<RHS>() };

                self.ranged_div(rhs)
            }

            /// Raise `self` to a power.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<7, 9>::new::<8>();")]
            #[doc = concat!("let output = ", stringify!($type), "::<49, 81>::new::<64>();")]
            /// assert_eq!(a.pow::<2, _, _>(), output);
            /// ```
            #[must_use]
            pub const fn pow<
                const RHS: u32,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                let rhs = const { RangedU32::<RHS, RHS>::new::<RHS>() };

                self.ranged_pow(rhs)
            }

            /// Compare and return the minimum of two values.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<7, 10>::new::<9>();")]
            #[doc = concat!("let output = ", stringify!($type), "::<7, 8>::new::<8>();")]
            /// assert_eq!(a.min::<8, _, _>(), output);
            /// ```
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<7, 12>::new::<9>();")]
            #[doc = concat!("let output = ", stringify!($type), "::<7, 10>::new::<9>();")]
            /// assert_eq!(a.min::<10, _, _>(), output);
            /// ```
            pub const fn min<const OTHER: $p, const OUTPUT_MIN: $p, const OUTPUT_MAX: $p>(self) -> $type<OUTPUT_MIN, OUTPUT_MAX>
            {
                self.ranged_min($type::<OTHER, OTHER>::new::<OTHER>())
            }

            /// Compare and return the maximum of two values.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<7, 10>::new::<9>();")]
            #[doc = concat!("let output = ", stringify!($type), "::<8, 10>::new::<9>();")]
            /// assert_eq!(a.max::<8, _, _>(), output);
            /// ```
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<7, 12>::new::<9>();")]
            #[doc = concat!("let output = ", stringify!($type), "::<10, 12>::new::<10>();")]
            /// assert_eq!(a.max::<10, _, _>(), output);
            /// ```
            pub const fn max<const OTHER: $p, const OUTPUT_MIN: $p, const OUTPUT_MAX: $p>(self) -> $type<OUTPUT_MIN, OUTPUT_MAX>
            {
                self.ranged_max($type::<OTHER, OTHER>::new::<OTHER>())
            }

            /// Restrict a value to a certain interval.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<5, 10>::new::<7>();")]
            #[doc = concat!("let output = ", stringify!($type), "::<8, 10>::new::<8>();")]
            /// assert_eq!(a.clamp::<8, 12, _, _>(), output);
            /// ```
            pub const fn clamp<
                const TO_MIN: $p,
                const TO_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p
            >(
                self
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX>
            {
                self.ranged_clamp(
                    $type::<TO_MIN, TO_MIN>::new::<TO_MIN>(),
                    $type::<TO_MAX, TO_MAX>::new::<TO_MAX>(),
                )
            }
        }
    };
}

impl_ops!(RangedI8, i8);
impl_ops!(RangedI16, i16);
impl_ops!(RangedI32, i32);
impl_ops!(RangedI64, i64);
impl_ops!(RangedI128, i128);

impl_ops!(RangedU8, u8);
impl_ops!(RangedU16, u16);
impl_ops!(RangedU32, u32);
impl_ops!(RangedU64, u64);
impl_ops!(RangedU128, u128);
