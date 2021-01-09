use toml::Value;
use super::network::{Network, get_network};
use super::workload::{Workload, get_workload};
use crate::lib::helper::error::LError;


/// Spec
///
/// # Description
/// Contain the properties of the type of templates
/// - Workload => will contains containers spec
/// - Network => will contains networks spec
#[derive(Debug)]
pub struct Spec {
    pub workload: Result<Workload, LError>,
    // pub network: Result<Network, LError>,
    pub network: Result<Network, LError>
}

/// Get Spec
///
/// # Description
/// Small decorator to wrap the call of of inner method
///
/// # Arguments
/// * `ast` &Value
///
/// # Return
/// Spec
pub fn get_spec(ast: &Value) -> Spec {
    let workload = get_workload(ast);
    let network = get_network(ast);

    Spec {
        workload,
        network
    }
}