use toml::Value;
use serde::Serialize;
use crate::lib::helper::error::{
    LError,
    crd::Error
};

#[derive(Debug, Clone, Default, Serialize)]
pub struct CustomCrd {
    pub spec: Option<Value>
}

impl CustomCrd {
    /// New
    ///
    /// # Description
    /// Create a new CustomCrd
    ///
    /// # Description
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn new(ast: &Value) -> Self {
        CustomCrd { spec: Some(ast.to_owned()) }
    }
}

/// Get Custom Crd
///
/// # Description
/// Get the CustomCrd wrapper
///
/// # Arguments
/// * `ast` - &Value
///
/// # Return
/// Result<CustomCrd, LError>
pub fn get_custom_crd(ast: &Value) -> Result<CustomCrd, LError> {
    let crd = ast.get("spec")
        .ok_or_else(|| LError::from(Error::SpecNotFound))?;

    Ok(CustomCrd::new(crd))
}

#[cfg(test)]
mod test {
    use toml::Value;

    #[test]
    fn expect_to_parse_custom_workload() {
        let template = r#"
        kind = "custom::ManagedCertificate"
        version = "networking.gke.io/v1"
        metadata = { name = "rusty-certificate" }

        [spec]
            domains = [
                "rusty-dev.co.kr",
                "rusty-dyn.co.kr"
            ]
        "#;

        let ast = template.parse::<Value>().unwrap();
        let custom = super::get_custom_crd(&ast);

        assert!(custom.is_ok());
    }

    #[test]
    fn expect_to_fail_spec_not_found() {
        let template = r#"
        kind = "custom::ManagedCertificate"
        version = "networking.gke.io/v1"
        metadata = { name = "rusty-certificate" }

        [foo]
            domains = [
                "rusty-dev.co.kr",
                "rusty-dyn.co.kr"
            ]
        "#;

        let ast = template.parse::<Value>().unwrap();
        let custom = super::get_custom_crd(&ast);

        assert!(custom.is_err());
    }
}