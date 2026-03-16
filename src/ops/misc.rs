use core::{
    num::NonZero,
    ops::{Add, Div, Mul, Rem, Sub},
};

use as_repr::AsRepr;

use crate::{multirange::Ranged, range::Range, *};

macro_rules! impl_ops {
    (
        $type:ident,
        $p:ty,
        $nonzero:ident,
        $ret:ident,
        $nan_unreachable:ident $(,)?
    ) => {
        impl<T, const MIN: $p, const MAX: $p> Add<T> for $type::<MIN, MAX>
        where
            T: AsRepr<$p>,
        {
            type Output = Self;

            fn add(self, other: T) -> Self {
                self.checked_add(other).expect("out of range")
            }
        }

        impl<T, const MIN: $p, const MAX: $p> Sub<T> for $type::<MIN, MAX>
        where
            T: AsRepr<$p>,
        {
            type Output = Self;

            fn sub(self, other: T) -> Self {
                self.checked_sub(other).expect("out of range")
            }
        }

        impl<T, const MIN: $p, const MAX: $p> Mul<T> for $type::<MIN, MAX>
        where
            T: AsRepr<$p>,
        {
            type Output = Self;

            fn mul(self, other: T) -> Self {
                self.checked_mul(other).expect("out of range")
            }
        }

        impl<T, const MIN: $p, const MAX: $p> Div<T> for $type::<MIN, MAX>
        where
            T: AsRepr<$p>,
        {
            type Output = Self;

            fn div(self, other: T) -> Self {
                self.checked_div(other)
                    .expect("out of range")
                    .number()
                    .expect("cannot divide by zero")
            }
        }

        impl<T, const MIN: $p, const MAX: $p> Rem<T> for $type::<MIN, MAX>
        where
            T: AsRepr<$p>,
        {
            type Output = Self;

            fn rem(self, other: T) -> Self {
                self.checked_rem(other)
                    .expect("out of range")
                    .number()
                    .expect("cannot divide by zero")
            }
        }

        impl<const MIN: $p, const MAX: $p> $nonzero<MIN, MAX> {
            /// Convert to a ranged type with a fully expanded range.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($nonzero), ";")]
            #[doc = concat!("let a = ", stringify!($nonzero), "::<48, 96>::new::<0b_0100_0000>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<48, 96>::new::<0b_0100_0011>();")]
            ///
            /// assert_eq!(a.to_full().count_ones().get(), 1);
            /// assert_eq!(b.to_full().count_ones().get(), 3);
            /// assert_eq!(a.to_full().trailing_zeros().get(), 6);
            /// assert_eq!(b.to_full().trailing_zeros().get(), 0);
            #[doc = concat!("assert_eq!(a.to_full().leading_zeros().get(), ", stringify!($p), "::BITS - 7);")]
            #[doc = concat!("assert_eq!(b.to_full().leading_zeros().get(), ", stringify!($p), "::BITS - 7);")]
            /// ```
            pub const fn to_full(self) -> $nonzero<
                { NonZero::<$p>::MIN.get() },
                { NonZero::<$p>::MAX.get() },
            > {
                $nonzero::from_unchecked(self.to_nonzero())
            }
        }

        impl<const MIN: $p, const MAX: $p> $type<MIN, MAX> {
            /// Convert to a ranged type with a fully expanded range.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<48, 96>::new::<0b_0100_0000>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<48, 96>::new::<0b_0100_0011>();")]
            ///
            /// assert_eq!(a.to_full().count_ones().get(), 1);
            /// assert_eq!(b.to_full().count_ones().get(), 3);
            /// assert_eq!(a.to_full().trailing_zeros().get(), 6);
            /// assert_eq!(b.to_full().trailing_zeros().get(), 0);
            #[doc = concat!("assert_eq!(a.to_full().leading_zeros().get(), ", stringify!($p), "::BITS - 7);")]
            #[doc = concat!("assert_eq!(b.to_full().leading_zeros().get(), ", stringify!($p), "::BITS - 7);")]
            /// ```
            pub const fn to_full(self) -> $type<{ <$p>::MIN }, { <$p>::MAX }> {
                $type::from_unchecked(self.get())
            }

            /// Add a number to `self`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<15, 85>::new::<16>();")]
            #[doc = concat!("let output: ", stringify!($type), "<38, 108> = a.add::<23, _, _>();")]
            ///
            /// assert_eq!(output, 39);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn add<const RHS: $p, R: Range<$p>>(
                self,
            ) -> Ranged::<$p, R> {
                let rhs = const { $type::<RHS, RHS>::new::<RHS>() };

                self.add_ranged(rhs)
            }

            /// Subtract a number from `self`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<38, 108>::new::<39>();")]
            #[doc = concat!("let output: ", stringify!($type), "<15, 85> = a.sub::<23, _, _>();")]
            ///
            /// assert_eq!(output, 16);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn sub<const RHS: $p, R: Range<$p>>(
                self,
            ) -> Ranged::<$p, R> {
                let rhs = const { $type::<RHS, RHS>::new::<RHS>() };

                self.sub_ranged(rhs)
            }

            /// Multiply a number to `self`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<23, 42>::new::<30>();")]
            #[doc = concat!("let output: ", stringify!($type), "<46, 84> = a.mul::<2, _, _>();")]
            ///
            /// assert_eq!(output, 60);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn mul<
                const RHS: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                let rhs = const { $type::<RHS, RHS>::new::<RHS>() };

                self.mul_ranged(rhs)
            }

            /// Divide `self` by a number.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<46, 84>::new::<60>();")]
            #[doc = concat!("let output: ", stringify!($type), "<23, 42> = a.div::<2, _, _>();")]
            ///
            /// assert_eq!(output, 30);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn div<
                const RHS: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                let rhs = const { $nonzero::<RHS, RHS>::new::<RHS>() };

                self.div_ranged_nonzero(rhs)
            }

            /// Raise `self` to a power.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<7, 9>::new::<8>();")]
            #[doc = concat!("let output: ", stringify!($type), "<49, 81> = a.pow::<2, _, _>();")]
            ///
            /// assert_eq!(output, 64);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn pow<
                const RHS: u32,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                let rhs = const { RangedU32::<RHS, RHS>::new::<RHS>() };

                self.pow_ranged(rhs)
            }

            /// Compare and return the minimum of two values.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<7, 10>::new::<9>();")]
            #[doc = concat!("let output: ", stringify!($type), "<7, 8> = a.min::<8, _, _>();")]
            ///
            /// assert_eq!(output, 8);
            /// ```
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<7, 12>::new::<9>();")]
            #[doc = concat!("let output: ", stringify!($type), "<7, 10> = a.min::<10, _, _>();")]
            ///
            /// assert_eq!(output, 9);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn min<
                const OTHER: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(self) -> $type<OUTPUT_MIN, OUTPUT_MAX>
            {
                self.min_ranged($type::<OTHER, OTHER>::new::<OTHER>())
            }

            /// Compare and return the maximum of two values.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<7, 10>::new::<9>();")]
            #[doc = concat!("let output: ", stringify!($type), "<8, 10> = a.max::<8, _, _>();")]
            ///
            /// assert_eq!(output, 9);
            /// ```
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<7, 12>::new::<9>();")]
            #[doc = concat!("let output: ", stringify!($type), "<10, 12> = a.max::<10, _, _>();")]
            ///
            /// assert_eq!(output, 10);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn max<
                const OTHER: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(self) -> $type<OUTPUT_MIN, OUTPUT_MAX>
            {
                self.max_ranged($type::<OTHER, OTHER>::new::<OTHER>())
            }

            /// Restrict a value to a certain interval.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<5, 10>::new::<7>();")]
            #[doc = concat!("let output: ", stringify!($type), "<8, 10> = a.clamp::<8, 12, _, _>();")]
            ///
            /// assert_eq!(output, 8);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn clamp<
                const TO_MIN: $p,
                const TO_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p
            >(
                self
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX>
            {
                self.clamp_ranged(
                    $type::<TO_MIN, TO_MIN>::new::<TO_MIN>(),
                    $type::<TO_MAX, TO_MAX>::new::<TO_MAX>(),
                )
            }

            /// Checked integer division by a non-zero number.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 50>::new::<50>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 50>::new::<2>();")]
            ///
            /// assert_eq!(a.checked_div_nonzero(b).unwrap(), 25);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_div_nonzero(
                self,
                rhs: impl AsRepr<NonZero<$p>>,
            ) -> $ret::<Self> {
                let rhs = as_repr::as_repr(rhs);

                $nan_unreachable(self.checked_div(rhs))
            }

            /// Checked euclidean integer division by a non-zero number.
            ///
            /// This is the same as non-euclidean division for unsigned
            /// integers.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 50>::new::<50>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 50>::new::<2>();")]
            ///
            /// assert_eq!(a.checked_div_euclid_nonzero(b).unwrap(), 25);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_div_euclid_nonzero(
                self,
                rhs: impl AsRepr<NonZero<$p>>,
            ) -> $ret::<Self> {
                let rhs = as_repr::as_repr(rhs);

                $nan_unreachable(self.checked_div_euclid(rhs))
            }

            /// Checked integer remainder by a non-zero number.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 51>::new::<51>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            ///
            /// assert_eq!(a.checked_rem_nonzero(b).unwrap(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_rem_nonzero(
                self,
                rhs: impl AsRepr<NonZero<$p>>,
            ) -> $ret::<Self> {
                let rhs = as_repr::as_repr(rhs);

                $nan_unreachable(self.checked_rem(rhs))
            }

            /// Checked euclidean integer remainder by a non-zero number.
            ///
            /// This is the same as non-euclidean remainder for unsigned
            /// integers.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 51>::new::<51>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            ///
            /// assert_eq!(a.checked_rem_euclid_nonzero(b).unwrap(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_rem_euclid_nonzero(
                self,
                rhs: impl AsRepr<NonZero<$p>>,
            ) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);

                unsigned_nan_unreachable(self.checked_rem_euclid(rhs))
            }

            /// Saturating integer division by a non-zero number.
            ///
            /// Returns [`Self::MIN`] on overflow.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 50>::new::<50>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 50>::new::<2>();")]
            #[doc = concat!("let c = ", stringify!($nonzero), "::<1, 120>::new::<120>();")]
            ///
            /// assert_eq!(a.saturating_div_nonzero(b), 25);
            /// assert_eq!(a.saturating_div_nonzero(c), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn saturating_div_nonzero(
                self,
                rhs: impl AsRepr<NonZero<$p>>,
            ) -> Self {
                let rhs = as_repr::as_repr(rhs);
                let Quotient::Number(number) = self.saturating_div(rhs) else {
                    unreachable!()
                };

                number
            }

            /// Divide `self` by a non-zero number.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<1, 2> = a.div_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<1>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.div_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn div_ranged_nonzero<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $nonzero::<RHS_MIN, RHS_MAX>,
            ) -> $type::<OUTPUT_MIN, OUTPUT_MAX> {
                match self.div_ranged::<
                    RHS_MIN,
                    RHS_MAX,
                    OUTPUT_MIN,
                    OUTPUT_MAX,
                >(rhs.to_ranged())
                {
                    Quotient::Number(num) => num,
                    Quotient::Nan => unreachable!(),
                }
            }
        }
    };
}

macro_rules! impl_ops_nonzero_signed {
    ($type:ident, $p:ty $(,)?) => {
        impl<const MIN: $p, const MAX: $p> $type<MIN, MAX> {
            /// Checked integer division by a non-zero number.
            ///
            /// Returns [`None`] on overflow.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 50>::new::<50>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 50>::new::<2>();")]
            ///
            /// assert_eq!(a.checked_div_nonzero(b).unwrap().unwrap(), 25);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_div_nonzero(
                self,
                rhs: impl AsRepr<NonZero<$p>>,
            ) -> Result<Option<Self>> {
                let rhs = as_repr::as_repr(rhs);

                match self.checked_div(rhs) {
                    Err(e) => Err(e),
                    Ok(None) => return Ok(None),
                    Ok(Some(Quotient::Number(x))) => Ok(Some(x)),
                    Ok(Some(Quotient::Nan)) => unreachable!(),
                }
            }

            /// Multiply two non-zero ranged integers together.
            ///
            /// Returns an [`Error`] on overflow.
            ///
            /// ```rust
            /// # use ranch::{Error, RangedNonZeroI32};
            /// let a = RangedNonZeroI32::<-100, 100>::new::<50>();
            /// let b = RangedNonZeroI32::<-100, 100>::new::<5>();
            /// let c = RangedNonZeroI32::<-100, 100>::new::<-75>();
            ///
            /// assert_eq!(b.checked_mul_nonzero(b).unwrap().get(), 25);
            /// assert_eq!(a.checked_mul_nonzero(c).unwrap_err(), Error::NegOverflow);
            /// assert_eq!(c.checked_mul_nonzero(c).unwrap_err(), Error::PosOverflow);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_mul_nonzero(
                self,
                rhs: impl AsRepr<NonZero<$p>>,
            )
                -> Result<Self>
            {
                let rhs = as_repr::as_repr(rhs);

                match self.checked_mul(rhs) {
                    Ok(Some(value)) => Ok(value),
                    Ok(None) => unreachable!(),
                    Err(e) => Err(e),
                }
            }
        }
    };
}

macro_rules! impl_ops_nonzero_unsigned {
    ($type:ident, $p:ty $(,)?) => {
        impl<const MIN: $p, const MAX: $p> $type<MIN, MAX> {
            /// Checked integer division by a non-zero number.
            ///
            /// Returns [`None`] on overflow.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 50>::new::<50>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 50>::new::<2>();")]
            ///
            /// assert_eq!(a.checked_div_nonzero(b).unwrap(), 25);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_div_nonzero(
                self,
                rhs: impl AsRepr<NonZero<$p>>,
            ) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);

                unsigned_nan_unreachable(self.checked_div(rhs))
            }

            /// Return the smallest power of two greater than or equal to self.
            ///
            /// Returns [`None`] on overflow.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 33>::new::<1>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 33>::new::<9>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<1, 33>::new::<32>();")]
            #[doc = concat!("let d = ", stringify!($type), "::<1, 33>::new::<33>();")]
            ///
            /// assert_eq!(a.checked_next_power_of_two().unwrap().get(), 1);
            /// assert_eq!(b.checked_next_power_of_two().unwrap().get(), 16);
            /// assert_eq!(c.checked_next_power_of_two().unwrap().get(), 32);
            /// assert_eq!(d.checked_next_power_of_two(), None);
            /// ```
            #[must_use]
            pub const fn checked_next_power_of_two(self) -> Option<Self> {
                let Some(value) = self.get().checked_next_power_of_two() else {
                    return None;
                };
                let Some(value) = NonZero::new(value) else {
                    unreachable!()
                };
                let Ok(value) = $type::with_nonzero(value) else {
                    return None;
                };

                Some(value)
            }

            /// Return the smallest power of two greater than or equal to self.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 33>::new::<1>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 33>::new::<9>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<1, 33>::new::<32>();")]
            #[doc = concat!("let d = ", stringify!($type), "::<1, 33>::new::<33>();")]
            ///
            /// assert_eq!(a.next_power_of_two::<1, 64>().get(), 1);
            /// assert_eq!(b.next_power_of_two::<1, 64>().get(), 16);
            /// assert_eq!(c.next_power_of_two::<1, 64>().get(), 32);
            /// assert_eq!(d.next_power_of_two::<1, 64>().get(), 64);
            /// ```
            #[must_use]
            pub const fn next_power_of_two<
                const OUT_MIN: $p,
                const OUT_MAX: $p,
            >(
                self,
            ) -> $type::<OUT_MIN, OUT_MAX> {
                const {
                    if OUT_MIN != MIN.checked_next_power_of_two().unwrap() {
                        panic!("mismatched OUT_MIN")
                    }

                    if OUT_MAX != MAX.checked_next_power_of_two().unwrap() {
                        panic!("mismatched OUT_MAX")
                    }
                }

                let Some(value) = NonZero::new(self.get().next_power_of_two())
                else {
                    unreachable!()
                };

                $type::from_unchecked(value)
            }

            /// Returns true if and only if `self == (1 << k)` for some `k`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 32>::new::<1>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 32>::new::<9>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<1, 32>::new::<32>();")]
            ///
            /// assert!(a.is_power_of_two());
            /// assert!(!b.is_power_of_two());
            /// assert!(c.is_power_of_two());
            /// ```
            #[must_use]
            pub const fn is_power_of_two(self) -> bool {
                self.get().is_power_of_two()
            }

            /// Calculates the smallest value greater than or equal to self that
            /// is a multiple of `rhs`.
            ///
            /// Returns [`None`] on overflow or `rhs == 0`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 33>::new::<16>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 33>::new::<23>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<1, 33>::new::<33>();")]
            ///
            /// assert_eq!(a.checked_next_multiple_of(8).unwrap().get(), 16);
            /// assert_eq!(b.checked_next_multiple_of(8).unwrap().get(), 24);
            /// assert!(c.checked_next_multiple_of(8).is_none());
            /// assert!(a.checked_next_multiple_of(0).is_none());
            /// ```
            #[must_use]
            pub const fn checked_next_multiple_of(self, rhs: impl AsRepr<$p>) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);
                let Some(value) = self.get().checked_next_multiple_of(rhs) else {
                    return None;
                };
                let Some(value) = NonZero::new(value) else {
                    unreachable!()
                };
                let Ok(value) = $type::with_nonzero(value) else {
                    return None;
                };

                Some(value)
            }

            /// Return the smallest power of two greater than or equal to self.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 33>::new::<16>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 33>::new::<23>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<1, 33>::new::<33>();")]
            ///
            /// assert_eq!(a.next_multiple_of::<8, 8, 40>().get(), 16);
            /// assert_eq!(b.next_multiple_of::<8, 8, 40>().get(), 24);
            /// assert_eq!(c.next_multiple_of::<8, 8, 40>().get(), 40);
            /// ```
            #[must_use]
            pub const fn next_multiple_of<
                const RHS: $p,
                const OUT_MIN: $p,
                const OUT_MAX: $p,
            >(
                self,
            ) -> $type<OUT_MIN, OUT_MAX> {
                const {
                    if OUT_MIN != MIN.next_multiple_of(RHS) {
                        panic!("mismatched OUT_MIN")
                    }

                    if OUT_MAX != MAX.next_multiple_of(RHS) {
                        panic!("mismatched OUT_MAX")
                    }
                }

                let value = self.get().next_multiple_of(RHS);
                let Some(value) = NonZero::new(value) else {
                    unreachable!()
                };

                $type::from_unchecked(value)
            }

            /// Return `true` if `self` is an integer multiple of `rhs`, and
            /// `false` otherwise.
            ///
            /// This function is equivalent to `self % rhs == 0`, except that it
            /// will not panic for `rhs == 0`. Instead,
            /// `0.is_multiple_of(0) == true`, and for any non-zero `n`,
            /// `n.is_multiple_of(0) == false`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 8>::new::<5>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 8>::new::<6>();")]
            ///
            /// assert!(!a.is_multiple_of(2));
            /// assert!(b.is_multiple_of(2));
            ///
            /// assert!(!a.is_multiple_of(0));
            /// assert!(!b.is_multiple_of(0));
            /// ```
            #[must_use]
            pub const fn is_multiple_of(self, rhs: impl AsRepr<$p>) -> bool {
                let rhs = as_repr::as_repr(rhs);

                match rhs {
                    0 => self.get() == 0,
                    _ => self.get() % rhs == 0,
                }
            }
        }
    };
}

macro_rules! impl_ops_unsigned {
    ($type:ident, $p:ty, $nonzero:ident, $with:ident $(,)?) => {
        impl<const MIN: $p, const MAX: $p> $type<MIN, MAX> {
            /// Return the smallest power of two greater than or equal to self.
            ///
            /// Returns [`None`] on overflow.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<0, 33>::new::<0>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<0, 33>::new::<9>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<0, 33>::new::<32>();")]
            #[doc = concat!("let d = ", stringify!($type), "::<0, 33>::new::<33>();")]
            ///
            /// assert_eq!(a.checked_next_power_of_two().unwrap().get(), 1);
            /// assert_eq!(b.checked_next_power_of_two().unwrap().get(), 16);
            /// assert_eq!(c.checked_next_power_of_two().unwrap().get(), 32);
            /// assert_eq!(d.checked_next_power_of_two(), None);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_next_power_of_two(self) -> Option<Self> {
                let Some(value) = self.get().checked_next_power_of_two() else {
                    return None;
                };
                let Some(value) = NonZero::new(value) else {
                    unreachable!()
                };
                let Ok(value) = $type::$with(value) else {
                    return None;
                };

                Some(value)
            }

            /// Return the smallest power of two greater than or equal to self.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<0, 33>::new::<0>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<0, 33>::new::<9>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<0, 33>::new::<32>();")]
            #[doc = concat!("let d = ", stringify!($type), "::<0, 33>::new::<33>();")]
            ///
            /// assert_eq!(a.next_power_of_two::<1, 64>().get(), 1);
            /// assert_eq!(b.next_power_of_two::<1, 64>().get(), 16);
            /// assert_eq!(c.next_power_of_two::<1, 64>().get(), 32);
            /// assert_eq!(d.next_power_of_two::<1, 64>().get(), 64);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn next_power_of_two<
                const OUT_MIN: $p,
                const OUT_MAX: $p,
            >(
                self,
            ) -> $nonzero<OUT_MIN, OUT_MAX> {
                const {
                    if OUT_MIN != MIN.checked_next_power_of_two().unwrap() {
                        panic!("mismatched OUT_MIN")
                    }

                    if OUT_MAX != MAX.checked_next_power_of_two().unwrap() {
                        panic!("mismatched OUT_MAX")
                    }
                }

                let Some(value) = NonZero::new(self.get().next_power_of_two())
                else {
                    unreachable!()
                };

                $nonzero::from_unchecked(value)
            }

            /// Returns true if and only if `self == (1 << k)` for some `k`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<0, 32>::new::<0>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<0, 32>::new::<9>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<0, 32>::new::<32>();")]
            #[doc = concat!("let d = ", stringify!($type), "::<0, 32>::new::<1>();")]
            ///
            /// assert!(!a.is_power_of_two());
            /// assert!(!b.is_power_of_two());
            /// assert!(c.is_power_of_two());
            /// assert!(d.is_power_of_two());
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn is_power_of_two(self) -> bool {
                self.get().is_power_of_two()
            }

            /// Calculates the smallest value greater than or equal to self that
            /// is a multiple of `rhs`.
            ///
            /// Returns [`None`] on overflow or `rhs == 0`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<0, 33>::new::<16>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<0, 33>::new::<23>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<0, 33>::new::<33>();")]
            ///
            /// assert_eq!(a.checked_next_multiple_of(8).unwrap().get(), 16);
            /// assert_eq!(b.checked_next_multiple_of(8).unwrap().get(), 24);
            /// assert!(c.checked_next_multiple_of(8).is_none());
            /// assert!(a.checked_next_multiple_of(0).is_none());
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_next_multiple_of(self, rhs: impl AsRepr<$p>) -> Option<Self> {
                let rhs = as_repr::as_repr(rhs);
                let Some(value) = self.get().checked_next_multiple_of(rhs) else {
                    return None;
                };
                let Some(value) = NonZero::new(value) else {
                    unreachable!()
                };
                let Ok(value) = $type::$with(value) else {
                    return None;
                };

                Some(value)
            }

            /// Return the smallest power of two greater than or equal to self.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<0, 33>::new::<16>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<0, 33>::new::<23>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<0, 33>::new::<33>();")]
            ///
            /// assert_eq!(a.next_multiple_of::<8, 0, 40>().get(), 16);
            /// assert_eq!(b.next_multiple_of::<8, 0, 40>().get(), 24);
            /// assert_eq!(c.next_multiple_of::<8, 0, 40>().get(), 40);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn next_multiple_of<
                const RHS: $p,
                const OUT_MIN: $p,
                const OUT_MAX: $p,
            >(
                self,
            ) -> $type<OUT_MIN, OUT_MAX> {
                const {
                    if OUT_MIN != MIN.next_multiple_of(RHS) {
                        panic!("mismatched OUT_MIN")
                    }

                    if OUT_MAX != MAX.next_multiple_of(RHS) {
                        panic!("mismatched OUT_MAX")
                    }
                }

                $type::from_unchecked(self.get().next_multiple_of(RHS))
            }

            /// Return `true` if `self` is an integer multiple of `rhs`, and
            /// `false` otherwise.
            ///
            /// This function is equivalent to `self % rhs == 0`, except that it
            /// will not panic for `rhs == 0`. Instead,
            /// `0.is_multiple_of(0) == true`, and for any non-zero `n`,
            /// `n.is_multiple_of(0) == false`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<0, 8>::new::<0>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<0, 8>::new::<5>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<0, 8>::new::<6>();")]
            ///
            /// assert!(a.is_multiple_of(2));
            /// assert!(!b.is_multiple_of(2));
            /// assert!(c.is_multiple_of(2));
            ///
            /// assert!(a.is_multiple_of(0));
            /// assert!(!b.is_multiple_of(0));
            /// assert!(!c.is_multiple_of(0));
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn is_multiple_of(self, rhs: impl AsRepr<$p>) -> bool {
                let rhs = as_repr::as_repr(rhs);

                match rhs {
                    0 => self.get() == 0,
                    _ => self.get() % rhs == 0,
                }
            }

            /// Get the remainder from dividing `self` by a number.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 1> = a.rem_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem_ranged<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $type<RHS_MIN, RHS_MAX>,
            ) -> Quotient<$type<0, OUTPUT_MAX>> {
                const {
                    if OUTPUT_MAX != RHS_MAX - 1 {
                        panic!("Max mismatch");
                    }
                }

                if rhs.get() == 0 {
                    Quotient::Nan
                } else {
                    Quotient::Number($type::from_unchecked(self.get() % rhs.get()))
                }
            }

            /// Get the least remainder of `self (mod rhs)`.
            ///
            /// Since, for the positive integers, all common definitions of
            /// division are equal, this is exactly equal to
            /// [`Self::rem_ranged()`].
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 1> = a.rem_euclid_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem_euclid_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem_euclid_ranged<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $type<RHS_MIN, RHS_MAX>,
            ) -> Quotient<$type<0, OUTPUT_MAX>> {
                self.rem_ranged::<RHS_MIN, RHS_MAX, OUTPUT_MAX>(rhs)
            }

            /// Get the remainder from dividing `self` by a non-zero number.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 1> = a.rem_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem_ranged_nonzero<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $nonzero<RHS_MIN, RHS_MAX>,
            ) -> $type<0, OUTPUT_MAX> {
                const {
                    if OUTPUT_MAX != RHS_MAX - 1 {
                        panic!("Max mismatch");
                    }
                }

                $type::from_unchecked(self.get() % rhs.get())
            }

            /// Get the least remainder of `self (mod rhs)`.
            ///
            /// Since, for the positive integers, all common definitions of
            /// division are equal, this is exactly equal to
            /// [`Self::rem_ranged_nonzero()`].
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 1> = a.rem_euclid_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem_euclid_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem_euclid_ranged_nonzero<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $nonzero<RHS_MIN, RHS_MAX>,
            ) -> $type<0, OUTPUT_MAX> {
                self.rem_ranged_nonzero::<RHS_MIN, RHS_MAX, OUTPUT_MAX>(rhs)
            }

            /// Get the least remainder of `self (mod rhs)`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<46, 84>::new::<65>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem::<3, _>();")]
            ///
            /// assert_eq!(output, 2);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem<
                const RHS: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<0, OUTPUT_MAX> {
                let rhs = const { $nonzero::<RHS, RHS>::new::<RHS>() };

                self.rem_ranged_nonzero(rhs)
            }

            /// Get the least remainder of `self (mod RHS)`.
            ///
            /// Since, for the positive integers, all common definitions of
            /// division are equal, this is exactly equal to
            /// [`Self::rem()`].
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<46, 84>::new::<65>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem_euclid::<3, _>();")]
            ///
            /// assert_eq!(output, 2);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem_euclid<
                const RHS: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<0, OUTPUT_MAX> {
                let rhs = const { $nonzero::<RHS, RHS>::new::<RHS>() };

                self.rem_euclid_ranged_nonzero(rhs)
            }

            /// Perform Euclidean division.
            ///
            /// Since, for the positive integers, all common definitions of
            /// division are equal, this is exactly equal to
            /// [`Self::div()`].
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<46, 84>::new::<60>();")]
            #[doc = concat!("let output: ", stringify!($type), "<23, 42> = a.div_euclid::<2, _, _>();")]
            ///
            /// assert_eq!(output, 30);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn div_euclid<
                const RHS: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                self.div::<RHS, OUTPUT_MIN, OUTPUT_MAX>()
            }

            /// Perform Euclidean division.
            ///
            /// Since, for the positive integers, all common definitions of
            /// division are equal, this is exactly equal to
            /// [`Self::div_ranged_nonzero()`].
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<1, 2> = a.div_euclid_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<1>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.div_euclid_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn div_euclid_ranged_nonzero<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $nonzero::<RHS_MIN, RHS_MAX>,
            ) -> $type::<OUTPUT_MIN, OUTPUT_MAX> {
                self.div_ranged_nonzero::<RHS_MIN, RHS_MAX, OUTPUT_MIN, OUTPUT_MAX>(rhs)
            }

            /// Divide `self` by a number.
            ///
            /// Since, for the positive integers, all common definitions of
            /// division are equal, this is exactly equal to
            /// [`Self::div_ranged()`].
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "::<1, 2> = a.div_euclid_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<1>();")]
            #[doc = concat!("let output: ", stringify!($type), "::<0, 2> = a.div_euclid_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn div_euclid_ranged<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $type<RHS_MIN, RHS_MAX>,
            ) -> Quotient<$type<OUTPUT_MIN, OUTPUT_MAX>> {
                self.div_ranged::<RHS_MIN, RHS_MAX, OUTPUT_MIN, OUTPUT_MAX>(rhs)
            }

            /// Checked integer division.
            ///
            /// Returns [`None`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", Quotient };")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 50>::new::<50>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 50>::new::<1>();")]
            ///
            /// assert_eq!(
            ///     a.checked_div(2),
            #[doc = concat!("    Some(Quotient::Number(", stringify!($type), "::new::<25>())),")]
            /// );
            /// assert_eq!(a.checked_div(0), Some(Quotient::Nan));
            /// assert_eq!(b.checked_div(2), None);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_div(
                self,
                rhs: impl AsRepr<$p>,
            ) -> Option<Quotient<Self>> {
                let rhs = as_repr::as_repr(rhs);
                let Some(value) = self.get().checked_div(rhs) else {
                    return if rhs == 0 {
                        Some(Quotient::Nan)
                    } else {
                        None
                    };
                };

                match Self::$with(value) {
                    Ok(value) => Some(Quotient::Number(value)),
                    Err(_) => None,
                }
            }

            /// Checked euclidean integer division.
            ///
            /// Since, for the positive integers, all common definitions of
            /// division are equal, this is exactly equal to
            /// [`Self::checked_div()`].
            ///
            /// Returns [`None`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", Quotient };")]
            #[doc = concat!("let a = ", stringify!($type), "::<1, 50>::new::<50>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 50>::new::<1>();")]
            ///
            /// assert_eq!(
            ///     a.checked_div_euclid(2),
            #[doc = concat!("    Some(Quotient::Number(", stringify!($type), "::new::<25>())),")]
            /// );
            /// assert_eq!(a.checked_div(0), Some(Quotient::Nan));
            /// assert_eq!(b.checked_div(2), None);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_div_euclid(
                self,
                rhs: impl AsRepr<$p>,
            ) -> Option<Quotient<Self>> {
                self.checked_div(rhs)
            }

            /// Checked integer remainder.
            ///
            /// Returns [`None`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", Quotient };")]
            #[doc = concat!("let a = ", stringify!($type), "::<0, 50>::new::<50>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<0, 50>::new::<1>();")]
            ///
            /// assert_eq!(
            ///     a.checked_rem(2),
            #[doc = concat!("    Some(Quotient::Number(", stringify!($type), "::new::<0>())),")]
            /// );
            /// assert_eq!(a.checked_rem(0), Some(Quotient::Nan));
            /// assert_eq!(b.checked_rem(2).unwrap().number().unwrap(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_rem(
                self,
                rhs: impl AsRepr<$p>,
            ) -> Option<Quotient<Self>> {
                let rhs = as_repr::as_repr(rhs);
                let Some(value) = self.get().checked_rem(rhs) else {
                    return if rhs == 0 {
                        Some(Quotient::Nan)
                    } else {
                        None
                    };
                };

                match Self::$with(value) {
                    Ok(value) => Some(Quotient::Number(value)),
                    Err(_) => None,
                }
            }

            /// Checked integer euclidean remainder.
            ///
            /// Since, for the positive integers, all common definitions of
            /// division are equal, this is exactly equal to
            /// [`Self::checked_rem()`].
            ///
            /// Returns [`None`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", Quotient };")]
            #[doc = concat!("let a = ", stringify!($type), "::<0, 50>::new::<50>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<0, 50>::new::<1>();")]
            ///
            /// assert_eq!(
            ///     a.checked_rem_euclid(2),
            #[doc = concat!("    Some(Quotient::Number(", stringify!($type), "::new::<0>())),")]
            /// );
            /// assert_eq!(a.checked_rem_euclid(0), Some(Quotient::Nan));
            /// assert_eq!(b.checked_rem_euclid(2).unwrap().number().unwrap(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_rem_euclid(
                self,
                rhs: impl AsRepr<$p>,
            ) -> Option<Quotient<Self>> {
                self.checked_rem(rhs)
            }
        }
    };
}

macro_rules! impl_ops_signed {
    ($type:ident, $p:ty, $nonzero:ident, $with:ident $(,)?) => {
        impl<const MIN: $p, const MAX: $p> $type<MIN, MAX> {
            /// Checked integer remainder.
            ///
            /// Returns [`Err`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", Quotient };")]
            #[doc = concat!("let a = ", stringify!($type), "::<0, 5>::new::<5>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<0, 5>::new::<1>();")]
            ///
            /// assert_eq!(
            ///     a.checked_rem(2),
            #[doc = concat!("    Ok(Quotient::Number(", stringify!($type), "::new::<1>())),")]
            /// );
            /// assert_eq!(a.checked_rem(0), Ok(Quotient::Nan));
            /// assert_eq!(b.checked_rem(2).unwrap().number().unwrap(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_rem(
                self,
                rhs: impl AsRepr<$p>,
            ) -> Result<Quotient<Self>> {
                let rhs = as_repr::as_repr(rhs);

                if rhs == 0 {
                    return Ok(Quotient::Nan);
                }

                let Some(value) = self.get().checked_rem(rhs) else {
                    return Err(if self.is_negative() ^ rhs.is_negative() {
                        Error::PosOverflow
                    } else {
                        Error::NegOverflow
                    });
                };

                match Self::$with(value) {
                    Ok(v) => Ok(Quotient::Number(v)),
                    Err(e) => Err(e),
                }
            }

            /// Checked integer euclidean remainder.
            ///
            /// Returns [`None`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", Quotient };")]
            #[doc = concat!("let a = ", stringify!($type), "::<0, 5>::new::<5>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<0, 5>::new::<1>();")]
            ///
            /// assert_eq!(
            ///     a.checked_rem_euclid(2),
            #[doc = concat!("    Some(Quotient::Number(", stringify!($type), "::new::<1>())),")]
            /// );
            /// assert_eq!(a.checked_rem_euclid(0), Some(Quotient::Nan));
            /// assert_eq!(b.checked_rem_euclid(2).unwrap().number().unwrap(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_rem_euclid(
                self,
                rhs: impl AsRepr<$p>,
            ) -> Option<Quotient<Self>> {
                let rhs = as_repr::as_repr(rhs);

                if rhs == 0 {
                    return Some(Quotient::Nan);
                }

                let Some(value) = self.get().checked_rem_euclid(rhs) else {
                    return None;
                };
                let Ok(value) = Self::$with(value) else {
                    return None;
                };

                Some(Quotient::Number(value))
            }

            /// Checked integer division.
            ///
            /// Returns an [`Error`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", Quotient };")]
            #[doc = concat!("let a = ", stringify!($type), "::<-100, 10>::new::<-50>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<-10, 100>::new::<50>();")]
            ///
            /// assert_eq!(
            ///     a.checked_div(2),
            #[doc = concat!("    Ok(Quotient::Number(", stringify!($type), "::new::<-25>())),")]
            /// );
            /// assert_eq!(a.checked_div(0), Ok(Quotient::Nan));
            /// assert_eq!(a.checked_div(-1), Err(Error::PosOverflow));
            /// assert_eq!(b.checked_div(-2), Err(Error::NegOverflow));
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_div(
                self,
                rhs: impl AsRepr<$p>,
            ) -> Result<Quotient<Self>> {
                let rhs = as_repr::as_repr(rhs);

                if rhs == 0 {
                    return Ok(Quotient::Nan);
                }

                let Some(value) = self.get().checked_div(rhs) else {
                    return Err(if self.is_negative() ^ rhs.is_negative() {
                        Error::PosOverflow
                    } else {
                        Error::NegOverflow
                    });
                };

                match Self::$with(value) {
                    Ok(v) => Ok(Quotient::Number(v)),
                    Err(e) => Err(e),
                }
            }

            /// Checked euclidean integer division.
            ///
            /// Returns an [`Error`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{Error, ", stringify!($type), ", Quotient };")]
            #[doc = concat!("let a = ", stringify!($type), "::<-100, 10>::new::<-50>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<-10, 100>::new::<50>();")]
            #[doc = concat!("let c = ", stringify!($type), "::<-10, 10>::new::<7>();")]
            #[doc = concat!("let d = ", stringify!($type), "::<-10, 10>::new::<4>();")]
            ///
            /// assert_eq!(
            ///     a.checked_div_euclid(2),
            #[doc = concat!("    Ok(Quotient::Number(", stringify!($type), "::new::<-25>())),")]
            /// );
            /// assert_eq!(a.checked_div_euclid(0), Ok(Quotient::Nan));
            /// assert_eq!(a.checked_div_euclid(-1), Err(Error::PosOverflow));
            /// assert_eq!(b.checked_div_euclid(-2), Err(Error::NegOverflow));
            ///
            /// assert_eq!(c.checked_div_euclid(d).unwrap().number().unwrap(), 1);
            /// assert_eq!(c.checked_div_euclid(-d).unwrap().number().unwrap(), -1);
            /// assert_eq!((-c).checked_div_euclid(d).unwrap().number().unwrap(), -2);
            /// assert_eq!((-c).checked_div_euclid(-d).unwrap().number().unwrap(), 2);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn checked_div_euclid(
                self,
                rhs: impl AsRepr<$p>,
            ) -> Result<Quotient<Self>> {
                let rhs = as_repr::as_repr(rhs);

                if rhs == 0 {
                    return Ok(Quotient::Nan);
                }

                let Some(value) = self.get().checked_div_euclid(rhs) else {
                    return Err(if self.is_negative() ^ rhs.is_negative() {
                        Error::PosOverflow
                    } else {
                        Error::NegOverflow
                    });
                };

                match Self::$with(value) {
                    Ok(v) => Ok(Quotient::Number(v)),
                    Err(e) => Err(e),
                }
            }

            /// Perform Euclidean division.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<46, 84>::new::<60>();")]
            #[doc = concat!("let output: ", stringify!($type), "<23, 42> = a.div_euclid::<2, _, _>();")]
            ///
            /// assert_eq!(output, 30);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn div_euclid<
                const RHS: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                let rhs = const { $nonzero::new::<RHS>() };

                self.div_euclid_ranged_nonzero::<RHS, RHS, OUTPUT_MIN, OUTPUT_MAX>(rhs)
            }

            /// Perform Euclidean division.
            ///
            /// Since, for the positive integers, all common definitions of
            /// division are equal, this is exactly equal to
            /// [`Self::div_ranged_nonzero()`].
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<1, 2> = a.div_euclid_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<1>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.div_euclid_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn div_euclid_ranged_nonzero<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $nonzero::<RHS_MIN, RHS_MAX>,
            ) -> $type::<OUTPUT_MIN, OUTPUT_MAX> {
                match self.div_euclid_ranged::<RHS_MIN, RHS_MAX, OUTPUT_MIN, OUTPUT_MAX>(rhs.to_ranged()) {
                    Quotient::Number(x) => x,
                    Quotient::Nan => unreachable!(),
                }
            }

            /// Divide `self` by a number.
            ///
            /// Since, for the positive integers, all common definitions of
            /// division are equal, this is exactly equal to
            /// [`Self::div_ranged()`].
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "::<1, 2> = a.div_euclid_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<1>();")]
            #[doc = concat!("let output: ", stringify!($type), "::<0, 2> = a.div_euclid_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output.get(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn div_euclid_ranged<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $type<RHS_MIN, RHS_MAX>,
            ) -> Quotient<$type<OUTPUT_MIN, OUTPUT_MAX>> {
                const {
                    let (min_min, min_max) = (MIN.div_euclid(RHS_MIN), MIN.div_euclid(RHS_MAX));
                    let (max_min, max_max) = (MAX.div_euclid(RHS_MIN), MAX.div_euclid(RHS_MAX));
                    let min = if min_min < min_max { min_min } else { min_max };
                    let min = if max_min < min { max_min } else { min };
                    let min = if max_max < min { max_max } else { min };
                    let max = if max_min > max_max { max_min } else { max_max };
                    let max = if min_min > min { min_min } else { max };
                    let max = if min_max > min { min_max } else { max };

                    if min != OUTPUT_MIN {
                        panic!("Min mismatch");
                    }

                    if max != OUTPUT_MAX {
                        panic!("Max mismatch");
                    }
                }

                if rhs.get() == 0 {
                    Quotient::Nan
                } else {
                    Quotient::Number($type::from_unchecked(self.get().div_euclid(rhs.get())))
                }
            }

            /// Get the remainder from dividing `self` by a number.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 1> = a.rem_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem_ranged<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $type<RHS_MIN, RHS_MAX>,
            ) -> Quotient<$type<OUTPUT_MIN, OUTPUT_MAX>> {
                const {
                    let (min, max) = match
                        (RHS_MIN < 0, RHS_MAX > 0, MIN < 0, MAX > 0)
                    {
                        (true, true, _, _)
                            | (_, _, true, true)
                            | (true, false, false, true)
                            | (false, true, true, false)
                        => {
                            let min = RHS_MIN.abs();
                            let max = RHS_MAX.abs();
                            let bounds = if max > min { max } else { min };

                            (-(bounds - 1), bounds - 1)
                        }
                        (false, true, false, true) => (0, RHS_MAX - 1),
                        (true, false, true, false) => (RHS_MIN + 1, 0),
                        (false, false, _, _) | (_, _, false, false) => (0, 0),
                    };

                    if min != OUTPUT_MIN {
                        panic!("Max mismatch");
                    }

                    if max != OUTPUT_MAX {
                        panic!("Max mismatch");
                    }
                }

                if rhs.get() == 0 {
                    Quotient::Nan
                } else {
                    Quotient::Number($type::from_unchecked(self.get() % rhs.get()))
                }
            }

            /// Get the least remainder of `self (mod rhs)`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 1> = a.rem_euclid_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($type), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem_euclid_ranged(b).number().unwrap();")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem_euclid_ranged<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $type<RHS_MIN, RHS_MAX>,
            ) -> Quotient<$type<0, OUTPUT_MAX>> {
                const {
                    let max_abs = RHS_MAX.abs();
                    let min_abs = RHS_MIN.abs();
                    let rhs_limit = if max_abs > min_abs {
                        max_abs
                    } else {
                        min_abs
                    };

                    if OUTPUT_MAX != rhs_limit - 1 {
                        panic!("Max mismatch");
                    }
                }

                if rhs.get() == 0 {
                    Quotient::Nan
                } else {
                    Quotient::Number($type::from_unchecked(self.get().rem_euclid(rhs.get())))
                }
            }

            /// Get the remainder from dividing `self` by a non-zero number.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 1> = a.rem_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem_ranged_nonzero<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $nonzero<RHS_MIN, RHS_MAX>,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                match self.rem_ranged::<RHS_MIN, RHS_MAX, OUTPUT_MIN, OUTPUT_MAX>(
                    rhs.to_ranged()
                ) {
                    Quotient::Number(n) => n,
                    Quotient::Nan => unimplemented!(),
                }
            }

            /// Get the least remainder of `self (mod rhs)`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 1> = a.rem_euclid_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            ///
            /// Does not compile:
            //
            /// ```compile_fail,E0080
            #[doc = concat!("# use ranch::{", stringify!($type), ", ", stringify!($nonzero), "};")]
            #[doc = concat!("let a = ", stringify!($type), "::<2, 5>::new::<3>();")]
            #[doc = concat!("let b = ", stringify!($nonzero), "::<1, 2>::new::<2>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem_euclid_ranged_nonzero(b);")]
            ///
            /// assert_eq!(output, 1);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem_euclid_ranged_nonzero<
                const RHS_MIN: $p,
                const RHS_MAX: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
                rhs: $nonzero<RHS_MIN, RHS_MAX>,
            ) -> $type<0, OUTPUT_MAX> {
                match self.rem_euclid_ranged::<RHS_MIN, RHS_MAX, OUTPUT_MAX>(
                    rhs.to_ranged()
                ) {
                    Quotient::Number(n) => n,
                    Quotient::Nan => unimplemented!(),
                }
            }

            /// Get the least remainder of `self (mod rhs)`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<46, 84>::new::<65>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem::<3, _, _>();")]
            ///
            /// assert_eq!(output, 2);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem<
                const RHS: $p,
                const OUTPUT_MIN: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<OUTPUT_MIN, OUTPUT_MAX> {
                let rhs = const { $nonzero::<RHS, RHS>::new::<RHS>() };

                self.rem_ranged_nonzero(rhs)
            }

            /// Get the least remainder of `self (mod RHS)`.
            ///
            /// ```rust
            #[doc = concat!("# use ranch::", stringify!($type), ";")]
            #[doc = concat!("let a = ", stringify!($type), "::<46, 84>::new::<65>();")]
            #[doc = concat!("let output: ", stringify!($type), "<0, 2> = a.rem_euclid::<3, _>();")]
            ///
            /// assert_eq!(output, 2);
            /// ```
            #[must_use = "this returns the result of the operation, \
                          without modifying the original"]
            pub const fn rem_euclid<
                const RHS: $p,
                const OUTPUT_MAX: $p,
            >(
                self,
            ) -> $type<0, OUTPUT_MAX> {
                let rhs = const { $nonzero::<RHS, RHS>::new::<RHS>() };

                self.rem_euclid_ranged_nonzero(rhs)
            }
        }
    };
}

impl_ops!(
    RangedI8,
    i8,
    RangedNonZeroI8,
    Result,
    signed_nan_unreachable,
);
impl_ops!(
    RangedI16,
    i16,
    RangedNonZeroI16,
    Result,
    signed_nan_unreachable,
);
impl_ops!(
    RangedI32,
    i32,
    RangedNonZeroI32,
    Result,
    signed_nan_unreachable,
);
impl_ops!(
    RangedI64,
    i64,
    RangedNonZeroI64,
    Result,
    signed_nan_unreachable,
);
impl_ops!(
    RangedI128,
    i128,
    RangedNonZeroI128,
    Result,
    signed_nan_unreachable,
);

impl_ops!(
    RangedU8,
    u8,
    RangedNonZeroU8,
    Option,
    unsigned_nan_unreachable,
);
impl_ops!(
    RangedU16,
    u16,
    RangedNonZeroU16,
    Option,
    unsigned_nan_unreachable,
);
impl_ops!(
    RangedU32,
    u32,
    RangedNonZeroU32,
    Option,
    unsigned_nan_unreachable,
);
impl_ops!(
    RangedU64,
    u64,
    RangedNonZeroU64,
    Option,
    unsigned_nan_unreachable,
);
impl_ops!(
    RangedU128,
    u128,
    RangedNonZeroU128,
    Option,
    unsigned_nan_unreachable,
);

impl_ops_nonzero_signed!(RangedNonZeroI8, i8);
impl_ops_nonzero_signed!(RangedNonZeroI16, i16);
impl_ops_nonzero_signed!(RangedNonZeroI32, i32);
impl_ops_nonzero_signed!(RangedNonZeroI64, i64);
impl_ops_nonzero_signed!(RangedNonZeroI128, i128);

impl_ops_nonzero_unsigned!(RangedNonZeroU8, u8);
impl_ops_nonzero_unsigned!(RangedNonZeroU16, u16);
impl_ops_nonzero_unsigned!(RangedNonZeroU32, u32);
impl_ops_nonzero_unsigned!(RangedNonZeroU64, u64);
impl_ops_nonzero_unsigned!(RangedNonZeroU128, u128);

impl_ops_unsigned!(RangedU8, u8, RangedNonZeroU8, with_u8);
impl_ops_unsigned!(RangedU16, u16, RangedNonZeroU16, with_u16);
impl_ops_unsigned!(RangedU32, u32, RangedNonZeroU32, with_u32);
impl_ops_unsigned!(RangedU64, u64, RangedNonZeroU64, with_u64);
impl_ops_unsigned!(RangedU128, u128, RangedNonZeroU128, with_u128);

impl_ops_signed!(RangedI8, i8, RangedNonZeroI8, with_i8);
impl_ops_signed!(RangedI16, i16, RangedNonZeroI16, with_i16);
impl_ops_signed!(RangedI32, i32, RangedNonZeroI32, with_i32);
impl_ops_signed!(RangedI64, i64, RangedNonZeroI64, with_i64);
impl_ops_signed!(RangedI128, i128, RangedNonZeroI128, with_i128);

const fn signed_nan_unreachable<T>(result: Result<Quotient<T>>) -> Result<T>
where
    T: Copy + Clone,
{
    match result {
        Ok(Quotient::Number(number)) => Ok(number),
        Ok(Quotient::Nan) => unreachable!(),
        Err(e) => Err(e),
    }
}

const fn unsigned_nan_unreachable<T>(option: Option<Quotient<T>>) -> Option<T>
where
    T: Copy + Clone,
{
    match option {
        Some(Quotient::Number(number)) => Some(number),
        Some(Quotient::Nan) => unreachable!(),
        None => None,
    }
}
