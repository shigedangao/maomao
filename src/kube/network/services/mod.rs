use k8s_openapi::api::core::v1::Service;
use crate::kube::common;
use crate::lib::parser::Object;
use crate::kube::helper::error::{
    KubeError,
    common::Error
};

mod spec;

struct ServiceWrapper {
    service: Service
}


impl ServiceWrapper {
    /// New
    ///
    /// # Description
    /// Create a new service object
    ///
    /// # Arguments
    /// * `object` - &Object
    /// * `kind` - String
    ///
    /// # Return
    /// Self
    fn new(object: &Object) -> Self {
        let service = Service {
            metadata: common::get_metadata_from_object(&object),
            ..Default::default()
        };

        ServiceWrapper {
            service,
        }
    }
    /// Set Spec
    ///
    /// # Description
    /// Set the spec of a service
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `object` - &Object
    ///
    /// # Return
    /// Result<Self, KubeError>
    fn set_spec(mut self, object: &Object) -> Result<Self, KubeError>{
        let spec = object
            .spec
            .to_owned()
            .ok_or_else(|| KubeError::from(Error::MissingSpec))?;

        let network = spec.to_owned()
            .network
            .ok_or_else(|| KubeError::from(spec.error.unwrap()))?;

        if let Some(service) = network.service {
            let mut service_spec = spec::get_service_spec(service);
            service_spec.selector = object.metadata.to_owned();
            self.service.spec = Some(service_spec);
        }

        Ok(self)
    }
}

/// Get Service From Object
///
/// # Description
/// Retrieve service from parser object
///
/// # Arguments
/// * `object` - Object
///
/// # Return
/// Result<String, KubeError>
pub fn get_service_from_object(object: Object) -> Result<String, KubeError> {
    let service = ServiceWrapper::new(&object).set_spec(&object)?;
    let service_string = serde_yaml::to_string(&service.service)?;

    Ok(service_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::parser::get_parsed_objects;
    use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

    #[test]
    fn expect_to_parse_service() {
        let template = "
            kind = 'network::service'
            version = 'v1'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }
        
            [annotations]
            'external-dns.alpha.kubernetes.io/hostname' = 'rusty.dev.org.'
        
            [service]
                type = 'NodePort'

                [service.ports]

                    [service.ports.http]
                        protocol = 'TCP'
                        port = 80
                        target_port = 90
                        node_port = 30310
        ";

        let object = get_parsed_objects(template).unwrap();
        let service = ServiceWrapper::new(&object).set_spec(&object);
        assert!(service.is_ok());

        let service = service.unwrap().service;
        assert_eq!(service.metadata.labels.get("name").unwrap(), "rusty");

        let annotations = service.metadata.annotations;
        assert_eq!(annotations.get("external-dns.alpha.kubernetes.io/hostname").unwrap(), "rusty.dev.org.");

        let spec = service.spec.unwrap();
        assert_eq!(spec.type_.unwrap(), "NodePort");
        
        let ports = spec.ports.get(0).unwrap();
        assert_eq!(ports.name.as_ref().unwrap(), "http");
        assert_eq!(ports.port, 80);
        assert_eq!(ports.target_port.as_ref().unwrap(), &IntOrString::Int(90));
        assert_eq!(ports.protocol.as_ref().unwrap(), "TCP");
        assert_eq!(ports.node_port.unwrap(), 30310);
    }

    #[test]
    fn expect_to_return_yaml() {
        let template = "
            kind = 'network::service'
            version = 'v1'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }
        
            [annotations]
            'external-dns.alpha.kubernetes.io/hostname' = 'rusty.dev.org.'
        
            [service]
                type = 'NodePort'

                [service.ports]

                    [service.ports.http]
                        protocol = 'TCP'
                        port = 80
                        target_port = 90
                        node_port = 30310
        ";

        let object = get_parsed_objects(template).unwrap();
        let service = super::get_service_from_object(object);
        assert!(service.is_ok());
    }
}