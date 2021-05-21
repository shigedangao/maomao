use toml::Value;
use std::collections::BTreeMap;
use crate::lib::helper::toml::get_value_for_t_lax;

#[derive(Debug, Default, Clone)]
pub struct Env {
    pub binary: bool,
    pub data: Option<BTreeMap<String, String>>
}

impl Env {
    /// New
    ///
    /// # Arguments
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn new(ast: &Value) -> Self {
        let data = get_value_for_t_lax::<BTreeMap<String, String>>(&ast, "data");
        let binary = get_value_for_t_lax::<bool>(ast, "binary").unwrap_or_default();

        Env {
            binary,
            data,
        }
    }
}

/// Get Env
///
/// # Description
/// Retrieve the env
///
/// # Arguments
/// * `ast` - &Value
///
/// # Return
/// Option<Env>
pub fn get_env(ast: &Value) -> Option<Env> {
    let e = Env::new(ast);

    Some(e)
}

#[cfg(test)]
mod tests {
    use toml::Value;

    #[test]
    fn expect_to_parse_env() {
        let template = r#"
        [data]
            foo = "bbtea"
            lol = """
                bobba=10
            """
        "#;

        let ast = template.parse::<Value>().unwrap();
        let res = super::get_env(&ast);
        assert!(res.is_some());

        let env = res.unwrap();
        assert_eq!(env.binary, false);
        assert!(env.data.is_some());

        let d = env.data.unwrap();
        assert_eq!(d.get("foo").unwrap(), "bbtea");
        assert!(d.get("lol").is_some());
    }

    #[test]
    fn expect_to_have_binary() {
        let template = r#"
        [data]
            foo = "bar"
        "#;

        let ast = template.parse::<Value>().unwrap();
        let res = super::get_env(&ast);
        assert!(res.is_some());

        let env = res.unwrap();
        assert_eq!(env.binary, false);
    }

    #[test]
    fn expect_data_to_be_empty() {
        let template = r#"
        binary = true
        "#;

        let ast = template.parse::<Value>().unwrap();
        let res = super::get_env(&ast);
        assert!(res.is_some());
        
        let env = res.unwrap();
        assert_eq!(env.binary, true);
        assert!(env.data.is_none());
    }
}