use toml::Value;
use crate::lib::helper::conv::Convert;
use crate::lib::helper::toml::get_value_for_t;

#[derive(Debug, Default, Clone)]
pub struct Backend {
    pub name: String,
    pub port: i64
}

#[derive(Debug)]
pub struct Resource {
    api_group: String,
    kind: String,
    name: String
}

impl Convert for Backend {
    fn convert(data: &Value) -> Self {
        let name = get_value_for_t::<String>(&data, "name").unwrap_or_default();
        let port = get_value_for_t::<i64>(&data, "port").unwrap_or(80);

        Backend {
            name,
            port
        }
    }
}
