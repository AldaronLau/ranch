use crate::{Error, ParsingError, ParsingResult, Result};

/// [`u128`] with a specified minimum and maximum value
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RangedU128<const MIN: u128, const MAX: u128>(u128);

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
    pub const fn new(value: u128) -> Result<Self> {
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
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        let Some(value) = self.get().checked_add(other.get()) else {
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
    pub const fn saturating_add(self, other: Self) -> Self {
        match Self::new(self.get().saturating_add(other.get())) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Multiply two ranged integers together.
    ///
    /// Returns [`None`] on overflow.
    pub const fn checked_mul(self, other: Self) -> Option<Self> {
        let Some(value) = self.get().checked_mul(other.get()) else {
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
    pub const fn saturating_mul(self, other: Self) -> Self {
        match Self::new(self.get().saturating_mul(other.get())) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Raise to an integer power.
    ///
    /// Returns [`None`] on overflow.
    pub const fn checked_pow(self, other: u32) -> Option<Self> {
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
    pub const fn saturating_pow(self, other: u32) -> Self {
        match Self::new(self.get().saturating_pow(other)) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }

    /// Checked integer division.
    ///
    /// Returns [`None`] on overflow or `rhs == 0`.
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        let Some(value) = self.get().checked_div(rhs.get()) else {
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
    pub const fn saturating_div(self, rhs: Self) -> Self {
        match Self::new(self.get().saturating_div(rhs.get())) {
            Ok(value) => value,
            Err(_) => Self::MIN,
        }
    }

    /// Subtract a ranged integers from another.
    ///
    /// Returns [`None`] on overflow.
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        let Some(value) = self.get().checked_sub(other.get()) else {
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
    pub const fn saturating_sub(self, other: Self) -> Self {
        match Self::new(self.get().saturating_sub(other.get())) {
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
    pub const fn midpoint(self, rhs: Self) -> Self {
        let Ok(value) = Self::new(self.get().midpoint(rhs.get())) else {
            panic!("unexpected midpoint value")
        };

        value
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
