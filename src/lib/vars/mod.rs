use toml::Value;
use regex::Regex;
use super::helper::error::LError;

mod util;

/// Load Variables
///
/// # Description
/// Load variables 
///
/// # Arguments
/// * `tmpl` - &str
///
/// # Return
/// Result<Value, LError>
fn load_variables(tmpl: &str) -> Result<Value, LError> {    
    match tmpl.parse::<Value>() {
        Ok(res) => Ok(res),
        Err(err) => Err(LError { message: err.to_string() })
    }
}

/// Detect Variables
///
/// # Description
/// Detect variables in a TOML value and return the list of variables
/// detected in the toml field
/// example: image: $[provider]/$[repository]
///
/// /!\ Caveat, this might be expensive as we create a new Regex for every new
/// TOML variables. For now this is the most simpler solution... as it either work
/// for custom than implemened template
///
/// # Arguments
/// * `item` - &str
///
/// # Return
/// Option<T>
pub fn replace_variables(template: &str, variables_template: &Option<String>) -> Result<String, LError> {
    if variables_template.is_none() {
        return Ok(template.to_owned());
    }

    let ast = load_variables(variables_template.clone().unwrap().as_str())?;
    
    // make a copy of the current template.
    // This will be used to dynamically replace variables in the template
    let mut new_template = String::from(template);
    if let Some(values) = ast.as_table() {
        for (key, value) in values {
            // convert values to string automatically
            new_template = replace_string_variables(&new_template, &key, value)?;
            // in case if the value is different than a string replace with adequated value
            new_template = replace_with_typed_toml(&new_template, &key, value)?;
        }
    }

    Ok(new_template)
}

/// Replace String Variables only
///
/// # Description
/// Replace variable that are defined as a string
///
/// # Arguments
/// * `template` - &str
/// * `key` - &str
/// * `value` - &Value
///
/// # Return
/// Result<String, LError>
fn replace_string_variables(template: &str, key: &str, value: &Value) -> Result<String, LError> {
    // check if there are some variables that only need to be replaced by a stringify value
    let pattern = format!(r"\$\[{}\]", key);
    let regex = Regex::new(pattern.as_str())?;

    // check if it's a string already
    if let Some(s) = value.as_str() {
        let res = regex.replace_all(template, s);
        return Ok(res.into_owned());
    }

    let res = regex.replace_all(template, value.to_string().as_str());
    // otherwise return the template
    Ok(res.to_string())
}

/// Replace With Typed Toml
///
/// # Description
/// Replace values with typed toml values (i.e: number, array)
/// i.e: foo = "$[bar::typed]"
///      _vars.toml => bar = ["hello"]
/// will output => foo = ["hello"]
///
/// # Arguments
/// * `template` - &str
/// * `key` - &str
/// * `value` - &Value
///
/// # Return
/// Result<String, LError>
fn replace_with_typed_toml(template: &str, key: &str, value: &Value) -> Result<String, LError> {
    // check for a pattern which look like this "\$\[<key>::(\w+)\]"
    let pattern = format!(r#""\$\[{}::typed\]""#, key);
    let regex = Regex::new(pattern.as_str())?;

    // toml value will be implicitely convert as a string
    // /!\ If the value is an inline table, the to_string method will print a full table which is not practicle
    // as a result for a table we are converting automatically to an inline table format
    if let Some(t) = value.as_table() {
        // replace array differently
        let res = util::table_to_toml_inline_array(t);
        let replace_res = regex.replace_all(template, res.as_str());

        Ok(replace_res.into_owned())
    } else {
        let res = regex.replace_all(template, value.to_string().as_str());
        Ok(res.into_owned())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn expect_to_replace_variables() {
        let tmpl = r#"
        kind = "workload::deployment"
        name = "rusty"
        metadata = "$[metadata::typed]"
        
        # container name rust
        [workload]
            replicas = "$[replicas::typed]"  
            
            [workload.rust]
            image = "$[provider]/$[image]"

                [workload.rust.env_from]
                map = "$[arr::typed]"

                [workload.rust.env]
                from = [
                    "$[from::typed]"
                ]
        "#;

        let var = r#"
        replicas = 5

        provider = "eu.gcr.io"
        image = "nginx"
        arr = ["foo"]
        from = { type = "map", name = "foo", item = "lol" }
        metadata = { name = "rusty", tier = "backend" }
        "#;

        let res = super::replace_variables(tmpl, &Some(var.to_owned()));
        assert!(res.is_ok());

        let new_template = res.unwrap();
        assert!(new_template.contains(r#"replicas = 5"#));
        assert!(new_template.contains(r#"image = "eu.gcr.io/nginx""#));
        assert!(new_template.contains(r#"map = ["foo"]"#));
        assert!(new_template.contains(r#"{ name = "rusty", tier = "backend" }"#));

        let parser_object = super::super::parser::get_parsed_objects(&new_template);
        assert!(parser_object.is_ok());
    }

    #[test]
    fn expect_to_replace_mix_array() {
        let template = r#"
        foo = ["$[from::typed]", "$[str]"]
        "#;

        let var = r#"
        from = { hey = "you" }
        str = "wou"
        "#;

        let res = super::replace_variables(template, &Some(var.to_owned()));
        assert!(res.is_ok());

        let new_template = res.unwrap();
        assert!(new_template.contains(r#"foo = [{ hey = "you" }, "wou"]"#));
    }
}