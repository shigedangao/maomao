use toml::Value;
use std::collections::HashMap;

/// ConvertNative trait
///
/// # Description
/// Convert a Value to a native type
pub trait ConvertNative<T> {
    fn to(item: &Value) -> Option<T> ;
}

impl ConvertNative<i64> for i64 {
    fn to(item: &Value) -> Option<i64> {
        if item.is_integer() {
            return Some(item.as_integer().unwrap());
        }

        None
    }
}

impl ConvertNative<String> for String {
    fn to(item: &Value) -> Option<String> {
        if item.is_str() {
            return Some(
                item
                    .as_str()
                    .unwrap()
                    .to_owned()
            );
        }

        None
    }
}

impl ConvertNative<Vec<String>> for Vec<String> {
    fn to(item: &Value) -> Option<Vec<String>> {
        let array = item.as_array()?;
        let vec = array
            .iter()
            .map(|f| f.as_str())
            .filter(|f| f.is_some())
            .map(|f| f.unwrap().to_owned())
            .collect::<Vec<String>>();

        Some(vec)
    }
}

impl ConvertNative<HashMap<String, String>> for HashMap<String, String> {
    fn to(item: &Value) -> Option<HashMap<String, String>> {
        let mut map = HashMap::new();
        let fields = item.as_table()?;
        for (n, v) in fields.iter() {
            if let Some(s) = v.as_str() {
                map.insert(n.to_owned(), s.to_owned());
            }
        }

        Some(map)
    }
}