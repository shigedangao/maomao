use std::convert::From;
use toml::value::Value;
use toml::map::Map;
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

/// EnvMap
///
/// # Description
/// EnvMap is a reference to a ConfigEnvMap & SecretEnvMap use in a container
#[derive(Default, Debug)]
pub struct EnvMap {
    map: Option<Vec<String>>,
    raw: Option<Vec<Raw>>,
    from: Option<Vec<Key>>
}

#[derive(Default, Debug)]
pub struct Raw {
    name: String,
    value: String
}

#[derive(Default, Debug)]
pub struct Key {
    kind: EnvRefKind,
    name: String,
    from: String
}

impl EnvMap {
    /// New
    ///
    /// # Description
    /// Create a new EnvMap object
    pub fn new() -> EnvMap {
        EnvMap::default()
    }

    /// Finish
    ///
    /// # Description
    /// Set the minimal field to create the EnvMap object
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `item` - &Map<String, Value>
    pub fn finish(mut self, item: &Map<String, Value>) -> EnvMap {
        let map = get_map_ref(item.get("map"));
        let raw = get_raw_env(item.get("raw"));
        let from = get_from_env(item.get("from"));

        self.map = map;
        self.raw = raw;
        self.from = from;

        self
    }
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

/// Get map Ref
///
/// # Description
/// Get the value from the key name "map" of the env TOML section
///
/// # Arguments
/// * `item` - &Value
pub fn get_map_ref(item: Option<&Value>) -> Option<Vec<String>> {
    let map_item = match item {
        Some(it) => it,
        None => return None
    };

    if !map_item.is_array() {
        return None;
    }
    
    Vec::to(map_item)
}

/// Get Raw Env
///
/// # Description
/// Get the value from the key name "raw" of the env TOML section
///
/// # Arguments
/// * `item` - &Value
pub fn get_raw_env(item: Option<&Value>) -> Option<Vec<Raw>> {
    let raw_item = match item {
        Some(it) => it,
        None => return None
    };

    get_array_for_type::<Raw>(raw_item, None)
}

/// Get From Env
///
/// # Description
/// Get the value from the key name "from" of the env TOML section
///
/// # Arguments
/// * `item` - &Value
pub fn get_from_env(item: Option<&Value>) -> Option<Vec<Key>> {
    let from_item = match item {
        Some(it) => it,
        None => return None
    };

    get_array_for_type::<Key>(from_item, None)
}

#[cfg(test)]
mod env_test {
    use toml::Value;
    use super::{
        get_map_ref,
        get_raw_env,
        get_from_env,
        EnvRefKind,
        EnvMap
    };

    #[test]
    fn get_map_test() {
        let content = "
            map = [
                'configEnvMap::misc',
                'configEnvMap::api',
                'secrets::foo'
            ]
        ";

        let res = content.parse::<Value>().unwrap();
        let m = res.get("map").unwrap();
        let map_ref = get_map_ref(Some(&m));

        assert!(map_ref.is_some());
        let res = map_ref.unwrap();

        assert_eq!(res.get(0).unwrap(), "configEnvMap::misc");
        assert_eq!(res.get(1).unwrap(), "configEnvMap::api");
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
        let r = res.get("raw").unwrap();
        let raw = get_raw_env(Some(&r));
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
        let f = res.get("from").unwrap();
        let from = get_from_env(Some(&f));

        assert!(from.is_some());

        let content = from.unwrap();
        assert_eq!(content.get(0).unwrap().kind, EnvRefKind::SecretMap);
        assert_eq!(content.get(0).unwrap().name, "google-api-key");   
        assert_eq!(content.get(0).unwrap().from, "google::main.key");   
    }

    #[test]
    fn get_env_from_value() {
        let content = "
        [spec.containers.node.env]
            map = [
                'configEnvMap::misc',
                'configEnvMap::api',
                'secrets::foo'
            ]
            raw = [
                { name = 'greeting', value = 'bar' }
            ]
        ";

        let res = content.parse::<Value>().unwrap();
        let spec = res.get("spec").unwrap().as_table().unwrap();
        let containers = spec.get("containers").unwrap().as_table().unwrap();
        let node = containers.get("node").unwrap().as_table().unwrap();
        let env_item = node.get("env").unwrap().as_table().unwrap();

        let env = EnvMap::new().finish(env_item);
        assert!(env.raw.is_some());
        assert!(env.map.is_some());
        assert!(env.from.is_none());

        let raw = env.raw.unwrap();
        assert_eq!(raw.get(0).unwrap().name, "greeting");
 
        let env = env.map.unwrap();
        assert_eq!(env.get(0).unwrap(), "configEnvMap::misc");   
    }
}