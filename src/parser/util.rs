use std::convert::From;
use toml::Value;
use super::conv::ConvertNative;

/// Get String Value
///
/// # Description
/// Get string value of a field
///
/// # Arguments
/// * `t_content` - Value
/// * `key` &str
pub fn get_string_value(t_content: &Value, key: &str) -> Option<String> {
    let string = t_content.get(key);
    if let Some(st) = string {
        if st.is_str() {
            return Some(st
                .as_str()
                .unwrap()
                .to_owned()
            );
        }
    }

    None
}

/// Get Value For Type
///
/// # Description
/// Get a value for a Type T: ConvertNative<T>. This will retrieve a key and return the type wrap in an Option
///
/// # Arguments
/// * `t_content` - &Value
/// * `key` - &str
pub fn get_value_for_type<T: ConvertNative<T>>(t_content: &Value, key: &str) -> Option<T> {
    let value = t_content.get(key);
    if let Some(v) = value {
        return T::to(v);
    }

    None
} 

/// Get Array For Type
///
/// # Description
/// Retrieve an array for the targeted TOML key
/// The T must implement the toml::Value by using the std::convert::From trait
/// The method will build a Vector and use the method from to build the desire T
///
/// # Arguments
/// * `t_content` - &Value
/// * `key` - &str
pub fn get_array_for_type<T: From<Value>>(t_content: &Value, key: Option<&str>) -> Option<Vec<T>> {
    let content = match key {
        Some(k) =>  t_content.get(k),
        None => Some(t_content)
    };

    let res = match content {
        Some(c) => c,
        None => return None
    };

    if !res.is_array() {
        return None;
    }

    let array = res.as_array().unwrap();
    let t_vec = array
        .iter()
        .map(|v| T::from(v.to_owned()))
        .collect::<Vec<T>>();

    Some(t_vec)
}
