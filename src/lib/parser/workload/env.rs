use toml::Value;
use std::convert::From;
use crate::lib::helper::error::{
    LError,
    workload::Error
};
use crate::lib::helper::toml::{
    get_value_for_t,
    get_value_for_t_lax
};
use crate::lib::helper::conv::Convert;

// Key constant
const ENV_NAME: &str = "env";
const ENV_FROM_NAME: &str = "env_from";
const ENV_KEYNAME_FROM: &str = "from";
const ENV_KEYNAME_RAW: &str = "raw";
const ENV_FROM_MAP_KEYNAME: &str = "map";
const ENV_FROM_SECRET_KEYNAME: &str = "secret";


#[derive(Debug, Default, Clone)]
pub struct EnvFrom {
    pub map: Vec<String>,
    pub secret: Vec<String>
}

#[derive(Debug, Default, Clone)]
pub struct Env {
    pub from: Vec<EnvRefKey>,
    pub raw: Vec<EnvRefKey>
}

// Use by Env
#[derive(Debug, Default, Clone)]
pub struct EnvRefKey {
    pub from_field: Option<String>,
    pub item: Option<String>,
    pub name: String
}

impl From<Value> for EnvRefKey {
    fn from(ast: Value) -> Self {
        let from_field = get_value_for_t_lax::<String>(&ast, "from_field");
        let item = get_value_for_t_lax::<String>(&ast, "item");
        let name = get_value_for_t::<String>(&ast, "name").unwrap_or_default();

        EnvRefKey {
            from_field,
            item,
            name
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
            .map(EnvRefKey::from)
            .collect::<Vec<Self>>()
    }
} 


/// Get Envs
///
/// # Description
/// Get the env struct from a TOML template
/// The definition of the toml template should be
///
/// [workload]
///   [workload.<container>]
///     [workload.<container>.env]
///
/// # Arguments
/// * `ast` - &Value
///
/// # Return
/// Result<Env, LError>
pub fn get_envs(ast: &Value) -> Result<Env, LError> {
    let envs = ast.get(ENV_NAME)
        .ok_or_else(|| LError::from(Error::EnvFieldNotFound(ENV_NAME)))?
        .as_table()
        .ok_or_else(|| LError::from(Error::EnvFieldMalformatted(ENV_NAME)))?;
    
    // retrieve the map table from toml 
    // [workload.<container>.env]
    // from = [ {...EnvRefKey struct fields } ]
    let from = envs.get(ENV_KEYNAME_FROM)
        .ok_or_else(|| LError::from(Error::KeyNotFound(ENV_KEYNAME_FROM)))?
        .as_array()
        .ok_or_else(|| LError::from(Error::KeyNotArray(ENV_KEYNAME_FROM)))?;

    // retrieve the map table from toml 
    // [workload.<container>.env]
    // raw = [ {...EnvRefKey struct fields } ]
    let raw = envs.get(ENV_KEYNAME_RAW)
        .ok_or_else(|| LError::from(Error::KeyNotFound(ENV_KEYNAME_RAW)))?
        .as_array()
        .ok_or_else(|| LError::from(Error::KeyNotArray(ENV_KEYNAME_RAW)))?;

    Ok(Env {
        from: EnvRefKey::from_vec(from.to_owned()),
        raw: EnvRefKey::from_vec(raw.to_owned())
    })
}

/// Get Env From
///
/// # Description
/// Retrieve the EnvFrom struct from a toml template
/// The definition of the toml template should be
///
/// [workload]
///   [workload.<container>]
///     [workload.<container>.env_from]
///
/// # Arguments
/// * `ast` - &Value
///
/// # Return
/// Result<EnvFrom, LError>
pub fn get_env_from(ast: &Value) -> Result<EnvFrom, LError> {
    let envs_from = ast.get(ENV_FROM_NAME)
        .ok_or_else(|| LError::from(Error::EnvFieldNotFound(ENV_FROM_NAME)))?
        .as_table()
        .ok_or_else(|| LError::from(Error::EnvFieldMalformatted(ENV_FROM_NAME)))?;

    // retrieve the map table from toml 
    // [workload.<container>.env_from]
    // map = [...]
    let map = envs_from.get(ENV_FROM_MAP_KEYNAME)
        .ok_or_else(|| LError::from(Error::EnvFieldNotFound(ENV_FROM_MAP_KEYNAME)))?;

    // retrieve the map table from toml 
    // [workload.<container>.env_from]
    // secret = [...]
    let secret = envs_from.get(ENV_FROM_SECRET_KEYNAME)
        .ok_or_else(|| LError::from(Error::KeyNotFound(ENV_FROM_SECRET_KEYNAME)))?;

    Ok(EnvFrom {
        map: Vec::convert(map),
        secret: Vec::convert(secret)
    })
}
