use toml::Value;
use std::convert::From;
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
    pub map: Option<Vec<String>>,
    pub secret: Option<Vec<String>>
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
/// Option<Env>
pub fn get_envs(ast: &Value) -> Option<Env> {
    let envs = ast.get(ENV_NAME).as_ref()?.as_table()?;
    let mut env = Env::default();
    
    // retrieve the map table from toml 
    // [workload.<container>.env]
    // from = [ {...EnvRefKey struct fields } ]
    if let Some(from) = envs.get(ENV_KEYNAME_FROM) {
        if let Some(from_array) = from.as_array() {
            env.from = EnvRefKey::from_vec(from_array.to_owned());
        }
    }

    // retrieve the map table from toml 
    // [workload.<container>.env]
    // raw = [ {...EnvRefKey struct fields } ]
    if let Some(raw) = envs.get(ENV_KEYNAME_RAW) {
        if let Some(raw_array) = raw.as_array() {
            env.raw = EnvRefKey::from_vec(raw_array.to_owned());
        }
    }
    
    Some(env)
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
/// Option<EnvFrom>
pub fn get_env_from(ast: &Value) -> Option<EnvFrom> {
    let envs_from = ast.get(ENV_FROM_NAME).as_ref()?.as_table()?;
    let mut res = EnvFrom::default();

    // retrieve the map table from toml 
    // [workload.<container>.env_from]
    // map = [...]
    if let Some(configmap)= envs_from.get(ENV_FROM_MAP_KEYNAME) {
        res.map = Some(Vec::convert(configmap));
    }
    // retrieve the map table from toml 
    // [workload.<container>.env_from]
    // secret = [...]
    if let Some(secret) = envs_from.get(ENV_FROM_SECRET_KEYNAME) {
        res.secret = Some(Vec::convert(secret));
    }

    Some(res)
}
