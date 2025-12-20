use as_repr::AsRepr;

use crate::{Error, ParsingError, ParsingResult, Quotient, RangedU32, Result};

/// [`u8`] with a specified minimum and maximum value
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct RangedU8<const MIN: u8, const MAX: u8>(pub(crate) u8);

// unsafe: `repr(transparent)` is `repr(u8)`
#[expect(unsafe_code)]
unsafe impl<const MIN: u8, const MAX: u8> AsRepr<u8> for RangedU8<MIN, MAX> {}

impl<const MIN: u8, const MAX: u8> RangedU8<MIN, MAX> {
    /// The size of this integer type in bits.
    pub const BITS: u32 = u8::BITS;
    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Self(MAX);
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self(MIN);

    /// Try to create a new ranged integer.
    ///
    /// Returns `None` if out of bounds.
    ///
    /// ```rust
    /// # use ranch::{RangedU8, Error};
    /// RangedU8::<1, 2>::with_u8(1).unwrap();
    /// RangedU8::<1, 2>::with_u8(2).unwrap();
    /// assert_eq!(RangedU8::<1, 2>::with_u8(0).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(RangedU8::<1, 2>::with_u8(3).unwrap_err(), Error::PosOverflow);
    /// ```
    pub const fn with_u8(value: impl AsRepr<u8>) -> Result<Self> {
        let value = as_repr::as_repr(value);

        if value < MIN {
            return Err(Error::NegOverflow);
        }

        if value > MAX {
            return Err(Error::PosOverflow);
        }

        Ok(Self(value))
    }

    /// Create a new ranged integer.
    ///
    /// Won't compile if out of bounds.
    ///
    /// Compiles:
    ///
    /// ```rust
    /// # use ranch::RangedU8;
    /// RangedU8::<1, 3>::new_const::<1>();
    /// RangedU8::<1, 3>::new_const::<2>();
    /// RangedU8::<1, 3>::new_const::<3>();
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// RangedU8::<1, 3>::new_const::<0>();
    /// ```
    ///
    /// ```compile_fail
    /// RangedU8::<1, 3>::new_const::<4>();
    /// ```
    #[must_use]
    pub const fn new_const<const N: u8>() -> Self {
        const {
            if N < MIN || N > MAX {
                panic!("Out of bounds");
            }
        }

        Self(N)
    }

    /// Return the contained value as a primitive type.
    ///
    /// ```rust
    /// # use ranch::RangedU8;
    /// assert_eq!(42, RangedU8::<1, 100>::new_const::<42>().get());
    /// ```
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }

    /// Return the number of leading zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedU8;
    /// let n = RangedU8::<{ u8::MIN }, { u8::MAX }>::MAX;
    ///
    /// assert_eq!(n.leading_zeros(), 0);
    /// ```
    #[must_use]
    pub const fn leading_zeros(self) -> u32 {
        self.get().leading_zeros()
    }

    /// Return the number of trailing zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedU8;
    /// let n = RangedU8::<0, 255>::new_const::<0b0101000>();
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
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<0, 255>::new_const::<0b100_0000>();
    /// let b = RangedU8::<0, 255>::new_const::<0b100_0011>();
    ///
    /// assert_eq!(a.count_ones(), 1);
    /// assert_eq!(b.count_ones(), 3);
    /// ```
    #[must_use]
    pub const fn count_ones(self) -> u32 {
        self.get().count_ones()
    }

    /// Add two ranged integers together.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<1, 100>::new_const::<50>();
    /// let b = RangedU8::<1, 100>::new_const::<5>();
    /// let c = a.checked_add(b).unwrap();
    ///
    /// assert!(c.checked_add(a).is_none());
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.checked_add(a).unwrap().get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_add(self, other: impl AsRepr<u8>) -> Option<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_add(other) else {
            return None;
        };

        match Self::with_u8(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Add two ranged integers together.
    ///
    /// Returns [`Self::MAX`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<1, 100>::new_const::<50>();
    /// let b = RangedU8::<1, 100>::new_const::<5>();
    /// let c = a.saturating_add(b);
    ///
    /// assert_eq!(c.saturating_add(a).get(), 100);
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.saturating_add(a).get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_add(self, other: impl AsRepr<u8>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_u8(self.get().saturating_add(other)) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU8};
    /// let a = RangedU8::<0, 100>::new_const::<50>();
    /// let b = RangedU8::<0, 100>::new_const::<5>();
    /// let c = RangedU8::<0, 100>::new_const::<75>();
    ///
    /// assert_eq!(b.checked_mul(b).unwrap().get(), 25);
    /// assert_eq!(a.checked_mul(c), None);
    /// assert_eq!(c.checked_mul(c), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_mul(self, other: impl AsRepr<u8>) -> Option<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_mul(other) else {
            return None;
        };

        match Self::with_u8(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns [`Self::MAX`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU8};
    /// let a = RangedU8::<0, 100>::new_const::<50>();
    /// let b = RangedU8::<0, 100>::new_const::<5>();
    /// let c = RangedU8::<0, 100>::new_const::<75>();
    ///
    /// assert_eq!(b.saturating_mul(b).get(), 25);
    /// assert_eq!(a.saturating_mul(c).get(), 100);
    /// assert_eq!(c.saturating_mul(c).get(), 100);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_mul(self, other: impl AsRepr<u8>) -> Self {
        let other = as_repr::as_repr(other);
        match Self::with_u8(self.get().saturating_mul(other)) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU8};
    /// let a = RangedU8::<0, 100>::new_const::<50>();
    /// let b = RangedU8::<0, 100>::new_const::<5>();
    /// let c = RangedU8::<0, 100>::new_const::<2>();
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

        match Self::with_u8(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns [`Self::MAX`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU8};
    /// let a = RangedU8::<0, 100>::new_const::<50>();
    /// let b = RangedU8::<0, 100>::new_const::<5>();
    /// let c = RangedU8::<0, 100>::new_const::<2>();
    ///
    /// assert_eq!(a.saturating_pow(2).get(), 100);
    /// assert_eq!(b.saturating_pow(2).get(), 25);
    /// assert_eq!(c.saturating_pow(3).get(), 8);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_pow(self, other: impl AsRepr<u32>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_u8(self.get().saturating_pow(other)) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Checked integer division.
    ///
    /// Returns [`None`] on overflow; [`Quotient::Nan`] if `rhs == 0`.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU8, Quotient};
    /// let a = RangedU8::<1, 50>::new_const::<50>();
    /// let b = RangedU8::<1, 50>::new_const::<1>();
    ///
    /// assert_eq!(
    ///     a.checked_div(2),
    ///     Some(Quotient::Number(RangedU8::new_const::<25>())),
    /// );
    /// assert_eq!(a.checked_div(0), Some(Quotient::Nan));
    /// assert_eq!(b.checked_div(2), None);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_div(
        self,
        rhs: impl AsRepr<u8>,
    ) -> Option<Quotient<Self>> {
        let rhs = as_repr::as_repr(rhs);
        let Some(value) = self.get().checked_div(rhs) else {
            return Some(Quotient::Nan);
        };

        match Self::with_u8(value) {
            Ok(value) => Some(Quotient::Number(value)),
            Err(_) => None,
        }
    }

    /// Saturating integer division.
    ///
    /// Returns [`Self::MIN`] on overflow, and [`Quotient::Nan`] if `rhs` is 0.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU8, Quotient};
    /// let a = RangedU8::<1, 50>::new_const::<50>();
    /// let b = RangedU8::<1, 50>::new_const::<1>();
    ///
    /// assert_eq!(
    ///     a.saturating_div(2),
    ///     Quotient::Number(RangedU8::new_const::<25>()),
    /// );
    /// assert_eq!(a.saturating_div(0), Quotient::Nan);
    /// assert_eq!(
    ///     b.saturating_div(2),
    ///     Quotient::Number(RangedU8::new_const::<1>()),
    /// );
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_div(self, rhs: impl AsRepr<u8>) -> Quotient<Self> {
        let rhs = as_repr::as_repr(rhs);

        if rhs == 0 {
            return Quotient::Nan;
        }

        Quotient::Number(match Self::with_u8(self.get().saturating_div(rhs)) {
            Ok(value) => value,
            Err(_) => Self::MIN,
        })
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<1, 100>::new_const::<50>();
    /// let b = a.checked_sub(5).unwrap();
    ///
    /// assert_eq!(b.get(), 45);
    /// assert!(a.checked_sub(a).is_none());
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn checked_sub(self, other: impl AsRepr<u8>) -> Option<Self> {
        let other = as_repr::as_repr(other);
        let Some(value) = self.get().checked_sub(other) else {
            return None;
        };

        match Self::with_u8(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`Self::MIN`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU8};
    /// let a = RangedU8::<1, 100>::new_const::<50>();
    /// let b = a.saturating_sub(5);
    ///
    /// assert_eq!(b.get(), 45);
    /// assert_eq!(a.saturating_sub(a).get(), 1);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn saturating_sub(self, other: impl AsRepr<u8>) -> Self {
        let other = as_repr::as_repr(other);

        match Self::with_u8(self.get().saturating_sub(other)) {
            Ok(value) => value,
            Err(_) => Self::MIN,
        }
    }

    /// Return the smallest power of two greater than or equal to self.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU8};
    /// let a = RangedU8::<0, 33>::new_const::<0>();
    /// let b = RangedU8::<0, 33>::new_const::<9>();
    /// let c = RangedU8::<0, 33>::new_const::<32>();
    /// let d = RangedU8::<0, 33>::new_const::<33>();
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

        Some(match Self::with_u8(value) {
            Ok(value) => value,
            Err(_) => return None,
        })
    }

    /// Returns true if and only if `self == (1 << k)` for some `k`.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedU8};
    /// let a = RangedU8::<0, 32>::new_const::<0>();
    /// let b = RangedU8::<0, 32>::new_const::<9>();
    /// let c = RangedU8::<0, 32>::new_const::<32>();
    /// let d = RangedU8::<0, 32>::new_const::<1>();
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
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<0, 8>::new_const::<0>();
    /// let b = RangedU8::<0, 8>::new_const::<2>();
    /// let c = RangedU8::<0, 8>::new_const::<4>();
    /// let d = RangedU8::<0, 8>::new_const::<3>();
    /// let e = RangedU8::<0, 8>::new_const::<7>();
    ///
    /// assert_eq!(a.midpoint(c), b);
    /// assert_eq!(a.midpoint(e), d);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn midpoint(self, rhs: Self) -> Self {
        let Ok(value) = Self::with_u8(self.get().midpoint(rhs.get())) else {
            panic!("unexpected midpoint value")
        };

        value
    }

    /// Add two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<1, 3>::new_const::<1>();
    /// let b = RangedU8::<1, 3>::new_const::<2>();
    /// let output: RangedU8::<2, 6> = a.add(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<1, 3>::new_const::<1>();
    /// let b = RangedU8::<1, 3>::new_const::<2>();
    /// let output: RangedU8::<1, 6> = a.add(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn add<
        const RHS_MIN: u8,
        const RHS_MAX: u8,
        const OUTPUT_MIN: u8,
        const OUTPUT_MAX: u8,
    >(
        self,
        rhs: RangedU8<RHS_MIN, RHS_MAX>,
    ) -> RangedU8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN + RHS_MIN != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX + RHS_MAX != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedU8(self.get() + rhs.get())
    }

    /// Subtract a number from `self`.
    ///
    /// ```rust
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<2, 5>::new_const::<3>();
    /// let b = RangedU8::<1, 2>::new_const::<1>();
    /// let output: RangedU8::<0, 4> = a.sub(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<2, 5>::new_const::<3>();
    /// let b = RangedU8::<1, 2>::new_const::<1>();
    /// let output: RangedU8::<0, 3> = a.sub(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn sub<
        const RHS_MIN: u8,
        const RHS_MAX: u8,
        const OUTPUT_MIN: u8,
        const OUTPUT_MAX: u8,
    >(
        self,
        rhs: RangedU8<RHS_MIN, RHS_MAX>,
    ) -> RangedU8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN - RHS_MAX != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX - RHS_MIN != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedU8(self.get() - rhs.get())
    }

    /// Multiply two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<1, 3>::new_const::<1>();
    /// let b = RangedU8::<2, 3>::new_const::<2>();
    /// let output: RangedU8::<2, 9> = a.mul(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<1, 3>::new_const::<1>();
    /// let b = RangedU8::<2, 3>::new_const::<2>();
    /// let output: RangedU8::<1, 9> = a.mul(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn mul<
        const RHS_MIN: u8,
        const RHS_MAX: u8,
        const OUTPUT_MIN: u8,
        const OUTPUT_MAX: u8,
    >(
        self,
        rhs: RangedU8<RHS_MIN, RHS_MAX>,
    ) -> RangedU8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN * RHS_MIN != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX * RHS_MAX != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedU8(self.get() * rhs.get())
    }

    /// Divide `self` by a number.
    ///
    /// ```rust
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<2, 5>::new_const::<3>();
    /// let b = RangedU8::<1, 2>::new_const::<2>();
    /// let output: RangedU8::<1, 2> = a.div(b);
    ///
    /// assert_eq!(output.get(), 1);
    /// ```
    ///
    /// Does not compile:
    //
    /// ```compile_fail
    /// # use ranch::RangedU8;
    /// let a = RangedU8::<2, 5>::new_const::<3>();
    /// let b = RangedU8::<1, 2>::new_const::<1>();
    /// let output: RangedU8::<0, 2> = a.div(b);
    ///
    /// assert_eq!(output.get(), 1);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn div<
        const RHS_MIN: u8,
        const RHS_MAX: u8,
        const OUTPUT_MIN: u8,
        const OUTPUT_MAX: u8,
    >(
        self,
        rhs: RangedU8<RHS_MIN, RHS_MAX>,
    ) -> RangedU8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if RHS_MIN == 0 {
                panic!("Division by zero not allowed");
            }

            if MIN / RHS_MAX != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX / RHS_MAX != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedU8(self.get() / rhs.get())
    }

    /// Raise to an integer power.
    ///
    /// ```rust
    /// # use ranch::{RangedU8, RangedU32};
    /// let a = RangedU8::<1, 3>::new_const::<2>();
    /// let b = RangedU32::<2, 3>::new_const::<2>();
    /// let output: RangedU8::<1, 27> = a.pow(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::{RangedU8, RangedU32};
    /// let a = RangedU8::<1, 3>::new_const::<2>();
    /// let b = RangedU32::<2, 3>::new_const::<2>();
    /// let output: RangedU8::<0, 27> = a.pow(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn pow<
        const RHS_MIN: u32,
        const RHS_MAX: u32,
        const OUTPUT_MIN: u8,
        const OUTPUT_MAX: u8,
    >(
        self,
        rhs: RangedU32<RHS_MIN, RHS_MAX>,
    ) -> RangedU8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN.pow(RHS_MIN) != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX.pow(RHS_MAX) != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedU8(self.get().pow(rhs.get()))
    }
}

impl<const MIN: u8, const MAX: u8> core::str::FromStr for RangedU8<MIN, MAX> {
    type Err = ParsingError;

    fn from_str(src: &str) -> ParsingResult<Self> {
        let parsed = src.parse::<u8>()?;

        Self::with_u8(parsed).map_err(From::from)
    }
}

impl<const MIN: u8, const MAX: u8> crate::error::Saturate
    for RangedU8<MIN, MAX>
{
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
}
