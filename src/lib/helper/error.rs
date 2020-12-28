use std::fmt;
use std::io;
use std::convert::From;

/// LError
///
/// # Description
/// A generic error representation
pub struct LError {
    pub message: String
}

impl fmt::Display for LError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.message)
    }
}

impl fmt::Debug for LError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Parser library encounter an error: message: {}",
            self.message
        )
    }
}

impl From<io::Error> for LError {
    fn from(error: io::Error) -> Self {
        LError {
            message: error.to_string() 
        }
    }
}