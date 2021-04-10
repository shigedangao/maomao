use std::fs;
use crate::cli::helper::error::CError;

/// Read Files To String
///
/// # Description
/// Read the content of a folder based on the provided path
///
/// # Arguments
/// * `path` - &str
///
/// # Return
/// * `Result<Vec<String>, CError>`
pub fn read_files_to_string(path: &str) -> Result<Vec<String>, CError> {
    let mut templates = Vec::new();
    let dir = fs::read_dir(path)?;

    for entry in dir {
        let entry = entry?;

        let path = entry.path();
        if path.is_file() {
            let tmpl = fs::read_to_string(path)?;

            templates.push(tmpl);
        }
    }

    Ok(templates)
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
    fs::write(path, contents)
        .map_err(|err| CError::from(err))
}