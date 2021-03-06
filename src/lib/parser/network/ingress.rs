use toml::Value;
use std::convert::From;
use crate::lib::helper::error::{
    LError,
    network::Error
};
use crate::lib::helper::toml::{
    get_value_for_t,
    get_value_for_t_lax
};
use crate::lib::helper::conv::Convert;
use super::backend;

#[derive(Debug, Clone)]
pub struct Ingress {
    pub default: Option<backend::Backend>,
    pub rules: Option<Vec<IngressRule>>,
    pub tls: Option<Tls>
}

#[derive(Debug, Clone)]
pub struct IngressRule {
    pub host: String,
    pub paths: Option<Vec<IngressHTTPPath>>
}

#[derive(Debug, Clone)]
pub struct IngressHTTPPath {
    pub kind: String,
    pub path: String,
    pub backend: backend::Backend
}

#[derive(Debug, Default, Clone)]
pub struct Tls {
    pub hosts: Option<Vec<String>>,
    pub secrets: Option<String>
}

impl Ingress {
    /// New
    ///
    /// # Description
    /// Create a new Ingress struct
    fn new() -> Self {
        Ingress {
            rules: None,
            default: None,
            tls: None
        }
    }

    /// Set Rules
    ///
    /// # Description
    /// Set the rules property of the Ingress property
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `ast` - &Value
    ///
    /// # Return
    /// Result<Self, LError>
    fn set_rules(mut self, ast: &Value) -> Result<Self, LError> {
        let rules = ast.get("rules");
        if rules.is_none() {
            return Ok(self);
        }

        let rules = rules
            .ok_or_else(|| LError::from(Error::MissingRules))?
            .as_table()
            .ok_or_else(|| LError::from(Error::IngressWrongType))?;

        let mut ingress_rules = Vec::new();
        for (_, rules) in rules.into_iter() {
            let host = get_value_for_t::<String>(&rules, "host")
                .unwrap_or_default();

            let paths = rules.get("paths")
                .ok_or_else(|| LError::from(Error::PathNotFound))?;

            if paths.is_table() {
                ingress_rules.push(IngressRule {
                    host,
                    paths: Some(Vec::convert(paths))
                })
            } else {
                ingress_rules.push(IngressRule {
                    host,
                    paths: None
                })
            }
        }

        self.rules = Some(ingress_rules);
        Ok(self)
    }

    /// Set Default
    ///
    /// # Description
    /// Set the default backend of the ingress
    /// For now we only support the backend option for default 
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn set_default(mut self, ast: &Value) -> Result<Self, LError> {
        let default = ast.get("default");
        if default.is_none() {
            return Ok(self)
        }

        let default = get_value_for_t::<backend::Backend>(default.unwrap(), "backend");
        if let Ok(d) = default {
            self.default = Some(d)
        }

        Ok(self)
    }

    /// Set Tls
    ///
    /// # Description
    /// Set TLS struct to ingress struct
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `ast` - &Value
    ///
    /// # Return
    /// Result<Self, LError>
    fn set_tls(mut self, ast: &Value) -> Result<Self, LError> {        
        let tls_ast = ast.get("tls");
        if tls_ast.is_none() {
            return Ok(self);
        }

        let secrets = get_value_for_t_lax::<String>(tls_ast.unwrap(), "secrets");
        let hosts = get_value_for_t_lax::<Vec<String>>(tls_ast.unwrap(), "hosts");

        let tls = Tls { hosts, secrets };
        self.tls = Some(tls);
        Ok(self)
    }   
}

impl Convert for Vec<IngressHTTPPath> {
    fn convert(v: &Value) -> Self {
        let paths = v.as_table().unwrap();
        
        paths
            .into_iter()
            .map(|(_, data)| IngressHTTPPath::from(data.to_owned()))
            .collect::<Vec<IngressHTTPPath>>()
    }
}

impl From<Value> for IngressHTTPPath {
    /// New
    ///
    /// # Description
    /// Create a new IngresssHTTPPath struct
    ///
    /// # Arguments
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn from(ast: Value) -> Self {
        let kind = get_value_for_t::<String>(&ast, "type").unwrap_or_default();
        let path = get_value_for_t::<String>(&ast, "path").unwrap_or_default();
        
        if let Some(backend_ast) = ast.get("backend") {
            return IngressHTTPPath {
                kind,
                path,
                backend: backend::Backend::convert(backend_ast)
            }
        }

        IngressHTTPPath {
            kind,
            path,
            backend: backend::Backend::default()
        }
    }
}

/// Get Ingress
///
/// # Description
/// Get the ingress resource
///
/// # Arguments
/// * `ast` - &Value
///
/// # Return
/// Result<Ingress, LError>
pub fn get_ingress(ast: &Value) -> Result<Ingress, LError> {
    let ingress = Ingress::new()
        .set_rules(ast)?
        .set_default(ast)?
        .set_tls(ast)?;

    Ok(ingress)
}

#[cfg(test)]
mod test {
    use toml::Value;

    #[test]
    fn expect_to_parse_ingress() {
        // Yaml output would be
        //
        // spec:
        //  rules:
        //  - host: <>
        //    http
        //      paths:
        //      - pathType: 'Prefix'
        //        path: /
        //
        let template = "
        [ingress]
            [ingress.rules]
                [ingress.rules.rusty]
                    host = 'foo.bar.com'

                    [ingress.rules.rusty.paths]
                        [ingress.rules.rusty.paths.0]
                            type = 'Prefix'
                            path = '/'
                            backend = { name = 'rusty', port = 90 }
        ";

        let ast = template.parse::<Value>().unwrap();
        let ingress_ast = ast.get("ingress").unwrap();

        let ingress = super::get_ingress(&ingress_ast);
        assert!(ingress.is_ok());

        let ingress = ingress.unwrap();
        assert!(ingress.rules.is_some());

        let rules = ingress.rules.unwrap();
        let first_rules = rules.get(0).unwrap();
        assert_eq!(first_rules.host, "foo.bar.com");

        let path = first_rules.paths.as_ref().unwrap();
        let first_path = path.get(0).unwrap();
        assert_eq!(first_path.kind, "Prefix");
        assert_eq!(first_path.path, "/");
        assert_eq!(first_path.backend.name, "rusty");
        assert_eq!(first_path.backend.port, 90);
    }

    #[test]
    fn expect_to_parse_ingress_simple_spec() {
        // Yaml output would be
        //
        // spec:
        //  rules:
        //  - http:
        //      paths:
        //      - pathType: 'Prefix'
        //        path: /
        //
        let template = "
        [ingress]
            [ingress.rules]
                [ingress.rules.rusty]
                    [ingress.rules.rusty.paths]
                        [ingress.rules.rusty.paths.0]
                            type = 'Prefix'
                            path = '/'
                            backend = { name = 'rusty', port = 90 }
        ";

        let ast = template.parse::<Value>().unwrap();
        let ingress_ast = ast.get("ingress").unwrap();

        let ingress = super::get_ingress(&ingress_ast);
        assert!(ingress.is_ok());

        let ingress = ingress.unwrap();
        let rules = ingress.rules.unwrap();
        let first_rules = rules.get(0).unwrap();

        assert_eq!(first_rules.host, "");

        let first_rules_path = first_rules.paths.as_ref().unwrap();
        let path = first_rules_path.get(0).unwrap();
        assert_eq!(path.kind, "Prefix");
        assert_eq!(path.path, "/");
        assert_eq!(path.backend.name, "rusty");
        assert_eq!(path.backend.port, 90);
    }

    #[test]
    fn expect_to_not_parse_ingress() {
        let template = "
            [ingress]
                [ingress.rusty]
        ";

        let ast = template.parse::<Value>().unwrap();
        let ingress_ast = ast.get("ingress").unwrap();
        let ingress = super::get_ingress(&ingress_ast);
        assert!(ingress.is_ok());
        assert!(ingress.unwrap().rules.is_none())
    }

    #[test]
    fn expect_to_parse_default_ingress() {
        let template = "
            [ingress]
                [ingress.default]
                    backend = { name = 'capoo', port = 8000 }
        ";

        let ast = template.parse::<Value>().unwrap();
        let ingress_ast = ast.get("ingress").unwrap();
        let ingress = super::get_ingress(&ingress_ast);
        assert!(ingress.is_ok());

        let ingress = ingress.unwrap();
        let default = ingress.default.unwrap();

        assert_eq!(default.name, "capoo");
        assert_eq!(default.port, 8000);
    }

    #[test]
    fn expect_to_parse_tls() {
        let template = "
            [ingress]
                [ingress.default]
                    backend = { name = 'capoo', port = 8000 }

            [ingress.tls]
                hosts = [
                    'foo.bar.com'
                ]
                secrets = 'foo-ssl-certificates'
        ";

        let ast = template.parse::<Value>().unwrap();
        let ingress_ast = ast.get("ingress").unwrap();

        let ingress = super::get_ingress(&ingress_ast);
        assert!(ingress.is_ok());

        let ingress = ingress.unwrap();
        let tls = ingress.tls.unwrap();

        assert_eq!(tls.hosts.unwrap().get(0).unwrap(), "foo.bar.com");
        assert_eq!(tls.secrets.unwrap(), "foo-ssl-certificates");
    }

    #[test]
    fn expect_to_parse_tls_only_secrets() {
        let template = "
            [ingress]
                [ingress.tls]
                    secrets = 'bar-ssl-certificates'
        ";

        let ast = template.parse::<Value>().unwrap();
        let ingress_ast = ast.get("ingress").unwrap();

        let ingress = super::get_ingress(&ingress_ast);
        assert!(ingress.is_ok());

        let ingress = ingress.unwrap();
        let tls = ingress.tls.unwrap();

        assert_eq!(tls.secrets.unwrap(), "bar-ssl-certificates");
    }
}