use std::convert::From;
use k8s_openapi::api::core::v1::{
    PodTemplateSpec,
    PodSpec,
    Container
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use crate::lib::parser::workload::{
    Workload,
    Container as ParserContainer
};

mod env;
mod env_from;

struct PodSpecWrapper {
    spec: PodSpec
}

/// Get Pod Template Spec
///
/// # Description
/// Create a k8s_openapi::api::core::v1::PodTemplateSpec
///
/// # Arguments
/// * `workload` - Workload
/// * `metadata` - ObjectMetadata
///
/// # Return
/// k8s_openapi::api::core::v1::PodTemplateSpec
pub fn get_pod_template_spec(workload: Workload, metadata: ObjectMeta) -> PodTemplateSpec {
    let mut template = PodTemplateSpec {
        metadata: Some(metadata),
        spec: None
    };

    let wrapper = PodSpecWrapper::new().set_containers(workload.containers);
    template.spec = Some(wrapper.spec);
    
    template
}

impl PodSpecWrapper {
    /// New
    ///
    /// # Description
    /// Create a new PodSpecWrapper. This wrapper is used as a Decorator to easily manipulate the PodTemplateSpec
    ///
    /// # Return
    /// Self
    fn new() -> Self {
        PodSpecWrapper {
            spec: PodSpec::default()
        }
    }

    /// Set Container
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `parser_containers` - Vec<ParserContainer>
    ///
    /// # Return
    /// Self
    fn set_containers(mut self, parser_containers: Vec<ParserContainer>) -> Self {
        let containers = parser_containers.into_iter()
            .map(|c| Container::from(c))
            .collect::<Vec<Container>>();

        self.spec.containers = containers;

        self
    }
}

impl From<ParserContainer> for Container {
    fn from(c: ParserContainer) -> Container {
        let mut container = Container {
            name: c.name,
            image: Some(format!("{}:{}", c.image.repo, c.image.tag)),
            image_pull_policy: c.image.policy,
            ..Default::default()
        };

        if let Some(env) = c.env {
            let mut from_env = env::get_env_vars(env.from);
            let mut raw_env = env::get_env_vars(env.raw);
            from_env.append(&mut raw_env);

            container.env = Some(from_env);
        }

        if let Some(env) = c.env_from {
            container.env_from = Some(env_from::get_env_source_from_envfrom(env));
        }

        container
    }
}