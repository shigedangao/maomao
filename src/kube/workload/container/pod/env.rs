use std::convert::From;
use k8s_openapi::api::core::v1::{
    EnvVar,
    EnvVarSource,
    ObjectFieldSelector,
    ConfigMapKeySelector,
    ResourceFieldSelector,
    SecretKeySelector
};
use crate::lib::parser::workload::env::EnvRefKey;

/// Get Env Vars
///
/// # Description
/// Retrieve a list of EnvVar from a vector of EnvRefKey
///
/// # Arguments
/// * `vec` - Vec<EnvRefKey>
///
/// # Return
/// Vec<EnvVar>
pub fn get_env_vars(vec: Vec<EnvRefKey>) -> Vec<EnvVar> {
    vec.into_iter()
        .map(|item| EnvVar::from(item))
        .collect::<Vec<EnvVar>>()
}

impl From<EnvRefKey> for EnvVar {
    fn from(k: EnvRefKey) -> Self {
        let value_from = get_env_var_source(&k);
        let mut env = EnvVar {
            name: k.name,
            ..Default::default()
        };

        if value_from.is_some() {
            env.value_from = value_from;
        } else {
            env.value = k.item;
        }
    
        env
    }
}

/// Get Env Var Source
///
/// # Description
/// Construct an EnvVarSource based on the EnvRefKey struct
///
/// # Arguments
/// * `key` - &EnvRefKey
///
/// # Return
/// Option<EnvVarSource>
fn get_env_var_source(key: &EnvRefKey) -> Option<EnvVarSource> {
    if key.from_field.is_none() {
        return None;
    }

    let mut env_var_source = EnvVarSource::default();

    // an env with a definition containing the from_field value can contain the following definition
    // - { from_field: fieldRef, name: <field definition (metadata.namespace)>, item: <empty> }
    // - { from_field: configMapKeyRef, name: <configmap>, item: <key> }
    // - { from_field: resourcenv_var_sourceeFieldRef, name: <container name>, item: <resource> }
    // - { from_field: secretRef, name: <secret>, item: <key> }
    let from_field = key.from_field.to_owned().unwrap();
    let key = key.to_owned();

    match from_field.as_str() {
        "fieldRef" => env_var_source.field_ref = Some(ObjectFieldSelector {
            field_path: key.name,
            ..Default::default()
        }),
        "configMapRef" => env_var_source.config_map_key_ref = Some(ConfigMapKeySelector {
            key: key.item.unwrap_or("".to_owned()),
            name: Some(key.name),
            ..Default::default()
        }),
        "resourceFieldRef" => env_var_source.resource_field_ref = Some(ResourceFieldSelector {
            container_name: Some(key.name),
            divisor: None,
            resource: key.item.unwrap_or("".to_owned()),
        }),
        "secretRef" => env_var_source.secret_key_ref = Some(SecretKeySelector {
            name: Some(key.name),
            key: key.item.unwrap_or("".to_owned()),
            ..Default::default() 
        }),
        _ => {}
    };

    Some(env_var_source)
}