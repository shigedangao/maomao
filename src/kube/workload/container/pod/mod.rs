use std::convert::From;
use k8s_openapi::api::core::v1::{
    PodTemplateSpec,
    PodSpec,
    Container,
    Toleration,
    VolumeMount
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use crate::lib::parser::workload::{
    Workload,
    Container as ParserContainer,
    toleration::Toleration as ParserToleration,
    volume::VolumeMount as ParserVolumeMount
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

    let wrapper = PodSpecWrapper::new()
        .set_containers(workload.containers)
        .set_tolerations(workload.tolerations);

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

    /// Set Tolerations
    ///
    /// # Description
    /// Set the tolerations to the PodSpec
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `parser_tolerations` - Vec<ParserToleration>
    ///
    /// # Return
    /// Self
    fn set_tolerations(mut self, parser_tolerations: Option<Vec<ParserToleration>>) -> Self {
        if parser_tolerations.is_none() {
            return self;
        }

        let tolerations = parser_tolerations.unwrap()
            .into_iter()
            .map(|t | Toleration::from(t))
            .collect::<Vec<Toleration>>();

        self.spec.tolerations = Some(tolerations);

        self
    }
}

impl From<ParserContainer> for Container {
    fn from(c: ParserContainer) -> Self {
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

        if let Some(volume_mounts) = c.volume_mounts {
            let mounts = volume_mounts
                .into_iter()
                .map(|m| VolumeMount::from(m))
                .collect::<Vec<VolumeMount>>();

            container.volume_mounts = Some(mounts);
        }

        container
    }
}

impl From<ParserToleration> for Toleration {
    fn from(t: ParserToleration) -> Self {
        Toleration {
            effect: t.effect,
            key: t.key,
            operator: t.operator,
            toleration_seconds: t.toleration_seconds,
            value: t.value
        }
    }
}

impl From<ParserVolumeMount> for VolumeMount {
    fn from(t: ParserVolumeMount) -> Self {
        VolumeMount {
            name: t.name.unwrap_or("".to_owned()),
            mount_path: t.path.unwrap_or("".to_owned()),
            read_only: t.read_only,
            ..Default::default()
        }
    }
}