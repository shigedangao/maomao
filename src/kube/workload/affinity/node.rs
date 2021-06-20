use std::convert::From;
use k8s_openapi::api::core::v1::{
    NodeSelectorTerm,
    NodeSelectorRequirement,
    PreferredSchedulingTerm,
    NodeAffinity,
    NodeSelector
};
use crate::lib::parser::affinity::{
    Expression,
    PreferredAffinityDefinition,
    RequiredAffinityDefinition,
    AffinityType
};

pub struct NodeAffinityWrapper {
    pub affinity: NodeAffinity
}

trait NodeAffinityTrait {
    fn get_exp(&self) -> Vec<Expression>;

    /// Get NodeSelectorTerem from Vec<Expression>
    ///
    /// # Arguments
    ///
    /// * `&self` - impl
    fn get_node_selector_term(&self) -> NodeSelectorTerm {
        let terms = self.get_exp()
            .into_iter()
            .map(NodeSelectorRequirement::from)
            .collect::<Vec<NodeSelectorRequirement>>();

        NodeSelectorTerm {
            match_expressions: terms,
            ..Default::default()
        }
    }
}


impl NodeAffinityWrapper {
    /// New
    ///
    /// # Description
    /// Create a New NodeAffinityWrapper
    ///
    /// # Return
    /// Self
    pub fn new() -> Self {
        NodeAffinityWrapper {
            affinity: NodeAffinity::default()
        }
    }

    /// Set Required Aff
    ///
    /// # Description
    /// Set Required Affinity to NodeAffinityWrapper
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `affinity` - &AffinityType
    ///
    /// # Return
    /// Self
    pub fn set_required_aff(mut self, affinity: &AffinityType) -> Self {
        if let Some(required) = affinity.required.to_owned() {
            let defs = required
                .into_iter()
                .map(|(_, v)| v.get_node_selector_term())
                .collect::<Vec<NodeSelectorTerm>>();

            let node_selector = NodeSelector {
                node_selector_terms: defs
            };

            self.affinity.required_during_scheduling_ignored_during_execution = Some(node_selector);
        }

        self
    }

    /// Set Preferred Aff
    ///
    /// # Description
    /// Set Preferred Affinity to NodeAffinityWrapper
    ///
    /// # Arguments
    /// * `mut self` - Self
    /// * `affinity` - &AffinityType
    ///
    /// # Return
    /// Self
    pub fn set_preferred_aff(mut self, affinity: &AffinityType) -> Self {
        if let Some(preferred) = affinity.preferred.to_owned() {
            let defs = preferred
                .into_iter()
                .map(|(_, v)| PreferredSchedulingTerm {
                    preference: v.get_node_selector_term(),
                    weight: v.weight.unwrap_or_default()
                })
                .collect::<Vec<PreferredSchedulingTerm>>();

            self.affinity.preferred_during_scheduling_ignored_during_execution = defs;
        }

        self
    }
}

impl From<Expression> for NodeSelectorRequirement {
    fn from(exp: Expression) -> Self {
        NodeSelectorRequirement {
            key: exp.key,
            operator: exp.operator,
            values: exp.values
        }
    }
}

impl NodeAffinityTrait for RequiredAffinityDefinition {
    fn get_exp(&self) -> Vec<Expression> {
        self.expressions.to_owned()
    }
}

impl NodeAffinityTrait for PreferredAffinityDefinition {
    fn get_exp(&self) -> Vec<Expression> {
        self.expressions.to_owned()
    }
}