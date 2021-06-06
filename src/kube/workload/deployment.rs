use k8s_openapi::api::apps::v1::{
    Deployment,
    DeploymentSpec
};
use crate::lib::parser::Object;
use crate::kube::common;
use crate::kube::helper::error::{
    KubeError,
    common::Error
};
use super::container::pod;

struct DeploymentWrapper {
    workload: Deployment
}

impl DeploymentWrapper {
    /// New
    ///
    /// # Description
    /// Create a new DeploymentWrapper. It's used to create the Deployment struct
    /// which represent a Kubernetes Deployment
    ///
    /// # Arguments
    /// * `object` - &Object
    ///
    /// # Return
    /// Self
    fn new(object: &Object) -> Self {
        let workload = Deployment {
            metadata: common::get_metadata_from_object(&object),
            ..Default::default()
        };

        DeploymentWrapper {
            workload
        }
    }

    /// Set Spec
    ///
    /// # Description
    /// Set the spec of a Deployment by using the wrapper to operate the Deployment struct
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `object` - &Object
    ///
    /// # Return
    /// Result<Self, KubeError>
    fn set_spec(mut self, object: &Object) -> Result<Self, KubeError> {
        let metadata = common::get_workload_metadata_from_object(&object);
        let parser_spec = object.spec.to_owned()
            .ok_or_else(|| KubeError::from(Error::MissingSpec))?;

        if let Some(workload) = parser_spec.workload {
            let spec = DeploymentSpec {
                replicas: workload.replicas,
                selector: common::get_label_selector_from_object(&object),
                template: pod::get_pod_template_spec(workload, metadata),
                ..Default::default()
            };

            self.workload.spec = Some(spec);
        }

        if let Some(err) = parser_spec.error {
            return Err(KubeError::from(err));
        }

        Ok(self)
    }
}

/// Get Deployment From Object
///
/// # Description
/// Retrieve a Kubernetes deployment from a parser object
///
/// # Arguments
/// * `object` - &Object
///
/// # Return
/// Result<String, KubeError>
pub fn get_deployment_from_object(object: &Object) -> Result<String, KubeError> {
    let deployment = DeploymentWrapper::new(&object).set_spec(&object)?;
    let deployment_string = serde_yaml::to_string(&deployment.workload)?;

    Ok(deployment_string)
}

#[cfg(test)]
mod tests {
    use crate::lib::parser::get_parsed_objects;

    use super::DeploymentWrapper;

    #[test]
    fn create_deployment_from_object() {
        let template = r#"
            kind = 'workload::deployment'
            name = 'rusty'
            version = 'apps/v1'
            metadata = { name = 'rusty', tier = 'backend' }
            namespace = 'foo'

            [workload]
                replicas = 3

                [workload.rust]
                    image = 'foo'
                    tag = 'bar'
        "#;

        let object = get_parsed_objects(template).unwrap();
        let deployment = DeploymentWrapper::new(&object).set_spec(&object);
        assert!(deployment.is_ok());

        let workload = deployment.unwrap().workload;
        assert_eq!(workload.metadata.labels.unwrap().get("name").unwrap(), "rusty");
        assert_eq!(workload.metadata.namespace.unwrap(), "foo");
        assert!(workload.spec.is_some());

        let workload_spec = workload.spec.unwrap();
        assert_eq!(workload_spec.replicas.unwrap(), 3);

        let spec_metadata = workload_spec.template.metadata.unwrap();
        assert_eq!(spec_metadata.labels.unwrap().get("name").unwrap(), "rusty");

        let pod_spec = workload_spec.template.spec.unwrap();
        let container = pod_spec.containers.get(0);

        assert!(container.is_some());
        let rust = container.unwrap();

        assert_eq!(rust.image.to_owned().unwrap(), "foo:bar");
        assert!(rust.image_pull_policy.is_none());
        assert!(rust.env.is_none());
    }

    #[test]
    fn expect_to_parse_env() {
        let template = "
            kind = 'workload::deployment'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [workload]
                replicas = 3

                [workload.rust]
                    image = 'foo'
                    tag = 'bar'

                    [workload.rust.env]
                    from = [
                        { from_field = 'configMapRef', name = 'foo', item = 'lol' },
                        { from_field = 'resourceFieldRef', name = 'rust', item = 'limits.cpu' }
                    ]
                    raw = [
                        { name = 'A_VALUE', item = 'bar' }
                    ]
        ";

        let object = get_parsed_objects(template).unwrap();
        let deployment = DeploymentWrapper::new(&object).set_spec(&object);
        assert!(deployment.is_ok());

        let workload = deployment.unwrap().workload.spec.unwrap();
        let pod = workload.template.spec.unwrap();
        let container = pod.containers.get(0).unwrap();

        assert_eq!(container.name, "rust");
        let env = container.env.to_owned().unwrap();
        let from_configmap = env.get(0).unwrap();
        assert_eq!(from_configmap.name, "foo");
        assert!(from_configmap.value.is_none());

        let value_from = from_configmap.value_from.to_owned().unwrap();
        let configmap_value = value_from.config_map_key_ref.unwrap();

        assert_eq!(configmap_value.name.unwrap(), "foo");
        assert_eq!(configmap_value.key, "lol");

        // getting just a raw value (2 index (3))
        let raw = env.get(2).unwrap();
        assert_eq!(raw.name, "A_VALUE");
        assert_eq!(raw.value.to_owned().unwrap(), "bar");
    }

    #[test]
    fn expect_to_parse_env_from() {
        let template = "
            kind = 'workload::deployment'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [workload]
                replicas = 3

                [workload.node]
                    image = 'foo'
                    tag = 'bar'

                    [workload.node.env_from]
                    map = [
                        'default_configmap'
                    ]
                    secret = [
                        'default_secret'
                    ]
        ";

        let object = get_parsed_objects(template).unwrap();
        let deployment = DeploymentWrapper::new(&object).set_spec(&object);
        assert!(deployment.is_ok());

        let workload = deployment.unwrap().workload.spec.unwrap();
        let pod = workload.template.spec.unwrap();
        let container = pod.containers.get(0).unwrap();

        assert_eq!(container.name, "node");
        let env_from = container.env_from.to_owned().unwrap();
        let map = env_from.get(0).unwrap();
        assert!(map.config_map_ref.is_some());
        assert!(map.secret_ref.is_none());
        assert_eq!(map.config_map_ref.to_owned().unwrap().name.unwrap(), "default_configmap");

        let secret = env_from.get(1).unwrap();
        assert!(secret.config_map_ref.is_none());
        assert!(secret.secret_ref.is_some());
        assert_eq!(secret.secret_ref.to_owned().unwrap().name.unwrap(), "default_secret");
    }

    #[test]
    fn expect_to_generate_yaml() {
        let template = "
            kind = 'workload::deployment'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }


            [annotations]
            'external-dns.alpha.kubernetes.io/hostname' = 'rusty.dev.org.'


            [workload]
                replicas = 3

                [workload.node]
                    image = 'foo'
                    tag = 'bar'

                    [workload.node.env]
                    from = [
                        { from_field = 'configMapRef', name = 'foo', item = 'lol' },
                        { from_field = 'resourceFieldRef', name = 'rust', item = 'limits.cpu' }
                    ]
                    raw = [
                        { name = 'A_VALUE', item = 'bar' }
                    ]

                    [workload.node.env_from]
                    map = [
                        'default_configmap'
                    ]
                    secret = [
                        'default_secret'
                    ]
        ";

        let object = get_parsed_objects(template).unwrap();
        let res = super::get_deployment_from_object(&object);
        assert!(res.is_ok());

        println!("{}", res.unwrap());
    }
}