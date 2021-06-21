use std::convert::Into;
use std::collections::BTreeMap;
use k8s_openapi::api::core::v1::{
    Container,
    VolumeMount,
    ResourceRequirements
};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use crate::lib::parser::workload::{
    Container as ParserContainer,
    resource::Resource as ParserResource
};
use super::{
    env,
    env_from
};

pub struct ContainerWrapper {
    pub container: Container
}

impl ContainerWrapper {
    /// Create a Container with basic data
    ///
    /// # Arguments
    ///
    /// * `c` - &ParserContainer
    pub fn new(c: &ParserContainer) -> Self {
        let container = Container {
            name: c.name.to_owned(),
            image: Some(format!("{}:{}", c.image.repo, c.image.tag)),
            image_pull_policy: c.image.policy.to_owned(),
            ..Default::default()
        };

        ContainerWrapper {
            container
        }
    }

    /// Set container.env or container.envFrom
    ///
    /// # Arguments
    ///
    /// * `mut self` - Self
    /// * `c` - &ParserContainer
    pub fn set_env(mut self, c: &ParserContainer) -> Self {
        if let Some(env) = c.env.to_owned() {
            let mut from_env = env::get_env_vars(env.from);
            let mut raw_env = env::get_env_vars(env.raw);
            from_env.append(&mut raw_env);

            self.container.env = from_env;
        }

        if let Some(env) = c.env_from.to_owned() {
            self.container.env_from = env_from::get_env_source_from_envfrom(env);
        }

        self
    }

    /// Set container.volumeMounts
    ///
    /// # Arguments
    ///
    /// * `mut self` - Self
    /// * `c` - &ParserContainer
    pub fn set_volumes(mut self, c: &ParserContainer) -> Self {
        if let Some(volume_mounts) = c.volume_mounts.to_owned() {
            let mounts = volume_mounts
                .into_iter()
                .map(VolumeMount::from)
                .collect::<Vec<VolumeMount>>();

            self.container.volume_mounts = mounts;
        }

        self
    }

    /// Set container.resources
    ///
    /// # Arguments
    ///
    /// * `mut self` - Self
    /// * `c` -  &ParserContainer
    pub fn set_resources(mut self, c: &ParserContainer) -> Self {
        if let Some(res) = c.resources.to_owned() {
            let mut res_req = ResourceRequirements::default();
            if let Some(lim) = res.limits {
                res_req.limits = lim.into();
            }

            if let Some(req) = res.requests {
                res_req.requests = req.into();
            }

            self.container.resources = Some(res_req);
        }

        self
    }
}

impl Into<BTreeMap<String, Quantity>> for ParserResource {
    fn into(self) -> BTreeMap<String, Quantity> {
        let mut map = BTreeMap::new();

        map.insert("cpu".to_owned(), Quantity(self.cpu.unwrap_or_default()));
        map.insert("memory".to_owned(), Quantity(self.memory.unwrap_or_default()));

        map
    }
}