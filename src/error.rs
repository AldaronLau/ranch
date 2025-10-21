use core::{error, fmt};

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
    /// Saturate out of bounds values.
    ///
    /// ```rust
    /// # use ranch::{RangedU8, Error};
    /// assert_eq!(
    ///     RangedU8::<1, 3>::new(0).unwrap_or_else(Error::saturate),
    ///     RangedU8::<1, 3>::new_const::<1>(),
    /// );
    /// assert_eq!(
    ///     RangedU8::<1, 3>::new(4).unwrap_or_else(Error::saturate),
    ///     RangedU8::<1, 3>::new_const::<3>(),
    /// );
    /// ```
    pub const fn saturate<T>(self) -> T
    where
        T: Saturate,
    {
        match self {
            Self::PosOverflow => T::MAX,
            Self::NegOverflow => T::MIN,
        }
    }
}

pub trait Saturate {
    const MIN: Self;
    const MAX: Self;
}
