use k8s_openapi::api::apps::v1::{
    DaemonSet,
    DaemonSetSpec
};
use crate::lib::parser::Object;
use crate::kube::common;
use crate::kube::helper::error::KubeError;
use super::container::pod;

struct DaemonSetWrapper {
    workload: DaemonSet
}

impl DaemonSetWrapper {
    /// New
    ///
    /// # Description
    /// Create a new DaemonSetWrapper which will help to create a k8s_openapi::api::apps::v1::DaemonSet workload
    ///
    /// # Arguments
    /// * `object` - &Object
    ///
    /// # Return
    /// Self
    fn new(object: &Object) -> Self {
        let workload = DaemonSet {
            metadata: common::get_metadata_from_object(&object),
            ..Default::default()
        };

        DaemonSetWrapper {
            workload
        }
    }

    /// Set Spec
    ///
    /// # Description
    /// Set the spec field on a DaemonSet workload by creating a DaemonSetSpec struct
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
            .ok_or_else(|| KubeError { message: common::error::MISSING_SPEC.to_owned() })?;

        let workload = parser_spec.workload?;
        let spec = DaemonSetSpec {
            selector: common::get_label_selector_from_object(&object),
            template: pod::get_pod_template_spec(workload, metadata),
            ..Default::default()
        };

        self.workload.spec = Some(spec);
        Ok(self)
    }
}

/// Get Daemonset From Object
///
/// # Description
/// Create a DaemonSet from a crate::lib::parser::Object and return the generated
/// YAML of a DaemonSet
///
/// # Arguments
/// * `object` - &Object
///
/// # Return
/// Result<String, KubeError>
pub fn get_daemonset_from_object(object: &Object) -> Result<String, KubeError> {
    let daemonset = DaemonSetWrapper::new(&object).set_spec(&object)?;
    let daemonset_string = serde_yaml::to_string(&daemonset.workload)?;

    Ok(daemonset_string)
}

#[cfg(test)]
mod tests {
    use crate::lib::parser::get_parsed_objects;

    #[test]
    fn expect_to_generate_daemonset() {
        let template = r#"
            # a daemonset
            kind = 'workload::daemonset'
            name = 'rusty'
            metadata = { name = 'rusty-elasticsearch', tier = 'monitoring' }
        
            # container name rust
            [workload]
                tolerations = [
                    { key = 'node-role.kubernetes.io/master', effect = 'NoSchedule' }
                ]
        
                [workload.rust]
                image = 'foo'
                tag = 'bar'
                policy = 'IfNotPresent'
        
                    # env from
                    [workload.rust.env_from]
                    map = [
                        'default_configmap'
                    ]
                    secret = [
                        'default_secret'
                    ]
        "#;

        let object = get_parsed_objects(template).unwrap();
        let daemonset = super::DaemonSetWrapper::new(&object).set_spec(&object);

        assert!(daemonset.is_ok());
        let daemonset = daemonset.unwrap();
        let spec = daemonset.workload.spec.unwrap();
        let pod_spec = spec.template.spec.unwrap();
        let tolerations = pod_spec.tolerations.unwrap();

        assert_eq!(tolerations.get(0).unwrap().key.to_owned().unwrap(), "node-role.kubernetes.io/master");
        assert_eq!(tolerations.get(0).unwrap().effect.to_owned().unwrap(), "NoSchedule");
    }

    #[test]
    fn expect_to_generate_yaml() {
        let template = r#"
            # a daemonset
            kind = 'workload::daemonset'
            name = 'rusty'
            metadata = { name = 'rusty-elasticsearch', tier = 'monitoring' }
        
            # container name rust
            [workload]
                tolerations = [
                    { key = 'node-role.kubernetes.io/master', effect = 'NoSchedule' }
                ]
        
                [workload.rust]
                image = 'foo'
                tag = 'bar'
                policy = 'IfNotPresent'
        
                    # env from
                    [workload.rust.env_from]
                    map = [
                        'default_configmap'
                    ]
                    secret = [
                        'default_secret'
                    ]
        "#;

        let object = get_parsed_objects(template).unwrap();
        let yaml = super::get_daemonset_from_object(&object);
        assert!(yaml.is_ok());

        println!("{}", yaml.unwrap());
    }
}