pub mod cli;
pub mod op;
pub mod units;
pub mod util;

#[macro_use]
extern crate lazy_static;

use std::error::Error;
use std::fmt;
use std::process::exit;

/// Trait to print a message and exit the program.
/// Implemented for `Result` and `Option`.
///
/// Use like `expect(...)`.
pub trait ErrorAbort<T> {
    fn exit(self, message: &str) -> T;
}

impl<T> ErrorAbort<T> for Option<T> {
    fn exit(self, message: &str) -> T {
        match self {
            Some(v) => v,
            None => {
                eprintln!("Terminated with ERROR:");
                eprintln!("{}", message);
                exit(1);
            }
        }
    }
}

impl<T, E> ErrorAbort<T> for Result<T, E>
where
    E: Error,
{
    fn exit(self, message: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Terminated with ERROR:");
                eprintln!("{} ({:?})", message, e);
                exit(1);
            }
        }
    }
}

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

/// Error type for illegal image operation parameters.
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
