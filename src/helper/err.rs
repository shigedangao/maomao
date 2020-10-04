use std::fmt;
use std::io;

/// LibError
///
/// # Description
/// A generic error representation
pub struct LibError {
    pub kind: String,
    pub message: String
}

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl fmt::Debug for LibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Library encounter an error: code {}, message: {}",
            self.kind,
            self.message
        )
    }
}

impl From<io::Error> for LibError {
    fn from(error: io::Error) -> Self {
        LibError {
            kind: String::from("io"),
            message: error.to_string() 
        }
    }
}