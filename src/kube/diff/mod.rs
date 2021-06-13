use kube::{Client, Api};
use kube::api::DynamicObject;
use crate::kube::helper::error::{
    KubeError,
    dry_run::Error as KubeRuntimeError
};
use super::common::{
    Extract,
    get_gvk,
    parse_kube_error
};

const DEFAULT_NS: &str = "default";

/// Get Current Spec
///
/// # Description
/// Retrieve the current spec from an existing resource and transform it in String
///
/// # Arguments
/// * `content` - &str
///
/// # Return
/// Result<String, KubeError>
pub async fn get_current_spec(content: &str) -> Result<String, KubeError> {
    let client = Client::try_default()
        .await
        .map_err(|err| KubeError { message: err.to_string() })?;

    // parse the generated yaml file
    let extract: Extract = serde_yaml::from_str(content)?;
    
    // get the dynamic group
    let gvk = get_gvk(&extract)?;
    
    // retrieve the name & namespace 
    let ns = extract.metadata.namespace
        .unwrap_or_else(|| DEFAULT_NS.to_owned());

    let name = extract.metadata.name
        .ok_or_else(|| KubeError::from(KubeRuntimeError::MissingSpecName))?;
        
    let dynamic: Api<DynamicObject> = Api::namespaced_with(client, &ns, &gvk);
    let mut res = dynamic.get(&name)
        .await
        .map_err(parse_kube_error)?;

    // remove un-necessary field
    res.data["status"].take();
    res.metadata.uid.take();
    res.metadata.resource_version.take();
    
    let yaml = serde_yaml::to_string(&res)?;

    Ok(yaml)
}