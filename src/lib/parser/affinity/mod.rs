use std::collections::HashMap;
use toml::Value;
use crate::lib::helper::conv::Convert;
use crate::lib::helper::toml::get_value_for_t_lax;

enum AffinityKind {
    Node,
    Pod
}

#[derive(Debug, Default, Clone)]
pub struct Affinity {
    pub node: Option<AffinityType>,
    pub pod: Option<AffinityType>
}

#[derive(Debug, Default, Clone)]
pub struct AffinityType {
    pub required: Option<RequiredAffinityDefinition>,
    pub preferred: Option<HashMap<String, PreferredAffinityDefinition>>
}

#[derive(Debug, Default, Clone)]
pub struct RequiredAffinityDefinition {
    pub expressions: Vec<Expression>,
    pub with_multiple_expressions: Vec<Vec<Expression>>
}

#[derive(Debug, Default, Clone)]
pub struct PreferredAffinityDefinition {
    pub expression: Vec<Expression>,
    pub weight: Option<i32>
}

#[derive(Debug, Default, Clone)]
pub struct Expression {
    pub key: String,
    pub operator: String,
    pub values: Vec<String>
}

impl Affinity {
    /// New
    ///
    /// # Description
    /// Create a new Affinity
    ///
    /// # Return
    /// Affinity
    fn new() -> Self {
        Affinity {
            ..Default::default()
        }
    }

    /// Set Affinity
    ///
    /// # Description
    /// Set the kind of affinity, node, pod
    ///
    /// # Arguments
    /// * `&mut self` - Self
    /// * `ast` - &Value
    /// * `kind` - AffinityKind
    ///
    /// # Return
    /// Self
    fn set_affinity(&mut self, ast: &Value, kind: AffinityKind) {
        let mut affinity_type = AffinityType::default();
        if let Some(value) = ast.get("required") {
            affinity_type.required = RequiredAffinityDefinition::new(value);
        }

        if let Some(value) = ast.get("preferred") {
            if let Some(map) = value.as_table() {
                let defs: HashMap<String, PreferredAffinityDefinition> = map
                    .into_iter()
                    .map(|(n, v)| (n.to_owned(), PreferredAffinityDefinition::new(v)))
                    .collect();    
                    
                affinity_type.preferred = Some(defs);
            }
        }

        match kind {
            AffinityKind::Node => self.node = Some(affinity_type),
            AffinityKind::Pod => self.pod = Some(affinity_type)
        }
    }
}

impl RequiredAffinityDefinition {
    /// New
    ///
    /// # Description
    /// Create a new AffinityDefinition
    ///
    /// # Arguments
    /// * `ast` - &Value
    ///
    /// # Return
    /// Option<Self>
    fn new(ast: &Value) -> Option<Self> {
        let mut def = RequiredAffinityDefinition::default();
        if let Some(exp) = ast.get("expressions") {
            def.expressions = Expression::as_vec(exp);
        }

        if let Some(with_multiple_exp) = ast.get("with_multiple_expressions") {
            if let Some(arr) = with_multiple_exp.as_array() {
                let multi_vec: Vec<Vec<Expression>> = arr
                    .into_iter()
                    .map(|v| Expression::as_vec(v))
                    .collect();

                def.with_multiple_expressions = multi_vec;
            }
        }

        if def.expressions.is_empty() && def.with_multiple_expressions.is_empty() {
            return None;
        }

        Some(def)
    } 
}

impl PreferredAffinityDefinition {
    /// New
    ///
    /// # Description
    /// Create a new PreferredAffinityDefinition
    ///
    /// # Arguments
    /// * `ast` - &Value
    ///
    /// # Return
    /// Self
    fn new(ast: &Value) -> Self {
        let mut def = PreferredAffinityDefinition::default();
        if let Some(exp) = ast.get("expressions") {
            def.expression = Expression::as_vec(exp);
        }
        
        def.weight = get_value_for_t_lax::<i32>(ast, "weight");

        def
    }
}

impl Convert for Expression {
    fn convert(v: &Value) -> Self {
        let key = get_value_for_t_lax::<String>(v, "key").unwrap_or_default();
        let operator = get_value_for_t_lax::<String>(v, "operator").unwrap_or_default();
        let values = get_value_for_t_lax::<Vec<String>>(v, "values").unwrap_or_default();

        Expression {
            key,
            operator,
            values
        }
    }
}

impl Expression {
    fn as_vec(ast: &Value) -> Vec<Self> {
        if let Some(exp_array) = ast.as_array() {
            return exp_array
                .iter()
                .map(|v| Expression::convert(v))
                .collect::<Vec<Expression>>();
        }

        Vec::new()
    }
}

/// Get Affinity From Ast
///
/// # Description
/// Create an Affinity which represent nodeAffinity & PodAffinity
///
/// # Arguments
/// * `ast` - &Value
///
/// # Return
/// Option<Affinity>
pub fn get_affinity_from_ast(ast: &Value) -> Option<Affinity> {
    if !ast.is_table() {
        return None;
    }

    let affinity_map = ast.as_table().unwrap();
    
    let mut affinity = Affinity::new();
    if let Some(node) = affinity_map.get("node") {
        affinity.set_affinity(node, AffinityKind::Node);
    }

    if let Some(pod) = affinity_map.get("pod") {
        affinity.set_affinity(pod, AffinityKind::Pod);
    }

    Some(affinity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn expect_to_get_affinity() {
        let template = r#"
        [affinity]
            [affinity.node]
                [affinity.node.required]
                    expressions = [
                        { key = "kubernetes.io/e2e-az-name", operator = "in", values = ["foo", "bar"] }
                    ]
                [affinity.node.preferred]
                    [affinity.node.preferred.preemptible]
                        weight = 1
                        expressions = [
                            { key = "kubernetes.io/e2e-az-name", operator = "in", values = ["foo", "bar"] }
                        ]
        "#;

        let ast = template.parse::<Value>().unwrap();
        let affinity_ast = ast.get("affinity").unwrap();

        let affinity = get_affinity_from_ast(&affinity_ast);
        assert!(affinity.is_some());

        let affinity = affinity.unwrap();
        assert!(affinity.node.is_some());
    }

    #[test]
    fn expect_to_get_node_affinity() {
        let template = r#"
        [affinity]
            [affinity.node]
                [affinity.node.required]
                    expressions = [
                        { key = "kubernetes.io/e2e-az-name", operator = "in", values = ["foo", "bar"] }
                    ]
                [affinity.node.preferred]
                    [affinity.node.preferred.preemptible]
                        weight = 1
                        expressions = [
                            { key = "kubernetes.io/e2e-az-name", operator = "in", values = ["foo", "bar"] }
                        ] 
        "#;

        let ast = template.parse::<Value>().unwrap();
        let affinity_ast = ast.get("affinity").unwrap();

        let affinity = get_affinity_from_ast(&affinity_ast).unwrap();
        let node_affinity = affinity.node.unwrap();

        assert!(node_affinity.preferred.is_some());
        assert!(node_affinity.required.is_some());

        let required = node_affinity.required.unwrap();

        let first_exp = required.expressions.get(0).unwrap();
        assert_eq!(first_exp.key, "kubernetes.io/e2e-az-name");
        assert_eq!(first_exp.operator, "in");
        assert_eq!(first_exp.values, ["foo", "bar"]);

        let preferred = node_affinity.preferred.unwrap();
        let preemptible = preferred.get("preemptible");
        assert!(preemptible.is_some());

        let preemptible = preemptible.unwrap();
        assert_eq!(preemptible.weight.unwrap(), 1);
    }

    #[test]
    fn expect_to_get_node_affinity_with_multiple_terms_only() {
        let template = r#"
        [affinity]
            [affinity.node]
                [affinity.node.required]
                    expressions = [
                        { key = "kubernetes.io/e2e-az-name", operator = "in", values = ["foo", "bar"] }
                    ]
                    with_multiple_expressions = [
                        [ { key = "node-role.kubernetes.io/role", operator = "in", values = ["app"] } ],
                        [ { key = "node-role.kubernetes.io/environment", operator = "in", values = ["perf"] } ]
                    ]
        "#;

        let ast = template.parse::<Value>().unwrap();
        let affinity_ast = ast.get("affinity").unwrap();

        let affinity = get_affinity_from_ast(&affinity_ast).unwrap();
        let node_affinity = affinity.node.unwrap();

        assert!(node_affinity.preferred.is_none());
        assert!(node_affinity.required.is_some());   

        let affinity_required = node_affinity.required.unwrap();
        let multiple_terms = affinity_required.with_multiple_expressions;

        assert!(!multiple_terms.is_empty());
        let first_list_term = multiple_terms.get(0).unwrap();
        let first_term = first_list_term.get(0);

        assert!(first_term.is_some());
        let first_term = first_term.unwrap();

        assert_eq!(first_term.operator, "in");
        assert_eq!(first_term.key, "node-role.kubernetes.io/role");
    }

    #[test]
    fn expect_to_not_retrive_node_affinity() {
        let template = r#"
        [affinity]
            [affinity.node]
                [affinity.node.required]
                    foo = []
                    bar = []
        "#;


        let ast = template.parse::<Value>().unwrap();
        let affinity_ast = ast.get("affinity").unwrap();

        let affinity = get_affinity_from_ast(&affinity_ast).unwrap();
        let node_affinity = affinity.node.unwrap();

        assert!(node_affinity.preferred.is_none());
        assert!(node_affinity.required.is_none());   
    }

    #[test]
    fn expect_to_get_pod_affinity() {
        let template = r#"
        [affinity]
            [affinity.pod]
                [affinity.pod.preferred]
                    [affinity.pod.preferred.region]
                        weight = 1
                        expressions = [
                            { key = "kubernetes.io/e2e-az-name", operator = "in", values = ["foo", "bar"] }
                        ] 
        "#;

        let ast = template.parse::<Value>().unwrap();
        let affinity_ast = ast.get("affinity").unwrap();

        let affinity = get_affinity_from_ast(&affinity_ast).unwrap();
        let pod_affinity = affinity.pod.unwrap();

        assert!(pod_affinity.preferred.is_some());
        assert!(pod_affinity.required.is_none());
        
        let preferred = pod_affinity.preferred.unwrap();
        let region = preferred.get("region").unwrap();
        assert_eq!(region.weight.unwrap(), 1);
        assert_eq!(region.expression.get(0).unwrap().key, "kubernetes.io/e2e-az-name");
    }
}