use k8s_openapi::api::networking::v1::Ingress;
use crate::lib::parser::Object;
use crate::kube::common;
use crate::kube::helper::error::{
    KubeError,
    common::Error
};

mod spec;

struct IngressWrapper {
    ingress: Ingress
}

impl IngressWrapper {
    /// New
    ///
    /// # Description
    /// Create a new ingress by first setting up the metadata
    ///
    /// # Arguments
    /// * `object` - &Object
    ///
    /// # Return
    /// Self
    fn new(object: &Object) -> Self {
        let ingress = Ingress {
            metadata: common::get_metadata_from_object(object),
            ..Default::default()
        };

        IngressWrapper {
            ingress
        }
    }
    /// Set Spec
    ///
    // # Description
    /// Set the spec of an Ingress resource
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `object` - &Object
    ///
    /// # Return
    /// Result<Self, KubeError>
    fn set_spec(mut self, object: &Object) -> Result<Self, KubeError>{
        if object.spec.is_none() {
            return Err(KubeError::from(Error::MissingSpec));
        }

        let s = object.spec.to_owned().unwrap();
        if let Some(err) = s.error {
            return Err(KubeError::from(err));
        }

        if let Some(network) = s.network {
            let spec = spec::get_ingress_spec(network.ingress);
            self.ingress.spec = spec;
        }

        Ok(self)
    }
}

/// Get Ingress From Object
///
/// # Description
/// Generate an Ingress Resource from a Parser Object
///
/// # Arguments
/// * `object` - Object
///
/// # Return
/// Result<String, KubeError>
pub fn get_ingress_from_object(object: Object) -> Result<String, KubeError> {
    let ingress = IngressWrapper::new(&object).set_spec(&object)?;
    let ingress_string = serde_yaml::to_string(&ingress.ingress)?;

    Ok(ingress_string)
}

#[cfg(test)]
mod tests {
    use crate::lib::parser::get_parsed_objects;
    use super::*;

    #[test]
    fn expect_to_parse_ingress() {
        let template = r"
        kind = 'network::ingress'
        name = 'rusty'
        metadata = { name = 'rusty', tier = 'ingress' }
        
        [ingress]
        
            # ingress.default is a reserved keyword
            [ingress.default]
                backend = { name = 'rusty', port = 80 }
        
            [ingress.rules]
        
                [ingress.rules.rusty]
                    host = 'foo.bar.com'
        
                    [ingress.rules.rusty.paths]
        
                        [ingress.rules.rusty.paths.0]
                            type = 'Prefix'
                            path = '/'
                            backend = { name = 'rusty', port = 80 }
        
            [ingress.tls]
                hosts = [
                    'foo.bar.com',
                    'bar.com.capoo'
                ]
                secrets = 'foo-ssl-certificates'
        ";

        let object = get_parsed_objects(template).unwrap();
        let ingress = IngressWrapper::new(&object).set_spec(&object);
        assert!(ingress.is_ok());

        let ingress = ingress.unwrap().ingress;
        assert_eq!(ingress.metadata.labels.get("name").unwrap(), "rusty");

        let annotations = ingress.metadata.annotations;
        assert!(annotations.is_empty());

        let spec = ingress.spec.unwrap();
        let tls = spec.tls;
        let tls = tls.get(0).unwrap();
        assert_eq!(tls.secret_name.to_owned().unwrap(), "foo-ssl-certificates");
        assert_eq!(tls.hosts.get(0).unwrap(), "foo.bar.com");
        
        let rules = spec.rules;
        let first_rules = rules.get(0).unwrap();
        assert_eq!(first_rules.host.to_owned().unwrap(), "foo.bar.com");

        let http = first_rules.http.to_owned().unwrap();
        let paths = http.paths.get(0).unwrap();
        assert_eq!(paths.path.to_owned().unwrap(), "/");
        assert_eq!(paths.path_type.to_owned().unwrap(), "Prefix");

        let backend_service = paths.backend.to_owned().service.unwrap();
        assert_eq!(backend_service.name, "rusty");
        assert_eq!(backend_service.port.unwrap().number.unwrap(), 80);
    }

    #[test]
    fn expect_to_return_yaml() {
        let template = r"
        kind = 'network::ingress'
        name = 'rusty'
        metadata = { name = 'rusty', tier = 'ingress' }
        
        [ingress]
        
            # ingress.default is a reserved keyword
            [ingress.default]
                backend = { name = 'rusty', port = 80 }
        
            [ingress.rules]
        
                [ingress.rules.rusty]
                    host = 'foo.bar.com'
        
                    [ingress.rules.rusty.paths]
        
                        [ingress.rules.rusty.paths.0]
                            type = 'Prefix'
                            path = '/'
                            backend = { name = 'rusty', port = 80 }
        
            [ingress.tls]
                hosts = [
                    'foo.bar.com',
                    'bar.com.capoo'
                ]
                secrets = 'foo-ssl-certificates'
        ";

        let object = get_parsed_objects(template).unwrap();
        let ingress = get_ingress_from_object(object);
        assert!(ingress.is_ok());
    }
}