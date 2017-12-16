use std::error::Error as StdError;
use std::fmt;

pub type Error = GuardError;

/// A transmutation error.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{ErrorReason, Error, guarded_transmute};
/// # unsafe {
/// assert_eq!(guarded_transmute::<u16>(&[0x00]),
///            Err(Error {
///                required: 16 / 8,
///                actual: 1,
///                reason: ErrorReason::NotEnoughBytes,
///            }));
/// # }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GuardError {
    /// The required amount of bytes for transmutation.
    pub required: usize,
    /// The actual amount of bytes.
    pub actual: usize,
    /// Why this `required`/`actual`/`T` combo is an error.
    pub reason: ErrorReason,
}

/// How the type's size compares to the received byte count and the transmutation function's characteristic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ErrorReason {
    /// Too few bytes to fill even one instance of a type.
    NotEnoughBytes,
    /// Too many bytes to fill a type.
    ///
    /// Currently unused.
    TooManyBytes,
    /// The byte amount received is not the same as the type's size.
    InexactByteCount,
    /// The byte count is fine, but the data contains an invalid value for the target type.
    InvalidValue,
}

impl StdError for GuardError {
    fn description(&self) -> &str {
        match self.reason {
            ErrorReason::NotEnoughBytes => "Not enough bytes to fill type",
            ErrorReason::TooManyBytes => "Too many bytes for type",
            ErrorReason::InexactByteCount => "Not exactly the amount of bytes for type",
            ErrorReason::InvalidValue => "Invalid target value detected",
        }
    }
}

impl fmt::Display for GuardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (required: {}, actual: {})", self.description(), self.required, self.actual)
    }
}
