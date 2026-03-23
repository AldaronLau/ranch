use as_repr::AsRepr;

use crate::*;

impl<const MIN: i16, const MAX: i16> RangedI16<MIN, MAX> {
    /// Create a new ranged integer.
    ///
    /// Won't compile if out of bounds.
    ///
    /// Compiles:
    ///
    /// ```rust
    /// # use ranch::RangedI16;
    /// RangedI16::<1, 3>::new::<1>();
    /// RangedI16::<1, 3>::new::<2>();
    /// RangedI16::<1, 3>::new::<3>();
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail,E0080
    /// # use ranch::RangedI16;
    /// RangedI16::<1, 3>::new::<0>();
    /// ```
    ///
    /// ```compile_fail,E0080
    /// # use ranch::RangedI16;
    /// RangedI16::<1, 3>::new::<4>();
    /// ```
    #[must_use]
    pub const fn new<const N: i16>() -> Self {
        const {
            Self::assert_range();

            if N < MIN || N > MAX {
                panic!("Out of bounds");
            }
        }

        Self::from_unchecked(N)
    }

    /// Try to create a new ranged integer.
    ///
    /// Returns `Err` if out of bounds.
    ///
    /// ```rust
    /// # use ranch::{RangedI16, Error};
    /// RangedI16::<1, 2>::with_i16(1).unwrap();
    /// RangedI16::<1, 2>::with_i16(2).unwrap();
    /// assert_eq!(RangedI16::<1, 2>::with_i16(0).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(RangedI16::<1, 2>::with_i16(3).unwrap_err(), Error::PosOverflow);
    /// ```
    pub const fn with_i16(value: impl AsRepr<i16>) -> Result<Self> {
        const { Self::assert_range() };

        let value = as_repr::as_repr(value);

        if value < MIN {
            return Err(Error::NegOverflow);
        }

        if value > MAX {
            return Err(Error::PosOverflow);
        }

        Ok(Self::from_unchecked(value))
    }

    /// Return the contained value as a primitive type.
    ///
    /// ```rust
    /// # use ranch::RangedI16;
    /// assert_eq!(42, RangedI16::<1, 100>::new::<42>().get());
    /// ```
    #[must_use]
    pub const fn get(self) -> i16 {
        self.0
    }

    /// Return the number of leading zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedI16;
    /// let n = RangedI16::<{ i16::MIN }, { i16::MAX }>::MAX;
    ///
    /// assert_eq!(n.leading_zeros().get(), 1);
    /// ```
    #[must_use]
    pub const fn leading_zeros(self) -> RangedU32<0, { i16::BITS }> {
        RangedU32::from_unchecked(self.get().leading_zeros())
    }

    /// Return the number of trailing zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedI16;
    /// let n = RangedI16::<-128, 127>::new::<0b0101000>();
    ///
    /// assert_eq!(n.trailing_zeros().get(), 3);
    /// ```
    #[must_use]
    pub const fn trailing_zeros(self) -> RangedU32<0, { i16::BITS }> {
        RangedU32::from_unchecked(self.get().trailing_zeros())
    }

    /// Return the number of ones in the binary representation of `self`.
    ///
    /// ```rust
    /// # use ranch::RangedI16;
    /// let a = RangedI16::<-128, 127>::new::<0b100_0000>();
    /// let b = RangedI16::<-128, 127>::new::<0b100_0011>();
    ///
    /// assert_eq!(a.count_ones().get(), 1);
    /// assert_eq!(b.count_ones().get(), 3);
    /// ```
    #[must_use]
    pub const fn count_ones(self) -> RangedU32<0, { i16::BITS }> {
        RangedU32::from_unchecked(self.get().count_ones())
    }

    /// Add two ranged integers together.
    ///
    /// Returns an [`Error`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedI16;
    /// let a = RangedI16::<1, 100>::new::<50>();
    /// let b = RangedI16::<1, 100>::new::<5>();
    /// let c = a.checked_add(b).unwrap();
    ///
    /// assert!(c.checked_add(a).is_err());
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.checked_add(a).unwrap().get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_add(self, other: impl AsRepr<i16>) -> Result<Self> {
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

        Self::with_i16(value)
    }

    /// Add two ranged integers together.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    ///
    /// ```rust
    /// # use ranch::RangedI16;
    /// let a = RangedI16::<-100, 100>::new::<50>();
    /// let b = RangedI16::<-100, 100>::new::<5>();
    /// let c = a.saturating_add(b);
    /// let d = RangedI16::<-100, 100>::new::<-75>();
    ///
    /// assert_eq!(c.saturating_add(a).get(), 100);
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.saturating_add(a).get(), 100);
    /// assert_eq!(d.saturating_add(d).get(), -100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_add(self, other: impl AsRepr<i16>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_i16(self.get().saturating_add(other)) {
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
    /// # use ranch::{Error, RangedI16};
    /// let a = RangedI16::<-100, 100>::new::<50>();
    /// let b = RangedI16::<-100, 100>::new::<5>();
    /// let c = RangedI16::<-100, 100>::new::<-75>();
    ///
    /// assert_eq!(b.checked_mul(b).unwrap().get(), 25);
    /// assert_eq!(a.checked_mul(c).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(c.checked_mul(c).unwrap_err(), Error::PosOverflow);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_mul(self, other: impl AsRepr<i16>) -> Result<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_mul(other) else {
            return Err(if self.is_negative() ^ other.is_negative() {
                Error::NegOverflow
            } else {
                Error::PosOverflow
            });
        };

        Self::with_i16(value)
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedI16};
    /// let a = RangedI16::<-100, 100>::new::<50>();
    /// let b = RangedI16::<-100, 100>::new::<5>();
    /// let c = RangedI16::<-100, 100>::new::<-75>();
    ///
    /// assert_eq!(b.saturating_mul(b).get(), 25);
    /// assert_eq!(a.saturating_mul(c).get(), -100);
    /// assert_eq!(c.saturating_mul(c).get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_mul(self, other: impl AsRepr<i16>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_i16(self.get().saturating_mul(other)) {
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
    /// # use ranch::{Error, RangedI16};
    /// let a = RangedI16::<-100, 100>::new::<50>();
    /// let b = RangedI16::<-100, 100>::new::<5>();
    /// let c = RangedI16::<-100, 100>::new::<-75>();
    /// let d = RangedI16::<-100, 100>::new::<2>();
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

        Self::with_i16(value)
    }

    /// Raise to an integer power.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedI16};
    /// let a = RangedI16::<-100, 100>::new::<50>();
    /// let b = RangedI16::<-100, 100>::new::<5>();
    /// let c = RangedI16::<-100, 100>::new::<-75>();
    /// let d = RangedI16::<-100, 100>::new::<2>();
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

        match Self::with_i16(self.get().saturating_pow(other)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        }
    }

    /// Saturating integer division.
    ///
    /// Returns [`Self::MIN`] on negative overflow, [`Self::MAX`] on positive
    /// overflow, and [`Quotient::Nan`] if `rhs` is 0.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedI16, Quotient};
    /// let a = RangedI16::<-100, 10>::new::<-50>();
    /// let b = RangedI16::<-10, 100>::new::<50>();
    ///
    /// assert_eq!(
    ///     a.saturating_div(2),
    ///     Quotient::Number(RangedI16::new::<-25>()),
    /// );
    /// assert_eq!(a.saturating_div(0), Quotient::Nan);
    /// assert_eq!(
    ///     a.saturating_div(-1),
    ///     Quotient::Number(RangedI16::new::<10>()),
    /// );
    /// assert_eq!(
    ///     b.saturating_div(-2),
    ///     Quotient::Number(RangedI16::new::<-10>()),
    /// );
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_div(self, rhs: impl AsRepr<i16>) -> Quotient<Self> {
        let rhs = as_repr::as_repr(rhs);

        if rhs == 0 {
            return Quotient::Nan;
        }

        Quotient::Number(match Self::with_i16(self.get().saturating_div(rhs)) {
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
    /// # use ranch::{Error, RangedI16};
    /// let a = RangedI16::<1, 100>::new::<50>();
    /// let b = a.checked_sub(5).unwrap();
    ///
    /// assert_eq!(a.checked_sub(-51), Err(Error::PosOverflow));
    /// assert_eq!(b.get(), 45);
    /// assert_eq!(a.checked_sub(a), Err(Error::NegOverflow));
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_sub(self, other: impl AsRepr<i16>) -> Result<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_sub(other) else {
            return Err(if other.is_negative() {
                Error::PosOverflow
            } else {
                Error::NegOverflow
            });
        };

        Self::with_i16(value)
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedI16};
    /// let a = RangedI16::<1, 100>::new::<50>();
    /// let b = a.saturating_sub(5);
    ///
    /// assert_eq!(a.saturating_sub(-51).get(), 100);
    /// assert_eq!(b.get(), 45);
    /// assert_eq!(a.saturating_sub(a).get(), 1);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_sub(self, other: impl AsRepr<i16>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_i16(self.get().saturating_sub(other)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        }
    }

    /// Return `true` if `self` is negative; `false` if zero or positive.
    ///
    /// ```rust
    /// # use ranch::RangedI16;
    /// assert!(!RangedI16::<-100, 100>::new::<10>().is_negative());
    /// assert!(RangedI16::<-100, 100>::new::<-10>().is_negative());
    /// ```
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.get().is_negative()
    }

    /// Return `true` if `self` is positive; `false` if zero or negative.
    ///
    /// ```rust
    /// # use ranch::RangedI16;
    /// assert!(RangedI16::<-100, 100>::new::<10>().is_positive());
    /// assert!(!RangedI16::<-100, 100>::new::<-10>().is_positive());
    /// ```
    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.get().is_positive()
    }

    /// Calculate the midpoint (average) between `self` and `rhs`.
    ///
    /// ```rust
    /// # use ranch::RangedI16;
    /// let a = RangedI16::<-8, 8>::new::<0>();
    /// let b = RangedI16::<-8, 8>::new::<2>();
    /// let c = RangedI16::<-8, 8>::new::<4>();
    /// let d = RangedI16::<-8, 8>::new::<-1>();
    /// let e = RangedI16::<-8, 8>::new::<-7>();
    /// let f = RangedI16::<-8, 8>::new::<-3>();
    /// let g = RangedI16::<-8, 8>::new::<3>();
    /// let h = RangedI16::<-8, 8>::new::<7>();
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
        let Ok(value) = Self::with_i16(midpoint(self.get(), rhs.get())) else {
            panic!("unexpected midpoint value")
        };

        value
    }
}

impl<const MIN: i16, const MAX: i16> core::str::FromStr
    for RangedI16<MIN, MAX>
{
    type Err = ParsingError;

    fn from_str(src: &str) -> ParsingResult<Self> {
        let parsed = src.parse::<i16>()?;

        Self::with_i16(parsed).map_err(From::from)
    }
}

// polyfill for midpoint (Added in Rust 1.87.0, MSRV is Rust 1.85.0)
const fn midpoint(a: i16, b: i16) -> i16 {
    let t = ((a ^ b) >> 1) + (a & b);
    t + (if t < 0 { 1 } else { 0 } & (a ^ b))
}
