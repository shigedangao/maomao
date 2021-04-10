pub mod network;
pub mod workload;
pub mod volume;

mod crd;
mod spec;

use std::collections::{BTreeMap, HashMap};
use toml::Value;
use super::helper::error::LError;
use super::helper::toml::{
    get_value_for_t,
    get_value_for_t_lax
};
use super::helper::conv::Convert;

// Constant
const SPLIT_DELIMITER: &str = "::";

/// Kind
///
/// # Description
/// Kind of toml file
/// - Workload => workload::{kubernetes workfload} i.e workload::deployment
/// - Network => network::{kubernetes network object} i.e: network::service
#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    Workload(String),
    Network(String),
    Custom(String),
    None
}

impl Default for Kind {
    fn default() -> Self {
        Kind::None
    }
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
            "custom" => Kind::Custom(arg),
            _ => Kind::None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Object {
    pub kind: Kind,
    pub name: Option<String>,
    pub version: Option<String>,
    
    pub metadata: BTreeMap<String, String>,
    pub annotations: Option<BTreeMap<String, String>>,
    pub spec: Option<spec::Spec>,

    pub volume_claim: Option<HashMap<String, volume::VolumeClaimTemplates>>
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
        let name = get_value_for_t_lax::<String>(ast, "name");
        let version = get_value_for_t_lax::<String>(ast, "version");
        let kind = Kind::convert(ast);
        let metadata = get_value_for_t::<BTreeMap<String, String>>(ast, "metadata")?;

        Ok(Object {
            kind,
            name,
            version,
            metadata,
            annotations: None,
            spec: None,
            ..Default::default()
        })
    }

    /// Set Annotations
    ///
    /// # Description
    /// Set the annotations of a template
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `ast` - &Value
    fn set_annotations(mut self, ast: &Value) -> Self {
        let annotations = get_value_for_t::<BTreeMap<String, String>>(ast, "annotations");
        if let Ok(res) = annotations {
            self.annotations = Some(res);
        }

        self
    }

    /// Set Spec
    ///
    /// # Description
    /// Set the spec field in the Object struct
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `ast` - &Value
    fn set_spec(mut self, ast: &Value) -> Self {
        let spec = spec::get_spec(ast);
        self.spec = Some(spec);

        self
    }

    /// Set Volumes
    ///
    /// # Description
    /// Set the volumes that could be used by a workload (container > spec > volume_claim_templates)
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn set_volumes(mut self, ast: &Value) -> Self {
        let volumes_claims = ast.get("volume_claims");
        if volumes_claims.is_none() {
            return self;
        }

        let volumes_claims = volumes_claims.unwrap();
        if !volumes_claims.is_table() {
            return self;
        }

        let volumes_claims_table = volumes_claims.as_table().unwrap();
        let computed_volumes = volume::get_volumes_from_toml_tables(volumes_claims_table);
        if let Ok(volumes) = computed_volumes {
            self.volume_claim = Some(volumes);
        } else {
            // print the error
            // @TODO replace the println! with a print error or something else...
            println!("{:?}", computed_volumes.unwrap_err());
        }

        self
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

    let object = Object::new(&ast)?
        .set_annotations(&ast)
        .set_volumes(&ast)
        .set_spec(&ast);
    Ok(object)
}


// Test
#[cfg(test)]
mod test {
    
    #[test]
    fn expect_parse_basic_metadata() {
        let template = "
            kind = 'workload::deployment'
            version = 'apps/v1'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }
        ";

        let object = super::get_parsed_objects(template);
        assert!(object.is_ok());

        let object = object.unwrap();
        assert_eq!(object.name.unwrap(), "rusty");
        assert_eq!(object.version.unwrap(), "apps/v1");
        assert_eq!(object.metadata.get("tier").unwrap(), "backend");
        assert_eq!(object.kind, super::Kind::Workload("deployment".to_owned()))
    }

    #[test]
    fn expect_to_parse_annotations() {
        let template = "
            kind = 'workload::deployment'
            version = 'apps/v1'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [annotations]
                foo = 'rusty'
                'bar' = 'bobba'
        ";

        let object = super::get_parsed_objects(template);
        assert!(object.is_ok());

        let object = object.unwrap();
        assert!(object.annotations.is_some());
        
        let annotations = object.annotations.unwrap();
        assert_eq!(annotations.get("foo").unwrap(), "rusty");
        assert_eq!(annotations.get("bar").unwrap(), "bobba");
    }

    #[test]
    fn expect_kind_to_none_metadata() {
        let template = "
            kind = 'wrongworkload'
            version = 'apps/v1'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }
        ";

        let object = super::get_parsed_objects(template);
        assert!(object.is_ok());

        let object = object.unwrap();
        assert_eq!(object.kind, super::Kind::None);
    }

    #[test]
    fn exepct_to_not_return_err_missing_name_metadata() {
        let template = "
            kind = 'workload::deployment'
            metadata = { name = 'rusty', tier = 'backend' }
        ";

        let object = super::get_parsed_objects(template);
        assert!(object.is_ok());
    }

    #[test]
    fn expect_object_to_contains_spec() {
        let template = "
            kind = 'workload::deployment'
            version = 'apps/v1'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [workload]
                replicas = 3

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
            version = 'v1'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }

            [service]
                type = 'nodeport'

                [service.ports]

                    [service.ports.http]
                        protocol = 'HTTP'
                        port = 90
                        target_port = 1000
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
            version = 'v1'
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

    #[test]
    fn expect_to_parse_volumes() {
        let template = r#"
            kind = 'workload::statefulset'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }
        
            [volume_claims]
                [volume_claims.rust]
                    access_modes = [ 'ReadWriteOnce' ]
                    data_source = { name = 'kind', kind = 'VolumeSnapshot' }
                    resources_limit = [
                        { key_name = 'storage', value = '1g' }
                    ]
                    resources_request = [
                        { key_name = 'key', value = '' }
                    ]
        "#;

        let object = super::get_parsed_objects(template);
        assert!(object.is_ok());

        let object = object.unwrap();
        assert!(object.volume_claim.is_some());

        let volumes = object.volume_claim.unwrap();
        let rust = volumes.get("rust");
        assert!(rust.is_some());

        let rust = rust.unwrap();
        let name = rust.metadata.get("name");
        assert_eq!(name.unwrap(), "rust");

        let desc = rust.description.to_owned().unwrap();
        assert_eq!(desc.access_modes.unwrap().get(0).unwrap(), "ReadWriteOnce");

        let datasource = desc.data_source;
        assert!(datasource.is_some());
        let datasource = datasource.unwrap();
        assert_eq!(datasource.name.unwrap(), "kind");
        assert_eq!(datasource.kind.unwrap(), "VolumeSnapshot");

        let resources = rust.resources.to_owned().unwrap();
        assert!(resources.limit.is_some());
        let resource_map = resources.limit.unwrap();
        let storage_rule = resource_map.get("storage");
        assert!(storage_rule.is_some());
        assert_eq!(storage_rule.unwrap(), "1g");
    }

    #[test]
    fn expect_to_parse_statefulset() {
        let template = "
            kind = 'workload::statefulset'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }
        
            [volume_claims]
                [volume_claims.rust]
                    access_modes = []
                    resources_limit = [
                        { name = 'key', value = '' }
                    ]
          
            # container name rust
            [workload]
                replicas = 3

                [workload.rust]
                    image = 'foo'
                    tag = 'bar'
                    # name must match the table of the volume_claims
                    volume_mounts = [
                        { name = 'rust', mount_path = 'www', read_only = true }
                    ]
        ";

        let object = super::get_parsed_objects(template);
        assert!(object.is_ok());

        let spec = object.unwrap().spec.unwrap();
        let workload = spec.workload.unwrap();

        let rust = workload.containers.get(0).unwrap();
        assert!(rust.volume_mounts.is_some());

        let volume_mounts = rust.volume_mounts.to_owned().unwrap();
        let item = volume_mounts.get(0).unwrap();
        assert_eq!(item.name.to_owned().unwrap(), "rust");
        assert_eq!(item.path.to_owned().unwrap(), "www");
        assert_eq!(item.read_only.to_owned().unwrap(), true);
    }
}