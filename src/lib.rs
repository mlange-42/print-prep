pub mod cli;
pub mod op;
pub mod units;
pub mod util;

use std::error::Error;
use std::fmt;

/// Error type for failed parsing of `String`s to `enum`s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEnumError(String);

impl Error for ParseEnumError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
impl fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Error type for failed parsing of `String`s to `struct`s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseStructError(String);

impl Error for ParseStructError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
impl fmt::Display for ParseStructError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Error type for failed parsing of `String`s to `struct`s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationParametersError(String);

impl Error for OperationParametersError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
impl fmt::Display for OperationParametersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
