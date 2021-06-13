use std::collections::HashMap;
use clap::ArgMatches;
use crate::cli::helper::error::{
    CError,
    TypeError
};
use crate::cli::{
    helper::io,
    helper::logger::{Logger, LogLevel}
};
use crate::lib::{
    parser,
    vars
};
use crate::kube;

// Constant
const ARG_PATH: &str = "path";
const ARG_OUTPUT: &str = "output";
const ARG_MERGE: &str = "merge";
const ARG_QUIET: &str = "quiet";

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
        .ok_or_else(|| CError::from(TypeError::MissingArg(ARG_PATH)))?;

    let output = args.value_of(ARG_OUTPUT);
    let merge = args.is_present(ARG_MERGE);
    let quiet = args.is_present(ARG_QUIET);

    // generate logger based on quiet
    let logger = Logger::new(quiet);
    let generated_yaml = generate_yaml_from_toml(path, &logger)?;
    let stitched_yaml = generated_yaml
        .clone()
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<String>>()
        .join("");

    if let Some(output_path) = output {
        if !merge {
            return match io::write_multiple_files(output_path, generated_yaml) {
                Ok(()) => {
                    logger.print(LogLevel::Success(&format!("Successfully generate Kubernetes template to {}", output_path)));
                    Ok(())
                },
                Err(err) => Err(err)
            }
        }

        return match io::write_file(output_path, &stitched_yaml) {
            Ok(()) => {
                logger.print(LogLevel::Success(&format!("Successfully generate Kubernetes template to {}", output_path)));
                Ok(())
            },
            Err(err) => Err(err)
        }
    }

    // Otherwise print on the console
    println!("{}", stitched_yaml);

    Ok(())
}

/// Generate Yaml From Toml
///
/// # Description
/// Replace variables by _vars.toml value in TOML template
///
/// # Arguments
/// * `path` - &str
///
/// # Return
/// Result<HashMap<String, String>, CError>
pub fn generate_yaml_from_toml(path: &str, logger: &Logger) -> Result<HashMap<String, String>, CError> {
    let (templates, variables) = io::read_files_to_string(path)?;
    let mut generated_yaml = HashMap::new();

    for (name, tmpl) in templates {
        logger.print(LogLevel::Info(&format!("⚙️ Processing template `{}`.toml", name)));

        logger.print(LogLevel::Info("💅 Templating template with variables"));
        let updated_templates = vars::replace_variables(tmpl.as_str(), &variables)
            .map_err(|err| CError::from(TypeError::Lib(&err.message)))?;
        
        logger.print(LogLevel::Info("⚙️ Parsing template"));
        let res = parser::get_parsed_objects(updated_templates.as_str())
            .map_err(|err| CError::from(TypeError::Lib(&err.message)))?;

        logger.print(LogLevel::Info("✍️ Generate Kubernetes YAML spec"));
        let yaml = kube::generate_yaml(res)
            .map_err(|err| CError::from(TypeError::Lib(&err.message)))?;

        generated_yaml.insert(name, yaml);
    }

    Ok(generated_yaml)
}