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
    let mut map = Vec::new();
    
    if let Some(configmap) = env.map {
        let mut map_res = configmap
            .into_iter()
            .map(|name| EnvFromSource {
            config_map_ref: Some(ConfigMapEnvSource {
                name: Some(name),
                optional: None
            }),
            secret_ref: None,
            ..Default::default()
        })
        .collect::<Vec<EnvFromSource>>();

        map.append(&mut map_res);
    }

    if let Some(secret) = env.secret {
        let mut env_res = secret
            .into_iter()
            .map(|name | EnvFromSource {
                config_map_ref: None,
                secret_ref: Some(SecretEnvSource {
                    name: Some(name),
                    optional: None
                }),
                ..Default::default()
            })
            .collect::<Vec<EnvFromSource>>();
        
        map.append(&mut env_res);
    }

    map
}