use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use toml::Value;
use serde::Serialize;
use crate::lib::parser::Object;
use crate::kube::common;
use super::helper::error::KubeError;

#[derive(Debug, Clone, Default, Serialize)]
struct CustomCrd {
    kind: String,
    metadata: ObjectMeta,
    version: Option<String>,
    spec: Option<Value>
}

impl CustomCrd {
    /// New
    ///
    /// # Description
    /// Create a new CustomCrd
    ///
    /// # Arguments
    /// * `object` - &Object
    /// * `kind` - String
    ///
    /// # Return
    /// Self
    fn new(object: &Object, kind: String) -> Self {
        CustomCrd {
            kind,
            metadata: common::get_metadata_from_object(&object),
            version: object.version.clone(),
            spec: None
        }
    }

    /// Set Spec
    ///
    /// # Description
    /// Set the spec of a CustomCRD
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `object` - &Object
    ///
    /// # Return
    /// Result<Self, KubeError>
    fn set_spec(mut self, object: &Object) -> Result<Self, KubeError> {
        if let Some(spec) = object.spec.to_owned() {
            if let Some(crd) = spec.crd {
                self.spec = crd.spec;

                return Ok(self);
            }

            if let Some(err) = spec.error {
                return Err(KubeError::from(err));
            }
        }

        Ok(self)
    }
}

/// Crd To Yaml
///
/// # Description
/// Create a custom crd from toml to yaml
///
/// # Arguments
/// * `object` - Object
/// * `kind` - String
///
/// # Return
/// Result<String, KubeError>
pub fn crd_to_yaml(object: Object, kind: String) -> Result<String, KubeError> {
    let crd = CustomCrd::new(&object, kind).set_spec(&object)?;
    let crd_string = serde_yaml::to_string(&crd)?;

    Ok(crd_string)
}

#[cfg(test)]
mod test {
    use crate::lib::parser::get_parsed_objects;
    use super::CustomCrd;

    #[test]
    fn expect_to_generate_argo_yaml_crd() {
        let template = r#"
        kind = "custom::Workflow"
        version = "argoproj.io/v1alpha1"
        metadata = { generatedName = "steps-" }
        
        [spec]
            entrypoint = "hello"
            [[spec.templates]]
                # parameter such as name are given in the toml table
                name = "hello"
                [[spec.templates.steps]]
                    name = "hello world"
                    template = "whalesay"
                    [spec.templates.steps.arguments]
                        parameters = [
                            { name = "message", value = "hello1" }
                        ]

                [[spec.templates.steps]]
                    name = "hello bar"
                    template = "whalesay"

                    [spec.templates.steps.arguments]
                        parameters = [
                            { name = "message", value = "hello1" }
                        ]
                        
            [[spec.templates]]
                name = "whalesay"
                [spec.templates.inputs]
                    parameters = [
                        { name = "message" }
                    ]
                [spec.templates.container]
                    image = "docker/whalesay"
                    command = ["cowsay"]
                    args = ["{{inputs.parameters.message}}"]
        "#;

        let object = get_parsed_objects(template);
        assert!(object.is_ok());

        let object = object.unwrap();
        let res = super::crd_to_yaml(object, "Workflow".to_owned());
        assert!(res.is_ok());
    }

    #[test]
    fn expect_to_generate_mcrt() {
        let template = r#"
        kind = "custom::ManagedCertificate"
        version = "networking.gke.io/v1"
        metadata = { name = "rusty-certificate" }

        [spec]
            domains = [
                "rusty-dev.co.kr",
                "rusty-dyn.co.kr"
            ]
        "#;

        let object = get_parsed_objects(template);
        assert!(object.is_ok());

        let object = object.unwrap();
        let res = CustomCrd::new(&object, "ManagedCertificate".to_owned()).set_spec(&object);
        assert!(res.is_ok());

        let crd = res.unwrap();
        assert_eq!(crd.kind, "ManagedCertificate");
        assert_eq!(crd.version.unwrap(), "networking.gke.io/v1");
        assert_eq!(crd.metadata.name.unwrap(), "rusty-certificate");
        assert!(crd.spec.is_some());

        let yaml = super::crd_to_yaml(object, "ManagedCertificate".to_owned());
        assert!(yaml.is_ok());
    }
}