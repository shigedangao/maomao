use k8s_openapi::api::core::v1::Pod;
use crate::lib::parser::Object;
use crate::kube::helper::error::{
    KubeError,
    common::Error
};
use crate::kube::common;
use super::container::pod;

struct PodWrapper {
    workload: Pod
}

impl PodWrapper {
    /// Create a new PodWrapper which will be used to create a Pod
    ///
    /// # Arguments
    ///
    /// * `object` - &Object
    fn new(object: &Object) -> Self {
        let workload = Pod {
            metadata: common::get_metadata_from_object(object),
            ..Default::default()
        };

        PodWrapper {
            workload
        }
    }

    /// Set the spec of a Pod by using the PodWrapper
    ///
    /// # Arguments
    ///
    /// * `mut self` - Self
    /// * `object` - &Object
    fn set_spec(mut self, object: &Object) -> Result<Self, KubeError> {
        let parser_spec = object.spec.to_owned()
            .ok_or_else(|| KubeError::from(Error::MissingSpec))?;

        if let Some(workload) = parser_spec.workload {
            let template_spec = pod::get_pod_template_spec(
                workload,
                object,
                self.workload.metadata.to_owned()
            );
            self.workload.spec = template_spec.spec;
        }

        Ok(self)
    }
}

/// Get a String representation of a Pod workload
///
/// # Arguments
///
/// * `object` - &Object
pub fn get_pod_from_object(object: &Object) -> Result<String, KubeError> {
    let po = PodWrapper::new(object).set_spec(object)?;
    let po_string = serde_yaml::to_string(&po.workload)?;

    Ok(po_string)
}

#[cfg(test)]
mod tests {
    use crate::lib::parser::get_parsed_objects;
    use super::*;

    #[test]
    fn expect_to_create_pod_object() {
        let template = r#"
            kind = 'workload::pod'
            name = 'rusty'
            version = 'apps/v1'
            metadata = { name = 'rusty', tier = 'backend' }
            namespace = 'foo'

            [workload]

                [workload.rust]
                    image = 'foo'
                    tag = 'bar'
        "#;

        let object = get_parsed_objects(template).unwrap();
        let pod = PodWrapper::new(&object).set_spec(&object);
        assert!(pod.is_ok());

        let po = pod.unwrap();
        let spec = po.workload.spec.unwrap();
        let container = spec.containers.get(0);
        assert!(container.is_some());

        let rust_container = container.unwrap();
        assert_eq!(rust_container.image.as_ref().unwrap(), "foo:bar");
    }

    #[test]
    fn expect_to_generate_pod_string() {
        let template = r#"
            kind = 'workload::pod'
            name = 'rusty'
            version = 'apps/v1'
            metadata = { name = 'rusty', tier = 'backend' }
            namespace = 'foo'

            [workload]

                [workload.rust]
                    image = 'foo'
                    tag = 'bar'
        "#;

        let object = get_parsed_objects(template).unwrap();
        let pod_string = get_pod_from_object(&object);
        assert!(pod_string.is_ok());

        let pod_string = pod_string.unwrap();
        assert!(!pod_string.is_empty());
    }
}