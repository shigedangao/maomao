mod probe;
mod port;
mod env;

use toml::Value;
use toml::map::Map;
use crate::parser::util::{
    get_string_value,
    get_array_for_type
};

// (Option<String>, Option<Vec<T>>) -> either return a string mean that we make a reference to a patch. If not we make a reference to a description

#[derive(Debug)]
struct Container {
    name: String,
    image: String,
    ports: Option<Vec<port::PortMapping>>,
    env: Option<env::EnvMap>,
    probes: Option<ProbesMapping>
}

#[derive(Debug)]
struct ProbesMapping {
    liveness: Option<probe::Probe>,
    readiness: Option<probe::Probe>
}

impl Container {
    /// New
    ///
    /// # Description
    /// Create a new Container object
    ///
    /// # Arguments
    /// * `item` - &Value
    fn new(item: &Value) -> Result<Self, ()> {
        let name = match get_string_value(&item, "name") {
            Some(n) => n,
            None => return Err(())
        };

        let image = match get_string_value(&item, "image") {
            Some(img) => img,
            None => return Err(())
        };

        Ok(Container {
            name,
            image,
            ports: None,
            env: None,
            probes: None
        })
    }

    /// Set Env
    /// 
    /// # Description
    /// Set the environment variable into the Container
    fn set_env(mut self, item: &Value) -> Self {
        let env_item = match item.get("env") {
            Some(e) => e,
            None => return self
        };

        let items = match env_item.as_table() {
            Some(item) => item,
            None => return self
        };

        let env = env::EnvMap::new().finish(items);
        self.env = Some(env);
        self
    }
}


#[cfg(test)]
mod container_test {
    use toml::Value;
    use crate::parser::util::get_array_for_type;
    use super::port;
        
    #[test]
    fn test_retrieve_array() {
        let content = "
            ports = [
                { name = 'http', value = '$port' },
                { name = 'bar', value = 'mao' }
            ]
        ";

        let t_content = content.parse::<Value>().unwrap();
        let port_vec = get_array_for_type::<port::PortMapping>(&t_content, Some("ports"));

        assert!(port_vec.is_some());

        let ports = port_vec.unwrap();

        let http_port = ports.get(0).unwrap();
        assert_eq!(http_port.name, "http");
        assert_eq!(http_port.value, "$port");

        let bar_port = ports.get(1).unwrap();
        assert_eq!(bar_port.name, "bar");
        assert_eq!(bar_port.value, "mao");
    }

    #[test]
    fn test_retrieve_container() {
        let content = "
            [spec.containers.node]
                name = 'node'
                image = 'node:$tag'
                ports = [
                    { name = 'http', value = '$port' }
                ]

                [spec.containers.node.env]
                    map = [
                        # will make reference to a set of econfigEnvMap / secrets
                        'configmap::misc',
                        'configmap::api',
                        'secrets::foo'
                    ]
                    from = [
                        # will make a reference to a set of secrets variables
                        { kind = 'secret', name = 'google-api-key', from = 'google::main.key' }
                    ]
                    raw = [
                        # raw kubernetes env value
                        { name = 'greeting', value = 'bar' }
                    ]

        ";

        let toml_containers = content.parse::<Value>().unwrap();
        let spec = toml_containers.get("spec").unwrap().as_table().unwrap();
        let containers = spec.get("containers").unwrap().as_table().unwrap();
        let node = containers.get("node").unwrap();
        
        let container = super::Container::new(&node).unwrap();
        let cnv = container.set_env(&node);

        assert!(cnv.env.is_some());

        let env = cnv.env.unwrap();
        assert!(env.map.is_some());
        assert!(env.raw.is_some());
        assert!(env.from.is_some());

        let map = env.map.unwrap();
        assert_eq!(map.get(0).unwrap(), "configmap::misc");
        assert_eq!(map.get(1).unwrap(), "configmap::api");
        assert_eq!(map.get(2).unwrap(), "secrets::foo");

        let raw = env.raw.unwrap();
        assert_eq!(raw.get(0).unwrap().name, "greeting");
        assert_eq!(raw.get(0).unwrap().value, "bar");

        let from = env.from.unwrap();
        assert_eq!(from.get(0).unwrap().kind, super::env::EnvRefKind::SecretMap);
        assert_eq!(from.get(0).unwrap().name, "google-api-key");
        assert_eq!(from.get(0).unwrap().from, "google::main.key");
    }
}