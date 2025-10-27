use crate::{
    Error, ParsingError, ParsingResult, RangedU32, Result,
    conversions::{self, AsRepr},
};

/// [`u128`] with a specified minimum and maximum value
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct RangedU128<const MIN: u128, const MAX: u128>(u128);

// unsafe: `repr(transparent)` is `repr(u128)`
#[expect(unsafe_code)]
unsafe impl<const MIN: u128, const MAX: u128> AsRepr<u128>
    for RangedU128<MIN, MAX>
{
}

impl<const MIN: u128, const MAX: u128> RangedU128<MIN, MAX> {
    /// The size of this integer type in bits.
    pub const BITS: u32 = u128::BITS;
    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Self(MAX);
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self(MIN);

    /// Try to create a new ranged integer.
    ///
    /// Returns `None` if out of bounds.
    ///
    /// ```rust
    /// # use ranch::{RangedU128, Error};
    /// RangedU128::<1, 2>::new(1).unwrap();
    /// RangedU128::<1, 2>::new(2).unwrap();
    /// assert_eq!(RangedU128::<1, 2>::new(0).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(RangedU128::<1, 2>::new(3).unwrap_err(), Error::PosOverflow);
    /// ```
    pub const fn new(value: impl AsRepr<u128>) -> Result<Self> {
        let value = conversions::as_repr(value);

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
    /// # use ranch::RangedU128;
    /// RangedU128::<1, 3>::new_const::<1>();
    /// RangedU128::<1, 3>::new_const::<2>();
    /// RangedU128::<1, 3>::new_const::<3>();
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// RangedU128::<1, 3>::new_const::<0>();
    /// ```
    ///
    /// ```compile_fail
    /// RangedU128::<1, 3>::new_const::<4>();
    /// ```
    pub const fn new_const<const N: u128>() -> Self {
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
    /// # use ranch::RangedU128;
    /// assert_eq!(42, RangedU128::<1, 100>::new_const::<42>().get());
    /// ```
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
    /// assert_eq!(n.leading_zeros(), 0);
    /// ```
    pub const fn leading_zeros(self) -> u32 {
        self.get().leading_zeros()
    }

    /// Return the number of trailing zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let n = RangedU128::<0, 255>::new_const::<0b0101000>();
    ///
    /// assert_eq!(n.trailing_zeros(), 3);
    /// ```
    pub const fn trailing_zeros(self) -> u32 {
        self.get().trailing_zeros()
    }

    /// Return the number of ones in the binary representation of `self`.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<0, 255>::new_const::<0b100_0000>();
    /// let b = RangedU128::<0, 255>::new_const::<0b100_0011>();
    ///
    /// assert_eq!(a.count_ones(), 1);
    /// assert_eq!(b.count_ones(), 3);
    /// ```
    pub const fn count_ones(self) -> u32 {
        self.get().count_ones()
    }

    /// Add two ranged integers together.
    ///
    /// Returns [`None`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<1, 100>::new_const::<50>();
    /// let b = RangedU128::<1, 100>::new_const::<5>();
    /// let c = a.checked_add(b).unwrap();
    ///
    /// assert!(c.checked_add(a).is_none());
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.checked_add(a).unwrap().get(), 100);
    /// ```
    pub const fn checked_add(self, other: impl AsRepr<u128>) -> Option<Self> {
        let other = conversions::as_repr(other);
        let Some(value) = self.get().checked_add(other) else {
            return None;
        };

        match Self::new(value) {
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
    /// let a = RangedU128::<1, 100>::new_const::<50>();
    /// let b = RangedU128::<1, 100>::new_const::<5>();
    /// let c = a.saturating_add(b);
    ///
    /// assert_eq!(c.saturating_add(a).get(), 100);
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.saturating_add(a).get(), 100);
    /// ```
    pub const fn saturating_add(self, other: impl AsRepr<u128>) -> Self {
        let other = conversions::as_repr(other);

        match Self::new(self.get().saturating_add(other)) {
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
    /// let a = RangedU128::<0, 100>::new_const::<50>();
    /// let b = RangedU128::<0, 100>::new_const::<5>();
    /// let c = RangedU128::<0, 100>::new_const::<75>();
    ///
    /// assert_eq!(b.checked_mul(b).unwrap().get(), 25);
    /// assert_eq!(a.checked_mul(c), None);
    /// assert_eq!(c.checked_mul(c), None);
    /// ```
    pub const fn checked_mul(self, other: impl AsRepr<u128>) -> Option<Self> {
        let other = conversions::as_repr(other);
        let Some(value) = self.get().checked_mul(other) else {
            return None;
        };

        match Self::new(value) {
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
    /// let a = RangedU128::<0, 100>::new_const::<50>();
    /// let b = RangedU128::<0, 100>::new_const::<5>();
    /// let c = RangedU128::<0, 100>::new_const::<75>();
    ///
    /// assert_eq!(b.saturating_mul(b).get(), 25);
    /// assert_eq!(a.saturating_mul(c).get(), 100);
    /// assert_eq!(c.saturating_mul(c).get(), 100);
    /// ```
    pub const fn saturating_mul(self, other: impl AsRepr<u128>) -> Self {
        let other = conversions::as_repr(other);

        match Self::new(self.get().saturating_mul(other)) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns [`None`] on overflow.
    pub const fn checked_pow(self, other: impl AsRepr<u32>) -> Option<Self> {
        let other = conversions::as_repr(other);
        let Some(value) = self.get().checked_pow(other) else {
            return None;
        };

        match Self::new(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns [`Self::MAX`] on overflow.
    pub const fn saturating_pow(self, other: impl AsRepr<u32>) -> Self {
        let other = conversions::as_repr(other);

        match Self::new(self.get().saturating_pow(other)) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Checked integer division.
    ///
    /// Returns [`None`] on overflow or `rhs == 0`.
    pub const fn checked_div(self, rhs: impl AsRepr<u128>) -> Option<Self> {
        let rhs = conversions::as_repr(rhs);
        let Some(value) = self.get().checked_div(rhs) else {
            return None;
        };

        match Self::new(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Saturating integer division.
    ///
    /// Returns [`Self::MIN`] on overflow.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    pub const fn saturating_div(self, rhs: impl AsRepr<u128>) -> Self {
        let rhs = conversions::as_repr(rhs);

        match Self::new(self.get().saturating_div(rhs)) {
            Ok(value) => value,
            Err(_) => Self::MIN,
        }
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`None`] on overflow.
    pub const fn checked_sub(self, other: impl AsRepr<u128>) -> Option<Self> {
        let other = conversions::as_repr(other);
        let Some(value) = self.get().checked_sub(other) else {
            return None;
        };

        match Self::new(value) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`Self::MIN`] on overflow.
    pub const fn saturating_sub(self, other: impl AsRepr<u128>) -> Self {
        let other = conversions::as_repr(other);

        match Self::new(self.get().saturating_sub(other)) {
            Ok(value) => value,
            Err(_) => Self::MIN,
        }
    }

    /// Return the smallest power of two greater than or equal to self.
    ///
    /// Returns [`None`] on overflow.
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        let Some(value) = self.get().checked_next_power_of_two() else {
            return None;
        };

        Some(match Self::new(value) {
            Ok(value) => value,
            Err(_) => return None,
        })
    }

    /// Returns true if and only if `self == (1 << k)` for some `k`.
    pub const fn is_power_of_two(self) -> bool {
        self.get().is_power_of_two()
    }

    /// Calculate the midpoint (average) between `self` and `rhs`.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<0, 8>::new_const::<0>();
    /// let b = RangedU128::<0, 8>::new_const::<2>();
    /// let c = RangedU128::<0, 8>::new_const::<4>();
    /// let d = RangedU128::<0, 8>::new_const::<3>();
    /// let e = RangedU128::<0, 8>::new_const::<7>();
    ///
    /// assert_eq!(a.midpoint(c), b);
    /// assert_eq!(a.midpoint(e), d);
    /// ```
    pub const fn midpoint(self, rhs: Self) -> Self {
        let Ok(value) = Self::new(self.get().midpoint(rhs.get())) else {
            panic!("unexpected midpoint value")
        };

        value
    }

    /// Add two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<1, 3>::new_const::<1>();
    /// let b = RangedU128::<1, 3>::new_const::<2>();
    /// let output: RangedU128::<2, 6> = a.add(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<1, 3>::new_const::<1>();
    /// let b = RangedU128::<1, 3>::new_const::<2>();
    /// let output: RangedU128::<1, 6> = a.add(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    pub const fn add<
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
    /// let a = RangedU128::<2, 5>::new_const::<3>();
    /// let b = RangedU128::<1, 2>::new_const::<1>();
    /// let output: RangedU128::<0, 4> = a.sub(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<2, 5>::new_const::<3>();
    /// let b = RangedU128::<1, 2>::new_const::<1>();
    /// let output: RangedU128::<0, 3> = a.sub(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    pub const fn sub<
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
    /// let a = RangedU128::<1, 3>::new_const::<1>();
    /// let b = RangedU128::<2, 3>::new_const::<2>();
    /// let output: RangedU128::<2, 9> = a.mul(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<1, 3>::new_const::<1>();
    /// let b = RangedU128::<2, 3>::new_const::<2>();
    /// let output: RangedU128::<1, 9> = a.mul(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    pub const fn mul<
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
    /// let a = RangedU128::<2, 5>::new_const::<3>();
    /// let b = RangedU128::<1, 2>::new_const::<2>();
    /// let output: RangedU128::<1, 2> = a.div(b);
    ///
    /// assert_eq!(output.get(), 1);
    /// ```
    ///
    /// Does not compile:
    //
    /// ```compile_fail
    /// # use ranch::RangedU128;
    /// let a = RangedU128::<2, 5>::new_const::<3>();
    /// let b = RangedU128::<1, 2>::new_const::<1>();
    /// let output: RangedU128::<0, 2> = a.div(b);
    ///
    /// assert_eq!(output.get(), 1);
    /// ```
    pub const fn div<
        const RHS_MIN: u128,
        const RHS_MAX: u128,
        const OUTPUT_MIN: u128,
        const OUTPUT_MAX: u128,
    >(
        self,
        rhs: RangedU128<RHS_MIN, RHS_MAX>,
    ) -> RangedU128<OUTPUT_MIN, OUTPUT_MAX> {
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

        RangedU128(self.get() / rhs.get())
    }

    /// Raise to an integer power.
    ///
    /// ```rust
    /// # use ranch::{RangedU128, RangedU32};
    /// let a = RangedU128::<1, 3>::new_const::<2>();
    /// let b = RangedU32::<2, 3>::new_const::<2>();
    /// let output: RangedU128::<1, 27> = a.pow(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::{RangedU128, RangedU32};
    /// let a = RangedU128::<1, 3>::new_const::<2>();
    /// let b = RangedU32::<2, 3>::new_const::<2>();
    /// let output: RangedU128::<0, 27> = a.pow(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    pub const fn pow<
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

        Self::new(parsed).map_err(From::from)
    }
}

impl<const MIN: u128, const MAX: u128> crate::error::Saturate
    for RangedU128<MIN, MAX>
{
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
}
