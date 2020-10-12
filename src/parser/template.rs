use toml::Value;
use std::collections::HashMap;
use crate::helper::err::LibError;

// Constant
const OPERATION_KIND: &str = "template parser";
// Field constant
const VARIABLE_FIELD_NAME: &str = "variables";
const LABELS_FIELD_NAME: &str = "labels";

// Error message
const ERROR_KIND_NOT_FOUNDED: &str = "File type 'kind' not founded";
const ERROR_EMPTY_TEMPLATE: &str = "Template is empty";

#[derive(Debug, PartialEq)]
pub enum TemplateKind {
    K8SObject,
    Patch
}

#[derive(Debug)]
pub struct Template {
    kind: TemplateKind,
    variables: Option<HashMap<String, String>>,
    labels: Option<HashMap<String, String>>,
    content: Option<Value>
}

/// Parse Template
///
/// # Description
/// Parse a TOML template
///
/// # Arguments
/// * `content` - Content of a template
pub fn parse_template(content: String) -> Result<Template, LibError> {
    if content.is_empty() {
        // @TODO return an error
        return Err(LibError{
            kind: OPERATION_KIND.to_owned(),
            message: ERROR_EMPTY_TEMPLATE.to_owned()
        });
    }

    // get the root Value
    let toml_content = content.parse::<Value>().unwrap();

    let variables = get_map_from_array_str(VARIABLE_FIELD_NAME, &toml_content);
    let labels = get_map_from_array_str(LABELS_FIELD_NAME, &toml_content);
    let spec = get_spec(&toml_content);

    match get_kind(&toml_content) {
        Ok(res) => Ok(Template {
            kind: res,
            variables,
            labels,
            content: spec
        }),
        Err(err) => Err(err)
    }
}

/// Get Map From Array Str
///
/// # Description
/// Retrieve string fields from a TOML array.
///
/// # Arguments
/// * `t_content` - content of a TOML template
pub fn get_map_from_array_str(field: &str, t_content: &Value) -> Option<HashMap<String, String>> {
    let mut var = HashMap::new();
    let fields = t_content[field].as_table();
    if fields.is_none() {
        return None;
    }

    let variables = fields.unwrap();
    for (name, value) in variables.iter() {
        if let Some(v) = value.as_str() {
            let string_value = v.into();
            var.insert(name.to_owned(), string_value);
        }
    }

    if var.is_empty() {
        return None;
    }

    Some(var)
}

/// Get Kind
///
/// # Description
/// Get kind of template
///
/// # Arguments
/// * `t_content` - content of a TOML template
pub fn get_kind(t_content: &Value) -> Result<TemplateKind, LibError> {
    let kind = t_content["kind"].as_str();
    if kind.is_none() {
        return Err(LibError {
            kind: OPERATION_KIND.to_owned(),
            message: ERROR_KIND_NOT_FOUNDED.to_owned()
        });
    }

    let template_kind = match kind
        .unwrap()
        .to_lowercase()
        .as_str() {
        "kubernetes" => TemplateKind::K8SObject,
        "patch" => TemplateKind::Patch,
        _ => TemplateKind::Patch
    };

    Ok(template_kind)
}

/// Get Spec
///
/// # Description
/// Retrieve the spec item from the toml configuration file
///
/// # Arguments
/// * `t_content` - content of a TOML template
pub fn get_spec(t_content: &Value) -> Option<Value> {
    let spec = t_content["spec"].to_owned();
    if spec.is_table() {
        return Some(spec);
    }

    None
}

#[cfg(test)]
mod parser_test {
    use toml::Value;

    #[test]
    fn test_get_str_variables() {
        let content = "
            [variables]
            foo = 'bar'
            port = '80'
        ";

        let value = content.parse::<Value>().unwrap();
        let vars = super::get_map_from_array_str("variables", &value);
        assert!(vars.is_some());

        let vars_res = vars.unwrap();
        let foo = vars_res.get("foo").unwrap().to_owned();
        assert_eq!(foo, "bar");

        let port = vars_res.get("port").unwrap().to_owned();
        assert_eq!(port, "80");
    }

    #[test]
    fn test_get_labels() {
        let content = "
            [labels]
            tier = 'backend'
            app  = 'node'
        ";

        let value = content.parse::<Value>().unwrap();
        let labels = super::get_map_from_array_str("labels", &value);
        assert!(labels.is_some());

        let tier = labels.as_ref().unwrap().get("tier").unwrap().as_str();
        assert_eq!(tier, "backend");
        
        let app = labels.as_ref().unwrap().get("app").unwrap().as_str();
        assert_eq!(app, "node");
    }

    #[test]
    fn test_get_template_kind() {
        let content = "
            kind = 'Kubernetes'
            type = 'Deployment'
        ";

        let value = content.parse::<Value>().unwrap();
        let kind = super::get_kind(&value);

        assert!(!kind.is_err());
        assert_eq!(kind.unwrap(), super::TemplateKind::K8SObject);
    }

    #[test]
    fn test_get_specs() {
        let content = "
            [spec]

                [spec.liveness]
                kind = 'exec'
        ";

        let value = content.parse::<Value>().unwrap();
        let spec = super::get_spec(&value);

        assert!(spec.is_some());
    }
}