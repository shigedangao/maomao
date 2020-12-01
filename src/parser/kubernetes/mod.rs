mod container;
mod controller;
mod network;

use std::collections::HashMap;
use toml::Value;
use crate::helper::err::LibError;

// Constant error
const MISSING_KIND: &str = "Missing kubernetes object type";
const MALFORMATED_KIND: &str = "Template `type` value is malformated";
const MISSING_KIND_ERR: &str = "Check if the `type` property is set in the template";

// Constant
const SPLIT_PATTERN: &str = "::";

/// Kubernetes Resources
///
/// # Description
/// Trait use to create a generic Kubernetes resources, a container, a service etc...
pub trait KubernetesResources {
    /// New
    ///
    /// # Description
    /// Initialize the kubernetes resource
    ///
    /// # Arguments
    /// * `content` - Option<Value>
    fn new() -> Self;
    /// Set Kind
    ///
    /// # Description
    /// Set the type of the sub object (i.e: Deployment for controller, NodePort for Service)
    ///
    /// # Arguments
    /// * `kind` - &str
    fn set_kind(self, kind: &str) -> Self;
    /// Set Metadata
    ///
    /// # Description
    /// Set metadata to the kubernetes resource
    ///
    /// # Arguments
    /// * `&self` - KubernetesResources
    /// * `labels` - Option<HashMap<String, String>>
    fn set_metadata(self, labels: Option<HashMap<String, String>>) -> Self;
}

/// Get Kubernetes Kind Object
///
/// # Description
/// Retrieve the kubernetes representation object for the targeted kubernetes resources
///
/// # Arguments
/// * `k8s_type` - Option<String>
pub fn get_kubernetes_controller(k8s_type: Option<String>, toml_content: Option<Value>) -> Result<(), LibError> {
    let t = k8s_type.ok_or(LibError {
        kind: MISSING_KIND.to_owned(),
        message: MISSING_KIND_ERR.to_owned()
    })?;

    // Split the type property on the toml file
    // i.e: type = "Controller::Deployment"
    let resource_kind: Vec<&str> = t.split(SPLIT_PATTERN).collect();
    if !resource_kind.len() != 2 {
        return Err(LibError {
            kind: MALFORMATED_KIND.to_owned(),
            message: MISSING_KIND_ERR.to_owned()
        })
    }

    let resource_type = resource_kind.get(1).ok_or_else(|| LibError {
        kind: MALFORMATED_KIND.to_owned(),
        message: MISSING_KIND.to_owned(),
    })?;

    let controller = controller::Controller::new()
        .set_containers(toml_content);

    Ok(())
}