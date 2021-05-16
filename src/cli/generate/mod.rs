use std::collections::HashMap;
use clap::ArgMatches;
use crate::cli::helper::error::{
    CError,
    TypeError
};
use crate::cli::helper::io;
use crate::lib::{
    parser,
    vars
};
use crate::kube;

// Constant
const ARG_PATH: &str = "path";
const ARG_OUTPUT: &str = "output";
const ARG_MERGE: &str = "merge";

/// Run
///
/// # Description
/// Generate a set of Kubernetes Yaml file from TOML templates
///
/// # Arguments
/// * `args` - &ArgMatches
///
/// # Return
/// Result<(), CError>
pub fn run(args: &ArgMatches) -> Result<(), CError> {
    let path = args.value_of(ARG_PATH)
        .ok_or_else(|| CError::from(TypeError::MissingArg(ARG_PATH.to_owned())))?;

    let output = args.value_of(ARG_OUTPUT);
    let merge = args.is_present(ARG_MERGE);

    let generated_yaml = template_variables(path)?;
    let stitched_yaml = generated_yaml
        .clone()
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<String>>()
        .join("");

    if let Some(output_path) = output {
        if !merge {
            return io::write_multiple_files(output_path, generated_yaml);
        }

        return io::write_file(output_path, &stitched_yaml);
    }

    // Otherwise print on the console
    println!("{}", stitched_yaml);

    Ok(())
}

/// Template Variables
///
/// # Description
/// Replace variables by _vars.toml value in TOML template
///
/// # Arguments
/// * `path` - &str
///
/// # Return
/// Result<HashMap<String, String>, CError>
pub fn template_variables(path: &str) -> Result<HashMap<String, String>, CError> {
    let (templates, variables) = io::read_files_to_string(path)?;
    let mut generated_yaml = HashMap::new();

    for (name, tmpl) in templates {
        let updated_templates = vars::replace_variables(tmpl.as_str(), &variables)
            .map_err(|err| CError::from(TypeError::Lib(err.to_string())))?;
        
        let res = parser::get_parsed_objects(updated_templates.as_str())
            .map_err(|err| CError::from(TypeError::Lib(err.to_string())))?;

        let yaml = kube::generate_yaml(res)
            .map_err(|err| CError::from(TypeError::Lib(err.to_string())))?;

        generated_yaml.insert(name, yaml);
    }

    Ok(generated_yaml)
}