use std::convert::From;
use toml::value::Value;
use crate::parser::conv::ConvertNative;
use crate::parser::util::get_array_for_type;

enum EnvRefKind {
    ConfigMap,
    SecretMap
}

/// Map
///
/// # Description
/// Map is a reference to a ConfigMap & SecretMap use in a container
pub struct Map {
    map_ref: Option<Vec<String>>,
    raw: Option<Vec<Raw>>,
    from_map: Option<Vec<Key>>
}

#[derive(Default)]
pub struct Raw {
    name: String,
    value: String
}

pub struct Key {
    kind: EnvRefKind,
    name: String,
    from: String,
    path: String
}

impl From<Value> for Raw {
    fn from(item: Value) -> Self {
        let name = item.get("name");
        let value = item.get("value");

        if name.is_none() || value.is_none() {
            return Raw::default();
        }

        // use variable shadowing
        let name = String::to(name.unwrap());
        let value = String::to(value.unwrap());

        if name.is_none() || value.is_none() {
            return Raw::default();
        }

        Raw {
            name: name.unwrap(),
            value: value.unwrap()
        }
    }
}

/// Retrieve Map Ref
///
/// # Description
/// Retrieve the value from the key name "map" of the env TOML section
///
/// # Arguments
/// * `item` - &Value
pub fn retrieve_map_ref(item: &Value) -> Option<Vec<String>> {
    let content = item.get("ref");
    if content.is_none() {
        return None;
    }

    if !content.unwrap().is_array() {
        return None;
    }
    
    Vec::to(content.unwrap())
}

/// Retrieve Raw Env
///
/// # Description
/// Retrieve the value from the key name "raw" of the env TOML section
///
/// # Arguments
/// * `item` - &Value
pub fn retrieve_raw_env(item: &Value) -> Option<Vec<Raw>> {
    get_array_for_type(item, "raw")
}

#[cfg(test)]
mod env_test {
    use toml::Value;
    use super::{
        retrieve_map_ref,
        retrieve_raw_env
    };

    #[test]
    fn retrieve_map_test() {
        let content = "
            ref = [
                'configmap::misc',
                'configmap::api',
                'secrets::foo'
            ]
        ";

        let res = content.parse::<Value>().unwrap();
        let map_ref = retrieve_map_ref(&res);

        assert!(map_ref.is_some());
        let res = map_ref.unwrap();

        assert_eq!(res.get(0).unwrap(), "configmap::misc");
        assert_eq!(res.get(1).unwrap(), "configmap::api");
        assert_eq!(res.get(2).unwrap(), "secrets::foo");
    }

    #[test]
    fn retrieve_raw_test() {
        let content = "
            raw = [
                { name = 'greeting', value = 'bar' }
            ]
        ";

        let res = content.parse::<Value>().unwrap();
        let res = retrieve_raw_env(&res);
        
        assert!(res.is_some());
        
        let content = res.unwrap();

        assert_eq!(content.get(0).unwrap().name, "greeting");
        assert_eq!(content.get(0).unwrap().value, "bar");
    }
}