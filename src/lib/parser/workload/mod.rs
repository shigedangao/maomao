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
        let container = Container::new(name, items)?;
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
}