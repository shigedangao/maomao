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

    let templates = io::read_files_to_string(path)?;
    let mut generated_yaml = Vec::new();

    for tmpl in templates {
        let res = parser::get_parsed_objects(tmpl.as_str())
            .map_err(|err| CError::from(TypeError::Lib(err.to_string())))?;

        let yaml = kube::generate_yaml(res)
            .map_err(|err| CError::from(TypeError::Lib(err.to_string())))?;

        generated_yaml.push(yaml);
    }

    let stitched_yaml = generated_yaml.join("");
    if let Some(output_path) = output {
        return io::write_file(output_path, &stitched_yaml);
    }

    // Otherwise print on the console
    println!("{}", stitched_yaml);

    Ok(())
}