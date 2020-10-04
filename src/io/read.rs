use std::fs;
use std::path::{Path, PathBuf};
use crate::helper::err::LibError;

// Constant
const TEMPLATE_FOLDER: &str = "templates";
const OPERATION_KIND: &str = "read";

/// Read Templates
///
/// # Description
/// Read file from templates folder and return a string representation
///
/// # Arguments
/// * `base_path` - A string which represent the path of the maomao project
pub fn read_templates(base_path: &str) -> Result<Vec<String>, LibError> {
    let template_path = [base_path, TEMPLATE_FOLDER].concat();
    let dir = Path::new(&template_path);
    
    let files_path_res = get_template_file_path(dir);
    if let Err(err) = files_path_res {
        return Err(err);
    }

    let files_path = files_path_res.unwrap();
    let files_content = files_path
        .iter()
        .map(|f| read_file(f))
        .filter(|s| !s.is_err())
        .map(|s|s.unwrap())
        .collect::<Vec<String>>();

    Ok(files_content)
}

/// Get Template File Path
///
/// # Description
/// Retrieve template file path from folder
///
/// # Arguments
/// * `dir` - path representation of the templates folder
fn get_template_file_path(dir: &Path) -> Result<Vec<PathBuf>, LibError> {
    if !dir.is_dir() {
        return Err(LibError {
            kind: String::from(OPERATION_KIND),
            message: String::from("Directory does not exist")
        });
    }

    let res = fs::read_dir(dir);
    if let Err(err) = res {
        return Err(LibError::from(err));
    }

    let entries = res.unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter(|res| !res.is_err())
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();    

    Ok(entries)
}

/// Read File
///
/// # Description
/// Read a file and return a string value
///
/// # Arguments
/// * `path` 
fn read_file(path: &PathBuf) -> Result<String, LibError> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}