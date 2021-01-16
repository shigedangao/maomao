mod service;
mod ingress;
mod backend;

use toml::Value;
use crate::lib::helper::error::LError;

#[derive(Debug, Default)]
pub struct Network {
    pub service: Option<service::Service>,
    pub ingress: Option<ingress::Ingress>
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
    /// Result<Self, LError>
    fn set_service(mut self, ast: Option<&Value>) -> Result<Self, LError> {
        if let Some(a) = ast {
            match service::get_service(a) {
                Ok(res) => self.service = Some(res),
                Err(err) => return Err(err)
            }
        }

        Ok(self)
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
    /// Result<Self, LError>
    fn set_ingress(mut self, ast: Option<&Value>) -> Result<Self, LError> {
        if let Some(node) = ast {
            match ingress::get_ingress(node) {
                Ok(res) => self.ingress = Some(res),
                Err(err) => return Err(err)
            };
        }

        Ok(self)
    }
}

/// Get Network
///
/// # Description
/// Get the network struct
///
/// # Arguments
/// * `ast` - &Value
///
/// # Return
/// Result<Network, LError>
pub fn get_network(ast: &Value) -> Result<Network, LError> {
    // get the default network struct
    let network = Network::new();

    // Retrieve the service object from toml_field
    let service_field = ast.get("service");
    let ingress_field = ast.get("ingress");

    let network = network
        .set_service(service_field)?
        .set_ingress(ingress_field)?;

    Ok(network)
}