use toml::Value;
use crate::lib::helper::error::{
    LError,
    workload::Error
};
use crate::lib::helper::toml::{
    get_value_for_t,
    get_value_for_t_lax
};

pub mod env;
pub mod toleration;
pub mod volume;

#[derive(Debug, Clone, Default)]
pub struct Workload {
    pub replicas: Option<i32>,
    pub tolerations: Option<Vec<toleration::Toleration>>,
    pub containers: Vec<Container>,
}

#[derive(Debug, Default, Clone)]
pub struct Image {
    pub repo: String,
    pub tag: String,
    pub policy: Option<String>
}

#[derive(Debug, Default, Clone)]
pub struct Container {
    pub name: String,
    pub image: Image,
    pub env_from: Option<env::EnvFrom>,
    pub env: Option<env::Env>,
    pub volume_mounts: Option<Vec<volume::VolumeMount>>,
}

impl Container {
    /// New
    ///
    /// # Description
    /// Create a new container by filling with the basic info
    /// - name
    /// - image
    ///
    /// # Arguments
    /// * `name` &str
    /// * `ast` &Value
    ///
    /// # Return
    /// Result<Self, LError>
    fn new(name: &str, ast: &Value) -> Result<Self, LError> {
        let image_repo = get_value_for_t::<String>(ast, "image")?;
        let image_tag = get_value_for_t::<String>(ast, "tag")?;
        let policy = get_value_for_t_lax::<String>(ast, "policy");

        Ok(Container {
            name: name.to_string(),
            image: Image {
                repo: image_repo,
                tag: image_tag,
                policy
            },
            ..Default::default()
        })
    }

    /// Set Envs
    ///
    /// # Description
    /// Set the environment variables from the workload template
    /// We'll set:
    /// - env
    /// - envFrom
    /// See examples/deployment.toml to see how looks this field. Or refer to the unit test below
    /// /!\ If an error occurred we consider that it might be because the field is missing. Hence we do nothing when there is an error
    ///     The code will continue for the next case
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn set_envs(mut self, ast: &Value) -> Self {
        if let Ok(res) = env::get_envs(ast) {
            self.env = Some(res);
        }

        if let Ok(res) = env::get_env_from(ast) {
            self.env_from = Some(res);
        }

        self
    }

    /// Set Volume Mounts
    ///
    /// # Description
    /// Set volume mounts field
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn set_volumes_mounts(mut self, ast: &Value) -> Self {
        let volume = ast.get("volume_mounts");
        if volume.is_none() {
            return self;
        }

        if let Some(v) = volume.unwrap().as_array() {
            self.volume_mounts = volume::VolumeMount::from_toml_array(v);
        }

        self
    } 
}

impl Workload {
    /// New
    ///
    /// # Description
    /// Create a new Workload
    ///
    /// # Arguments
    /// * `ast` - &Value
    ///
    /// # Return
    /// Result<Self, LError>
    fn new(ast: &Value) -> Result<Self, LError> {
        let replicas = get_value_for_t_lax::<i32>(ast, "replicas");
        Ok(Workload {
            replicas,
            tolerations: toleration::Toleration::get_toleration_list(&ast),
            ..Default::default()
        })
    }

    /// Set Spec
    ///
    /// # Description
    /// * `mut self` - Self
    /// * `ast` - &Value
    ///
    /// # Return
    /// Result<Self, LError>
    fn set_spec(mut self, ast: &Value) -> Result<Self, LError> {
        let specs = ast.as_table().ok_or_else(|| LError::from(Error::WorkloadMalformatted))?;

        let mut containers = Vec::new();
        for (name, items) in specs.into_iter() {
            if items.is_table() {
                let container = Container::new(name, items)?
                    .set_envs(items)
                    .set_volumes_mounts(items);
                containers.push(container);
            }
        }
        
        self.containers = containers;

        Ok(self)
    }
}

/// Get Workload
///
/// # Description
/// Retrieve a workload from the TOML template
///
/// # Arguments
/// * `ast` - &Value
///
/// # Result
/// Result<Workload, LError>
pub fn get_workload(ast: &Value) -> Result<Workload, LError> {
    let workload = ast.get("workload")
        .ok_or_else(|| LError::from(Error::WorkloadNotExist))?;

    Workload::new(workload)?.set_spec(workload)
}

#[cfg(test)]
mod test {
    use toml::Value;

    #[test]
    fn expect_parse_basic_workload() {
        let template = "
            kind = 'workload::deployment'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [workload]
                replicas = 3

                [workload.rust]
                    image = 'foo'
                    tag = 'bar'
                    policy = 'IfNotPresent'
        ";

        let ast = template.parse::<Value>().unwrap();
        let workload = super::get_workload(&ast);

        assert!(workload.is_ok());

        let workload = workload.unwrap();
        assert_eq!(workload.replicas.unwrap(), 3);
        let rust = workload.containers.get(0);
        assert!(rust.is_some());
        
        let container = rust.unwrap();
        assert_eq!(container.name, "rust");
        assert_eq!(container.image.repo, "foo");
        assert_eq!(container.image.tag, "bar");
        assert_eq!(container.image.policy.as_ref().unwrap(), "IfNotPresent");
    }

    #[test]
    fn expect_parse_env_workload() {
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
                        { name = 'foo', item = 'lol' },
                        { from_field = 'res_field', name = 'rust-container', item = 'limits.cpu' }
                    ]
                    raw = [
                        { name = 'A_VALUE', item = 'bar' }
                    ]
        ";

        let ast = template.parse::<Value>().unwrap();
        let workload = super::get_workload(&ast);

        assert!(workload.is_ok());

        let workload = workload.unwrap();
        let rust = workload.containers.get(0);
        assert!(rust.is_some());
        
        let container = rust.unwrap();
        assert!(container.env.is_some());

        let env = container.env.as_ref().unwrap();
        assert!(!env.from.is_empty());
        assert!(!env.raw.is_empty());

        let from = env.from.get(0).unwrap();
        assert_eq!(from.name, "foo");
        assert_eq!(from.item.to_owned().unwrap(), "lol");

        let raw = env.raw.get(0).unwrap();
        assert_eq!(raw.name, "A_VALUE");
        assert_eq!(raw.item.to_owned().unwrap(), "bar");
    }

    #[test]
    fn expect_parse_env_from_workload() {
        let template = "
            kind = 'workload::deployment'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [workload]
                replicas = 3

                [workload.rust]
                    image = 'foo'
                    tag = 'bar'

                    [workload.rust.env_from]
                    map = [
                        'default_configmap'
                    ]
                    secret = [
                        'default_secret'
                    ]
        ";

        let ast = template.parse::<Value>().unwrap();
        let workload = super::get_workload(&ast);

        assert!(workload.is_ok());

        let workload = workload.unwrap();
        let rust = workload.containers.get(0);
        assert!(rust.is_some());
        
        let container = rust.unwrap();
        assert!(container.env_from.is_some());

        let env_from = container.env_from.as_ref().unwrap();
        assert!(!env_from.map.is_empty());
        assert!(!env_from.secret.is_empty());

        let map = env_from.map.get(0).unwrap();
        assert_eq!(map, "default_configmap");

        let secret = env_from.secret.get(0).unwrap();
        assert_eq!(secret, "default_secret");
    }

    #[test]
    fn expect_to_parse_tolerations() {
        let template = "
            kind = 'workload::daemonset'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [workload]
                replicas = 3

                tolerations = [
                    { key = 'node-role.kubernetes.io/master', effect = 'NoSchedule' }
                ]

                [workload.rust]
                    image = 'foo'
                    tag = 'bar'
        ";

        let ast = template.parse::<Value>().unwrap();
        let workload = super::get_workload(&ast);

        assert!(workload.is_ok());
        let tolerations = workload.unwrap().tolerations;
        assert!(tolerations.is_some());

        let tolerations = tolerations.unwrap();
        let first_key = tolerations.get(0).unwrap();
        assert_eq!(first_key.key.to_owned().unwrap(), "node-role.kubernetes.io/master");
        assert_eq!(first_key.effect.to_owned().unwrap(), "NoSchedule");
    }
}