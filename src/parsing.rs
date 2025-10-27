//! Error types related to parsing

use core::{error, fmt, result};

/// Parsing ranged integer result
pub type Result<T = (), E = Error> = result::Result<T, E>;

/// Error parsing ranged integer
#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    /// Internal parsing error
    ParseInt(core::num::ParseIntError),
    /// Integer is too large to store in target integer type
    PosOverflow,
    /// Integer is too small to store in target integer type
    NegOverflow,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseInt(err) => err.fmt(f),
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

impl From<core::num::ParseIntError> for Error {
    fn from(error: core::num::ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}

impl From<crate::Error> for Error {
    fn from(error: crate::Error) -> Self {
        match error {
            crate::Error::PosOverflow => Self::PosOverflow,
            crate::Error::NegOverflow => Self::NegOverflow,
        }
    }
}
