use as_repr::AsRepr;

use crate::{Error, ParsingError, ParsingResult, Quotient, RangedU32, Result};

/// [`u128`] with a specified minimum and maximum value
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct RangedU128<const MIN: u128, const MAX: u128>(pub(crate) u128);

impl<const MIN: u128, const MAX: u128> RangedU128<MIN, MAX> {
    /// The size of this integer type in bits.
    pub const BITS: u32 = u128::BITS;
    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Self(MAX);
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self(MIN);

    /// Create a new ranged integer.
    ///
    /// Won't compile if out of bounds.
    ///
    /// Compiles:
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// RangedU128::<1, 3>::new::<1>();
    /// RangedU128::<1, 3>::new::<2>();
    /// RangedU128::<1, 3>::new::<3>();
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// RangedU128::<1, 3>::new::<0>();
    /// ```
    ///
    /// ```compile_fail
    /// RangedU128::<1, 3>::new::<4>();
    /// ```
    #[must_use]
    pub const fn new<const N: u128>() -> Self {
        const {
            Self::assert_range();

            if N < MIN || N > MAX {
                panic!("Out of bounds");
            }
        }

        Self(N)
    }

    /// Try to create a new ranged integer.
    ///
    /// Returns `Err` if out of bounds.
    ///
    /// ```rust
    /// # use ranch::{RangedU128, Error};
    /// RangedU128::<1, 2>::with_u128(1).unwrap();
    /// RangedU128::<1, 2>::with_u128(2).unwrap();
    /// assert_eq!(RangedU128::<1, 2>::with_u128(0).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(RangedU128::<1, 2>::with_u128(3).unwrap_err(), Error::PosOverflow);
    /// ```
    pub const fn with_u128(value: impl AsRepr<u128>) -> Result<Self> {
        const { Self::assert_range() };

        let value = as_repr::as_repr(value);

        if value < MIN {
            return Err(Error::NegOverflow);
        }

        if value > MAX {
            return Err(Error::PosOverflow);
        }

        Ok(Self(value))
    }

    /// Return the contained value as a primitive type.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// assert_eq!(42, RangedU128::<1, 100>::new::<42>().get());
    /// ```
    #[must_use]
    pub const fn get(self) -> u128 {
        self.0
    }

    /// Return the number of leading zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let n = RangedU128::<{ u128::MIN }, { u128::MAX }>::MAX;
    ///
    /// assert_eq!(n.leading_zeros().get(), 0);
    /// ```
    #[must_use]
    pub const fn leading_zeros(self) -> RangedU32<0, { u128::BITS }> {
        RangedU32(self.get().leading_zeros())
    }

    /// Return the number of trailing zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let n = RangedU128::<0, 255>::new::<0b0101000>();
    ///
    /// assert_eq!(n.trailing_zeros().get(), 3);
    /// ```
    #[must_use]
    pub const fn trailing_zeros(self) -> RangedU32<0, { u128::BITS }> {
        RangedU32(self.get().trailing_zeros())
    }

    /// Return the number of ones in the binary representation of `self`.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<0, 255>::new::<0b100_0000>();
    /// let b = RangedU128::<0, 255>::new::<0b100_0011>();
    ///
    /// assert_eq!(a.count_ones().get(), 1);
    /// assert_eq!(b.count_ones().get(), 3);
    /// ```
    #[must_use]
    pub const fn count_ones(self) -> RangedU32<0, { u128::BITS }> {
        RangedU32(self.get().count_ones())
    }

    /// Add two ranged integers together.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<1, 100>::new::<50>();
    /// let b = RangedU128::<1, 100>::new::<5>();
    /// let c = a.checked_add(b).unwrap();
    ///
    /// assert!(c.checked_add(a).is_none());
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.checked_add(a).unwrap().get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_add(self, other: impl AsRepr<u128>) -> Option<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_add(other) else {
            return None;
        };

        match Self::with_u128(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Add two ranged integers together.
    ///
    /// Returns [`Self::MAX`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<1, 100>::new::<50>();
    /// let b = RangedU128::<1, 100>::new::<5>();
    /// let c = a.saturating_add(b);
    ///
    /// assert_eq!(c.saturating_add(a).get(), 100);
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.saturating_add(a).get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_add(self, other: impl AsRepr<u128>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_u128(self.get().saturating_add(other)) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU128};
    /// let a = RangedU128::<0, 100>::new::<50>();
    /// let b = RangedU128::<0, 100>::new::<5>();
    /// let c = RangedU128::<0, 100>::new::<75>();
    ///
    /// assert_eq!(b.checked_mul(b).unwrap().get(), 25);
    /// assert_eq!(a.checked_mul(c), None);
    /// assert_eq!(c.checked_mul(c), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_mul(self, other: impl AsRepr<u128>) -> Option<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_mul(other) else {
            return None;
        };

        match Self::with_u128(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns [`Self::MAX`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU128};
    /// let a = RangedU128::<0, 100>::new::<50>();
    /// let b = RangedU128::<0, 100>::new::<5>();
    /// let c = RangedU128::<0, 100>::new::<75>();
    ///
    /// assert_eq!(b.saturating_mul(b).get(), 25);
    /// assert_eq!(a.saturating_mul(c).get(), 100);
    /// assert_eq!(c.saturating_mul(c).get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_mul(self, other: impl AsRepr<u128>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_u128(self.get().saturating_mul(other)) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU128};
    /// let a = RangedU128::<0, 100>::new::<50>();
    /// let b = RangedU128::<0, 100>::new::<5>();
    /// let c = RangedU128::<0, 100>::new::<2>();
    ///
    /// assert_eq!(a.checked_pow(2), None);
    /// assert_eq!(b.checked_pow(2).unwrap().get(), 25);
    /// assert_eq!(c.checked_pow(3).unwrap().get(), 8);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_pow(self, other: impl AsRepr<u32>) -> Option<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_pow(other) else {
            return None;
        };

        match Self::with_u128(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns [`Self::MAX`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU128};
    /// let a = RangedU128::<0, 100>::new::<50>();
    /// let b = RangedU128::<0, 100>::new::<5>();
    /// let c = RangedU128::<0, 100>::new::<2>();
    ///
    /// assert_eq!(a.saturating_pow(2).get(), 100);
    /// assert_eq!(b.saturating_pow(2).get(), 25);
    /// assert_eq!(c.saturating_pow(3).get(), 8);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_pow(self, other: impl AsRepr<u32>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_u128(self.get().saturating_pow(other)) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Checked integer division.
    ///
    /// Returns [`None`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU128, Quotient};
    /// let a = RangedU128::<1, 50>::new::<50>();
    /// let b = RangedU128::<1, 50>::new::<1>();
    ///
    /// assert_eq!(
    ///     a.checked_div(2),
    ///     Some(Quotient::Number(RangedU128::new::<25>())),
    /// );
    /// assert_eq!(a.checked_div(0), Some(Quotient::Nan));
    /// assert_eq!(b.checked_div(2), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_div(
        self,
        rhs: impl AsRepr<u128>,
    ) -> Option<Quotient<Self>> {
        let rhs = as_repr::as_repr(rhs);
        let Some(value) = self.get().checked_div(rhs) else {
            return Some(Quotient::Nan);
        };

        match Self::with_u128(value) {
            Ok(value) => Some(Quotient::Number(value)),
            Err(_) => None,
        }
    }

    /// Saturating integer division.
    ///
    /// Returns [`Self::MIN`] on overflow, and [`Quotient::Nan`] if `rhs` is 0.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU128, Quotient};
    /// let a = RangedU128::<1, 50>::new::<50>();
    /// let b = RangedU128::<1, 50>::new::<1>();
    ///
    /// assert_eq!(
    ///     a.saturating_div(2),
    ///     Quotient::Number(RangedU128::new::<25>()),
    /// );
    /// assert_eq!(a.saturating_div(0), Quotient::Nan);
    /// assert_eq!(
    ///     b.saturating_div(2),
    ///     Quotient::Number(RangedU128::new::<1>()),
    /// );
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_div(
        self,
        rhs: impl AsRepr<u128>,
    ) -> Quotient<Self> {
        let rhs = as_repr::as_repr(rhs);

        if rhs == 0 {
            return Quotient::Nan;
        }

        Quotient::Number(
            match Self::with_u128(self.get().saturating_div(rhs)) {
                Ok(value) => value,
                Err(_) => Self::MIN,
            },
        )
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<1, 100>::new::<50>();
    /// let b = a.checked_sub(5).unwrap();
    ///
    /// assert_eq!(b.get(), 45);
    /// assert!(a.checked_sub(a).is_none());
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_sub(self, other: impl AsRepr<u128>) -> Option<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_sub(other) else {
            return None;
        };

        match Self::with_u128(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`Self::MIN`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU128};
    /// let a = RangedU128::<1, 100>::new::<50>();
    /// let b = a.saturating_sub(5);
    ///
    /// assert_eq!(b.get(), 45);
    /// assert_eq!(a.saturating_sub(a).get(), 1);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_sub(self, other: impl AsRepr<u128>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_u128(self.get().saturating_sub(other)) {
            Ok(value) => value,
            Err(_) => Self::MIN,
        }
    }

    /// Return the smallest power of two greater than or equal to self.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU128};
    /// let a = RangedU128::<0, 33>::new::<0>();
    /// let b = RangedU128::<0, 33>::new::<9>();
    /// let c = RangedU128::<0, 33>::new::<32>();
    /// let d = RangedU128::<0, 33>::new::<33>();
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

        Some(match Self::with_u128(value) {
            Ok(value) => value,
            Err(_) => return None,
        })
    }

    /// Returns true if and only if `self == (1 << k)` for some `k`.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU128};
    /// let a = RangedU128::<0, 32>::new::<0>();
    /// let b = RangedU128::<0, 32>::new::<9>();
    /// let c = RangedU128::<0, 32>::new::<32>();
    /// let d = RangedU128::<0, 32>::new::<1>();
    ///
    /// assert!(!a.is_power_of_two());
    /// assert!(!b.is_power_of_two());
    /// assert!(c.is_power_of_two());
    /// assert!(d.is_power_of_two());
    /// ```
    #[must_use]
    pub const fn is_power_of_two(self) -> bool {
        self.get().is_power_of_two()
    }

    /// Calculate the midpoint (average) between `self` and `rhs`.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<0, 8>::new::<0>();
    /// let b = RangedU128::<0, 8>::new::<2>();
    /// let c = RangedU128::<0, 8>::new::<4>();
    /// let d = RangedU128::<0, 8>::new::<3>();
    /// let e = RangedU128::<0, 8>::new::<7>();
    ///
    /// assert_eq!(a.midpoint(c), b);
    /// assert_eq!(a.midpoint(e), d);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn midpoint(self, rhs: Self) -> Self {
        let Ok(value) = Self::with_u128(self.get().midpoint(rhs.get())) else {
            panic!("unexpected midpoint value")
        };

        value
    }

    /// Add two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<1, 3>::new::<1>();
    /// let b = RangedU128::<1, 3>::new::<2>();
    /// let output: RangedU128::<2, 6> = a.add_ranged(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<1, 3>::new::<1>();
    /// let b = RangedU128::<1, 3>::new::<2>();
    /// let output: RangedU128::<1, 6> = a.add_ranged(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn add_ranged<
        const RHS_MIN: u128,
        const RHS_MAX: u128,
        const OUTPUT_MIN: u128,
        const OUTPUT_MAX: u128,
    >(
        self,
        rhs: RangedU128<RHS_MIN, RHS_MAX>,
    ) -> RangedU128<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN + RHS_MIN != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX + RHS_MAX != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedU128(self.get() + rhs.get())
    }

    /// Subtract a number from `self`.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<2, 5>::new::<3>();
    /// let b = RangedU128::<1, 2>::new::<1>();
    /// let output: RangedU128::<0, 4> = a.sub_ranged(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<2, 5>::new::<3>();
    /// let b = RangedU128::<1, 2>::new::<1>();
    /// let output: RangedU128::<0, 3> = a.sub_ranged(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn sub_ranged<
        const RHS_MIN: u128,
        const RHS_MAX: u128,
        const OUTPUT_MIN: u128,
        const OUTPUT_MAX: u128,
    >(
        self,
        rhs: RangedU128<RHS_MIN, RHS_MAX>,
    ) -> RangedU128<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN - RHS_MAX != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX - RHS_MIN != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedU128(self.get() - rhs.get())
    }

    /// Multiply two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<1, 3>::new::<1>();
    /// let b = RangedU128::<2, 3>::new::<2>();
    /// let output: RangedU128::<2, 9> = a.mul_ranged(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<1, 3>::new::<1>();
    /// let b = RangedU128::<2, 3>::new::<2>();
    /// let output: RangedU128::<1, 9> = a.mul_ranged(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn mul_ranged<
        const RHS_MIN: u128,
        const RHS_MAX: u128,
        const OUTPUT_MIN: u128,
        const OUTPUT_MAX: u128,
    >(
        self,
        rhs: RangedU128<RHS_MIN, RHS_MAX>,
    ) -> RangedU128<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN * RHS_MIN != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX * RHS_MAX != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedU128(self.get() * rhs.get())
    }

    /// Divide `self` by a number.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<2, 5>::new::<3>();
    /// let b = RangedU128::<1, 2>::new::<2>();
    /// let output: RangedU128::<1, 2> = a.div_ranged(b).number().unwrap();
    ///
    /// assert_eq!(output.get(), 1);
    /// ```
    ///
    /// Does not compile:
    //
    /// ```compile_fail
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<2, 5>::new::<3>();
    /// let b = RangedU128::<1, 2>::new::<1>();
    /// let output: RangedU128::<0, 2> = a.div_ranged(b).number().unwrap();
    ///
    /// assert_eq!(output.get(), 1);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn div_ranged<
        const RHS_MIN: u128,
        const RHS_MAX: u128,
        const OUTPUT_MIN: u128,
        const OUTPUT_MAX: u128,
    >(
        self,
        rhs: RangedU128<RHS_MIN, RHS_MAX>,
    ) -> Quotient<RangedU128<OUTPUT_MIN, OUTPUT_MAX>> {
        const {
            if MIN / RHS_MAX != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX / RHS_MAX != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        if rhs.get() == 0 {
            Quotient::Nan
        } else {
            Quotient::Number(RangedU128(self.get() / rhs.get()))
        }
    }

    /// Raise to an integer power.
    ///
    /// ```rust
    /// # use ranch::{RangedU128, RangedU32};
    /// let a = RangedU128::<1, 3>::new::<2>();
    /// let b = RangedU32::<2, 3>::new::<2>();
    /// let output: RangedU128::<1, 27> = a.pow_ranged(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::{RangedU128, RangedU32};
    /// let a = RangedU128::<1, 3>::new::<2>();
    /// let b = RangedU32::<2, 3>::new::<2>();
    /// let output: RangedU128::<0, 27> = a.pow_ranged(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn pow_ranged<
        const RHS_MIN: u32,
        const RHS_MAX: u32,
        const OUTPUT_MIN: u128,
        const OUTPUT_MAX: u128,
    >(
        self,
        rhs: RangedU32<RHS_MIN, RHS_MAX>,
    ) -> RangedU128<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN.pow(RHS_MIN) != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX.pow(RHS_MAX) != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedU128(self.get().pow(rhs.get()))
    }
}

impl<const MIN: u128, const MAX: u128> core::str::FromStr
    for RangedU128<MIN, MAX>
{
    type Err = ParsingError;

    fn from_str(src: &str) -> ParsingResult<Self> {
        let parsed = src.parse::<u128>()?;

        Self::with_u128(parsed).map_err(From::from)
    }
}

impl<const MIN: u128, const MAX: u128> crate::error::Clamp
    for RangedU128<MIN, MAX>
{
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
}
