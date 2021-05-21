use k8s_openapi::api::core::v1::ConfigMap;
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::ByteString;
use std::collections::BTreeMap;
use crate::lib::parser::Object;
use crate::kube::helper::error::{
    KubeError,
    common::Error
};
use crate::kube::common;

#[derive(Debug, Default)]
struct EnvWrapper {
    configmap: Option<ConfigMap>,
    secret: Option<Secret>
}

impl EnvWrapper {
    fn new() -> Self {
        EnvWrapper {
            ..Default::default()
        }
    }

    /// Set Configmap
    ///
    /// # Description
    /// Create a configmap
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `object` - &Object
    ///
    /// # Return
    /// Result<Self, KubeError>
    fn set_configmap(mut self, object: &Object) -> Result<Self, KubeError> {
        let mut configmap = ConfigMap {
            metadata: common::get_metadata_from_object(object),
            ..Default::default()
        };

        if object.spec.is_none() {
            return Err(KubeError::from(Error::MissingSpec));
        }

        let spec = object.to_owned().spec.unwrap();
        if spec.env.is_none() {
            return Err(KubeError::from(Error::MissingSpec));
        }

        let env = spec.env.unwrap();
        if let Some(d) = env.data {
            if env.binary {
                let bin: BTreeMap<String, ByteString> = d
                    .into_iter()
                    .map(|(k, v)| (k, ByteString(v.as_bytes().to_vec())))
                    .collect();

                configmap.binary_data = Some(bin);
            } else {
                configmap.data = Some(d);
            }
        }

        self.configmap = Some(configmap);

        Ok(self)
    }

    /// Set Secret
    ///
    /// # Description
    /// Create a secret
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `object` - &Object
    ///
    /// # Return
    /// Result<Self, KubeError>
    fn set_secret(mut self, object: &Object) -> Result<Self, KubeError> {
        let mut secret = Secret {
            metadata: common::get_metadata_from_object(object),
            ..Default::default()
        };

        if object.spec.is_none() {
            return Err(KubeError::from(Error::MissingSpec));
        }

        let spec = object.to_owned().spec.unwrap();
        if spec.env.is_none() {
            return Err(KubeError::from(Error::MissingSpec));
        }

        let env = spec.env.unwrap();
        if let Some(d) = env.data.to_owned() {
            let bin: BTreeMap<String, ByteString> = d
                .into_iter()
                .map(|(k, v)| (k, ByteString(v.as_bytes().to_vec())))
                .collect();

            secret.data = Some(bin);
        }

        self.secret = Some(secret);

        Ok(self)
    }
}

/// Get Env From Object
///
/// # Description
/// Generate either configmap or secret
///
/// # Arguments
/// * `object` - Object
/// * `kind` - String
///
/// # Return
/// Result<String, KubeError>
pub fn get_env_from_object(object: Object, kind: String) -> Result<String, KubeError> {
    let env_str = match kind.as_str() {
        "map" => {
            let env = EnvWrapper::new().set_configmap(&object)?;
            serde_yaml::to_string(&env.configmap)?
        },
        "secret" => {
            let env = EnvWrapper::new().set_secret(&object)?;
            serde_yaml::to_string(&env.secret)?
        },
        _ => "".to_owned()
    };

    Ok(env_str)
}

#[cfg(test)]
mod tests {
    use crate::lib::parser::get_parsed_objects;

    #[test]
    fn expect_to_create_configmap() {
        let template = r#"
        kind = "env::map"
        name = "nginx-configmap"
        metadata = { name = "nginx-configmap", tier = "backend" }
        
        [data]
            foo = "bbtea"
            lol = """
                bobba=10
            """
        "#;

        let object = get_parsed_objects(template).unwrap();
        // create the wrapper here
        let env = super::EnvWrapper::new().set_configmap(&object);
        assert!(env.is_ok());

        let env = env.unwrap();
        assert!(env.configmap.is_some());

        let configmap = env.configmap.unwrap();
        assert!(configmap.data.is_some());
        assert!(configmap.binary_data.is_none());
    }

    #[test]
    fn expect_to_create_configmap_binary_data() {
        let template = r#"
        kind = "env::map"
        name = "nginx-configmap"
        metadata = { name = "nginx-configmap", tier = "backend" }
        
        binary = true
        [data]
            foo = "bbtea"
            lol = """
                bobba=10
            """
        "#;

        let object = get_parsed_objects(template).unwrap();
        // create the wrapper here
        let env = super::EnvWrapper::new().set_configmap(&object);
        assert!(env.is_ok());

        let env = env.unwrap();
        assert!(env.configmap.is_some());

        let configmap = env.configmap.unwrap();
        assert!(configmap.data.is_none());
        assert!(configmap.binary_data.is_some());
    }

    #[test]
    fn expect_to_create_secret() {
        let template = r#"
        kind = "env::secret"
        name = "nginx-secret"
        metadata = { name = "nginx-secret", tier = "backend" }
        
        [data]
            foo = "bbtea"
            lol = """
                bobba=10
            """
        "#;

        let object = get_parsed_objects(template).unwrap();
        // create the wrapper here
        let env = super::EnvWrapper::new().set_secret(&object);
        assert!(env.is_ok());

        let env = env.unwrap();
        assert!(env.secret.is_some());

        let secret = env.secret.unwrap();
        assert!(secret.data.is_some());
    }

    #[test]
    fn expect_to_generate_configmap_string() {
        let template = r#"
        kind = "env::map"
        name = "nginx-configmap"
        metadata = { name = "nginx-configmap", tier = "backend" }
        
        binary = true
        [data]
            foo = "bbtea"
            lol = """
                bobba=10
            """
        "#;

        let object = get_parsed_objects(template).unwrap();
        let res = super::get_env_from_object(object, "map".to_owned());
        assert!(res.is_ok());
    }

    #[test]
    fn expect_to_generate_secret_string() {
        let template = r#"
        kind = "env::secret"
        name = "nginx-secret"
        metadata = { name = "nginx-secret", tier = "backend" }
        
        [data]
            foo = "bbtea"
            lol = """
                bobba=10
            """
        "#;

        let object = get_parsed_objects(template).unwrap();
        let res = super::get_env_from_object(object, "secret".to_owned());
        assert!(res.is_ok());
    }
}