use kube::error::Error;
use crate::kube::helper::error::KubeError;

/// Parse Kube Error
///
/// # Description
/// Convert kube::error::Error to KubeError
///
/// # Arguments
/// * `err` - kube::error::Error
///
/// # Return
/// KubeError
pub fn parse_kube_error(err: Error) -> KubeError {
    match err {
        Error::Api(e) => KubeError { message: format!("error: {}, code: {}", e.message, e.code) },
        _ => KubeError { message: err.to_string() }
    }
}