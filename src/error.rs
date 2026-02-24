use core::{error, fmt};

use crate::range::MultiRange;

/// Creating ranged integer result
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

/// Error creating ranged integer
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Error {
    /// Integer is too large to store in target integer type
    PosOverflow,
    /// Integer is too small to store in target integer type
    NegOverflow,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PosOverflow => write!(
                f,
                "Integer is too large to store in target integer type",
            ),
            Self::NegOverflow => write!(
                f,
                "Integer is too small to store in target integer type",
            ),
        }
    }
}

impl Error {
    /// Clamp out of bounds values into the requested range.
    ///
    /// ```rust
    /// # use ranch::{RangedU8, Error};
    /// assert_eq!(
    ///     RangedU8::<1, 3>::with_u8(0).unwrap_or_else(Error::clamp),
    ///     RangedU8::<1, 3>::MIN,
    /// );
    /// assert_eq!(
    ///     RangedU8::<1, 3>::with_u8(4).unwrap_or_else(Error::clamp),
    ///     RangedU8::<1, 3>::MAX,
    /// );
    /// ```
    #[must_use]
    pub const fn clamp<T>(self) -> T
    where
        T: MultiRange,
    {
        match self {
            Self::PosOverflow => T::MAX,
            Self::NegOverflow => T::MIN,
        }
    }
}
