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
