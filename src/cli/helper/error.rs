use std::fmt;
use std::convert::From;
use std::error::Error;
use std::io::Error as IOError;

#[derive(Debug)]
pub enum TypeError<'a> {
    Io(&'a str),
    Lib(&'a str),
    MissingArg(&'a str),
    MissingRes(&'a str)
}

impl<'a> std::fmt::Display for TypeError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::Io(msg) => write!(f, "An error occurred during I/O Operation: {}", msg),
            TypeError::Lib(msg) => write!(f, "An error occured with the parser library: {}", msg),
            TypeError::MissingArg(msg) => write!(f, "The argument {} is missing", msg),
            TypeError::MissingRes(msg) => write!(f, "Missing result of: {}", msg)
        }
    }
}

impl<'a> Error for TypeError<'a> {}

/// CError
///
/// # Description
/// A generic error representation for the CLI
pub struct CError {
    pub message: String
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
            "CLI encountered an issue: {}",
            self.message,
        )
    }
}

impl Error for CError {
    fn description(&self) -> &str {
        self.message.as_str()
    }    
}

impl<'a> From<TypeError<'a>> for CError {
    fn from(err: TypeError) -> Self {
        CError {
            message: err.to_string()
        }
    }
}

impl From<IOError> for CError {
    fn from(err: IOError) -> Self {
        CError {
            message: err.to_string()
        }
    }
}