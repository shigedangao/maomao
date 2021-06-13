use serde::Deserialize;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{
    ObjectMeta,
    LabelSelector,
};
use kube::api::GroupVersionKind;
use kube::core::ApiResource;
use kube::error::Error;
use crate::lib::parser::Object;
use super::helper::error::{
    KubeError,
    dry_run::Error as DryRunError
};

// Constant
const API_VERSION_SEPARATOR: &str = "/";


#[derive(Deserialize)]
pub struct Extract {
    #[serde(rename(deserialize = "apiVersion"))]
    pub api_version: String,
    pub kind: String,
    pub metadata: ObjectMeta
}

/// Get Metadata From Object
///
/// # Description
/// Retrieve an ObjectMeta object which map the k8s_openapi ObjectMeta
/// We provide the following field by default
///
/// metadata:
///     annotations: <>
///     labels: <>
///
/// # Arguments
/// * `object` - &Object
///
/// # Return
/// ObjectMeta
pub fn get_metadata_from_object(object: &Object) -> ObjectMeta {
    ObjectMeta {
        annotations: object.annotations.to_owned(),
        labels: Some(object.metadata.to_owned()),
        name: object.metadata.get("name").cloned(),
        namespace: object.namespace.to_owned(),
        ..Default::default()
    }
}

/// Get Workload Metadata From Object
///
/// # Description
/// Retrieve an ObjectMeta object which map the k8s_openapi ObjectMeta
/// We provide the following field by default for workload (Deployment, StatefulSet...)
///
/// metadata:
///     labels: <>
///
/// # Arguments
/// * `object` - &Object
///
/// # Return
/// ObjectMeta
pub fn get_workload_metadata_from_object(object: &Object) -> ObjectMeta {
    ObjectMeta {
        labels: Some(object.metadata.to_owned()),
        ..Default::default()
    }
}

/// Get Selector From Object
///
/// # Description
/// Get the selector. The selector use the same TOML metadata values
///
/// # Arguments
/// * `object` - &Object
///
/// # Return
/// LabelSelector
pub fn get_label_selector_from_object(object: &Object) -> LabelSelector {
    LabelSelector {
        match_expressions: None,
        match_labels: Some(object.metadata.to_owned())
    }
}

/// Get ApiResource
///
/// # Description
/// Retrieve a ApiResource which will be used to generate a DynamicObject
///
/// # Arguments
/// * `extract` - &Extract
///
/// # Return
/// Result<ApiResource, KubeError>
pub fn get_api_resource(extract: &Extract) -> Result<ApiResource, KubeError> {
    // split the apiVersion to retrieve the apiGroup and the version
    // Usually it's represent by <apigroup>/<version>
    let args: Vec<&str> = extract.api_version.split(API_VERSION_SEPARATOR).collect();
    // Retrieve the api_group and the version
    let api_group = args.get(0);
    let api_version = args.get(1);

    if let (Some(group), Some(version)) = (api_group, api_version) {
        let gvk = GroupVersionKind::gvk(group, version, &extract.kind);
        let api_res = ApiResource::from_gvk(&gvk);
            
        return Ok(api_res)
    }

    if let Some(group) = api_group {
        let gvk = GroupVersionKind::gvk("", group, &extract.kind);
        let api_res = ApiResource::from_gvk(&gvk);
        
        return Ok(api_res);
    }
    
    Err(KubeError::from(DryRunError::MissingApiVersion))
}

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