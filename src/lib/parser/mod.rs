use std::collections::BTreeMap;
use toml::Value;
use super::helper::error::LError;
use super::helper::toml::get_value_for_t;

#[derive(Debug)]
pub enum Kind {
    Workload(String),
    Network(String)
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
    let metadata = get_value_for_t::<BTreeMap<String, String>>(&ast, "metadata")?;

    Ok(Object {
        kind: Kind::Workload("foo".to_string()),
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
    }
}