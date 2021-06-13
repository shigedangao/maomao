use clap::ArgMatches;
use tokio::runtime::Runtime;
use std::collections::HashMap;
use crate::cli::helper::error::{
    CError,
    TypeError
};
use crate::cli::helper::logger::{Logger, LogLevel};
use crate::kube::{
    dry,
    helper::error::KubeError
};

// Constant
const ARG_PATH: &str = "path";
const ARG_QUIET: &str = "quiet";
const ARG_UNRELEASED: &str = "unreleased";

/// Run
///
/// # Description
/// Dry run a TOML template by running the file in the cluster
/// This allow to check whenever the template contain any syntax error...
///
/// # Arguments
/// * `args` - &ArgMatches
///
/// # Return
/// Result<(), CError>
pub fn run(args: &ArgMatches) -> Result<(), CError> {
    let path = args.value_of(ARG_PATH)
        .ok_or_else(|| CError::from(TypeError::MissingArg(ARG_PATH)))?;
    
    let quiet = args.is_present(ARG_QUIET);
    let unreleased = args.is_present(ARG_UNRELEASED);
    let logger = Logger::new(quiet);

    let generated_yaml = super::generate::generate_yaml_from_toml(path, &logger)?;

    // spawning a runtime
    let rt = Runtime::new()?;
    let res = rt.block_on(dry_run_specs(&generated_yaml, unreleased))
        .map_err(|err| CError { message: err.message })?;

    for (name, dry_res) in res.into_iter() {
        match dry_res {
            Ok(_) => {
                logger.print(LogLevel::Success(&format!("Template {} is valid ✅", name)));
            },
            Err(err) => {
                logger.print(LogLevel::Warning(&format!("Template {} is not valid ⚠️", name)));
                logger.print(LogLevel::Warning(&err.message));
            }
        }
    }

    Ok(())
}

/// Dry Run Spec
///
/// # Description
/// Dry run the templates with the Kubernetes cluster
///
/// # Arguments
/// * `specs` - &HashMap<String, String>
///
/// # Return
/// Result<HashMap<String, Result<String, KubeError>>, KubeError>
async fn dry_run_specs(specs: &HashMap<String, String>, unreleased: bool) -> Result<HashMap<String, Result<(), KubeError>>, KubeError> {
    let mut m = HashMap::new();
    
    for (name, content) in specs.into_iter() {
        if unreleased{
            let res = dry::dry_run_create(content).await;
            m.insert(name.to_owned(), res);
        } else {
            let res = dry::dry_run(content).await;
            m.insert(name.to_owned(), res);
        }
    }

    Ok(m)
}