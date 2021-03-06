use crate::lib::parser::Object;
use crate::kube::helper::error::KubeError;

mod container;
mod deployment;

/// Parse Workload From Object
///
/// # Description
/// Parse a workload and try to return a String representation
///
/// # Arguments
/// * `object` - Object
/// * `kind` - String
///
/// # Return
/// Result<String, KubeError>
pub fn parse_workload_from_object(object: Object, kind: String) -> Result<String, KubeError> {
    match kind.as_str() {
        "deployment" => deployment::get_deployment_from_object(&object),
        _ => Ok("".to_owned())
    }
}