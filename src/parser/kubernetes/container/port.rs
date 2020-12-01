use std::convert::From;
use toml::Value;
use crate::parser::utils::helper::get_string_value;

#[derive(Debug, Default)]
pub struct PortMapping {
    pub name: String,
    pub value: String
}

impl From<Value> for PortMapping {
    fn from(item: Value) -> Self {
        let name = get_string_value(&item, "name");
        let value = get_string_value(&item, "value");

        if name.is_none() || value.is_none() {
            return PortMapping::default();
        }

        PortMapping {
            name: name.unwrap(),
            value: value.unwrap()
        }
    }
}
