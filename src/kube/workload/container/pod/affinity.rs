use std::convert::From;
use k8s_openapi::api::core::v1::{
    Affinity,
    NodeAffinity,
    NodeSelectorTerm,
    NodeSelectorRequirement,
    NodeSelector,
    PreferredSchedulingTerm
};
use crate::lib::parser::affinity::{
    Affinity as ParserAffinity,
    Expression,
    PreferredAffinityDefinition
};

pub struct AffinityWrapper {
    pub affinity: Affinity
}

pub struct NodeSelectorTermWrapper {
    pub term: NodeSelectorTerm
}

impl AffinityWrapper {
    pub fn new() -> Self {
        AffinityWrapper {
            affinity: Affinity::default()
        }
    }

    pub fn set_node_affinity(mut self, affinity: &ParserAffinity) -> Self {
        if affinity.node.is_none() {
            return self;
        }

        let node_affinity = affinity.node.to_owned().unwrap();
        let mut aff = NodeAffinity::default();
        if let Some(required) = node_affinity.required {
            if !required.expressions.is_empty() {
                // One matchExpression but multiple key (and)
                // nodeSelectorTerms:
                //  - matchExpressions:
                //    - key: ...
                //    - key: ...
                let nodes_selector_term = NodeSelectorTermWrapper::new(required.expressions);
                aff.required_during_scheduling_ignored_during_execution = Some(NodeSelector {
                    node_selector_terms: vec![nodes_selector_term.term]
                });

            } else if !required.with_multiple_expressions.is_empty() {
                // Multiple matchExpression (or)
                // nodeSelectorTerms:
                // - matchExpressions:
                //    - key: ...
                // - matchExpressions:
                //    - key: ...
                let node_selector_terms = NodeSelectorTermWrapper::set_with_multiple_terms(required.with_multiple_expressions);
                aff.required_during_scheduling_ignored_during_execution = Some(NodeSelector {
                    node_selector_terms
                })
            }
        }

        if let Some(preferred) = node_affinity.preferred {
            let preferred_scheduling_term = preferred
                .into_iter()
                .map(|(_, p)| PreferredSchedulingTerm::from(p))
                .collect::<Vec<PreferredSchedulingTerm>>();

            aff.preferred_during_scheduling_ignored_during_execution = preferred_scheduling_term;
        }

        self.affinity.node_affinity = Some(aff);

        self
    }
}

impl NodeSelectorTermWrapper {
    /// New
    ///
    /// # Description
    /// Create a NodeSelectorTermWrapper
    ///
    /// # Arguments
    /// * `exp` - Vec<Expression>
    ///
    /// # Return
    /// Self
    fn new(exp: Vec<Expression>) -> Self {
        let terms = exp
            .into_iter()
            .map(NodeSelectorRequirement::from)
            .collect::<Vec<NodeSelectorRequirement>>();

        let t = NodeSelectorTerm {
            match_expressions: terms,
            match_fields: Vec::new()
        };

        NodeSelectorTermWrapper {
            term: t
        }
    }

    fn set_with_multiple_terms(exps: Vec<Vec<Expression>>) -> Vec<NodeSelectorTerm> {
        let terms: Vec<NodeSelectorTerm> = exps
            .into_iter()
            .map(|exp| Self::new(exp))
            .map(|t| t.term)
            .collect();

        terms
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

impl From<PreferredAffinityDefinition> for PreferredSchedulingTerm {
    fn from(pref: PreferredAffinityDefinition) -> Self {
        let selector_term = NodeSelectorTermWrapper::new(pref.expression);
        PreferredSchedulingTerm {
            preference: selector_term.term,
            weight: pref.weight.unwrap_or_default()
        }
    }
}