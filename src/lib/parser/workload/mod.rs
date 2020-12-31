mod env;

use crate::lib::helper::error::LError;
use crate::lib::helper::toml::get_value_for_t;
use toml::Value;

// Constant
const WORKLOAD_NOT_EXIST: &str = "Workload does not exist. Make sure that [workload] is set on the template";
const WORKLOAD_MALFORMATTED: &str = "Workload is malformatted. Please check that workload is above it's children";

#[derive(Debug)]
pub struct Workload {
    containers: Vec<Container>
}

#[derive(Debug, Default)]
pub struct Image {
    repo: String,
    tag: String
}

#[derive(Debug, Default)]
pub struct Container {
    name: String,
    image: Image,
    env_from: Option<env::EnvFrom>,
    env: Option<env::Env>
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
    /// * `name` &String
    /// * `ast` &Value
    ///
    /// # Return
    /// Result<Self, LError>
    fn new(name: &String, ast: &Value) -> Result<Self, LError> {
        let image_repo = get_value_for_t::<String>(ast, "image")?;
        let image_tag = get_value_for_t::<String>(ast, "tag")?;

        Ok(Container {
            name: name.to_string(),
            image: Image {
                repo: image_repo,
                tag: image_tag
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
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn set_envs(mut self, ast: &Value) -> Self {
        match env::get_envs(ast) {
            Ok(res) => self.env = Some(res),
            // @TODO probably should log an error here ?
            //       see a logger on the CLI side
            Err(err) => {
                self.env = None
            }
        };

        match env::get_env_from(ast) {
            Ok(res) => self.env_from = Some(res),
            // @TODO probably should log an error here ?
            //       see a logger on the CLI side
            Err(err) => {
                self.env_from = None;
            }
        }

        self
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
    let workload = ast.get("workload").ok_or_else(|| LError {
        message: WORKLOAD_NOT_EXIST.to_owned()
    })?;

    let specs = workload.as_table().ok_or_else(|| LError {
        message: WORKLOAD_MALFORMATTED.to_owned()
    })?;

    let mut containers = Vec::new();
    // @TODO check if we can convert this into a more functional way
    for (name, items) in specs.into_iter() {
        let container = Container::new(name, items)?.set_envs(items);
        containers.push(container);
    }

    Ok(Workload {
        containers
    })
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

                [workload.rust]
                    image = 'foo'
                    tag = 'bar'
        ";

        let ast = template.parse::<Value>().unwrap();
        let workload = super::get_workload(&ast);

        assert!(workload.is_ok());

        let workload = workload.unwrap();
        let rust = workload.containers.get(0);
        assert!(rust.is_some());
        
        let container = rust.unwrap();
        assert_eq!(container.name, "rust");
        assert_eq!(container.image.repo, "foo");
        assert_eq!(container.image.tag, "bar");
    }

    #[test]
    fn expect_parse_env_workload() {
        let template = "
            kind = 'workload::deployment'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [workload]

                [workload.rust]
                    image = 'foo'
                    tag = 'bar'

                    [workload.rust.env]
                    from = [
                        { type = 'map', name = 'foo', item = 'lol' },
                        { type = 'res_field', name = 'rust-container', item = 'limits.cpu' }
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
        assert_eq!(from.kind.as_ref().unwrap(), "map");
        assert_eq!(from.name, "foo");
        assert_eq!(from.item, "lol");

        let raw = env.raw.get(0).unwrap();
        assert_eq!(raw.name, "A_VALUE");
        assert_eq!(raw.item, "bar");
    }

    #[test]
    fn expect_parse_env_from_workload() {
        let template = "
            kind = 'workload::deployment'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [workload]

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
}