use std::collections::HashMap;
use clap::ArgMatches;
use crate::cli::helper::error::{
    CError,
    TypeError
};
use crate::cli::helper::io;
use crate::lib::parser;
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

    let templates = io::read_files_to_string(path)?;
    let mut generated_yaml = HashMap::new();

    for (name, tmpl) in templates {
        let res = parser::get_parsed_objects(tmpl.as_str())
            .map_err(|err| CError::from(TypeError::Lib(err.to_string())))?;

        let yaml = kube::generate_yaml(res)
            .map_err(|err| CError::from(TypeError::Lib(err.to_string())))?;

        generated_yaml.insert(name, yaml);
    }

    let stitched_yaml = generated_yaml
        .clone()
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<String>>()
        .join("");

    if let Some(output_path) = output {
        if merge {
            return io::write_multiple_files(output_path, generated_yaml);
        }

        return io::write_file(output_path, &stitched_yaml);
    }

    // Otherwise print on the console
    println!("{}", stitched_yaml);

    Ok(())
}