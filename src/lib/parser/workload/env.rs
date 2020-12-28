use toml::Value;
use std::convert::From;
use crate::lib::helper::error::LError;
use crate::lib::helper::toml::get_value_for_t;


// Constant
const ENV_FIELD_NOT_FOUND: &str = "Env field does not exist. Make sure that it's within the workload";
const ENV_FIELD_MALFORMATTED: &str = "Env is not a toml table";
const KEY_FIELD_NOT_FOUND: &str = "key is not found. Make sure that it's within the env field";
const KEY_FIELD_NOT_ARRAY: &str = "key is not an array. Make sure that it's a valid TOML array";

#[derive(Debug, Default)]
pub struct EnvFrom {
    map: Vec<String>,
    secret: Vec<String>
}

#[derive(Debug, Default)]
pub struct Env {
    pub from: Vec<EnvRefKey>,
    pub raw: Vec<EnvRefKey>
}

// Use by Env
#[derive(Debug, Default)]
pub struct EnvRefKey {
    pub kind: Option<String>,
    pub name: String,
    pub item: String
}

impl From<Value> for EnvRefKey {
    fn from(ast: Value) -> Self {
        let tp = get_value_for_t::<String>(&ast, "type").unwrap_or(String::new());
        let name = get_value_for_t::<String>(&ast, "name").unwrap_or(String::new());
        let item = get_value_for_t::<String>(&ast, "item").unwrap_or(String::new());

        EnvRefKey {
            kind: Some(tp),
            name,
            item
        }
    }
}

impl EnvRefKey {
    /// From Vec
    ///
    /// # Description
    /// Convert a Vec<Value> to a Vec<EnvRefKey>. It's use to convert a toml env table
    /// [ { type = 'map', name = 'foo', item = 'lol' } ]
    ///
    /// # Arguments
    /// * `value` - Vec<Value>
    ///
    /// # Return
    /// Vec<Self>
    fn from_vec(value: Vec<Value>) -> Vec<Self> {
        value
            .into_iter()
            .map(|v| EnvRefKey::from(v))
            .collect::<Vec<Self>>()
    }
} 


/// Get Envs
///
/// # Description
/// Get the env struct
///
/// # Arguments
/// * `ast` - &Value
///
/// # Return
/// Result<Env, LError>
pub fn get_envs(ast: &Value) -> Result<Env, LError> {
    let envs = ast.get("env")
        .ok_or_else(|| LError { message: ENV_FIELD_NOT_FOUND.to_owned() })?
        .as_table()
        .ok_or_else(|| LError { message: ENV_FIELD_MALFORMATTED.to_owned() })?;
    
    // extract array from the env value
    let from = envs.get("from")
        .ok_or_else(|| LError { message: format!("{}{}", "from", KEY_FIELD_NOT_FOUND)} )?
        .as_array()
        .ok_or_else(|| LError { message: format!("{}{}", KEY_FIELD_NOT_ARRAY, "from")})?;

    let raw = envs.get("raw")
        .ok_or_else(|| LError { message: format!("{}{}", "raw", KEY_FIELD_NOT_FOUND)} )?
        .as_array()
        .ok_or_else(|| LError { message: format!("{}{}", KEY_FIELD_NOT_ARRAY, "raw")})?;

    Ok(Env {
        from: EnvRefKey::from_vec(from.to_owned()),
        raw: EnvRefKey::from_vec(raw.to_owned())
    })
}
