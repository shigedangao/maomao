use toml::Value;
use crate::lib::helper::toml::get_value_for_t_lax;
use crate::lib::helper::conv::Convert;

#[derive(Debug, Clone)]
pub struct Toleration {
    pub key: Option<String>,
    pub operator: Option<String>,
    pub value: Option<String>,
    pub effect: Option<String>,
    pub toleration_seconds: Option<i64>
}

impl Convert for Toleration {
    fn convert(ast: &Value) -> Self {
        let key = get_value_for_t_lax::<String>(&ast, "key");
        let operator = get_value_for_t_lax::<String>(&ast, "operator");
        let value = get_value_for_t_lax::<String>(&ast, "value");
        let effect = get_value_for_t_lax::<String>(&ast, "effect");
        let toleration_seconds = get_value_for_t_lax(&ast, "toleration_seconds");

        Toleration {
            key,
            operator,
            value,
            effect,
            toleration_seconds
        }
    }
}

impl Toleration {
    /// Get Toleration List
    ///
    /// # Description
    /// Retrieve a list of Toleration from the workload ast
    ///
    /// # Arguments
    /// * `ast` - &Values
    ///
    /// # Return
    /// Option<Vec<Toleration>>
    pub fn get_toleration_list(ast: &Value) -> Option<Vec<Toleration>> {
        let values = ast.get("tolerations");
        if values.is_none() {
            return None;
        }

        if let Some(arr) = values.unwrap().as_array() {
            let tolerations = arr.into_iter()
                .map(|v| Toleration::convert(v))
                .collect::<Vec<Toleration>>();

            return Some(tolerations);
        }

        None
    }
}