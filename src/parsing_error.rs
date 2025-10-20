use core::{error, fmt};

use crate::Error;

/// Parsing ranged integer result
pub type ParsingResult<T = (), E = ParsingError> = Result<T, E>;

/// Error parsing ranged integer
#[derive(Eq, PartialEq, Debug)]
pub enum ParsingError {
    /// Internal parsing error
    ParseInt(core::num::ParseIntError),
    /// Integer is too large to store in target integer type
    PosOverflow,
    /// Integer is too small to store in target integer type
    NegOverflow,
}

impl error::Error for ParsingError {}

impl fmt::Display for ParsingError {
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

impl From<core::num::ParseIntError> for ParsingError {
    fn from(error: core::num::ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}

impl From<Error> for ParsingError {
    fn from(error: Error) -> Self {
        match error {
            Error::PosOverflow => Self::PosOverflow,
            Error::NegOverflow => Self::NegOverflow,
        }
    }
}
