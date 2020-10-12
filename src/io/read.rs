use std::fs;
use std::path::{PathBuf};
use crate::helper::err::LibError;

// Constant
const OPERATION_KIND: &str = "read";

// Error message
pub const ERROR_DIRECTORY_NOT_FOUNDED: &str = "Directory does not exist";
pub const ERROR_FILE_NOT_FOUNDED: &str = "File does not exist";

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

pub mod templates {
    use std::fs;
    use std::path::{Path, PathBuf};
    use crate::helper::err::LibError;

    // Constant
    const TEMPLATE_FOLDER: &str = "/templates";

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
            .map(|f| super::read_file(f))
            .filter(|s| !s.is_err())
            .map(|s| s.unwrap())
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
                kind: super::OPERATION_KIND.to_owned(),
                message: super::ERROR_DIRECTORY_NOT_FOUNDED.to_owned()
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
}

mod config {
    use std::path::{PathBuf};
    use crate::helper::err::LibError;

    const CONFIG_FILE_PREFIX: &str = "maomao";

    /// Get Maomao env conf
    ///
    /// # Description
    /// Retrieve the configuration file for the environment
    ///
    /// # Arguments
    /// * `env` - Existing environment (i.e, dev, staging, prod)
    /// * `path` - Path of the file
    pub fn load_config(env: &str, path: &str) -> Result<String, LibError >{
        let file_name = [CONFIG_FILE_PREFIX, ".", env, ".toml"].concat();
        let file_path = [path, "/", file_name.as_str()].concat();

        let file_pathbuf = PathBuf::from(file_path);
        if !file_pathbuf.is_file() {
            return Err(LibError {
                kind: super::OPERATION_KIND.to_owned(),
                message: super::ERROR_FILE_NOT_FOUNDED.to_owned()
            })
        }

        super::read_file(&file_pathbuf)
    }
}

#[cfg(test)]
mod read_test {
    const TEMPLATE_EX_PATH: &str = "examples/node";

    #[test]
    fn read_success() {
        let templates = super::templates::read_templates(TEMPLATE_EX_PATH);
        assert!(!templates.is_err());

        let res = templates.unwrap();
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn read_wrong_dir() {
        let templates = super::templates::read_templates("");
        assert!(templates.is_err());

        let err = templates.unwrap_err();
        assert_eq!(err.message, super::ERROR_DIRECTORY_NOT_FOUNDED);
    }

    #[test]
    fn read_env_conf() {
        let config = super::config::load_config("dev", TEMPLATE_EX_PATH);
        assert!(!config.is_err());
        
        let content = config.unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn read_not_exist_conf() {
        let config = super::config::load_config("prod", TEMPLATE_EX_PATH);
        
        assert!(config.is_err());
        assert_eq!(config.unwrap_err().message, super::ERROR_FILE_NOT_FOUNDED);
    }
}