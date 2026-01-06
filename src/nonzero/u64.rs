use core::num::NonZero;

use as_repr::AsRepr;

use crate::{
    Error, ParsingError, ParsingResult, Quotient, RangedU32, RangedU64, Result,
};

/// [`u64`] not to equal zero with a specified minimum and maximum value
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct RangedNonZeroU64<const MIN: u64, const MAX: u64>(
    pub(crate) NonZero<u64>,
);

impl<const MIN: u64, const MAX: u64> RangedNonZeroU64<MIN, MAX> {
    /// The size of this integer type in bits.
    pub const BITS: u32 = u64::BITS;
    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Self::new::<MAX>();
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self::new::<MIN>();

    /// Create a new ranged integer.
    ///
    /// Won't compile if out of bounds.
    ///
    /// Compiles:
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroU64;
    /// RangedNonZeroU64::<1, 3>::new::<1>();
    /// RangedNonZeroU64::<1, 3>::new::<2>();
    /// RangedNonZeroU64::<1, 3>::new::<3>();
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// RangedNonZeroU64::<1, 3>::new::<0>();
    /// ```
    ///
    /// ```compile_fail
    /// RangedNonZeroU64::<1, 3>::new::<4>();
    /// ```
    #[must_use]
    pub const fn new<const N: u64>() -> Self {
        const {
            Self::assert_range();

            if N < MIN || N > MAX {
                panic!("Out of bounds");
            }

            Self(NonZero::new(N).unwrap())
        }
    }

    /// Try to create a new ranged integer.
    ///
    /// Returns `Err` if out of bounds, `Ok(None)` if zero.
    ///
    /// ```rust
    /// # use ranch::{RangedNonZeroU64, Error};
    /// RangedNonZeroU64::<2, 3>::with_u64(2).unwrap().unwrap();
    /// RangedNonZeroU64::<2, 3>::with_u64(3).unwrap().unwrap();
    /// assert_eq!(RangedNonZeroU64::<2, 3>::with_u64(0), Ok(None));
    /// assert_eq!(RangedNonZeroU64::<2, 3>::with_u64(1).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(RangedNonZeroU64::<2, 3>::with_u64(4).unwrap_err(), Error::PosOverflow);
    /// ```
    pub const fn with_u64(value: impl AsRepr<u64>) -> Result<Option<Self>> {
        const { Self::assert_range() };

        let value = as_repr::as_repr(value);
        let Some(value) = NonZero::new(value) else {
            return Ok(None);
        };

        match Self::with_nonzero(value) {
            Ok(v) => Ok(Some(v)),
            Err(e) => Err(e),
        }
    }

    /// Convert from [`NonZero`].
    ///
    /// ```rust
    /// # use std::num::NonZero;
    /// # use ranch::RangedNonZeroU64;
    /// assert_eq!(
    ///     RangedNonZeroU64::<1, 100>::with_nonzero(NonZero::new(42).unwrap()).unwrap(),
    ///     RangedNonZeroU64::<1, 100>::new::<42>(),
    /// );
    /// ```
    pub const fn with_nonzero(
        nonzero: impl AsRepr<NonZero<u64>>,
    ) -> Result<Self> {
        const { Self::assert_range() };

        let nonzero = as_repr::as_repr(nonzero);

        if nonzero.get() < MIN {
            return Err(Error::NegOverflow);
        }

        if nonzero.get() > MAX {
            return Err(Error::PosOverflow);
        }

        Ok(Self(nonzero))
    }

    /// Return the contained value as a primitive type.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroU64;
    /// assert_eq!(42, RangedNonZeroU64::<1, 100>::new::<42>().get());
    /// ```
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }

    /// Convert to [`NonZero`].
    ///
    /// ```rust
    /// # use std::num::NonZero;
    /// # use ranch::RangedNonZeroU64;
    /// assert_eq!(
    ///     NonZero::new(42).unwrap(),
    ///     RangedNonZeroU64::<1, 100>::new::<42>().to_nonzero(),
    /// );
    /// ```
    #[must_use]
    pub const fn to_nonzero(self) -> NonZero<u64> {
        self.0
    }

    /// Convert to [`RangedU64`].
    ///
    /// ```rust
    /// # use ranch::{RangedNonZeroU64, RangedU64};
    /// assert_eq!(
    ///     RangedU64::<1, 100>::new::<42>(),
    ///     RangedNonZeroU64::<1, 100>::new::<42>().to_ranged(),
    /// );
    /// ```
    #[must_use]
    pub const fn to_ranged(self) -> RangedU64<MIN, MAX> {
        RangedU64(self.get())
    }

    /// Return the number of leading zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroU64;
    /// let n = RangedNonZeroU64::<1, { u64::MAX }>::MAX;
    ///
    /// assert_eq!(n.leading_zeros().get(), 0);
    /// ```
    #[must_use]
    pub const fn leading_zeros(self) -> RangedU32<0, { u64::BITS }> {
        RangedU32(self.get().leading_zeros())
    }

    /// Return the number of trailing zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroU64;
    /// let n = RangedNonZeroU64::<1, 255>::new::<0b0101000>();
    ///
    /// assert_eq!(n.trailing_zeros().get(), 3);
    /// ```
    #[must_use]
    pub const fn trailing_zeros(self) -> RangedU32<0, { u64::BITS }> {
        RangedU32(self.get().trailing_zeros())
    }

    /// Return the number of ones in the binary representation of `self`.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroU64;
    /// let a = RangedNonZeroU64::<1, 255>::new::<0b100_0000>();
    /// let b = RangedNonZeroU64::<1, 255>::new::<0b100_0011>();
    ///
    /// assert_eq!(a.count_ones().get(), 1);
    /// assert_eq!(b.count_ones().get(), 3);
    /// ```
    #[must_use]
    pub const fn count_ones(self) -> RangedU32<0, { u64::BITS }> {
        RangedU32(self.get().count_ones())
    }

    /// Add two ranged integers together.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroU64;
    /// let a = RangedNonZeroU64::<1, 100>::new::<50>();
    /// let b = RangedNonZeroU64::<1, 100>::new::<5>();
    /// let c = a.checked_add(b).unwrap();
    ///
    /// assert!(c.checked_add(a).is_none());
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.checked_add(a).unwrap().get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_add(self, other: impl AsRepr<u64>) -> Option<Self> {
        let Some(value) = self.to_ranged().checked_add(other) else {
            return None;
        };

        value.to_ranged_nonzero()
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroU64};
    /// let a = RangedNonZeroU64::<1, 100>::new::<50>();
    /// let b = RangedNonZeroU64::<1, 100>::new::<5>();
    /// let c = RangedNonZeroU64::<1, 100>::new::<75>();
    ///
    /// assert_eq!(b.checked_mul(b).unwrap().get(), 25);
    /// assert_eq!(a.checked_mul(c), None);
    /// assert_eq!(c.checked_mul(c), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_mul(self, other: impl AsRepr<u64>) -> Option<Self> {
        let Some(value) = self.to_ranged().checked_mul(other) else {
            return None;
        };

        value.to_ranged_nonzero()
    }

    /// Raise to an integer power.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroU64};
    /// let a = RangedNonZeroU64::<1, 100>::new::<50>();
    /// let b = RangedNonZeroU64::<1, 100>::new::<5>();
    /// let c = RangedNonZeroU64::<1, 100>::new::<2>();
    ///
    /// assert_eq!(a.checked_pow(2), None);
    /// assert_eq!(b.checked_pow(2).unwrap().get(), 25);
    /// assert_eq!(c.checked_pow(3).unwrap().get(), 8);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_pow(self, other: impl AsRepr<u32>) -> Option<Self> {
        let Some(value) = self.to_ranged().checked_pow(other) else {
            return None;
        };

        value.to_ranged_nonzero()
    }

    /// Checked integer division.
    ///
    /// Returns [`None`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroU64, Quotient};
    /// let a = RangedNonZeroU64::<1, 50>::new::<50>();
    /// let b = RangedNonZeroU64::<1, 50>::new::<1>();
    ///
    /// assert_eq!(
    ///     a.checked_div(2),
    ///     Some(Quotient::Number(RangedNonZeroU64::new::<25>())),
    /// );
    /// assert_eq!(a.checked_div(0), Some(Quotient::Nan));
    /// assert_eq!(b.checked_div(2), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_div(
        self,
        rhs: impl AsRepr<u64>,
    ) -> Option<Quotient<Self>> {
        let Some(value) = self.to_ranged().checked_div(rhs) else {
            return None;
        };
        let Quotient::Number(number) = value else {
            return Some(Quotient::Nan);
        };
        let Some(number) = number.to_ranged_nonzero() else {
            return None;
        };

        Some(Quotient::Number(number))
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroU64;
    /// let a = RangedNonZeroU64::<1, 100>::new::<50>();
    /// let b = a.checked_sub(5).unwrap();
    ///
    /// assert_eq!(b.get(), 45);
    /// assert!(a.checked_sub(a).is_none());
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_sub(self, other: impl AsRepr<u64>) -> Option<Self> {
        let Some(value) = self.to_ranged().checked_sub(other) else {
            return None;
        };

        value.to_ranged_nonzero()
    }

    /// Return the smallest power of two greater than or equal to self.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroU64};
    /// let a = RangedNonZeroU64::<1, 33>::new::<1>();
    /// let b = RangedNonZeroU64::<1, 33>::new::<9>();
    /// let c = RangedNonZeroU64::<1, 33>::new::<32>();
    /// let d = RangedNonZeroU64::<1, 33>::new::<33>();
    ///
    /// assert_eq!(a.checked_next_power_of_two().unwrap().get(), 1);
    /// assert_eq!(b.checked_next_power_of_two().unwrap().get(), 16);
    /// assert_eq!(c.checked_next_power_of_two().unwrap().get(), 32);
    /// assert_eq!(d.checked_next_power_of_two(), None);
    /// ```
    #[must_use]
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        let Some(value) = self.to_ranged().checked_next_power_of_two() else {
            return None;
        };

        value.to_ranged_nonzero()
    }

    /// Returns true if and only if `self == (1 << k)` for some `k`.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroU64};
    /// let a = RangedNonZeroU64::<1, 32>::new::<3>();
    /// let b = RangedNonZeroU64::<1, 32>::new::<9>();
    /// let c = RangedNonZeroU64::<1, 32>::new::<32>();
    /// let d = RangedNonZeroU64::<1, 32>::new::<1>();
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

    /// Add two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroU64;
    /// let a = RangedNonZeroU64::<1, 3>::new::<1>();
    /// let b = RangedNonZeroU64::<1, 3>::new::<2>();
    /// let output: RangedNonZeroU64::<2, 6> = a.add_ranged(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedNonZeroU64;
    /// let a = RangedNonZeroU64::<1, 3>::new::<1>();
    /// let b = RangedNonZeroU64::<1, 3>::new::<2>();
    /// let output: RangedNonZeroU64::<1, 6> = a.add_ranged(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn add_ranged<
        const RHS_MIN: u64,
        const RHS_MAX: u64,
        const OUTPUT_MIN: u64,
        const OUTPUT_MAX: u64,
    >(
        self,
        rhs: RangedNonZeroU64<RHS_MIN, RHS_MAX>,
    ) -> RangedNonZeroU64<OUTPUT_MIN, OUTPUT_MAX> {
        self.to_ranged()
            .add_ranged::<RHS_MIN, RHS_MAX, OUTPUT_MIN, OUTPUT_MAX>(
                rhs.to_ranged(),
            )
            .to_ranged_nonzero()
            .unwrap()
    }

    /// Multiply two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroU64;
    /// let a = RangedNonZeroU64::<1, 3>::new::<1>();
    /// let b = RangedNonZeroU64::<2, 3>::new::<2>();
    /// let output: RangedNonZeroU64::<2, 9> = a.mul_ranged(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedNonZeroU64;
    /// let a = RangedNonZeroU64::<1, 3>::new::<1>();
    /// let b = RangedNonZeroU64::<2, 3>::new::<2>();
    /// let output: RangedNonZeroU64::<1, 9> = a.mul_ranged(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn mul_ranged<
        const RHS_MIN: u64,
        const RHS_MAX: u64,
        const OUTPUT_MIN: u64,
        const OUTPUT_MAX: u64,
    >(
        self,
        rhs: RangedNonZeroU64<RHS_MIN, RHS_MAX>,
    ) -> RangedNonZeroU64<OUTPUT_MIN, OUTPUT_MAX> {
        self.to_ranged()
            .mul_ranged::<RHS_MIN, RHS_MAX, OUTPUT_MIN, OUTPUT_MAX>(
                rhs.to_ranged(),
            )
            .to_ranged_nonzero()
            .unwrap()
    }

    /// Raise to an integer power.
    ///
    /// ```rust
    /// # use ranch::{RangedNonZeroU64, RangedU32};
    /// let a = RangedNonZeroU64::<1, 3>::new::<2>();
    /// let b = RangedU32::<2, 3>::new::<2>();
    /// let output: RangedNonZeroU64::<1, 27> = a.pow_ranged(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::{RangedNonZeroU64, RangedU32};
    /// let a = RangedNonZeroU64::<1, 3>::new::<2>();
    /// let b = RangedU32::<2, 3>::new::<2>();
    /// let output: RangedNonZeroU64::<0, 27> = a.pow_ranged(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn pow_ranged<
        const RHS_MIN: u32,
        const RHS_MAX: u32,
        const OUTPUT_MIN: u64,
        const OUTPUT_MAX: u64,
    >(
        self,
        rhs: RangedU32<RHS_MIN, RHS_MAX>,
    ) -> RangedNonZeroU64<OUTPUT_MIN, OUTPUT_MAX> {
        self.to_ranged()
            .pow_ranged::<RHS_MIN, RHS_MAX, OUTPUT_MIN, OUTPUT_MAX>(rhs)
            .to_ranged_nonzero()
            .unwrap()
    }

    /// Calculate the midpoint (average) between `self` and `rhs`.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroU64;
    /// let a = RangedNonZeroU64::<1, 8>::new::<1>();
    /// let b = RangedNonZeroU64::<1, 8>::new::<3>();
    /// let c = RangedNonZeroU64::<1, 8>::new::<5>();
    /// let d = RangedNonZeroU64::<1, 8>::new::<4>();
    /// let e = RangedNonZeroU64::<1, 8>::new::<8>();
    ///
    /// assert_eq!(a.midpoint(c), b);
    /// assert_eq!(a.midpoint(e), d);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn midpoint(self, rhs: Self) -> Self {
        let Ok(Some(value)) = Self::with_u64(self.get().midpoint(rhs.get()))
        else {
            panic!("unexpected midpoint value")
        };

        value
    }
}

impl<const MIN: u64, const MAX: u64> core::str::FromStr
    for RangedNonZeroU64<MIN, MAX>
{
    type Err = ParsingError;

    fn from_str(src: &str) -> ParsingResult<Self> {
        let parsed = src.parse::<NonZero<u64>>()?;

        Self::with_u64(parsed.get())
            .transpose()
            .unwrap()
            .map_err(From::from)
    }
}

impl<const MIN: u64, const MAX: u64> crate::error::Clamp
    for RangedNonZeroU64<MIN, MAX>
{
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
}
