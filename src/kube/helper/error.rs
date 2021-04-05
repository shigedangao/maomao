use std::fmt;
use std::convert::From;
use std::error::Error;
use serde_yaml::Error as SerdeYamlError;
use crate::lib::helper::error::LError;

/// KubeError
///
/// # Description
/// A generic error representation for the Kube library
pub struct KubeError {
    pub message: String
}

impl fmt::Display for KubeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.message)
    }
}

impl fmt::Debug for KubeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Kube lib encountered an issue: {}",
            self.message
        )
    }
}

impl Error for KubeError {
    fn description(&self) -> &str {
        self.message.as_str()
    }    
}

impl From<LError> for KubeError {
    fn from(err: LError) -> Self {
        KubeError {
            message: err.message
        }
    }
}

impl From<SerdeYamlError> for KubeError {
    fn from(err: SerdeYamlError) -> Self {
        KubeError {
            message: err.to_string()
        }
    }
}

pub mod common {
    use std::fmt;
    use std::convert::From;
    
    #[derive(Debug)]
    pub enum Error {
        MissingSpec
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Error::MissingSpec => write!(f, "Spec is missing from parser object body") 
            }
        }
    }

    impl std::error::Error for Error {}

    impl From<Error> for super::KubeError {
        fn from(err: Error) -> Self {
            super::KubeError {
                message: err.to_string()
            }
        }
    }
}