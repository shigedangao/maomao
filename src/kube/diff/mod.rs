use kube::{Client, Api};
use kube::api::DynamicObject;
use std::collections::BTreeMap;
use crate::kube::helper::error::{
    KubeError,
    dry_run::Error as KubeRuntimeError
};
use super::common::{
    Extract,
    get_api_resource,
    parse_kube_error
};

// Constant
const DEFAULT_NS: &str = "default";

// Annotation constant that need to be remove
const K8S_REVISION: &str = "deployment.kubernetes.io/revision";
const KUBECTL_LAST_CONFIG: &str = "kubectl.kubernetes.io/last-applied-configuration";

/// Clean Annotations
///
/// # Arguments
/// * `annotatuons` - &mut BTreeMap<String, String>
fn clean_annotations(annotations: &mut BTreeMap<String, String>) {
    annotations.remove(K8S_REVISION);
    annotations.remove(KUBECTL_LAST_CONFIG);
}

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
    
    // get ApiResource
    let api_res = get_api_resource(&extract)?;
    
    // retrieve the name & namespace 
    let ns = extract.metadata.namespace
        .unwrap_or_else(|| DEFAULT_NS.to_owned());

    let name = extract.metadata.name
        .ok_or_else(|| KubeError::from(KubeRuntimeError::MissingSpecName))?;
        
    let dynamic: Api<DynamicObject> = Api::namespaced_with(client, &ns, &api_res);
    let mut res = dynamic.get(&name)
        .await
        .map_err(parse_kube_error)?;

    // remove un-necessary field
    res.data["status"].take();
    res.metadata.uid.take();
    res.metadata.resource_version.take();
    res.metadata.creation_timestamp.take();
    
    clean_annotations(&mut res.metadata.annotations);    
    let yaml = serde_yaml::to_string(&res)?;

    Ok(yaml)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn expect_to_retrieve_spec() {
        let yaml = r#"
        apiVersion: apps/v1
        kind: Deployment
        metadata:
            labels:
                name: nginx
                tier: backend
            name: nginx
        "#;

        let spec = super::get_current_spec(yaml).await;
        assert!(spec.is_ok());
    }

    #[tokio::test]
    async fn expect_to_not_retrieve_spec() {
        let yaml = r#"
        apiVersion: apps/v1
        kind: Deployment
        metadata:
            labels:
                name: foo
                tier: backend
            name: foo
        "#;

        let spec = super::get_current_spec(yaml).await;
        assert!(spec.is_err());
    }
}