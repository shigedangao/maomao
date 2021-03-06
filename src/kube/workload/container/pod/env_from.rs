use k8s_openapi::api::core::v1::{
    EnvFromSource,
    ConfigMapEnvSource,
    SecretEnvSource
};
use crate::lib::parser::workload::env::EnvFrom;

/// Get Env Source From EnvFrom
///
/// # Description
/// Get the EnvFromSource struct
///
/// # Arguments
/// * `env` - EnvFrom
///
/// # Return
/// Vec<EnvFromSource>
pub fn get_env_source_from_envfrom(env: EnvFrom) -> Vec<EnvFromSource> {
    let mut configmap_ref = env.map.into_iter()
        .map(|name| EnvFromSource {
            config_map_ref: Some(ConfigMapEnvSource {
                name: Some(name),
                optional: None
            }),
            secret_ref: None,
            ..Default::default()
        })
        .collect::<Vec<EnvFromSource>>();

    let mut secret_ref = env.secret.into_iter()
        .map(|name | EnvFromSource {
            config_map_ref: None,
            secret_ref: Some(SecretEnvSource {
                name: Some(name),
                optional: None
            }),
            ..Default::default()
        })
        .collect::<Vec<EnvFromSource>>();

    configmap_ref.append(&mut secret_ref);

    configmap_ref
}