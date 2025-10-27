use crate::{
    Error, ParsingError, ParsingResult, RangedU32, Result,
    conversions::{self, AsRepr},
};

/// [`i8`] with a specified minimum and maximum value
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct RangedI8<const MIN: i8, const MAX: i8>(i8);

// unsafe: `repr(transparent)` is `repr(i8)`
#[expect(unsafe_code)]
unsafe impl<const MIN: i8, const MAX: i8> AsRepr<i8> for RangedI8<MIN, MAX> {}

impl<const MIN: i8, const MAX: i8> RangedI8<MIN, MAX> {
    /// The size of this integer type in bits.
    pub const BITS: u32 = i8::BITS;
    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Self(MAX);
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self(MIN);

    /// Try to create a new ranged integer.
    ///
    /// Returns `None` if out of bounds.
    ///
    /// ```rust
    /// # use ranch::{RangedI8, Error};
    /// RangedI8::<1, 2>::new(1).unwrap();
    /// RangedI8::<1, 2>::new(2).unwrap();
    /// assert_eq!(RangedI8::<1, 2>::new(0).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(RangedI8::<1, 2>::new(3).unwrap_err(), Error::PosOverflow);
    /// ```
    pub const fn new(value: impl AsRepr<i8>) -> Result<Self> {
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
    /// # use ranch::RangedI8;
    /// RangedI8::<1, 3>::new_const::<1>();
    /// RangedI8::<1, 3>::new_const::<2>();
    /// RangedI8::<1, 3>::new_const::<3>();
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// RangedI8::<1, 3>::new_const::<0>();
    /// ```
    ///
    /// ```compile_fail
    /// RangedI8::<1, 3>::new_const::<4>();
    /// ```
    pub const fn new_const<const N: i8>() -> Self {
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
    /// # use ranch::RangedI8;
    /// assert_eq!(42, RangedI8::<1, 100>::new_const::<42>().get());
    /// ```
    pub const fn get(self) -> i8 {
        self.0
    }

    /// Return the number of leading zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// let n = RangedI8::<{ i8::MIN }, { i8::MAX }>::MAX;
    ///
    /// assert_eq!(n.leading_zeros(), 1);
    /// ```
    pub const fn leading_zeros(self) -> u32 {
        self.get().leading_zeros()
    }

    /// Return the number of trailing zeros in the binary representation of
    /// `self`.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// let n = RangedI8::<-128, 127>::new_const::<0b0101000>();
    ///
    /// assert_eq!(n.trailing_zeros(), 3);
    /// ```
    pub const fn trailing_zeros(self) -> u32 {
        self.get().trailing_zeros()
    }

    /// Return the number of ones in the binary representation of `self`.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<-128, 127>::new_const::<0b100_0000>();
    /// let b = RangedI8::<-128, 127>::new_const::<0b100_0011>();
    ///
    /// assert_eq!(a.count_ones(), 1);
    /// assert_eq!(b.count_ones(), 3);
    /// ```
    pub const fn count_ones(self) -> u32 {
        self.get().count_ones()
    }

    /// Add two ranged integers together.
    ///
    /// Returns an [`Error`] on overflow.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<1, 100>::new_const::<50>();
    /// let b = RangedI8::<1, 100>::new_const::<5>();
    /// let c = a.checked_add(b).unwrap();
    ///
    /// assert!(c.checked_add(a).is_err());
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.checked_add(a).unwrap().get(), 100);
    /// ```
    pub const fn checked_add(self, other: impl AsRepr<i8>) -> Result<Self> {
        let other = conversions::as_repr(other);
        let Some(value) = self.get().checked_add(other) else {
            return Err(
                if self.get().saturating_add(other) == Self::MAX.get() {
                    Error::PosOverflow
                } else {
                    Error::NegOverflow
                },
            );
        };

        Self::new(value)
    }

    /// Add two ranged integers together.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<-100, 100>::new_const::<50>();
    /// let b = RangedI8::<-100, 100>::new_const::<5>();
    /// let c = a.saturating_add(b);
    /// let d = RangedI8::<-100, 100>::new_const::<-75>();
    ///
    /// assert_eq!(c.saturating_add(a).get(), 100);
    /// assert_eq!(c.get(), 55);
    /// assert_eq!(a.saturating_add(a).get(), 100);
    /// assert_eq!(d.saturating_add(d).get(), -100);
    /// ```
    pub const fn saturating_add(self, other: impl AsRepr<i8>) -> Self {
        let other = conversions::as_repr(other);

        match Self::new(self.get().saturating_add(other)) {
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
    /// # use ranch::{Error, RangedI8};
    /// let a = RangedI8::<-100, 100>::new_const::<50>();
    /// let b = RangedI8::<-100, 100>::new_const::<5>();
    /// let c = RangedI8::<-100, 100>::new_const::<-75>();
    ///
    /// assert_eq!(b.checked_mul(b).unwrap().get(), 25);
    /// assert_eq!(a.checked_mul(c).unwrap_err(), Error::NegOverflow);
    /// assert_eq!(c.checked_mul(c).unwrap_err(), Error::PosOverflow);
    /// ```
    pub const fn checked_mul(self, other: impl AsRepr<i8>) -> Result<Self> {
        let other = conversions::as_repr(other);
        let Some(value) = self.get().checked_mul(other) else {
            return Err(if self.is_negative() ^ other.is_negative() {
                Error::NegOverflow
            } else {
                Error::PosOverflow
            });
        };

        Self::new(value)
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    ///
    /// ```rust
    /// # use ranch::{Error, RangedI8};
    /// let a = RangedI8::<-100, 100>::new_const::<50>();
    /// let b = RangedI8::<-100, 100>::new_const::<5>();
    /// let c = RangedI8::<-100, 100>::new_const::<-75>();
    ///
    /// assert_eq!(b.saturating_mul(b).get(), 25);
    /// assert_eq!(a.saturating_mul(c).get(), -100);
    /// assert_eq!(c.saturating_mul(c).get(), 100);
    /// ```
    pub const fn saturating_mul(self, other: impl AsRepr<i8>) -> Self {
        let other = conversions::as_repr(other);

        match Self::new(self.get().saturating_mul(other)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns an [`Error`] on overflow.
    pub const fn checked_pow(self, other: impl AsRepr<u32>) -> Result<Self> {
        let other = conversions::as_repr(other);
        let Some(value) = self.get().checked_pow(other) else {
            return Err(if self.is_negative() && other % 2 == 1 {
                Error::NegOverflow
            } else {
                Error::PosOverflow
            });
        };

        Self::new(value)
    }

    /// Raise to an integer power.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    pub const fn saturating_pow(self, other: impl AsRepr<u32>) -> Self {
        let other = conversions::as_repr(other);

        match Self::new(self.get().saturating_pow(other)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        }
    }

    /// Checked integer division.
    ///
    /// Returns an [`Error`] on overflow; [`None`] if `rhs == 0`.
    pub const fn checked_div(
        self,
        rhs: impl AsRepr<i8>,
    ) -> Result<Option<Self>> {
        let rhs = conversions::as_repr(rhs);

        if rhs == 0 {
            return Ok(None);
        }

        let Some(value) = self.get().checked_div(rhs) else {
            return Err(if self.is_negative() ^ rhs.is_negative() {
                Error::PosOverflow
            } else {
                Error::NegOverflow
            });
        };

        match Self::new(value) {
            Ok(v) => Ok(Some(v)),
            Err(e) => Err(e),
        }
    }

    /// Saturating integer division.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    pub const fn saturating_div(self, rhs: impl AsRepr<i8>) -> Self {
        let rhs = conversions::as_repr(rhs);

        match Self::new(self.get().saturating_div(rhs)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        }
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns an [`Error`] on overflow.
    pub const fn checked_sub(self, other: impl AsRepr<i8>) -> Result<Self> {
        let other = conversions::as_repr(other);
        let Some(value) = self.get().checked_sub(other) else {
            return Err(if other.is_negative() {
                Error::PosOverflow
            } else {
                Error::NegOverflow
            });
        };

        Self::new(value)
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`Self::MIN`] on negative overflow, and [`Self::MAX`] on
    /// positive overflow.
    pub const fn saturating_sub(self, other: impl AsRepr<i8>) -> Self {
        let other = conversions::as_repr(other);

        match Self::new(self.get().saturating_sub(other)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        }
    }

    /// Return `true` if `self` is negative; `false` if zero or positive.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// assert!(!RangedI8::<-100, 100>::new_const::<10>().is_negative());
    /// assert!(RangedI8::<-100, 100>::new_const::<-10>().is_negative());
    /// ```
    pub const fn is_negative(self) -> bool {
        self.get().is_negative()
    }

    /// Return `true` if `self` is positive; `false` if zero or negative.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// assert!(RangedI8::<-100, 100>::new_const::<10>().is_positive());
    /// assert!(!RangedI8::<-100, 100>::new_const::<-10>().is_positive());
    /// ```
    pub const fn is_positive(self) -> bool {
        self.get().is_positive()
    }

    /// Calculate the midpoint (average) between `self` and `rhs`.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<-8, 8>::new_const::<0>();
    /// let b = RangedI8::<-8, 8>::new_const::<2>();
    /// let c = RangedI8::<-8, 8>::new_const::<4>();
    /// let d = RangedI8::<-8, 8>::new_const::<-1>();
    /// let e = RangedI8::<-8, 8>::new_const::<-7>();
    /// let f = RangedI8::<-8, 8>::new_const::<-3>();
    /// let g = RangedI8::<-8, 8>::new_const::<3>();
    /// let h = RangedI8::<-8, 8>::new_const::<7>();
    ///
    /// assert_eq!(a.midpoint(c), b);
    /// assert_eq!(d.midpoint(b), a);
    /// assert_eq!(e.midpoint(a), f);
    /// assert_eq!(a.midpoint(e), f);
    /// assert_eq!(a.midpoint(h), g);
    /// ```
    pub const fn midpoint(self, rhs: Self) -> Self {
        let Ok(value) = Self::new(midpoint(self.get(), rhs.get())) else {
            panic!("unexpected midpoint value")
        };

        value
    }

    /// Add two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<1, 3>::new_const::<1>();
    /// let b = RangedI8::<-1, 3>::new_const::<2>();
    /// let output: RangedI8::<0, 6> = a.add(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<1, 3>::new_const::<1>();
    /// let b = RangedI8::<-1, 3>::new_const::<2>();
    /// let output: RangedI8::<1, 6> = a.add(b);
    ///
    /// assert_eq!(output.get(), 3);
    /// ```
    pub const fn add<
        const RHS_MIN: i8,
        const RHS_MAX: i8,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedI8<RHS_MIN, RHS_MAX>,
    ) -> RangedI8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN + RHS_MIN != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX + RHS_MAX != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedI8(self.get() + rhs.get())
    }

    /// Subtract a number from `self`.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<2, 5>::new_const::<3>();
    /// let b = RangedI8::<-1, 3>::new_const::<1>();
    /// let output: RangedI8::<-1, 6> = a.sub(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<2, 5>::new_const::<3>();
    /// let b = RangedI8::<-1, 3>::new_const::<1>();
    /// let output: RangedI8::<0, 6> = a.sub(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    pub const fn sub<
        const RHS_MIN: i8,
        const RHS_MAX: i8,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedI8<RHS_MIN, RHS_MAX>,
    ) -> RangedI8<OUTPUT_MIN, OUTPUT_MAX> {
        const {
            if MIN - RHS_MAX != OUTPUT_MIN {
                panic!("Min mismatch");
            }

            if MAX - RHS_MIN != OUTPUT_MAX {
                panic!("Max mismatch");
            }
        }

        RangedI8(self.get() - rhs.get())
    }

    /// Multiply two numbers together.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<-2, 3>::new_const::<1>();
    /// let b = RangedI8::<0, 3>::new_const::<2>();
    /// let output: RangedI8::<-6, 9> = a.mul(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<-2, 3>::new_const::<1>();
    /// let b = RangedI8::<0, 3>::new_const::<2>();
    /// let output: RangedI8::<0, 9> = a.mul(b);
    ///
    /// assert_eq!(output.get(), 2);
    /// ```
    pub const fn mul<
        const RHS_MIN: i8,
        const RHS_MAX: i8,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedI8<RHS_MIN, RHS_MAX>,
    ) -> RangedI8<OUTPUT_MIN, OUTPUT_MAX> {
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

        RangedI8(self.get() * rhs.get())
    }

    /// Divide `self` by a number.
    ///
    /// ```rust
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<2, 5>::new_const::<3>();
    /// let b = RangedI8::<1, 2>::new_const::<2>();
    /// let output: RangedI8::<1, 2> = a.div(b);
    ///
    /// assert_eq!(output.get(), 1);
    /// ```
    ///
    /// Does not compile:
    //
    /// ```compile_fail
    /// # use ranch::RangedI8;
    /// let a = RangedI8::<2, 5>::new_const::<3>();
    /// let b = RangedI8::<1, 2>::new_const::<1>();
    /// let output: RangedI8::<0, 2> = a.div(b);
    ///
    /// assert_eq!(output.get(), 1);
    /// ```
    pub const fn div<
        const RHS_MIN: i8,
        const RHS_MAX: i8,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedI8<RHS_MIN, RHS_MAX>,
    ) -> RangedI8<OUTPUT_MIN, OUTPUT_MAX> {
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

        RangedI8(self.get() / rhs.get())
    }

    /// Raise to an integer power.
    ///
    /// ```rust
    /// # use ranch::{RangedI8, RangedU32};
    /// let a = RangedI8::<-1, 3>::new_const::<2>();
    /// let b = RangedU32::<2, 3>::new_const::<2>();
    /// let output: RangedI8::<-1, 27> = a.pow(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    ///
    /// Does not compile:
    ///
    /// ```compile_fail
    /// # use ranch::{RangedI8, RangedU32};
    /// let a = RangedI8::<1, 3>::new_const::<2>();
    /// let b = RangedU32::<2, 3>::new_const::<2>();
    /// let output: RangedI8::<0, 27> = a.pow(b);
    ///
    /// assert_eq!(output.get(), 4);
    /// ```
    pub const fn pow<
        const RHS_MIN: u32,
        const RHS_MAX: u32,
        const OUTPUT_MIN: i8,
        const OUTPUT_MAX: i8,
    >(
        self,
        rhs: RangedU32<RHS_MIN, RHS_MAX>,
    ) -> RangedI8<OUTPUT_MIN, OUTPUT_MAX> {
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

        RangedI8(self.get().pow(rhs.get()))
    }
}

impl<const MIN: i8, const MAX: i8> core::str::FromStr for RangedI8<MIN, MAX> {
    type Err = ParsingError;

    fn from_str(src: &str) -> ParsingResult<Self> {
        let parsed = src.parse::<i8>()?;

        Self::new(parsed).map_err(From::from)
    }
}

impl<const MIN: i8, const MAX: i8> crate::error::Saturate
    for RangedI8<MIN, MAX>
{
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
}

// polyfill for midpoint (Added in Rust 1.87.0, MSRV is Rust 1.85.0)
const fn midpoint(a: i8, b: i8) -> i8 {
    let t = ((a ^ b) >> 1) + (a & b);
    t + (if t < 0 { 1 } else { 0 } & (a ^ b))
}
