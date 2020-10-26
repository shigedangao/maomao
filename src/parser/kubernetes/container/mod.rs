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

struct Container {
    name: String,
    image: String,
    ports: (Option<String>, Option<Vec<port::PortMapping>>),
    config_map: (Option<String>, Option<Vec<String>>),
    secrets: (Option<String>, Option<Vec<String>>),
    probes: ProbesMapping
}

struct ProbesMapping {
    liveness: (Option<String>, Option<probe::Probe>),
    readiness: (Option<String>, Option<probe::Probe>)
}

/// Parse Container Spec
///
/// # Description
/// Parse a kubernetes container spec. Note we don't use Serde to deserialize the struct
///
/// # Arguments
/// * `t_content` - Contant of the [spec.containers.<name>] toml value
fn parse_container_spec(t_content: &Value) {
    let name = get_string_value(&t_content, "name");
    let image = get_string_value(&t_content, "image");

    let ports = get_array_for_type::<port::PortMapping>(&t_content, "ports");
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
        let port_vec = get_array_for_type::<port::PortMapping>(&t_content, "ports");

        assert!(port_vec.is_some());

        let ports = port_vec.unwrap();

        let http_port = ports.get(0).unwrap();
        assert_eq!(http_port.name, "http");
        assert_eq!(http_port.value, "$port");

        let bar_port = ports.get(1).unwrap();
        assert_eq!(bar_port.name, "bar");
        assert_eq!(bar_port.value, "mao");
    }
}