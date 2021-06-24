use std::convert::From;
use k8s_openapi::api::core::v1::{
    PodAffinity,
    PodAffinityTerm,
    WeightedPodAffinityTerm
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{
    LabelSelector,
    LabelSelectorRequirement
};
use crate::lib::parser::affinity::{
    Expression,
    AffinityType,
    RequiredAffinityDefinition,
    PreferredAffinityDefinition,
};

trait PodAffinityTrait {
    fn get_exp(&self) -> Vec<Expression>;
    fn get_topology(&self) -> String;

    /// Get LabelSelector From a Vec<Expression>
    ///
    /// # Arguments
    ///
    /// * `&self` - impl
    fn get_label(&self) -> LabelSelector {
        let expressions: Vec<LabelSelectorRequirement> = self.get_exp()
            .into_iter()
            .map(LabelSelectorRequirement::from)
            .collect();

        LabelSelector {
            match_expressions: expressions,
            ..Default::default()
        }
    }

    /// Retrieve PodAffinityTerm from LabelSelector
    ///
    /// # Arguments
    /// 
    /// * `&self` - impl
    fn get_pod_affinity_term(&self) -> PodAffinityTerm {
        let selector: LabelSelector = self.get_label();

        PodAffinityTerm {
            label_selector: Some(selector),
            topology_key: self.get_topology(),
            ..Default::default()
        }
    } 
}

/// A wrapper around the PodAffinity
pub struct PodAffinityWrapper {
    pub affinity: PodAffinity     
}

impl PodAffinityWrapper {
    pub fn new() -> Self {
        PodAffinityWrapper {
            affinity: PodAffinity::default()
        }
    }

    /// Set the PodAffinity required field
    ///
    /// # Arguments
    ///
    /// * `mut self` - Self
    /// * `affinity` - &AffinityType
    pub fn set_required_aff(mut self, affinity: &AffinityType) -> Self {
        if let Some(required) = affinity.required.to_owned() {
            let defs = required
                .into_iter()
                .map(|(_, v)| v.get_pod_affinity_term())
                .collect::<Vec<PodAffinityTerm>>();
            
            self.affinity.required_during_scheduling_ignored_during_execution = defs;
        }

        self
    }

    /// Set PodAffinity preferred field
    ///
    /// # Arguments
    ///
    /// * `mut self` - Self
    /// * `affinity` - &AffinityType
    pub fn set_preferred_aff(mut self, affinity: &AffinityType) -> Self {
        if let Some(preferred) = affinity.preferred.to_owned() {
            let defs = preferred
                .into_iter()
                .map(|(_, v)| WeightedPodAffinityTerm {
                    pod_affinity_term: v.get_pod_affinity_term(),
                    weight: v.weight.unwrap_or_default()
                })
                .collect::<Vec<WeightedPodAffinityTerm>>();

            self.affinity.preferred_during_scheduling_ignored_during_execution = defs;
        }

        self
    }
}

impl From<Expression> for LabelSelectorRequirement {
    fn from(exp: Expression) -> Self {
        LabelSelectorRequirement {
            key: exp.key,
            operator: exp.operator,
            values: exp.values
        }
    }
}

impl PodAffinityTrait for RequiredAffinityDefinition {
    fn get_exp(&self) -> Vec<Expression> {
        self.expressions.to_owned()
    }

    fn get_topology(&self) -> String {
        self.topology.to_owned().unwrap_or_default()
    }
}

impl PodAffinityTrait for PreferredAffinityDefinition {
    fn get_exp(&self) -> Vec<Expression> {
        self.expressions.to_owned()
    }

    fn get_topology(&self) -> String {
        self.topology.to_owned().unwrap_or_default()
    }
}