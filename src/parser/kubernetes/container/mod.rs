mod probe;
mod port;
mod env;

use toml::Value;
use crate::helper::err::LibError;
use crate::parser::utils::helper::get_string_value;

// Constant
const MISSING_PARAMETER: &str = "Missing parameter in the toml template file";

#[derive(Debug)]
pub struct Container {
    pub name: String,
    pub image: String,
    pub ports: Option<Vec<port::PortMapping>>,
    pub env: Option<env::EnvMap>,
    pub probes: Option<ProbesMapping>
}

#[derive(Debug, Default)]
pub struct ProbesMapping {
    pub liveness: Option<probe::Probe>,
    pub readiness: Option<probe::Probe>
}

impl Container {
    /// New
    ///
    /// # Description
    /// Create a new Container object
    ///
    /// # Arguments
    /// * `item` - &Value
    pub fn new(item: &Value) -> Result<Self, LibError> {
        let name = match get_string_value(&item, "name") {
            Some(n) => n,
            None => return Err(LibError {
                kind: MISSING_PARAMETER.to_owned(),
                message: "missing parameter: `name`".to_owned()
            })
        };

        let image = match get_string_value(&item, "image") {
            Some(img) => img,
            None => return Err(LibError {
                kind: MISSING_PARAMETER.to_owned(),
                message: "missing parameter `image`".to_owned()
            })
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
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `item` - &Value
    pub fn set_env(mut self, item: &Value) -> Self {
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

    /// Set Probes
    ///
    /// # Description
    /// Retrieve the probes sections
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `item` - &Value
    pub fn set_probes(mut self, item: &Value) -> Self {
        let probe_item = match item.get("probes") {
            Some(it) => it,
            None => return self
        };

        // create default probe mapping
        let mut probe_mapping = ProbesMapping::default();
        match probe_item.get("liveness") {
            Some(it) => {
                let probe = probe::Probe::new().set_probe_type(it).finish(it);
                probe_mapping.liveness = Some(probe);
            },
            None => {}
        };

        match probe_item.get("readiness") {
            Some(it) => {
                let probe = probe::Probe::new().set_probe_type(it).finish(it);
                probe_mapping.readiness = Some(probe);
            },
            None => {}
        }

        self.probes = Some(probe_mapping);
        self
    }
}


#[cfg(test)]
mod container_test {
    use toml::Value;
    use crate::parser::utils::helper::get_array_for_type;
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

    #[test]
    fn test_retrieve_probes() {
        let content = "
            [spec.containers.node]
                name = 'node'
                image = 'node:$tag'
                ports = [
                    { name = 'http', value = '$port' }
                ]
        
                [spec.containers.node.probes]
                    [spec.containers.node.probes.liveness]
                        kind = 'http'
                        path = 'foo'
                        port = 8080                
                        http_headers = [
                            { name = 'baz', value = 'wow' },
                            { name = 'yo', value = 'sabai' }
                        ]            
        ";

        let toml_containers = content.parse::<Value>().unwrap();
        let spec = toml_containers.get("spec").unwrap().as_table().unwrap();
        let containers = spec.get("containers").unwrap().as_table().unwrap();
        let node = containers.get("node").unwrap();
        
        let container = super::Container::new(&node).unwrap();
        
        let probes = container.set_probes(&node);
        assert!(probes.probes.is_some());

        // get probe for node container
        let node_probes = probes.probes.unwrap();
        assert!(node_probes.liveness.is_some());
        
        // get liveness probe for node container
        let liveness = node_probes.liveness.unwrap();
        assert!(liveness.http_get.is_some());

        // test liveness http probe
        let liveness_http_get = liveness.http_get.unwrap();
        assert_eq!(liveness_http_get.path, "foo");
        assert_eq!(liveness_http_get.port, 8080);

        // test http headers
        let liveness_http_headers = liveness_http_get.http_headers.unwrap();
        assert_eq!(liveness_http_headers.get(0).unwrap().name, "baz");
        assert_eq!(liveness_http_headers.get(0).unwrap().value, "wow");

        // check readiness does not contain anything
        assert!(node_probes.readiness.is_none());
    }
}