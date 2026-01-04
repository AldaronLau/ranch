use core::num::NonZero;

use as_repr::AsRepr;

use crate::{
    Error, ParsingError, ParsingResult, Quotient, RangedI8, RangedU32, Result,
};

/// [`i8`] not to equal zero with a specified minimum and maximum value
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct RangedNonZeroI8<const MIN: i8, const MAX: i8>(
    pub(crate) NonZero<i8>,
);

impl<const MIN: i8, const MAX: i8> RangedNonZeroI8<MIN, MAX> {
    /// The size of this integer type in bits.
    pub const BITS: u32 = i8::BITS;
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
    /// # use ranch::RangedNonZeroI8;
    /// RangedNonZeroI8::<-1, 3>::new::<1>();
    /// RangedNonZeroI8::<-1, 3>::new::<2>();
    /// RangedNonZeroI8::<-1, 3>::new::<3>();
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// RangedNonZeroI8::<-1, 3>::new::<-2>();
    /// ```
    ///
    /// ```compile_fail
    /// RangedNonZeroI8::<-1, 3>::new::<0>();
    /// ```
    ///
    /// ```compile_fail
    /// RangedNonZeroI8::<-1, 3>::new::<4>();
    /// ```
    #[must_use]
    pub const fn new<const N: i8>() -> Self {
        const {
            if MIN == 0 {
                panic!("Minimum can't be zero");
            }

            if MAX == 0 {
                panic!("Maximum can't be zero");
            }

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
    /// # use ranch::{RangedNonZeroI8, Error};
    /// RangedNonZeroI8::<-1, 1>::with_i8(-1).unwrap().unwrap();
    /// RangedNonZeroI8::<-1, 1>::with_i8(1).unwrap().unwrap();
    /// assert_eq!(RangedNonZeroI8::<-1, 1>::with_i8(0), Ok(None));
    /// assert_eq!(RangedNonZeroI8::<-1, 1>::with_i8(-2).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(RangedNonZeroI8::<-1, 1>::with_i8(2).unwrap_err(), Error::PosOverflow);
    /// ```
    pub const fn with_i8(value: impl AsRepr<i8>) -> Result<Option<Self>> {
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
    /// # use ranch::RangedNonZeroI8;
    /// assert_eq!(
    ///     RangedNonZeroI8::<1, 100>::with_nonzero(NonZero::new(42).unwrap()).unwrap(),
    ///     RangedNonZeroI8::<1, 100>::new::<42>(),
    /// );
    /// ```
    pub const fn with_nonzero(
        nonzero: impl AsRepr<NonZero<i8>>,
    ) -> Result<Self> {
        let nonzero = as_repr::as_repr(nonzero);

        if nonzero.get() < MIN {
            return Err(Error::NegOverflow);
        }

        if nonzero.get() > MAX {
            return Err(Error::PosOverflow);
        }

        Ok(Self(nonzero))
    }

    /// Convert from [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedNonZeroI8, RangedI8};
    /// assert_eq!(
    ///     RangedNonZeroI8::<1, 100>::with_ranged(RangedI8::new::<42>()).unwrap(),
    ///     RangedNonZeroI8::<1, 100>::new::<42>(),
    /// );
    /// assert!(
    ///     RangedNonZeroI8::<0, 100>::with_ranged(RangedI8::new::<0>())
    ///         .is_none(),
    /// );
    /// ```
    pub const fn with_ranged(ranged: RangedI8<MIN, MAX>) -> Option<Self> {
        let Some(nonzero) = NonZero::new(ranged.get()) else {
            return None;
        };

        Some(Self(nonzero))
    }

    /// Return the contained value as a primitive type.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// assert_eq!(42, RangedNonZeroI8::<1, 100>::new::<42>().get());
    /// ```
    #[must_use]
    pub const fn get(self) -> i8 {
        self.0.get()
    }

    /// Convert to [`NonZero`].
    ///
    /// ```rust
    /// # use std::num::NonZero;
    /// # use ranch::RangedNonZeroI8;
    /// assert_eq!(
    ///     NonZero::new(42).unwrap(),
    ///     RangedNonZeroI8::<1, 100>::new::<42>().to_nonzero(),
    /// );
    /// ```
    #[must_use]
    pub const fn to_nonzero(self) -> NonZero<i8> {
        self.0
    }

    /// Convert to [`RangedI8`].
    ///
    /// ```rust
    /// # use ranch::{RangedNonZeroI8, RangedI8};
    /// assert_eq!(
    ///     RangedI8::<1, 100>::new::<42>(),
    ///     RangedNonZeroI8::<1, 100>::new::<42>().to_ranged(),
    /// );
    /// ```
    #[must_use]
    pub const fn to_ranged(self) -> RangedI8<MIN, MAX> {
        RangedI8(self.get())
    }

    /// Return the number of leading zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let n = RangedNonZeroI8::<{ i8::MIN }, { i8::MAX }>::MAX;
    ///
    /// assert_eq!(n.leading_zeros().get(), 1);
    /// ```
    #[must_use]
    pub const fn leading_zeros(self) -> RangedU32<0, { i8::BITS }> {
        RangedU32(self.get().leading_zeros())
    }

    /// Return the number of trailing zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let n = RangedNonZeroI8::<-128, 127>::new::<0b0101000>();
    ///
    /// assert_eq!(n.trailing_zeros().get(), 3);
    /// ```
    #[must_use]
    pub const fn trailing_zeros(self) -> RangedU32<0, { i8::BITS }> {
        RangedU32(self.get().trailing_zeros())
    }

    /// Return the number of ones in the binary representation of `self`.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<-128, 127>::new::<0b100_0000>();
    /// let b = RangedNonZeroI8::<-128, 127>::new::<0b100_0011>();
    ///
    /// assert_eq!(a.count_ones().get(), 1);
    /// assert_eq!(b.count_ones().get(), 3);
    /// ```
    #[must_use]
    pub const fn count_ones(self) -> RangedU32<0, { i8::BITS }> {
        RangedU32(self.get().count_ones())
    }

    /// Add two ranged integers together.
    ///
    /// Returns an [`Error`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<1, 100>::new::<50>();
    /// let b = RangedNonZeroI8::<1, 100>::new::<5>();
    /// let c = a.checked_add(b.get()).unwrap().unwrap();
    ///
    /// assert!(c.checked_add(a.get()).is_err());
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.checked_add(a.get()).unwrap().unwrap().get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_add(
        self,
        other: impl AsRepr<i8>,
    ) -> Result<Option<Self>> {
        match self.to_ranged().checked_add(other) {
            Ok(value) => Ok(Self::with_ranged(value)),
            Err(e) => Err(e),
        }
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns an [`Error`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroI8};
    /// let a = RangedNonZeroI8::<-100, 100>::new::<50>();
    /// let b = RangedNonZeroI8::<-100, 100>::new::<5>();
    /// let c = RangedNonZeroI8::<-100, 100>::new::<-75>();
    ///
    /// assert_eq!(b.checked_mul(b.get()).unwrap().unwrap().get(), 25);
    /// assert_eq!(a.checked_mul(c.get()).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(c.checked_mul(c.get()).unwrap_err(), Error::PosOverflow);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_mul(
        self,
        other: impl AsRepr<i8>,
    ) -> Result<Option<Self>> {
        match self.to_ranged().checked_mul(other) {
            Ok(value) => Ok(Self::with_ranged(value)),
            Err(e) => Err(e),
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns an [`Error`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroI8};
    /// let a = RangedNonZeroI8::<-100, 100>::new::<50>();
    /// let b = RangedNonZeroI8::<-100, 100>::new::<5>();
    /// let c = RangedNonZeroI8::<-100, 100>::new::<-75>();
    /// let d = RangedNonZeroI8::<-100, 100>::new::<2>();
    ///
    /// assert_eq!(a.checked_pow(2).unwrap_err(), Error::PosOverflow);
    /// assert_eq!(b.checked_pow(2).unwrap().get(), 25);
    /// assert_eq!(c.checked_pow(3).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(d.checked_pow(3).unwrap().get(), 8);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_pow(self, other: impl AsRepr<u32>) -> Result<Self> {
        match self.to_ranged().checked_pow(other) {
            Ok(value) => Ok(Self::with_ranged(value).unwrap()),
            Err(e) => Err(e),
        }
    }

    /// Checked integer division.
    ///
    /// Returns an [`Error`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroI8, Quotient};
    /// let a = RangedNonZeroI8::<-100, 10>::new::<-50>();
    /// let b = RangedNonZeroI8::<-10, 100>::new::<50>();
    ///
    /// assert_eq!(
    ///     a.checked_div(2),
    ///     Ok(Some(Quotient::Number(RangedNonZeroI8::new::<-25>()))),
    /// );
    /// assert_eq!(a.checked_div(0), Ok(Some(Quotient::Nan)));
    /// assert_eq!(a.checked_div(-1), Err(Error::PosOverflow));
    /// assert_eq!(b.checked_div(-2), Err(Error::NegOverflow));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_div(
        self,
        rhs: impl AsRepr<i8>,
    ) -> Result<Option<Quotient<Self>>> {
        let value = match self.to_ranged().checked_div(rhs) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        let Quotient::Number(number) = value else {
            return Ok(Some(Quotient::Nan));
        };
        let Some(number) = Self::with_ranged(number) else {
            return Ok(None);
        };

        Ok(Some(Quotient::Number(number)))
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns an [`Error`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroI8};
    /// let a = RangedNonZeroI8::<1, 100>::new::<50>();
    /// let b = a.checked_sub(5).unwrap().unwrap();
    ///
    /// assert_eq!(a.checked_sub(-51), Err(Error::PosOverflow));
    /// assert_eq!(b.get(), 45);
    /// assert_eq!(a.checked_sub(a.get()), Err(Error::NegOverflow));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_sub(
        self,
        other: impl AsRepr<i8>,
    ) -> Result<Option<Self>> {
        match self.to_ranged().checked_sub(other) {
            Ok(value) => Ok(Self::with_ranged(value)),
            Err(e) => Err(e),
        }
    }

    /// Return `true` if `self` is negative; `false` if zero or positive.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// assert!(!RangedNonZeroI8::<-100, 100>::new::<10>().is_negative());
    /// assert!(RangedNonZeroI8::<-100, 100>::new::<-10>().is_negative());
    /// ```
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.get().is_negative()
    }

    /// Return `true` if `self` is positive; `false` if zero or negative.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// assert!(RangedNonZeroI8::<-100, 100>::new::<10>().is_positive());
    /// assert!(!RangedNonZeroI8::<-100, 100>::new::<-10>().is_positive());
    /// ```
    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.get().is_positive()
    }

    /// Multiply two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<-2, 3>::new::<1>();
    /// let b = RangedNonZeroI8::<-1, 3>::new::<2>();
    /// let output: RangedNonZeroI8::<-6, 9> = a.mul_ranged(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<-2, 3>::new::<1>();
    /// let b = RangedNonZeroI8::<-1, 3>::new::<2>();
    /// let output: RangedNonZeroI8::<0, 9> = a.mul_ranged(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn mul_ranged<
        const RHS_MIN: i8,
        const RHS_MAX: i8,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedNonZeroI8<RHS_MIN, RHS_MAX>,
    ) -> RangedNonZeroI8<OUTPUT_MIN, OUTPUT_MAX> {
        RangedNonZeroI8::with_ranged(
            self.to_ranged().mul_ranged(rhs.to_ranged()),
        )
        .unwrap()
    }

    /// Raise to an integer power.
    ///
    /// ```rust
    /// # use ranch::{RangedNonZeroI8, RangedU32};
    /// let a = RangedNonZeroI8::<-1, 3>::new::<2>();
    /// let b = RangedU32::<2, 3>::new::<2>();
    /// let output: RangedNonZeroI8::<-1, 27> = a.pow_ranged(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::{RangedNonZeroI8, RangedU32};
    /// let a = RangedNonZeroI8::<1, 3>::new::<2>();
    /// let b = RangedU32::<2, 3>::new::<2>();
    /// let output: RangedNonZeroI8::<0, 27> = a.pow_ranged(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn pow_ranged<
        const RHS_MIN: u32,
        const RHS_MAX: u32,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedU32<RHS_MIN, RHS_MAX>,
    ) -> RangedNonZeroI8<OUTPUT_MIN, OUTPUT_MAX> {
        RangedNonZeroI8::with_ranged(self.to_ranged().pow_ranged(rhs)).unwrap()
    }
}

impl<const MIN: i8, const MAX: i8> core::str::FromStr
    for RangedNonZeroI8<MIN, MAX>
{
    type Err = ParsingError;

    fn from_str(src: &str) -> ParsingResult<Self> {
        let parsed = src.parse::<NonZero<i8>>()?;

        Self::with_i8(parsed.get())
            .transpose()
            .unwrap()
            .map_err(From::from)
    }
}

impl<const MIN: i8, const MAX: i8> crate::error::Clamp
    for RangedNonZeroI8<MIN, MAX>
{
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
}
