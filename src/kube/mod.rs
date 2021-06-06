pub mod workload;
pub mod dry;
pub mod helper;

mod network;
mod common;
mod crd;
mod env;

use crate::lib::parser::{Object, Kind};

/// Get Yaml String From Object
///
/// # Description
/// Retrieve a YAML string representation of the Object
///
/// # Arguments
/// * `object` - Object
pub fn generate_yaml(object: Object) -> Result<String, helper::error::KubeError> {
    let res = match object.kind.to_owned() {
        Kind::Workload(kind) => workload::parse_workload_from_object(object, kind)?,
        Kind::Network(kind) => network::parse_network_from_object(object, kind)?,
        Kind::Env(kind) => env::get_env_from_object(object, kind)?,
        Kind::Custom(kind) => crd::crd_to_yaml(object, kind)?,
        _ => String::new()
    };

    Ok(res)
}