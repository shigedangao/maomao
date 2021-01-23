use clap::ArgMatches;
use super::ARG_NOT_FOUND;
use crate::cli::helper::error::CError;
use crate::cli::helper::io;
use crate::lib::parser;
use crate::kube;

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
    let path = args.value_of("path")
        .ok_or_else(|| CError { message: format!("{}{}", "path", ARG_NOT_FOUND), details: String::new()})?;

    let templates = io::read_files_to_string(path)?;
    let mut objects = Vec::new();
    for tmpl in templates {
        let res = parser::get_parsed_objects(tmpl.as_str())?;
        objects.push(res);
    }

    kube::generate_yaml(objects);

    Ok(())
}