use crate::lib::parser::Object;
use super::helper::error::KubeError;

mod services;
mod ingress;

/// Parse Network From Object
///
/// # Description
/// Parse a network object to a kubernetes network value in String
///
/// # Arguments
/// * `object` - Object
/// * `kind` - String
///
/// # Return
/// Result<String, KubeError>
pub fn parse_network_from_object(object: Object, kind: String) -> Result<String, KubeError> {
    let res = match kind.as_str() {
        "service" => services::get_service_from_object(object),
        "ingress" => ingress::get_ingress_from_object(object),
        // @TODO replace by something else
        _ => Ok("".to_owned())
    };
    
    res
}