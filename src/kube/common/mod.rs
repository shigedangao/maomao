/// This mod is used to define Common Struct used accross a Kubernetes Spec
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{
    ObjectMeta,
    LabelSelector
};
use crate::lib::parser::Object;

pub mod error;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    pub labels: BTreeMap<String, String>,
    pub annotations: Option<BTreeMap<String, String>>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Selector {
    pub labels: BTreeMap<String, String>
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