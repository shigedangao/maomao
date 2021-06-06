use k8s_openapi::api::core::v1::{
    PersistentVolumeClaim,
    PersistentVolumeClaimSpec,
    TypedLocalObjectReference,
    ResourceRequirements
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use crate::lib::{
    parser,
    parser::volume::DataSource
};
use crate::kube::helper;

struct PvcWrapper {
    pvc: PersistentVolumeClaim
}

impl PvcWrapper {
    /// New
    ///
    /// # Description
    /// Create a new PvcWrapper
    ///
    /// # Arguments
    /// * `parser_pvc` - &parser::volume::VolumeClaimTemplates
    ///
    /// # Return
    /// Self
    fn new(parser_pvc: &parser::volume::VolumeClaimTemplates) -> Self {
        let metadata = parser_pvc.metadata.to_owned();
        let mut pvc = PersistentVolumeClaim {
            metadata: ObjectMeta {
                annotations: Some(metadata.clone()),
                ..Default::default()
            },
            ..Default::default()
        };

        if let Some(m) = metadata.get("name") {
            pvc.metadata.name = Some(String::from(m));
        }

        PvcWrapper {
            pvc
        }
    }

    /// Set Spec
    ///
    /// # Description
    /// Set the spec of a PersistentVolumeClaim
    ///
    /// # Arguments
    /// * `mut self` - self
    /// * `parser_pvc` - &parser::volume::VolumeClaimTemplates
    ///
    /// # Return
    /// Self
    fn set_spec(mut self, parser_pvc: &parser::volume::VolumeClaimTemplates) -> Self {
        let mut spec = PersistentVolumeClaimSpec{
            ..Default::default()
        };

        if let Some(desc) = parser_pvc.description.to_owned() {
            spec.access_modes = desc.access_modes;
            spec.storage_class_name = desc.class_name;
            spec.volume_mode = desc.mode;
            spec.volume_name = desc.name;
            spec.data_source = get_typed_local_object_reference(desc.data_source);
        }

        if let Some(resources) = parser_pvc.resources.to_owned() {
            let req = ResourceRequirements {
                limits: helper::get_btree_quantity_from_hashmap(resources.limit),
                requests: helper::get_btree_quantity_from_hashmap(resources.request)
            };

            spec.resources = Some(req);
        }

        self.pvc.spec = Some(spec);
        self
    }
}

/// Get Typed Local Object Reference
///
/// # Description
/// Create a TypedLocalObjectRefererence from a HashMap which represent a set of field & value
/// i.e: { kind = "foo", name = "bar" }
///
/// # Arguments
/// * `m` - Option<DataSource>
///
/// # Return
/// Option<TypedLocalObjectReference>
fn get_typed_local_object_reference(m: Option<DataSource>) -> Option<TypedLocalObjectReference> {
    if let Some(data_source) = m {
        return Some(TypedLocalObjectReference {
            api_group: None,
            kind: data_source.kind.unwrap_or_default(),
            name: data_source.name.unwrap_or_default()
        });
    }

    None
}

/// Get Pvc List
///
/// # Arguments
/// * `object` - &parser::Object
///
/// # Return
/// Option<Vec<PersistentVolumeClaim>>
pub fn get_pvc_list(object: &parser::Object) -> Option<Vec<PersistentVolumeClaim>> {
    let volume_claim = object.volume_claim.as_ref()?;
    let claims = volume_claim
        .iter()
        .map(|(_, p)| PvcWrapper::new(&p).set_spec(&p))
        .map(|p| p.pvc)
        .collect::<Vec<PersistentVolumeClaim>>();

    Some(claims)
}