use std::fmt;
use std::io;
use std::convert::From;

/// LError
///
/// # Description
/// A generic error representation for the Lib
#[derive(Clone)]
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

impl From<regex::Error> for LError {
    fn from(error: regex::Error) -> Self {
        LError {
            message: error.to_string()
        }
    }
}

pub mod network {
    use std::fmt;
    use std::convert::From;

    #[derive(Debug)]
    pub enum Error {
        IngressWrongType,
        MissingRules,
        PathNotFound
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Error::IngressWrongType => write!(f, "Unable to convert the ingress definition to a map"),
                Error::MissingRules => write!(f, "Missing ingress [rules] property"),
                Error::PathNotFound => write!(f, "[paths] not found")
            }
        }
    }

    impl std::error::Error for Error {}

    impl From<Error> for super::LError {
        fn from(err: Error) -> Self {
            super::LError {
                message: err.to_string()
            }
        } 
    }
}

pub mod workload {
    use std::fmt;
    use std::convert::From;

    #[derive(Debug)]
    pub enum Error {
        WorkloadNotExist,
        WorkloadMalformatted,
    }

    impl std::error::Error for Error {}

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Error::WorkloadNotExist => write!(f, "Workload does not exist. Make sure that [workload] is set on the template"),
                Error::WorkloadMalformatted => write!(f, "Workload is malformatted. Please check that workload is above it's children")
            }
        }
    }

    impl From<Error> for super::LError {
        fn from(err: Error) -> Self {
            super::LError {
                message: err.to_string()
            }
        }
    }
}

pub mod crd {
    use std::fmt;
    use std::convert::From;

    #[derive(Debug)]
    pub enum Error {
        SpecNotFound
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Error::SpecNotFound => write!(f, "[Spec] could not be founded")
            }
        }
    }

    impl std::error::Error for Error {}

    impl From<Error> for super::LError {
        fn from(err: Error) -> Self {
            super::LError {
                message: err.to_string()
            }
        }
    }
}