use std::collections::{BTreeMap, HashMap};
use serde::Deserialize;
use toml::Value;
use toml::map::Map;
use crate::lib::helper::toml::get_value_for_t_lax;
use crate::lib::helper::conv::Convert;

#[derive(Debug, Clone, Default)]
pub struct VolumeClaimTemplates {
    pub metadata: BTreeMap<String, String>,
    pub selector: Option<BTreeMap<String, String>>,
    pub description: Option<VolumeMetadataInfo>,
    pub resources: Option<VolumeResources>
}

#[derive(Debug, Clone, Default)]
pub struct VolumeMetadataInfo {
    pub access_modes: Option<Vec<String>>,
    pub class_name: Option<String>,
    pub data_source: Option<DataSource>,
    pub mode: Option<String>,
    pub name: Option<String>
}

#[derive(Debug, Clone, Default)]
pub struct VolumeResources {
    pub limit: Option<HashMap<String, String>>,
    pub request: Option<HashMap<String, String>>
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DataSource {
    pub name: Option<String>,
    pub kind: Option<String>
}

impl Convert for DataSource {
    fn convert(v: &Value) -> Self {
        let name = get_value_for_t_lax::<String>(v, "name");
        let kind = get_value_for_t_lax::<String>(v, "kind");

        DataSource {
            name,
            kind
        }
    }
}

impl VolumeClaimTemplates {
    /// Create a new VolumeClaimTemplates
    ///
    /// # Arguments
    ///
    /// * `ast` - &Value
    fn new(ast: &Value, name: &str) -> Self {
        let selector = get_value_for_t_lax::<BTreeMap<String, String>>(&ast, "selector");

        let mut metadata: BTreeMap<String, String> = BTreeMap::new();
        metadata.insert("name".to_owned(), name.to_owned());

        VolumeClaimTemplates {
            metadata,
            selector,
            ..Default::default()
        }
    }

    /// Set Description
    ///
    /// # Description
    /// Set the description info of a VolumeTemplate
    ///
    /// # Arguments
    /// * `mut self` - self
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn set_description(mut self, ast: &Value) -> Self {
        let access_modes = get_value_for_t_lax::<Vec<String>>(&ast, "access_modes");
        let class_name = get_value_for_t_lax::<String>(&ast, "class_name");
        let name = get_value_for_t_lax::<String>(&ast, "name");
        let mode = get_value_for_t_lax::<String>(&ast, "mode");
        let data_source = get_value_for_t_lax::<DataSource>(&ast, "data_source");

        let desc = VolumeMetadataInfo {
            access_modes,
            class_name,
            data_source,
            mode,
            name
        };

        self.description = Some(desc);
        self
    }

    /// Set Resources
    ///
    /// # Description
    /// Set the resources limits & resource request for a volumeClaimTemplates
    ///
    /// # Arguments
    /// * `mut self` - self
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn set_resources(mut self, ast: &Value) -> Self {
        let limit = get_hmap_from_vec_toml(ast, "resources_limit");
        let request = get_hmap_from_vec_toml(ast, "resources_request");

        let volume_resources = VolumeResources {
            limit,
            request
        };

        self.resources = Some(volume_resources);
        self
    }
}

/// Get Hmap From Vec Toml
///
/// # Arguments
/// Get a Hashmap from a volume_claims.resource_<x>
///
/// # Arguments
/// * `ast` - &Value
/// * `key` - &str
///
/// # Return
/// Option<HashMap<String, String>>
fn get_hmap_from_vec_toml(ast: &Value, key: &str) -> Option<HashMap<String, String>> {
    let value = ast.get(key);
    let extracted = value.as_ref()?;
    if !extracted.is_array() {
        return None;
    }

    let extracted = extracted.as_array().unwrap();
    let map: HashMap<String, String> = extracted
        .iter()
        .map(|v| {
            let name = get_value_for_t_lax::<String>(v, "key_name").unwrap_or_default();
            let value = get_value_for_t_lax::<String>(v, "value").unwrap_or_default();

            (name, value)
        })
        .collect();

    Some(map)
}

/// Get Volumes From Toml Tables
///
/// # Description
/// Retrieve a list of volume_claims from the volume_claims field definition
///
/// # Arguments
/// * `m` - &Map<String, Value>
///
/// # Return
/// Result<HashMap<String, VolumeClaimTemplates>, LError>
pub fn get_volumes_from_toml_tables(m: &Map<String, Value>) -> Option<HashMap<String, VolumeClaimTemplates>> {
    let mut volumes = HashMap::new();
    for (name, items) in m.into_iter() {
        let volume = VolumeClaimTemplates::new(items, name)
            .set_description(items)
            .set_resources(items);

        volumes.insert(name.to_owned(), volume);
    }

    if volumes.is_empty() {
        return None
    }

    Some(volumes)
}

#[cfg(test)]
mod tests {
    use toml::Value;

    #[test]
    fn expect_to_parse_volume_claim() {
        let template = r#"
        [volume_claims]
            [volume_claims.nginx]
                access_modes = ["ReadWriteOnce"]
                resources_request = [
                    { key_name = "storage", value = "1Gi" }
                ]
        "#;

        let ast = template.parse::<Value>().unwrap();
        let map = ast.get("volume_claims").unwrap().as_table().unwrap();
        let claims = super::get_volumes_from_toml_tables(map);

        assert!(claims.is_some());

        let claims = claims.unwrap();
        let nginx = claims.get("nginx");
        assert!(nginx.is_some());

        let nginx = nginx.unwrap();
        let desc = nginx.description.to_owned().unwrap();
        let acl = desc.access_modes.unwrap();
        assert_eq!(acl.get(0).unwrap(), "ReadWriteOnce");

        assert!(nginx.resources.is_some());
        let resources = nginx.resources.to_owned().unwrap();
        let requests = resources.request.unwrap();
        let request_storage = requests.get("storage");

        assert!(request_storage.is_some());
        assert_eq!(request_storage.unwrap(), "1Gi");
    }
}