use std::convert::From;
use toml::value::Value;
use crate::parser::conv::ConvertNative;
use crate::parser::util::get_array_for_type;

#[derive(PartialEq, Debug)]
pub enum EnvRefKind {
    ConfigMap,
    SecretMap
}

impl Default for EnvRefKind {
    fn default() -> Self {
        EnvRefKind::ConfigMap
    }
}

impl From<String> for EnvRefKind {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "configmap" => EnvRefKind::ConfigMap,
            "secret" => EnvRefKind::SecretMap,
            _ => EnvRefKind::ConfigMap
        }
    }
}

/// Map
///
/// # Description
/// Map is a reference to a ConfigMap & SecretMap use in a container
pub struct Map {
    map_ref: Option<Vec<String>>,
    raw: Option<Vec<Raw>>,
    from: Option<Vec<Key>>
}

#[derive(Default)]
pub struct Raw {
    name: String,
    value: String
}

#[derive(Default)]
pub struct Key {
    kind: EnvRefKind,
    name: String,
    from: String
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

impl From<Value> for Key {
    fn from(item: Value) -> Self {
        let kind = item.get("kind");
        let name = item.get("name");
        let from = item.get("from");

        if kind.is_none() || name.is_none() || from.is_none() {
            return Key::default();
        }

        // use variable shadowing
        let kind = String::to(kind.unwrap());
        let name = String::to(name.unwrap());
        let from = String::to(from.unwrap());

        if kind.is_none() || name.is_none() || from.is_none() {
            return Key::default();
        }

        Key {
            kind: EnvRefKind::from(kind.unwrap()),
            name: name.unwrap(),
            from: from.unwrap()
        }
    }
}

/// Get Map Ref
///
/// # Description
/// Get the value from the key name "map" of the env TOML section
///
/// # Arguments
/// * `item` - &Value
pub fn get_map_ref(item: &Value) -> Option<Vec<String>> {
    let content = item.get("ref");
    if content.is_none() {
        return None;
    }

    if !content.unwrap().is_array() {
        return None;
    }
    
    Vec::to(content.unwrap())
}

/// Get Raw Env
///
/// # Description
/// Get the value from the key name "raw" of the env TOML section
///
/// # Arguments
/// * `item` - &Value
pub fn get_raw_env(item: &Value) -> Option<Vec<Raw>> {
    get_array_for_type::<Raw>(item, "raw")
}

/// Get From Env
///
/// # Description
/// Get the value from the key name "from" of the env TOML section
///
/// # Arguments
/// * `item` - &Value
pub fn get_from_env(item: &Value) -> Option<Vec<Key>> {
    get_array_for_type::<Key>(item, "from")
}

#[cfg(test)]
mod env_test {
    use toml::Value;
    use super::{
        get_map_ref,
        get_raw_env,
        get_from_env,
        EnvRefKind
    };

    #[test]
    fn get_map_test() {
        let content = "
            ref = [
                'configmap::misc',
                'configmap::api',
                'secrets::foo'
            ]
        ";

        let res = content.parse::<Value>().unwrap();
        let map_ref = get_map_ref(&res);

        assert!(map_ref.is_some());
        let res = map_ref.unwrap();

        assert_eq!(res.get(0).unwrap(), "configmap::misc");
        assert_eq!(res.get(1).unwrap(), "configmap::api");
        assert_eq!(res.get(2).unwrap(), "secrets::foo");
    }

    #[test]
    fn get_raw_test() {
        let content = "
            raw = [
                { name = 'greeting', value = 'bar' }
            ]
        ";

        let res = content.parse::<Value>().unwrap();
        let raw = get_raw_env(&res);
        assert!(raw.is_some());
        
        let content = raw.unwrap();

        assert_eq!(content.get(0).unwrap().name, "greeting");
        assert_eq!(content.get(0).unwrap().value, "bar");
    }

    #[test]
    fn get_from_test() {
        let content = "
            from = [
                { kind = 'secret', name = 'google-api-key', from = 'google::main.key' }
            ]
        ";

        let res = content.parse::<Value>().unwrap();
        let from = get_from_env(&res);

        assert!(from.is_some());

        let content = from.unwrap();
        assert_eq!(content.get(0).unwrap().kind, EnvRefKind::SecretMap);
        assert_eq!(content.get(0).unwrap().name, "google-api-key");   
        assert_eq!(content.get(0).unwrap().from, "google::main.key");   
    }
}