use std::collections::HashMap;
use clap::ArgMatches;
use tokio::runtime::Runtime;
use similar::{ChangeTag, TextDiff};
use termion::color;
use crate::cli::helper::error::{
    CError,
    TypeError
};
use crate::kube::{
    dry,
    helper::error::KubeError
};

const ARG_PATH: &str = "path";

/// Run
///
/// # Description
/// Diff generated TOML template with the Kubernetes APIServer
///
/// # Arguments
/// * `args` - &ArgMatches
///
/// # Return
/// Result<(), CError>
pub fn run(args: &ArgMatches) -> Result<(), CError> {
    let path = args.value_of(ARG_PATH)
        .ok_or_else(|| CError::from(TypeError::MissingArg(ARG_PATH.to_owned())))?;

    let generated_yaml = super::generate::template_variables(path)?;

    // generate a runtime in order to get the dry_run values
    let rt = Runtime::new()?;
    let res = rt.block_on(trigger_dry_run(generated_yaml.clone()))
        .map_err(|err| CError { message: err.message })?;

    // compare the spec
    // generate diff for each files
    for (name, content) in res {
        let original_spec = generated_yaml.get(&name)
            .ok_or_else(|| CError::from(
                TypeError::MissingRes("Unable to get the YAML spec".to_owned())
            ))?;

        let diff = TextDiff::from_lines(original_spec,&content);
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

/// Trigger Dry Run
///
/// # Description
/// Trigger the dry run by processing each template asynchronously
///
/// # Arguments
/// * `yaml` - HashMap<String, String>
///
/// # Return
/// impl Future<Output = Result<HashMap<String, String>, KubeError>>
async fn trigger_dry_run(yaml: HashMap<String, String>) -> Result<HashMap<String, String>, KubeError> {
    let mut dr = HashMap::new();
    for (name, content) in yaml {
        let res = dry::dry_run(&content).await;
        if let Err(err) = res {
            return Err(err);
        }
        
        dr.insert(name.to_owned(), res.unwrap().to_owned());
    }

    Ok(dr)
}
