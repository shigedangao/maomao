pub mod workload;

mod helper;
mod network;
mod common;

use crate::lib::parser::{Object, Kind};

/// Generate Yaml
///
/// # Description
/// Generate yaml for Objects
///
/// # Arguments
/// `objects` - Vec<Object>
pub fn generate_yaml(objects: Vec<Object>) {
    let yamls = objects
        .into_iter()
        .map(|o| get_yaml_string_from_object(o))
        .collect::<Vec<Result<String, helper::error::KubeError>>>();

    println!("{:?}", yamls);
}

/// Get Yaml String From Object
///
/// # Description
/// Retrieve a YAML string representation of the Object
///
/// # Arguments
/// * `object` - Object
fn get_yaml_string_from_object(object: Object) -> Result<String, helper::error::KubeError> {
    let res = match object.kind.to_owned() {
        Kind::Workload(kind) => workload::parse_workload_from_object(object, kind)?,
        Kind::Network(kind) => network::parse_network_from_object(object, kind)?,
        _ => String::new()
    };

    Ok(res)
}