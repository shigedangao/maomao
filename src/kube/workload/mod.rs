use crate::lib::parser::Object;
use crate::kube::helper::error::KubeError;

// utils to construct workloads
mod container;
mod volumes;

// workload
mod deployment;
mod daemonset;
mod statefulset;

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
        "daemonset" => daemonset::get_daemonset_from_object(&object),
        "statefulset" => statefulset::get_statefulset_from_object(&object),
        _ => Ok("".to_owned())
    }
}