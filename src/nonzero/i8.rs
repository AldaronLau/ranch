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

// unsafe: `repr(transparent)` is `repr(NonZero<i8>)`
#[expect(unsafe_code)]
unsafe impl<const MIN: i8, const MAX: i8> AsRepr<NonZero<i8>>
    for RangedNonZeroI8<MIN, MAX>
{
}

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
    /// assert_eq!(n.leading_zeros(), 1);
    /// ```
    #[must_use]
    pub const fn leading_zeros(self) -> u32 {
        self.get().leading_zeros()
    }

    /// Return the number of trailing zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let n = RangedNonZeroI8::<-128, 127>::new::<0b0101000>();
    ///
    /// assert_eq!(n.trailing_zeros(), 3);
    /// ```
    #[must_use]
    pub const fn trailing_zeros(self) -> u32 {
        self.get().trailing_zeros()
    }

    /// Return the number of ones in the binary representation of `self`.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<-128, 127>::new::<0b100_0000>();
    /// let b = RangedNonZeroI8::<-128, 127>::new::<0b100_0011>();
    ///
    /// assert_eq!(a.count_ones(), 1);
    /// assert_eq!(b.count_ones(), 3);
    /// ```
    #[must_use]
    pub const fn count_ones(self) -> u32 {
        self.get().count_ones()
    }

    /*
    /// Add two ranged integers together.
    ///
    /// Returns an [`Error`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<1, 100>::new::<50>();
    /// let b = RangedNonZeroI8::<1, 100>::new::<5>();
    /// let c = a.checked_add(b).unwrap();
    ///
    /// assert!(c.checked_add(a).is_err());
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.checked_add(a).unwrap().get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_add(self, other: impl AsRepr<i8>) -> Result<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_add(other) else {
            return Err(
                if self.get().saturating_add(other) == Self::MAX.get() {
                    Error::PosOverflow
                } else {
                    Error::NegOverflow
                },
            );
        };

        Self::with_i8(value)
    }

    /// Add two ranged integers together.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<-100, 100>::new::<50>();
    /// let b = RangedNonZeroI8::<-100, 100>::new::<5>();
    /// let c = a.saturating_add(b);
    /// let d = RangedNonZeroI8::<-100, 100>::new::<-75>();
    ///
    /// assert_eq!(c.saturating_add(a).get(), 100);
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.saturating_add(a).get(), 100);
    /// assert_eq!(d.saturating_add(d).get(), -100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_add(self, other: impl AsRepr<i8>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_i8(self.get().saturating_add(other)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
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
    /// assert_eq!(b.checked_mul(b).unwrap().get(), 25);
    /// assert_eq!(a.checked_mul(c).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(c.checked_mul(c).unwrap_err(), Error::PosOverflow);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_mul(self, other: impl AsRepr<i8>) -> Result<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_mul(other) else {
            return Err(if self.is_negative() ^ other.is_negative() {
                Error::NegOverflow
            } else {
                Error::PosOverflow
            });
        };

        Self::with_i8(value)
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroI8};
    /// let a = RangedNonZeroI8::<-100, 100>::new::<50>();
    /// let b = RangedNonZeroI8::<-100, 100>::new::<5>();
    /// let c = RangedNonZeroI8::<-100, 100>::new::<-75>();
    ///
    /// assert_eq!(b.saturating_mul(b).get(), 25);
    /// assert_eq!(a.saturating_mul(c).get(), -100);
    /// assert_eq!(c.saturating_mul(c).get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_mul(self, other: impl AsRepr<i8>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_i8(self.get().saturating_mul(other)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
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
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_pow(other) else {
            return Err(if self.is_negative() && other % 2 == 1 {
                Error::NegOverflow
            } else {
                Error::PosOverflow
            });
        };

        Self::with_i8(value)
    }

    /// Raise to an integer power.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroI8};
    /// let a = RangedNonZeroI8::<-100, 100>::new::<50>();
    /// let b = RangedNonZeroI8::<-100, 100>::new::<5>();
    /// let c = RangedNonZeroI8::<-100, 100>::new::<-75>();
    /// let d = RangedNonZeroI8::<-100, 100>::new::<2>();
    ///
    /// assert_eq!(a.saturating_pow(2).get(), 100);
    /// assert_eq!(b.saturating_pow(2).get(), 25);
    /// assert_eq!(c.saturating_pow(3).get(), -100);
    /// assert_eq!(d.saturating_pow(3).get(), 8);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_pow(self, other: impl AsRepr<u32>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_i8(self.get().saturating_pow(other)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
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
    ///     Ok(Quotient::Number(RangedNonZeroI8::new::<-25>())),
    /// );
    /// assert_eq!(a.checked_div(0), Ok(Quotient::Nan));
    /// assert_eq!(a.checked_div(-1), Err(Error::PosOverflow));
    /// assert_eq!(b.checked_div(-2), Err(Error::NegOverflow));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_div(
        self,
        rhs: impl AsRepr<i8>,
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

        match Self::with_i8(value) {
            Ok(v) => Ok(Quotient::Number(v)),
            Err(e) => Err(e),
        }
    }

    /// Saturating integer division.
    ///
    /// Returns [`Self::MIN`] on negative overflow, [`Self::MAX`] on positive
    /// overflow, and [`Quotient::Nan`] if `rhs` is 0.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroI8, Quotient};
    /// let a = RangedNonZeroI8::<-100, 10>::new::<-50>();
    /// let b = RangedNonZeroI8::<-10, 100>::new::<50>();
    ///
    /// assert_eq!(
    ///     a.saturating_div(2),
    ///     Quotient::Number(RangedNonZeroI8::new::<-25>()),
    /// );
    /// assert_eq!(a.saturating_div(0), Quotient::Nan);
    /// assert_eq!(
    ///     a.saturating_div(-1),
    ///     Quotient::Number(RangedNonZeroI8::new::<10>()),
    /// );
    /// assert_eq!(
    ///     b.saturating_div(-2),
    ///     Quotient::Number(RangedNonZeroI8::new::<-10>()),
    /// );
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_div(self, rhs: impl AsRepr<i8>) -> Quotient<Self> {
        let rhs = as_repr::as_repr(rhs);

        if rhs == 0 {
            return Quotient::Nan;
        }

        Quotient::Number(match Self::with_i8(self.get().saturating_div(rhs)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        })
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns an [`Error`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroI8};
    /// let a = RangedNonZeroI8::<1, 100>::new::<50>();
    /// let b = a.checked_sub(5).unwrap();
    ///
    /// assert_eq!(a.checked_sub(-51), Err(Error::PosOverflow));
    /// assert_eq!(b.get(), 45);
    /// assert_eq!(a.checked_sub(a), Err(Error::NegOverflow));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_sub(self, other: impl AsRepr<i8>) -> Result<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_sub(other) else {
            return Err(if other.is_negative() {
                Error::PosOverflow
            } else {
                Error::NegOverflow
            });
        };

        Self::with_i8(value)
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedNonZeroI8};
    /// let a = RangedNonZeroI8::<1, 100>::new::<50>();
    /// let b = a.saturating_sub(5);
    ///
    /// assert_eq!(a.saturating_sub(-51).get(), 100);
    /// assert_eq!(b.get(), 45);
    /// assert_eq!(a.saturating_sub(a).get(), 1);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_sub(self, other: impl AsRepr<i8>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_i8(self.get().saturating_sub(other)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
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

    /// Calculate the midpoint (average) between `self` and `rhs`.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<-8, 8>::new::<0>();
    /// let b = RangedNonZeroI8::<-8, 8>::new::<2>();
    /// let c = RangedNonZeroI8::<-8, 8>::new::<4>();
    /// let d = RangedNonZeroI8::<-8, 8>::new::<-1>();
    /// let e = RangedNonZeroI8::<-8, 8>::new::<-7>();
    /// let f = RangedNonZeroI8::<-8, 8>::new::<-3>();
    /// let g = RangedNonZeroI8::<-8, 8>::new::<3>();
    /// let h = RangedNonZeroI8::<-8, 8>::new::<7>();
    ///
    /// assert_eq!(a.midpoint(c), b);
    /// assert_eq!(d.midpoint(b), a);
    /// assert_eq!(e.midpoint(a), f);
    /// assert_eq!(a.midpoint(e), f);
    /// assert_eq!(a.midpoint(h), g);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn midpoint(self, rhs: Self) -> Self {
        let Ok(value) = Self::with_i8(midpoint(self.get(), rhs.get())) else {
            panic!("unexpected midpoint value")
        };

        value
    }

    /// Add two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<1, 3>::new::<1>();
    /// let b = RangedNonZeroI8::<-1, 3>::new::<2>();
    /// let output: RangedNonZeroI8::<0, 6> = a.ranged_add(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<1, 3>::new::<1>();
    /// let b = RangedNonZeroI8::<-1, 3>::new::<2>();
    /// let output: RangedNonZeroI8::<1, 6> = a.ranged_add(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn ranged_add<
        const RHS_MIN: i8,
        const RHS_MAX: i8,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedNonZeroI8<RHS_MIN, RHS_MAX>,
    ) -> RangedNonZeroI8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN + RHS_MIN != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX + RHS_MAX != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedNonZeroI8(self.get() + rhs.get())
    }

    /// Subtract a number from `self`.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<2, 5>::new::<3>();
    /// let b = RangedNonZeroI8::<-1, 3>::new::<1>();
    /// let output: RangedNonZeroI8::<-1, 6> = a.ranged_sub(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<2, 5>::new::<3>();
    /// let b = RangedNonZeroI8::<-1, 3>::new::<1>();
    /// let output: RangedNonZeroI8::<0, 6> = a.ranged_sub(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn ranged_sub<
        const RHS_MIN: i8,
        const RHS_MAX: i8,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedNonZeroI8<RHS_MIN, RHS_MAX>,
    ) -> RangedNonZeroI8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN - RHS_MAX != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX - RHS_MIN != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedNonZeroI8(self.get() - rhs.get())
    }

    /// Multiply two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<-2, 3>::new::<1>();
    /// let b = RangedNonZeroI8::<0, 3>::new::<2>();
    /// let output: RangedNonZeroI8::<-6, 9> = a.ranged_mul(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<-2, 3>::new::<1>();
    /// let b = RangedNonZeroI8::<0, 3>::new::<2>();
    /// let output: RangedNonZeroI8::<0, 9> = a.ranged_mul(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn ranged_mul<
        const RHS_MIN: i8,
        const RHS_MAX: i8,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedNonZeroI8<RHS_MIN, RHS_MAX>,
    ) -> RangedNonZeroI8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            let (min_min, min_max) = (MIN * RHS_MIN, MIN * RHS_MAX);
            let min = if min_min < min_max { min_min } else { min_max };
            let (max_min, max_max) = (MAX * RHS_MIN, MAX * RHS_MAX);
            let max = if max_min > max_max { max_min } else { max_max };

            if min != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if max != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedNonZeroI8(self.get() * rhs.get())
    }

    /// Divide `self` by a number.
    ///
    /// ```rust
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<2, 5>::new::<3>();
    /// let b = RangedNonZeroI8::<1, 2>::new::<2>();
    /// let output: RangedNonZeroI8::<1, 2> = a.ranged_div(b);
    ///
    /// assert_eq!(output.get(), 1);
    /// ```
    ///
    /// Does not compile:
    //
    /// ```compile_fail
    /// # use ranch::RangedNonZeroI8;
    /// let a = RangedNonZeroI8::<2, 5>::new::<3>();
    /// let b = RangedNonZeroI8::<1, 2>::new::<1>();
    /// let output: RangedNonZeroI8::<0, 2> = a.ranged_div(b);
    ///
    /// assert_eq!(output.get(), 1);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn ranged_div<
        const RHS_MIN: i8,
        const RHS_MAX: i8,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedNonZeroI8<RHS_MIN, RHS_MAX>,
    ) -> RangedNonZeroI8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if RHS_MIN == 0 || RHS_MAX == 0 {
                panic!("Division by zero not allowed");
            }

            let (min_min, min_max) = (MIN / RHS_MIN, MIN / RHS_MAX);
            let (max_min, max_max) = (MAX / RHS_MIN, MAX / RHS_MAX);
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

        RangedNonZeroI8(self.get() / rhs.get())
    }

    /// Raise to an integer power.
    ///
    /// ```rust
    /// # use ranch::{RangedNonZeroI8, RangedU32};
    /// let a = RangedNonZeroI8::<-1, 3>::new::<2>();
    /// let b = RangedU32::<2, 3>::new::<2>();
    /// let output: RangedNonZeroI8::<-1, 27> = a.ranged_pow(b);
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
    /// let output: RangedNonZeroI8::<0, 27> = a.ranged_pow(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn ranged_pow<
        const RHS_MIN: u32,
        const RHS_MAX: u32,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedU32<RHS_MIN, RHS_MAX>,
    ) -> RangedNonZeroI8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN.is_negative() {
                let min = MIN.pow(RHS_MIN);
                let max = MAX.pow(RHS_MAX);
                let rhs_max = if RHS_MAX % 2 == 0 {
                    RHS_MAX - 1
                } else {
                    RHS_MAX
                };
                let rhs_min = if RHS_MIN % 2 == 0 {
                    RHS_MIN - 1
                } else {
                    RHS_MIN
                };
                let min_min = MIN.pow(rhs_min);
                let min_max = MAX.pow(rhs_max);
                let min = if min_min < min { min_min } else { min };
                let min = if min_max < min { min_max } else { min };

                if min != OUTPUT_MIN {
                    panic!("Min mismatch");
                } else if max != OUTPUT_MAX {
                    panic!("Max mismatch");
                }
            } else if MIN.pow(RHS_MIN) != OUTPUT_MIN {
                panic!("Min mismatch");
            } else if MAX.pow(RHS_MAX) != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedNonZeroI8(self.get().pow(rhs.get()))
    }*/
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

// polyfill for midpoint (Added in Rust 1.87.0, MSRV is Rust 1.85.0)
const fn midpoint(a: i8, b: i8) -> i8 {
    let t = ((a ^ b) >> 1) + (a & b);
    t + (if t < 0 { 1 } else { 0 } & (a ^ b))
}
