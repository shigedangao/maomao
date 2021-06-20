use k8s_openapi::api::core::v1::Affinity;
use crate::lib::parser::affinity::Affinity as ParserAffinity;

mod node;
mod pod;

pub struct AffinityWrapper {
    pub affinity: Affinity
}

impl AffinityWrapper {
    /// Create an AffinityWrapper which will be used to create:
    ///  - NodeAffinity
    ///  - PodAffinity
    ///
    /// # Return
    /// Self
    pub fn new() -> Self {
        AffinityWrapper {
            affinity: Affinity::default()
        }
    }

    /// Set the node affinity to the affinity wrapper
    ///
    /// # Arguments
    ///
    /// * `mut self` - Self
    /// * `affinity` - &ParserAffinity
    pub fn set_node_affinity(mut self, affinity: &ParserAffinity) -> Self {
        if let Some(aff) = affinity.node.to_owned() {
            let node_affinity = node::NodeAffinityWrapper::new()
                .set_required_aff(&aff)
                .set_preferred_aff(&aff);

            self.affinity.node_affinity = Some(node_affinity.affinity);
        }

        self
    }

    /// Set the Pod Affinity to the wrapper
    ///
    /// # Arguments
    ///
    /// * `mut self` - Self
    /// * `affinity` - &ParserAffinity
    pub fn set_pod_affinity(mut self, affinity: &ParserAffinity) -> Self {
        if let Some(aff) = affinity.pod.to_owned() {
            let pod_affinity = pod::PodAffinityWrapper::new()
                .set_required_aff(&aff)
                .set_preferred_aff(&aff);

            self.affinity.pod_affinity = Some(pod_affinity.affinity);
        }

        self
    }
}