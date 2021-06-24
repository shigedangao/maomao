use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::cli::helper::error::CError;
use super::error::TypeError;

// Error constant
const NO_YAML: &str = "Target path is not of type .yaml";

/// Read Files To String
///
/// # Description
/// Read the content of a folder based on the provided path
///
/// # Arguments
/// * `path` - &str
///
/// # Return
/// * `Result<HashMap<String, String>, CError>`
pub fn read_files_to_string(path: &str) -> Result<(HashMap<String, String>, Option<String>), CError> {
    let mut templates = HashMap::new();
    let mut variables = None;
    let dir = fs::read_dir(path)?;

    for entry in dir {
        let entry = entry?;

        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.clone().file_stem().and_then(|u| u.to_str()) {
                let tmpl = fs::read_to_string(path)?;
                if name != "_vars" {
                    templates.insert(name.to_owned(), tmpl);
                } else {
                    variables = Some(tmpl);
                }
            }
        }
    }

    Ok((templates, variables))
}

/// Write File
///
/// # Description
/// Write the contents on a targeted file
///
/// # Arguments
/// * `path` - &str
/// * `contents` - &str
///
/// # Return
/// Result<(), CError>
pub fn write_file(path: &str, contents: &str) -> Result<(), CError> {
    // Check that the filename is at least a yaml
    let p = PathBuf::from(path);
    if let Some(ext) = p.extension() {
        if !ext.eq("yaml") {
            return Err(CError::from(TypeError::Io(NO_YAML)));
        }
    }

    fs::write(path, contents)
        .map_err(CError::from)
}

/// Write Multiple Files
///
/// # Description
/// Write the contents on multiple files
///
/// # Arguments
/// * `path` - &str
/// * `map` - HashMap<String, String>
///
/// # Return
/// Result<(), CError>
pub fn write_multiple_files(path: &str, map: HashMap<String, String>) -> Result<(), CError> {
    // Check that the path is a directory
    let p = PathBuf::from(path);
    if !p.is_dir() {
        fs::create_dir(p)?;
    }

    for (name, content) in map {
        let mut concat_path = PathBuf::from(path);
        concat_path.push(format!("{}.yaml", name));

        fs::write(concat_path, content)
            .map_err(CError::from)?;
    }

    Ok(())
}