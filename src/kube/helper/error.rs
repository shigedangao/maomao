use std::fmt;
use std::convert::From;
use std::error::Error;
use serde_yaml::Error as SerdeYamlError;
use serde_json::Error as SerdeJsonError;
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

impl From<SerdeJsonError> for KubeError {
    fn from(err: SerdeJsonError) -> Self {
        KubeError {
            message: err.to_string()
        }
    }
}

impl From<common::Error> for KubeError {
    fn from(err: common::Error) -> Self {
        KubeError { message: err.to_string() }
    }
}

impl<'a> From<dry_run::Error<'a>> for KubeError {
    fn from(err: dry_run::Error) -> Self {
        KubeError { message: err.to_string() }
    }
}

pub mod common {
    use std::fmt;
    
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
}

pub mod dry_run {
    use std::fmt;

    #[derive(Debug)]
    pub enum Error<'a> {
        MissingApiVersion,
        MissingSpecName,
        RemoveManagedField(&'a str)
    }

    impl<'a> fmt::Display for Error<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Error::MissingApiVersion => write!(f, "apiVersion is either missing or malformatted from the spec"),
                Error::MissingSpecName => write!(f, "`name` could not be founded in the metadata"),
                Error::RemoveManagedField(name) => write!(f, "Something went wrong when updating the metadata. Check the status of metadata.managedField for {}", name)
            }
        }
    }
}