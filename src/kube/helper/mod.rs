use std::collections::{HashMap, BTreeMap};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

pub mod error;


/// Get BTree Quantity From HashMap
///
/// # Description
/// Small utility method that is use to convert a HashMap<String, String> to a Option<BtreeMap<String, Quantity>>
///
/// # Arguments
/// * `map` - Option<HashMap<String, String>>
///
/// # Return
/// Option<BTreeMap<String, Quantity>>
pub fn get_btree_quantity_from_hashmap(map: Option<HashMap<String, String>>) -> BTreeMap<String, Quantity> {
    if let Some(m) = map {
        let converted = m.into_iter()
            .map(|(k, v)| (k, Quantity(v)))
            .collect();

        return converted;
    }

    let empty_map: BTreeMap<String, Quantity> = BTreeMap::new();
    empty_map
}