use serde_json::Value;
use kube::api::{Api, DynamicObject, Patch, PatchParams, PostParams};
use kube::Client;
use super::common::{
    Extract,
    get_gvk,
    parse_kube_error
};
use super::helper::error::{
    KubeError,
    dry_run::Error
};

// Constant
const PATCH_PARAM_MANAGER: &str = "maomao";
const DEFAULT_NS: &str = "default";

/// Clear Dynamic Object
///
/// # Description
/// /!\ Usually with Patch merge the managedField should be cleared. However it appear that sometimes
/// the managedField is sti:?ll present which is not ideal for future diff. This method make sure
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
    let mut metadata = extract.metadata;
    let ns = metadata.to_owned().namespace.unwrap_or_else(|| DEFAULT_NS.to_owned());
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
        .map_err(parse_kube_error)?;

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
pub async fn dry_run(content: &str) -> Result<(), KubeError> {
    let client = Client::try_default()
        .await
        .map_err(|err| KubeError { message: err.to_string() })?;
    
    // Extract some values from the yaml
    let extract: Extract = serde_yaml::from_str(content)?;
    // get the namespace from the metadata
    let metadata = extract.metadata.to_owned();
    let ns = metadata.namespace.unwrap_or_else(|| DEFAULT_NS.to_owned());

    let json = serde_yaml::from_str::<Value>(content)?.to_string();
    let patch: Value = serde_json::from_str(&json)?;
    let patch = Patch::Merge(&patch);

    // patch params define the way the patch will be run
    let patch_params = PatchParams::apply(PATCH_PARAM_MANAGER).dry_run();
    let gvk = get_gvk(&extract)?;

    // Retrieve the resource from the Cluster as a DynamicObject
    let d: Api<DynamicObject> = Api::namespaced_with(
        client.clone(), 
        &ns, 
        &gvk
    );

    // get the name from the metadata
    let name = extract.metadata.name
        .ok_or_else(|| KubeError::from(Error::MissingSpecName))?;

    let res = d.patch(&name, &patch_params, &patch)
        .await
        .map_err(parse_kube_error)?;

    // clear the managed_field in case if it's not already done
    if res.metadata.managed_fields.is_some() {
        // clear the dynamic object of it's managedField
        clear_dynamic_object(client, content, &name).await?;
    }
    
    Ok(())
}

/// Dry Run Create
///
/// # Description
/// Dry Run the TOML template but only used during the creation of the TOML template
///
/// # Arguments
/// * `content` - &str
///
/// # Return
/// Result<(), KubeError> 
pub async fn dry_run_create(content: &str) -> Result<(), KubeError> {
    let client = Client::try_default()
        .await
        .map_err(|err| KubeError { message: err.to_string() })?;

    // Extract some values from the yaml
    let extract: Extract = serde_yaml::from_str(content)?;
    let gvk = get_gvk(&extract)?;
    let ns = extract.metadata.namespace
        .unwrap_or_else(|| DEFAULT_NS.to_owned());

    let d: Api<DynamicObject> = Api::namespaced_with(client, &ns, &gvk);
    let mut pp = PostParams::default();
    pp.dry_run = true;

    let value: DynamicObject = serde_yaml::from_str(content)?;

    d.create(&pp, &value)
        .await
        .map_err(parse_kube_error)?;
    
    Ok(())
}

// These tests need at least the deployment.toml from the examples folder to be deploy
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn expect_to_run_dry_run() {
        let yaml = r#"     
        apiVersion: apps/v1
        kind: Deployment
        metadata:
          labels:
            name: nginx
            tier: backend
          name: nginx
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
            name: nginx
            tier: backend
          name: nginx
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
              name: nginx
              tier: backend
            name: nginx
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

    #[tokio::test]
    async fn expect_to_dry_run_unreleased() {
        let yaml = r#"     
        apiVersion: apps/v1
        kind: Deployment
        metadata:
            labels:
                name: nginx
                tier: backend
            name: nginx-unreleased
        spec:
            replicas: 5
            selector:
                matchLabels:
                    name: nginx
                    tier: backend
            template:
                metadata:
                    labels:
                        name: nginx
                        tier: backend
                spec:
                    containers:
                        - name: nginx
                          image: nginx
                          imagePullPolicy: Always   
        "#;

        let res = super::dry_run_create(yaml).await;
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn expect_to_not_dry_run_unreleased() {
        let yaml = r#"     
            apiVersion: apps/v1
            kind: Deployment
            metadata:
              labels:
                name: nginx
                tier: backend
              name: nginx-error
            spec:
              replicas: 5
              containers:
                - name: nginx
                  image: nginx
                  imagePullPolicy: Foo   
        "#;

        let res = super::dry_run_create(yaml).await;
        assert!(res.is_err());
    }
}