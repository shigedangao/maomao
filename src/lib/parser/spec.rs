use toml::Value;
use super::network::{Network, get_network};
use super::workload::{Workload, get_workload};
use super::crd::{CustomCrd, get_custom_crd};
use super::env::{Env, get_env};
use crate::lib::helper::error::LError;
use super::Kind;


/// Spec
///
/// # Description
/// Contain the properties of the type of templates
/// - Workload => will contains containers spec
/// - Network => will contains networks spec
#[derive(Debug, Clone, Default)]
pub struct Spec {
    pub workload: Option<Workload>,
    pub network: Option<Network>,
    pub crd: Option<CustomCrd>,
    pub env: Option<Env>,
    pub error: Option<LError>
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
pub fn get_spec(ast: &Value, kind: &Kind) -> Spec {
    let mut spec = Spec {..Default::default()};
    
    match kind {
        Kind::Workload(_) => {
            match get_workload(ast) {
                Ok(res) => spec.workload = Some(res),
                Err(err) => spec.error = Some(err)
            }
        },
        Kind::Network(_) => {
            match get_network(ast) {
                Ok(res) => spec.network = Some(res),
                Err(err) => spec.error = Some(err)
            }
        },
        Kind::Custom(_) => {
            match get_custom_crd(ast) {
                Ok(res) => spec.crd = Some(res),
                Err(err) => spec.error = Some(err)
            }
        },
        Kind::Env(_) => {
            if let Some(e) = get_env(ast) {
                spec.env = Some(e)
            }
        }
        _ => {}
    }

    spec
}