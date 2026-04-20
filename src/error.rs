//! Error handling

mod seal {
    use super::*;

    #[derive(Debug)]
    pub enum Data {
        None,
        Parse(core::num::ParseIntError),
    }
}

use core::fmt::Debug;

use self::seal::Data;

/// A subset of error kinds
trait Kind: Copy {
    /// Associated data for an error kind
    type Data: Debug;
}

impl Kind for ErrorKind {
    type Data = Data;
}

impl Kind for ParseIntErrorKind {
    type Data = core::num::ParseIntError;
}

impl Kind for ParseNonZeroIntErrorKind {
    type Data = core::num::ParseIntError;
}

impl Kind for RangeErrorKind {
    type Data = ();
}

impl Kind for NonZeroRangeErrorKind {
    type Data = ();
}

/// Convenience alias for catch-all error `Result`s
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

/// Non-zero range checking error
pub type NonZeroRangeError = Error<NonZeroRangeErrorKind>;
/// Integer parsing error
pub type ParseIntError = Error<ParseIntErrorKind>;
/// Non-zero integer parsing error
pub type ParseNonZeroIntError = Error<ParseNonZeroIntErrorKind>;
/// Range checking error
pub type RangeError = Error<RangeErrorKind>;

/// Convenience alias for non-zero range checking error `Result`s
pub type NonZeroRangeResult<T = (), E = NonZeroRangeError> = Result<T, E>;
/// Convenience alias for integer parsing error `Result`s
pub type ParseIntResult<T = (), E = ParseIntError> = Result<T, E>;
/// Convenience alias for non-zero integer parsing error `Result`s
pub type ParseNonZeroIntResult<T = (), E = ParseNonZeroIntError> = Result<T, E>;
/// Convenience alias for range checking error `Result`s
pub type RangeResult<T = (), E = RangeError> = Result<T, E>;

/// Catch-all for ranch error kinds
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
#[repr(u32)]
pub enum ErrorKind {
    /// A custom error that does not fall under any other error kind type
    ///
    /// Can be used to construct your own errors.  This variant is not used by
    /// ranch.
    Other,
    /// Integer is too large to store in target integer type
    PosOverflow,
    /// Integer is too small to store in target integer type
    NegOverflow,
    /// Integer is zero
    Zero,
    /// Internal parsing error
    ParseInt,
}

/// Ranch error kinds related to checking ranges
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum RangeErrorKind {
    /// Integer is too large to store in target integer type
    PosOverflow = ErrorKind::PosOverflow as _,
    /// Integer is too small to store in target integer type
    NegOverflow = ErrorKind::NegOverflow as _,
}

/// Ranch error kinds related to parsing integers
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum ParseIntErrorKind {
    /// Integer is too large to store in target integer type
    PosOverflow = ErrorKind::PosOverflow as _,
    /// Integer is too small to store in target integer type
    NegOverflow = ErrorKind::NegOverflow as _,
    /// Internal parsing error
    ParseInt = ErrorKind::ParseInt as _,
}

/// Ranch error kinds related to parsing non-zero integers
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum ParseNonZeroIntErrorKind {
    /// Integer is too large to store in target integer type
    PosOverflow = ErrorKind::PosOverflow as _,
    /// Integer is too small to store in target integer type
    NegOverflow = ErrorKind::NegOverflow as _,
    /// Integer is zero
    Zero = ErrorKind::Zero as _,
    /// Internal parsing error
    ParseInt = ErrorKind::ParseInt as _,
}

/// Ranch error kinds related to checking non-zero ranges
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum NonZeroRangeErrorKind {
    /// Integer is too large to store in target integer type
    PosOverflow = ErrorKind::PosOverflow as _,
    /// Integer is too small to store in target integer type
    NegOverflow = ErrorKind::NegOverflow as _,
    /// Internal parsing error
    Zero = ErrorKind::Zero as _,
}

/// Ranch error type
#[derive(Debug)]
pub struct Error<K: Kind = ErrorKind> {
    kind: K,
    data: K::Data,
}

impl<K> Error<K>
where
    K: Kind,
{
    /// Return the corresponding error kind for this error.
    pub const fn kind(&self) -> K {
        self.kind
    }
}

impl ParseIntError {
    /// Obtain the integer error kind.
    pub const fn int_error_kind(&self) -> core::num::IntErrorKind {
        use core::num::IntErrorKind;

        match self.kind {
            ParseIntErrorKind::PosOverflow => IntErrorKind::PosOverflow,
            ParseIntErrorKind::NegOverflow => IntErrorKind::NegOverflow,
            ParseIntErrorKind::ParseInt => *self.data.kind(),
        }
    }
}

impl ParseNonZeroIntError {
    /// Obtain the integer error kind.
    pub const fn int_error_kind(&self) -> core::num::IntErrorKind {
        use core::num::IntErrorKind;

        match self.kind {
            ParseNonZeroIntErrorKind::PosOverflow => IntErrorKind::PosOverflow,
            ParseNonZeroIntErrorKind::NegOverflow => IntErrorKind::NegOverflow,
            ParseNonZeroIntErrorKind::Zero => IntErrorKind::Zero,
            ParseNonZeroIntErrorKind::ParseInt => *self.data.kind(),
        }
    }
}

impl From<core::num::ParseIntError> for ParseIntError {
    fn from(err: core::num::ParseIntError) -> Self {
        use core::num::IntErrorKind;

        let kind = match err.kind() {
            IntErrorKind::PosOverflow => ParseIntErrorKind::PosOverflow,
            IntErrorKind::NegOverflow => ParseIntErrorKind::NegOverflow,
            _ => ParseIntErrorKind::ParseInt,
        };

        Self {
            kind: kind,
            data: err,
        }
    }
}

impl From<core::num::ParseIntError> for ParseNonZeroIntError {
    fn from(err: core::num::ParseIntError) -> Self {
        use core::num::IntErrorKind;

        let kind = match err.kind() {
            IntErrorKind::PosOverflow => ParseNonZeroIntErrorKind::PosOverflow,
            IntErrorKind::NegOverflow => ParseNonZeroIntErrorKind::NegOverflow,
            IntErrorKind::Zero => ParseNonZeroIntErrorKind::Zero,
            _ => ParseNonZeroIntErrorKind::ParseInt,
        };

        Self {
            kind: kind,
            data: err,
        }
    }
}

/// Wrapper struct to subvert orphan rule for error conversions.
pub struct To<T>(pub T);

impl<J, K> From<To<Error<J>>> for Error<K>
where
    J: Kind,
    K: Kind,
{
    fn from(To(err): To<Error<J>>) -> Self {
        const {}

        todo!()
    }
}
