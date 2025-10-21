use crate::{Error, ParsingError, ParsingResult, Result};

/// [`i8`] with a specified minimum and maximum value
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RangedI8<const MIN: i8, const MAX: i8>(i8);

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
    pub const fn new(value: i8) -> Result<Self> {
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
    pub const fn checked_add(self, other: Self) -> Result<Self> {
        let Some(value) = self.get().checked_add(other.get()) else {
            return Err(
                if self.get().saturating_add(other.get()) == Self::MAX.get() {
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
    pub const fn saturating_add(self, other: Self) -> Self {
        match Self::new(self.get().saturating_add(other.get())) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        }
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns an [`Error`] on overflow.
    pub const fn checked_mul(self, other: Self) -> Result<Self> {
        let Some(value) = self.get().checked_mul(other.get()) else {
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
    pub const fn saturating_mul(self, other: Self) -> Self {
        match Self::new(self.get().saturating_mul(other.get())) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns an [`Error`] on overflow.
    pub const fn checked_pow(self, other: u32) -> Result<Self> {
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
    pub const fn saturating_pow(self, other: u32) -> Self {
        match Self::new(self.get().saturating_pow(other)) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        }
    }

    /// Checked integer division.
    ///
    /// Returns an [`Error`] on overflow; [`None`] if `rhs == 0`.
    pub const fn checked_div(self, rhs: Self) -> Result<Option<Self>> {
        if rhs.get() == 0 {
            return Ok(None);
        }

        let Some(value) = self.get().checked_div(rhs.get()) else {
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
    pub const fn saturating_div(self, rhs: Self) -> Self {
        match Self::new(self.get().saturating_div(rhs.get())) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        }
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns an [`Error`] on overflow.
    pub const fn checked_sub(self, other: Self) -> Result<Self> {
        let Some(value) = self.get().checked_sub(other.get()) else {
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
    pub const fn saturating_sub(self, other: Self) -> Self {
        match Self::new(self.get().saturating_sub(other.get())) {
            Ok(value) => value,
            Err(Error::NegOverflow) => Self::MIN,
            Err(Error::PosOverflow) => Self::MAX,
        }
    }

    /// Return `true` if `self` is negative; `false` if zero or positive.
    pub const fn is_negative(self) -> bool {
        self.get().is_negative()
    }

    /// Return `true` if `self` is positive; `false` if zero or negative.
    pub const fn is_positive(self) -> bool {
        self.get().is_positive()
    }

    /// Calculate the midpoint (average) between `self` and `rhs`.
    pub const fn midpoint(self, rhs: Self) -> Self {
        let Ok(value) = Self::new(midpoint(self.get(), rhs.get())) else {
            panic!("unexpected midpoint value")
        };

        value
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
