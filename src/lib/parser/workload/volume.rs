use toml::Value;
use crate::lib::helper::toml::get_value_for_t_lax;

#[derive(Debug, Default, Clone)]
pub struct VolumeMount {
    pub name: Option<String>,
    pub path: Option<String>,
    pub read_only: Option<bool>
}

impl VolumeMount {
    /// New
    ///
    /// # Description
    /// Create a new VolumeMount
    ///
    /// # Arguments
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn new(ast: &Value) -> Self {
        let name = get_value_for_t_lax::<String>(ast, "name");
        let path = get_value_for_t_lax::<String>(ast, "mount_path");
        let read_only = get_value_for_t_lax::<bool>(ast, "read_only");

        VolumeMount {
            name,
            path,
            read_only
        }
    }

    pub fn from_toml_array(v: &[Value]) -> Option<Vec<VolumeMount>> {
        let v = v
            .iter()
            .map(|item| VolumeMount::new(item))
            .collect::<Vec<VolumeMount>>();

        Some(v)
    }
}