use std::collections::{HashMap, BTreeMap};
use k8s_openapi::api::core::v1::{
    PersistentVolumeClaim,
    PersistentVolumeClaimSpec,
    TypedLocalObjectReference,
    ResourceRequirements
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use crate::lib::parser;

struct PvcWrapper {
    pvc: PersistentVolumeClaim
}

impl PvcWrapper {
    fn new(parser_pvc: &parser::volume::VolumeClaimTemplates) -> Self {
        let pvc = PersistentVolumeClaim {
            metadata: ObjectMeta { annotations: Some(parser_pvc.metadata.to_owned()), ..Default::default() },
            ..Default::default()
        };

        PvcWrapper {
            pvc
        }
    }

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
                limits: get_btree_quantity_from_hashmap(resources.limit),
                requests: get_btree_quantity_from_hashmap(resources.request)
            };

            spec.resources = Some(req);
        }

        self.pvc.spec = Some(spec);
        self
    }
}


fn get_btree_quantity_from_hashmap(map: Option<HashMap<String, String>>) -> Option<BTreeMap<String, Quantity>> {
    if let Some(m) = map {
        let converted = m.into_iter()
            .map(|(k, v)| (k, Quantity(v)))
            .collect();

        return Some(converted);
    }

    None
}

fn get_typed_local_object_reference(m: Option<HashMap<String, String>>) -> Option<TypedLocalObjectReference> {
    if m.is_none() {
        return None;
    }
    
    let m = m.unwrap();
    let kind = m.get("kind");
    let name = m.get("name");

    if kind.is_none() || name.is_none() {
        return None;
    }

    Some(TypedLocalObjectReference {
        api_group: None,
        kind: kind.unwrap().to_owned(),
        name: name.unwrap().to_owned()
    })
}


pub fn get_pvc_list(object: &parser::Object) -> Option<Vec<PersistentVolumeClaim>> {
    if object.volume_claim.is_none() {
        return None;
    }

    let volume_claim = object.volume_claim.to_owned().unwrap();
    let claims = volume_claim
        .into_iter()
        .map(|(_, p)| PvcWrapper::new(&p).set_spec(&p))
        .map(|p| p.pvc)
        .collect::<Vec<PersistentVolumeClaim>>();

    Some(claims)
}