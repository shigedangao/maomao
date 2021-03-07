use k8s_openapi::api::apps::v1::{
    StatefulSet,
    StatefulSetSpec
};
use crate::lib::parser::Object;
use crate::kube::common;
use crate::kube::helper::error::KubeError;
use super::container::pod;
use super::volumes::claim;

#[derive(Debug)]
struct StatefulSetWrapper {
    workload: StatefulSet
}

impl StatefulSetWrapper {
    /// New
    ///
    /// # Description
    /// Create a new StatefulSetWrapper which will handle operation on the StatefulSet struct
    ///
    /// # Arguments
    /// * `object` - &Object
    ///
    /// # Return
    /// Self
    fn new(object: &Object) -> Self {
        let workload = StatefulSet {
            metadata: common::get_metadata_from_object(&object),
            ..Default::default()
        };

        StatefulSetWrapper {
            workload
        }
    }

    /// Set Spec
    ///
    /// # Description
    /// Set the spec of a StatefulSet object
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `object` - &Object
    ///
    /// # Return
    /// Result<Self, KubeError>
    fn set_spec(mut self, object: &Object) -> Result<Self, KubeError> {
        let metadata = common::get_metadata_from_object(&object);
        let parser_spec = object.spec.to_owned()
            .ok_or_else(|| KubeError { message: common::error::MISSING_SPEC.to_owned() })?;

        let workload = parser_spec.workload?;
        let spec = StatefulSetSpec {
            selector: common::get_label_selector_from_object(&object),
            template: pod::get_pod_template_spec(workload, metadata),
            volume_claim_templates: claim::get_pvc_list(&object),
            ..Default::default()
        };

        self.workload.spec = Some(spec);

        Ok(self)
    }
}

/// Get StatefulSet From Object
///
/// # Description
/// Create a StatefulSet from a crate::lib::parser::Object  & return a YAML representation of the statefulset
///
/// # Arguments
/// * `object` - &Object
///
/// # Return
/// Result<String, KubeError>
pub fn get_statefulset_from_object(object: &Object) -> Result<String, KubeError> {
    let statefulset = StatefulSetWrapper::new(&object).set_spec(&object)?;
    let statefulset_string = serde_yaml::to_string(&statefulset.workload)?;

    Ok(statefulset_string)
}

#[cfg(test)]
mod tests {
    use crate::lib::parser::get_parsed_objects;

    #[test]
    fn expect_to_generate_statefulset() {
        let template = r#"
            kind = "workload::statefulset"
            name = "rusty"
            metadata = { name = "rusty", tier = "backend" }
        
            [volume_claims]
                [volume_claims.rust]
                    access_modes = []
                    data_source = [
                        { name = "kind", value = "VolumeSnapshot" },
                        { name = "name", value = "source" }
                    ]
                    resources_limit = [
                        { name = "key", value = "" }
                    ]
                    resource_request = [
                        { name = "key", value = "" }
                    ]
          
            # container name rust
            [workload]
                replicas = 3

                [workload.rust]
                image = "foo"
                tag = "bar"
                policy = "IfNotPresent"
                # name must match the table of the volume_claims
                volume_mounts = [
                    { name = "rust", mount_path = "" }
                ]        
        "#;

        let object = get_parsed_objects(template).unwrap();
        let statefulset = super::StatefulSetWrapper::new(&object).set_spec(&object);

        assert!(statefulset.is_ok());
        let statefulset = statefulset.unwrap();

        let spec = statefulset.workload.spec.unwrap();
        assert!(spec.volume_claim_templates.is_some());
        let claims = spec.volume_claim_templates.unwrap();
        let rust = claims.get(0).unwrap();
        let rust_spec = rust.spec.to_owned().unwrap();
        let datasource = rust_spec.data_source.unwrap();

        assert_eq!(datasource.kind, "VolumeSnapshot");
        assert_eq!(datasource.name, "source");
    }
}