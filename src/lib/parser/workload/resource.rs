use toml::Value;
use crate::lib::helper::conv::Convert;
use crate::lib::helper::toml::get_value_for_t_lax;

#[derive(Default, Debug, Clone)]
pub struct Resources {
    pub limits: Option<Resource>,
    pub requests: Option<Resource> 
}

#[derive(Default, Debug, Clone)]
pub struct Resource {
    pub cpu: Option<String>,
    pub memory: Option<String>
}

impl Convert for Resource {
    fn convert(v: &Value) -> Self {
        let memory = get_value_for_t_lax::<String>(v, "memory");
        let cpu = get_value_for_t_lax::<String>(v, "cpu");

        Resource { cpu, memory }
    }
}

impl Resources {
    pub fn new() -> Self {
        Resources::default()
    }

    /// Set Resources Limits to the Resources
    ///
    /// # Arguments
    ///
    /// * `mut self` - Self
    /// * `ast` - &Value
    pub fn set_limits(mut self, ast: &Value) -> Self {
        if let Some(v) = ast.get("limits") {
            let limit = Resource::convert(v);
            self.limits = Some(limit);
        }

        self
    }

    /// Set Resource Request to Resources
    ///
    /// # Arguments
    ///
    /// * `mut self` - Self
    /// * `ast` - &Value
    pub fn set_requests(mut self, ast: &Value) -> Self {
        if let Some(v) = ast.get("requests") {
            let req = Resource::convert(v);
            self.requests = Some(req);
        }

        self
    }
}