use toml::Value;
use std::convert::From;
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

impl From<Value> for Backend {
    fn from(data: Value) -> Self {
        let name = get_value_for_t::<String>(&data, "name").unwrap_or("".to_owned());
        let port = get_value_for_t::<i64>(&data, "port").unwrap_or(80);

        Backend {
            name,
            port
        }
    }
}
