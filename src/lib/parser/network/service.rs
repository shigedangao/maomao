use toml::Value;
use std::convert::From;
use std::collections::BTreeMap;
use crate::lib::helper::error::LError;
use crate::lib::helper::toml::get_value_for_t;

#[derive(Debug)]
pub struct Service {
    // We don't do a check on the kind here
    // This will be done by an other module
    pub kind: String,
    pub ports: Option<BTreeMap<String, Port>>
}

#[derive(Debug)]
pub struct Port {
    protocol: String,
    port: i64,
    target_port: i64
}

impl Service {
    /// New
    ///
    /// # Description
    /// Create a new Service struct
    ///
    /// # Arguments
    /// * `ast` - &Value
    ///
    /// # Return
    /// Result<Self, LError>
    fn new(ast: &Value) -> Result<Self, LError> {
        let kind = get_value_for_t::<String>(ast, "type")?;

        Ok(Service {
            kind,
            ports: None
        })
    }

    /// Set Ports
    ///
    /// # Description
    /// Get the ports struct and set it to the Service struct
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `ast` - &Value
    fn set_ports(mut self, ast: &Value) -> Self {
        let ports_field = ast.get("ports");
        if let Some(p) = ports_field {
            self.ports = get_ports(p);
        }

        self
    }
}

impl From<Value> for Port {
    fn from(ast: Value) -> Self {
        let protocol = get_value_for_t::<String>(&ast, "protocol").unwrap_or(String::new());
        let port = get_value_for_t::<i64>(&ast, "port").unwrap_or(0);
        let target_port = get_value_for_t::<i64>(&ast, "target_port").unwrap_or(0);

        Port {
            protocol,
            port,
            target_port
        }
    }
}

/// Get Ports
///
/// # Description
/// Retrieve port field
/// The definition of the toml template should be
///
/// [service]
///
///   [service.ports]
///
///     [service.ports.<xx>]
fn get_ports(past: &Value) -> Option<BTreeMap<String, Port>> {
    let map = past.as_table();
    if map.is_none() {
        return None;
    }

    let map = map.unwrap();
    let btree = map
        .into_iter()
        .map(|(k, v)| {
            (k.to_owned(), Port::from(v.to_owned()))
        })
        .collect::<BTreeMap<String, Port>>();

    Some(btree)
}

/// Get Service
///
/// # Description
/// Get the service struct
///
/// # Arguments
/// * `ast` - &Value
///
/// # Return
/// Result<Service, LError>
pub fn get_service(ast: &Value) -> Result<Service, LError> {
    let service = Service::new(ast)?.set_ports(ast);

    Ok(service)
}

#[cfg(test)]
mod test {
    use toml::Value;

    use super::get_service;
    
    #[test]
    fn expect_to_parse_service_type() {
        let template = "
            [service]
                type = 'nodeport'
        ";

        let ast = template.parse::<Value>().unwrap();
        let service_ast = ast.get("service").unwrap();

        let service = get_service(&service_ast);
        assert!(service.is_ok());

        assert_eq!(service.unwrap().kind, "nodeport");
    }

    #[test]
    fn expect_to_parse_service_ports_type() {
        let template = "
            [service]
                type = 'nodeport'

                [service.ports]
                    [service.ports.http]
                        protocol = 'TCP'
                        port = 80
                        target_port = 90
        ";

        let ast = template.parse::<Value>().unwrap();
        let service_ast = ast.get("service").unwrap();

        let service = get_service(&service_ast).unwrap();
        let ports = service.ports.unwrap();

        assert!(ports.get("http").is_some());
        let http = ports.get("http").unwrap();

        assert_eq!(http.protocol, "TCP");
        assert_eq!(http.port, 80);
        assert_eq!(http.target_port, 90);
    }

    #[test]
    fn expect_to_not_parse_service() {
        let template = "
            [service]
        ";

        let ast = template.parse::<Value>().unwrap();
        let service_ast = ast.get("service").unwrap();

        let service = get_service(&service_ast);
        assert!(service.is_err());
    }
}