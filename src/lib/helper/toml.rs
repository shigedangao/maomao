/// Toml.rs
///
/// # Description
/// List of helper (decorator) method to operate more easily with the toml library
use toml::Value;
use std::convert::From;
use super::error::LError;
use super::conv::Convert;

// Constant
const KEY_NOT_FOUND: &str = "Key not found";

/// Get Value For T
///
/// # Description
/// Retrieve a T: Convert value from a toml Value
///
/// # Arguments
/// * `toml` &Value
/// * `key` &str
///
/// # Return
/// Result<T: Convert, LError>
pub fn get_value_for_t<T: Convert>(toml: &Value, key: &str) -> Result<T, LError> {
    let value = toml.get(key).ok_or_else(|| LError {
        message: KEY_NOT_FOUND.to_string()
    })?;

    Ok(T::convert(value))
}

/// Get Value For T From
///
/// # Description
/// Retrieve a T: From value from a toml value
///
/// # Arguments
/// * `toml` - &Value
/// * `key` - &str
///
/// # Return
/// Result<T: From<Value>, LError>
pub fn get_value_for_t_from<T: From<Value>>(toml: &Value, key: &str) -> Result<T, LError> {
    let value = toml.get(key).ok_or_else(|| LError {
        message: KEY_NOT_FOUND.to_string()
    })?;

    Ok(T::from(value.to_owned()))
}