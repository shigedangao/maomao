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

impl Convert for i64 {
    fn convert(v: &Value) -> Self {
        if v.is_integer() {
            return v
                .as_integer()
                .unwrap();
        }

        if let Some(number) = v.as_str() {
            return number.parse::<i64>().unwrap_or(0);
        }

        0
    }
}


impl Convert for i32 {
    fn convert(v: &Value) -> Self {
        if v.is_integer() {
            return v
                .as_integer()
                .unwrap()
                as i32;
        }

        if let Some(number) = v.as_str() {
            return number.parse::<i32>().unwrap_or(0);
        }

        0
    }
}

// @TODO should we return Result instead of Self only to enforce typing ?
impl Convert for bool {
    fn convert(v: &Value) -> Self {
        if v.is_bool() {
            return v.as_bool().unwrap();
        }

        false
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

impl Convert for Vec<String> {
    fn convert(v: &Value) -> Self {
        if !v.is_array() {
            return Vec::new();
        }

        let array = v.as_array().unwrap();
        array
            .iter()
            .map(|s| String::convert(s))
            .collect()
    }
}
