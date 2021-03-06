use std::fmt;
use std::convert::From;
use std::error::Error;
use std::io::Error as IOError;
use crate::lib::helper::error::LError;

// Constant errors
const IO: &str = "An error has been encountered with I/O operations";
const LIB: &str = "An error occurred with the parser library";

/// CError
///
/// # Description
/// A generic error representation for the CLI
pub struct CError {
    pub message: String,
    pub details: String
}

impl fmt::Display for CError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.message)
    }
}

impl fmt::Debug for CError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CLI encountered an issue: {}, trace: {}",
            self.message,
            self.details
        )
    }
}

impl Error for CError {
    fn description(&self) -> &str {
        self.message.as_str()
    }    
}

impl From<IOError> for CError {
    fn from(err: IOError) -> Self {
        CError {
            message: IO.to_owned(),
            details: err.to_string()
        }
    }
}

impl From<LError> for CError {
    fn from(err: LError) -> Self {
        CError {
            message: LIB.to_owned(),
            details: err.message
        }
    }
}
