/// Conv.rs
///
/// # Description
/// Convert a toml::Value to a T type
use toml::Value;
use std::collections::BTreeMap;

pub trait Convert {
    /// Convert
    ///
    /// # Description
    /// Convert a toml::Value value to a concrete Rust type
    ///
    /// # Arguments
    /// * `v` &Value
    ///
    /// # Return
    /// Self
    fn convert(v: &Value) -> Self;
}

impl Convert for String {
    fn convert(v: &Value) -> Self {
        if v.is_str() {
            return v
                .as_str()
                .unwrap()
                .to_string();
        }

        String::new()
    }
}

impl Convert for BTreeMap<String, String> {
    fn convert(v: &Value) -> Self {
        if !v.is_table() {
            return BTreeMap::new();
        }

        let table = v
            .as_table()
            .unwrap();

        let map: BTreeMap<String, String> = table
            .into_iter()
            .map(|(k, v)| (k.to_owned(), String::convert(v)))
            .collect();

        map
    }
}
