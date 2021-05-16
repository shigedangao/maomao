use serde::Deserialize;
use serde_json::Value;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{
    Api,
    Patch,
    PatchParams,
    GroupVersionKind,
    DynamicObject
};
use kube::Client;
use super::helper::error::{
    KubeError,
    dry_run::Error
};

mod util;

// Constant
const API_VERSION_SEPARATOR: &str = "/";
const PATCH_PARAM_MANAGER: &str = "maomao";
const DEFAULT_NS: &str = "default";

#[derive(Deserialize)]
pub struct Extract {
    #[serde(rename(deserialize = "apiVersion"))]
    api_version: String,
    kind: String,
    namespace: Option<String>,
    metadata: ObjectMeta
}

/// Get Gvk
///
/// # Description
/// Retrieve a GroupVersionKind which will be used to generate a DynamicObject
///
/// # Arguments
/// * `extract` - &Extract
///
/// # Return
/// Result<GroupVersionKind, KubeError>
fn get_gvk(extract: &Extract) -> Result<GroupVersionKind, KubeError> {
    // split the apiVersion to retrieve the apiGroup and the version
    // Usually it's represent by <apigroup>/<version>
    let args: Vec<&str> = extract.api_version.split(API_VERSION_SEPARATOR).collect();
    // Retrieve the api_group and the version
    let api_group = args.get(0);
    let api_version = args.get(1);

    if let (Some(group), Some(version)) = (api_group, api_version) {
        let gvk = GroupVersionKind::gvk(group, version, &extract.kind)
            .map_err(util::parse_kube_error)?;
            
        return Ok(gvk)
    }

    if let Some(group) = api_group {
        let gvk = GroupVersionKind::gvk("", group, &extract.kind)
            .map_err(util::parse_kube_error)?;
        
        return Ok(gvk);
    }
    
    Err(KubeError::from(Error::MissingApiVersion))
}

/// Remove unwanted field and Stringify
///
/// # Description
/// Remove field that we don't want to diff and return a YAML value
///
/// # Arguments
/// * `res` - DynamicObject
///
/// # Return
/// Result<String, KubeError>
fn remove_unwanted_field_and_stringify(mut res: DynamicObject) -> Result<String, KubeError> {
    res.data["status"].take();
    res.metadata.managed_fields.take();
    res.metadata.creation_timestamp.take();
    res.metadata.resource_version.take();
    res.metadata.uid.take();
    
    let yaml = serde_yaml::to_string(&res)?;
    Ok(yaml)
}  

/// Clear Dynamic Object
///
/// # Description
/// /!\ Usually with Patch merge the managedField should be cleared. However it appear that sometimes
/// the managedField is still present which is not ideal for future diff. This method make sure
/// to reset the managedField by setting an empty Vec to the metadata.namanged_field
/// See https://kubernetes.io/docs/reference/using-api/server-side-apply/#clearing-managedfields
///
/// # Arguments
/// * `client` - Client
/// * `content` - &str
/// * `name` - &str
///
/// # Return
/// Result<(), KubeError>
async fn clear_dynamic_object(client: Client, content: &str, name: &str) -> Result<(), KubeError> {
    let extract: Extract = serde_yaml::from_str(content)?;
    // get the patch params
    let pp = PatchParams::apply(PATCH_PARAM_MANAGER);
    // get the gvk
    let gvk = get_gvk(&extract)?;

    // get & edit metadata
    let ns = extract.namespace.unwrap_or_else(|| DEFAULT_NS.to_owned());
    let mut metadata = extract.metadata;
    metadata.managed_fields = Some(Vec::new());
    
    // create a Patch that remove the managedField metadata
    let patch_json = serde_json::json!({
        "apiVersion": extract.api_version,
        "kind": extract.kind,
        "metadata": metadata
    });

    let patch = Patch::Merge(patch_json);
    let dynamic: Api<DynamicObject> = Api::namespaced_with(client, &ns, &gvk);
    let res = dynamic.patch(name, &pp, &patch)
        .await
        .map_err(util::parse_kube_error)?;

    if res.metadata.managed_fields.is_some() {
        return Err(KubeError::from(Error::RemoveManagedField(name)));
    }
    
    Ok(())
} 

/// Dry RUn
///
/// # Description
/// Each generated yaml file will be processed by the Kubernetes APIServer in the 
/// `dry_run` mode. The dry_run mode allow the APIServer to show what will be applied by the cluster
/// by using a Patch::Merge. The result is a standard serde_json::Value.
///
/// This serde_json value will be convert to YAML. We'll then diff the generated yaml value
/// from the original generated value
///
/// /!\ Need to wait for a new release of kube-rs following the merge of the PR#512
///
/// # Arguments
/// * `content` - &str
///
/// # Return
/// Result<String, KubeError>
pub async fn dry_run(content: &str) -> Result<String, KubeError> {
    let client = Client::try_default()
        .await
        .map_err(|err| KubeError { message: err.to_string() })?;
    
    // Extract some values from the yaml
    let extract: Extract = serde_yaml::from_str(content)?;

    let json = serde_yaml::from_str::<Value>(content)?.to_string();
    let patch: Value = serde_json::from_str(&json)?;
    let patch = Patch::Merge(&patch);

    // patch params define the way the patch will be run
    let patch_params = PatchParams::apply(PATCH_PARAM_MANAGER).dry_run();
    let gvk = get_gvk(&extract)?;

    // Retrieve the resource from the Cluster as a DynamicObject
    let d: Api<DynamicObject> = Api::namespaced_with(
        client.clone(), 
        &extract.namespace.unwrap_or_else(|| DEFAULT_NS.to_owned()), 
        &gvk
    );

    // get the name from the metadata
    let name = extract.metadata.name;
    if name.is_none() {
        return Err(KubeError::from(Error::MissingSpecName));
    }

    let name = name.unwrap();
    let res = d.patch(&name, &patch_params, &patch)
        .await
        .map_err(util::parse_kube_error)?;

    // clear the managed_field in case if it's not already done
    if res.metadata.managed_fields.is_some() {
        // clear the dynamic object of it's managedField
        clear_dynamic_object(client, content, &name).await?;
    }
    
    remove_unwanted_field_and_stringify(res)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn expect_to_run_dry_run() {
        let yaml = r#"     
        apiVersion: apps/v1
        kind: Deployment
        metadata:
          labels:
            name: rusty
            tier: backend
          name: rusty
        spec:
          replicas: 5
        "#;

        let res = super::dry_run(yaml).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn expect_to_return_error() {
        let yaml = r#"     
        apiVersion: apps/v1
        kind: Deployment
        metadata:
          labels:
            name: rusty
            tier: backend
          name: rusty
        spec:
          replicas: foo
        "#;

        let res = super::dry_run(yaml).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn expect_to_run_dry_run_only_version() {
        let yaml = r#"
        apiVersion: v1
        kind: Service
        metadata:
            annotations:
              external-dns.alpha.kubernetes.io/hostname: rusty.dev.org.
            labels:
              name: rusty
              tier: backend
            name: rusty
        spec:
            ports:
                - name: http
                  port: 90
        "#;

        let res = super::dry_run(yaml).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn expect_to_fail_unknown_object() {
        let yaml = r#"
        apiVersion: v1
        kind: Service
        metadata:
            labels:
                name: foo
            name: foo
        spec:
            ports:
                - name: http
                  port: 80
        "#;

        let res = super::dry_run(yaml).await;
        assert!(res.is_err());
        let msg = res.unwrap_err();

        assert!(msg.message.contains("code: 404"));
    }
}