use toml::Value;
use super::network::{Network, get_network};
use super::workload::{Workload, get_workload};
use super::crd::{CustomCrd, get_custom_crd};
use crate::lib::helper::error::LError;


/// Spec
///
/// # Description
/// Contain the properties of the type of templates
/// - Workload => will contains containers spec
/// - Network => will contains networks spec
#[derive(Debug, Clone)]
pub struct Spec {
    pub workload: Result<Workload, LError>,
    pub network: Result<Network, LError>,
    pub crd: Result<CustomCrd, LError>
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
    let crd = get_custom_crd(ast);

    Spec {
        workload,
        network,
        crd
    }
}