mod service;

use toml::Value;
use crate::lib::helper::error::LError;

#[derive(Debug, Default)]
pub struct Network {
    service: Option<service::Service>
}

impl Network {
    /// New
    ///
    /// # Description
    /// Create a new Network struct
    ///
    /// # Return
    /// Self
    fn new() -> Self {
        Network {
            ..Default::default()
        }
    }

    /// Set Service
    ///
    /// # Description
    /// Set the service struct field. It has the following toml definition
    ///
    ///  <root>
    /// [service]
    ///
    /// # Arguments
    /// `mut self` - Self
    /// `ast` - Option<&Value>
    ///
    /// # Return
    /// Self
    fn set_service(mut self, ast: Option<&Value>) -> Self {
        self
    }

    /// Set Ingress
    ///
    /// # Description
    /// Set the ingress struct field. It has the following toml definition
    ///
    ///  <root>
    /// [ingress]
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `ast` - Option<&Value>
    ///
    /// # Return
    /// Self
    fn set_ingress(mut self, ast: Option<&Value>) -> Self {
        self
    }
}

pub fn get_network(ast: &Value) -> Result<Network, LError> {
    // get the default network struct
    let network = Network::new();

    // Retrieve the service object from toml_field
    let service_field = ast.get("service");
    let ingress_field = ast.get("ingress");

    let network = network
        .set_service(service_field)
        .set_ingress(ingress_field);

    Ok(network)
}