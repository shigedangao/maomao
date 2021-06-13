use std::collections::HashMap;
use clap::ArgMatches;
use tokio::runtime::Runtime;
use similar::{ChangeTag, TextDiff};
use termion::color;
use crate::cli::helper::error::{
    CError,
    TypeError,
};
use crate::cli::helper::logger::{Logger, LogLevel};
use crate::kube::{
    diff,
    helper::error::KubeError
};

const ARG_PATH: &str = "path";
const ARG_QUIET: &str = "quiet";

/// Run
///
/// # Description
/// Diff generated TOML template with the cluster
///     - Convert the TOML to YAML
///     - Get existing spec & convert to YAML
///     - Diff the generated YAML with the one from the Server
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
    let logger = Logger::new(quiet);

    let generated_yaml = super::generate::generate_yaml_from_toml(path, &logger)?;

    // generate a runtime in order to get the dry_run values
    logger.print(LogLevel::Warning("Retrieving existing spec from the cluster..."));
    let rt = Runtime::new()?;
    let res = rt.block_on(get_existing_spec(generated_yaml.clone(), &logger))
        .map_err(|err| CError { message: err.message })?;

    // compare the spec
    // generate diff for each files
    for (name, content) in res {
        logger.print(LogLevel::Info(&format!("🔎 Diff file {}.toml", name)));
        let original_spec = generated_yaml.get(&name)
            .ok_or_else(|| CError::from(
                TypeError::MissingRes("Unable to get the YAML spec")
            ))?;

        let diff = TextDiff::from_lines(&content,original_spec);
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Insert => print!("{}+{}", color::Fg(color::Green), change),
                ChangeTag::Delete => print!("{}-{}", color::Fg(color::Red), change),
                ChangeTag::Equal => print!("{} {}", color::Fg(color::White), change),
            };
        }
    }

    Ok(())
}

/// Get existing spec
///
/// # Arguments
/// * `yaml` - HashMap<String, String>
///
/// # Return
/// impl Future<Output = Result<HashMap<String, String>, KubeError>>
async fn get_existing_spec(yaml: HashMap<String, String>, logger: &Logger) -> Result<HashMap<String, String>, KubeError> {
    let mut dr = HashMap::new();
    for (name, content) in yaml {
        let res = diff::get_current_spec(&content).await;
        if let Err(err) = res {
            return Err(err);
        }
        
        logger.print(LogLevel::Info(&format!("🪞 Spec retrieved for {}.toml", name)));
        dr.insert(name.to_owned(), res.unwrap().to_owned());
    }

    Ok(dr)
}
