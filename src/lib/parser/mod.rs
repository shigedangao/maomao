mod network;
mod workload;
mod spec;

use std::collections::BTreeMap;
use toml::Value;
use super::helper::error::LError;
use super::helper::toml::get_value_for_t;
use super::helper::conv::Convert;

// Constant
const SPLIT_DELIMITER: &str = "::";

/// Kind
///
/// # Description
/// Kind of toml file
/// - Workload => workload::{kubernetes workfload} i.e workload::deployment
/// - Network => network::{kubernetes network object} i.e: network::service
#[derive(Debug, PartialEq)]
pub enum Kind {
    Workload(String),
    Network(String),
    None
}

impl Convert for Kind {
    fn convert(v: &Value) -> Self {
        let kind = get_value_for_t::<String>(v, "kind");
        if kind.is_err() {
            return Kind::None;
        }

        // split the type by using the character '::'
        let kind = kind.unwrap();
        let kind = kind
            .split(SPLIT_DELIMITER)
            .collect::<Vec<&str>>();

        let t = kind.get(0);
        let arg = kind.get(1)
            .unwrap_or_else(|| &"")
            .to_string();

        if t.is_none() {
            return Kind::None;
        }

        match t.unwrap().to_lowercase().as_str() {
            "workload" => Kind::Workload(arg),
            "network" => Kind::Network(arg),
            _ => Kind::None
        }
    }
}

#[derive(Debug)]
pub struct Object {
    kind: Kind,
    name: String,
    metadata: BTreeMap<String, String>,
    spec: Option<spec::Spec>,
}

impl Object {
    /// New
    ///
    /// # Description
    /// Construct a new Object structure by filling the basic informations
    /// - name
    /// - metadata
    /// - kind (kind of template)
    ///
    /// # Arguments
    /// * `ast` - &Value
    ///
    /// # Return
    /// Result<Object, LError>
    fn new(ast: &Value) -> Result<Object, LError> {
        let name = get_value_for_t::<String>(ast, "name")?;
        let kind = Kind::convert(ast);
        let metadata = get_value_for_t::<BTreeMap<String, String>>(ast, "metadata")?;

        Ok(Object {
            kind,
            name,
            metadata,
            spec: None
        })
    }

    /// Set Spec
    ///
    /// # Description
    /// Set the spec field in the Object struct
    ///
    /// # Arguments
    /// * `&mut self` - Self
    /// * `ast` - &Value
    fn set_spec(&mut self, ast: &Value) {
        let spec = spec::get_spec(ast);
        self.spec = Some(spec);
    }
}

/// Get Parsed Objects
///
/// # Description
/// Retrieve a parser::Object which is a representation of a template
///
/// # Arguments
/// * `tmpl` &str
///
/// # Return
/// Result<Object, LError>
pub fn get_parsed_objects(tmpl: &str) -> Result<Object, LError> {
    let ast = match tmpl.parse::<Value>() {
        Ok(res) => res,
        Err(err) => return Err(LError{ message: err.to_string() })
    };

    let mut object = Object::new(&ast)?;
    object.set_spec(&ast);
    
    Ok(object)
}


// Test
#[cfg(test)]
mod test {
    
    #[test]
    fn expect_parse_basic_metadata() {
        let template = "
            kind = 'workload::deployment'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }
        ";

        let object = super::get_parsed_objects(template);
        assert!(object.is_ok());

        let object = object.unwrap();
        assert_eq!(object.name, "rusty");
        assert_eq!(object.metadata.get("tier").unwrap(), "backend");
        assert_eq!(object.kind, super::Kind::Workload("deployment".to_owned()))
    }

    #[test]
    fn expect_kind_to_none_metadata() {
        let template = "
            kind = 'wrongworkload'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }
        ";

        let object = super::get_parsed_objects(template);
        assert!(object.is_ok());

        let object = object.unwrap();
        assert_eq!(object.kind, super::Kind::None);
    }

    #[test]
    fn exepct_to_return_err_missing_name_metadata() {
        let template = "
            kind = 'workload::deployment'
            metadata = { name = 'rusty', tier = 'backend' }
        ";

        let object = super::get_parsed_objects(template);
        assert!(object.is_err());
    }

    #[test]
    fn expect_object_to_contains_spec() {
        let template = "
            kind = 'workload::deployment'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [workload]

                [workload.rust]
                    image = 'foo'
                    tag = 'bar'
        ";

        let object = super::get_parsed_objects(template);
        assert!(object.is_ok());

        let object = object.unwrap();
        assert!(object.spec.is_some());
        assert!(object.spec.unwrap().workload.is_ok());
    }

    #[test]
    fn expect_to_parse_service_spec() {
        let template = "
            kind = 'network::service'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [service]
                type = 'nodeport'

                [service.ports]

                    [service.ports.http]
                        protocol = 'HTTP'
                        port = 90
                        target_port = 1000

            [ingress]
                [ingress.default]
                    backend = { name = 'capoo', port = 8000 }

        ";

        let object = super::get_parsed_objects(template);
        assert!(object.is_ok());
        
        let network = object.unwrap().spec.unwrap().network.unwrap();
        let service = network.service;
        assert!(service.is_some());
        assert_eq!(service.unwrap().kind, "nodeport");
    }


    #[test]
    fn expect_to_parse_ingress_spec() {
        let template = "
            kind = 'network::service'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [ingress]
                [ingress.default]
                    backend = { name = 'capoo', port = 8000 }

        ";

        let object = super::get_parsed_objects(template);
        assert!(object.is_ok());
        
        let network = object.unwrap().spec.unwrap().network.unwrap();
        let ingress = network.ingress;
        assert!(ingress.is_some());
        assert_eq!(ingress.unwrap().default.unwrap().name, "capoo");
    }
}