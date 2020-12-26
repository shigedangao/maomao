use std::collections::BTreeMap;
use toml::Value;
use super::helper::error::LError;
use super::helper::toml::get_value_for_t;
use super::helper::conv::Convert;

// Constant
const SPLIT_DELIMITER: &str = "::";

/// Kind
///
/// # Description
/// Kind of toml file
/// - Workload => workload::{kubernetes workfload} i.e workload::deployment
/// - Network => network::{kubernetes network object} i.e: network::service
#[derive(Debug, PartialEq)]
pub enum Kind {
    Workload(String),
    Network(String),
    None
}

impl Convert for Kind {
    fn convert(v: &Value) -> Self {
        let kind = get_value_for_t::<String>(v, "kind");
        if kind.is_err() {
            return Kind::None;
        }

        // split the type by using the character '::'
        let kind = kind.unwrap();
        let kind = kind
            .split(SPLIT_DELIMITER)
            .collect::<Vec<&str>>();

        let t = kind.get(0);
        let arg = kind.get(1)
            .unwrap_or_else(|| &"")
            .to_string();

        if t.is_none() {
            return Kind::None;
        }

        match t.unwrap().to_lowercase().as_str() {
            "workload" => Kind::Workload(arg),
            "network" => Kind::Network(arg),
            _ => Kind::None
        }
    }
}

#[derive(Debug)]
pub struct Object {
    kind: Kind,
    name: String,
    metadata: BTreeMap<String, String>,
    // spec: ?
}

/// Get Parsed Objects
///
/// # Description
/// Retrieve a parser::Object which is a representation of a template
///
/// # Arguments
/// * `tmpl` &str
///
/// # Return
/// Result<Object, LError>
pub fn get_parsed_objects(tmpl: &str) -> Result<Object, LError> {
    let ast = match tmpl.parse::<Value>() {
        Ok(res) => res,
        Err(err) => return Err(LError{ message: err.to_string() })
    };

    let name = get_value_for_t::<String>(&ast, "name")?;
    let kind = Kind::convert(&ast);
    let metadata = get_value_for_t::<BTreeMap<String, String>>(&ast, "metadata")?;

    Ok(Object {
        kind,
        name,
        metadata
    })
}


// Test
#[cfg(test)]
mod test {
    
    #[test]
    fn test_parsed_objects() {
        let template = "
            kind = 'workload::deployment'
            name = 'rusty'
            metadata = { name = 'rusty', tier = 'backend' }
        ";

        let object = super::get_parsed_objects(template);
        assert!(!object.is_err());

        let object = object.unwrap();
        assert_eq!(object.name, "rusty");
        assert_eq!(object.metadata.get("tier").unwrap(), "backend");
        assert_eq!(object.kind, super::Kind::Workload("deployment".to_owned()))
    }
}