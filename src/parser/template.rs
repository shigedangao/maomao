use toml::Value;
use std::collections::HashMap;
use crate::helper::err::{LibError};

// Constant
const OPERATION_KIND: &str = "template parser";

// Error message
const ERROR_KIND_NOT_FOUNDED: &str = "File type 'kind' not founded";

enum TemplateKind {
    KubernetesObject,
    Patch
}

struct Template {
    kind: TemplateKind,
    variables: HashMap<String, String>
}

/// Parse Template
///
/// # Description
/// Parse a TOML template
///
/// # Arguments
/// * `content` - Content of a template
pub fn parse_template(content: String) {
    if content.is_empty() {
        // @TODO return an error
        return;
    }

    let toml_content = content.parse::<Value>().unwrap();
    get_variables(toml_content);
}

/// Get Variables
///
/// # Description
/// Retrieve variables fields
///
/// # Arguments
/// * `t_content` - content of a TOML template
fn get_variables(t_content: Value) -> Option<HashMap<String, String>> {
    let mut var_map = HashMap::new();
    let variables_opts = t_content["variable"].as_table();
    if variables_opts.is_none() {
        return None;
    }

    let variables = variables_opts.unwrap();
    for (name, value) in variables.iter() {
        if value.is_str() {
            let string_value = value.as_str().unwrap().to_owned();
            var_map.insert(name.to_owned(), string_value);
        }
    }

    if var_map.is_empty() {
        return None;
    }

    Some(var_map)
}

/// Get Kind
///
/// # Description
/// Get kind of template
///
/// # Arguments
/// * `t_content` - content of a TOML template
fn get_kind(t_content: Value) -> Result<TemplateKind, LibError> {
    let kind = t_content["kind"].as_str();
    if kind.is_none() {
        return Err(LibError {
            kind: OPERATION_KIND.to_owned(),
            message: ERROR_KIND_NOT_FOUNDED.to_owned()
        });
    }

    let template_kind = match kind.unwrap().to_lowercase().as_str() {
        "deployment" => TemplateKind::KubernetesObject,
        "patch" => TemplateKind::Patch,
        _ => TemplateKind::Patch
    };

    Ok(template_kind)
}