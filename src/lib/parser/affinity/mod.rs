use toml::Value;
use crate::lib::helper::conv::Convert;
use crate::lib::helper::toml::get_value_for_t_lax;

enum AffinityKind {
    Node,
    Pod
}

#[derive(Debug, Default, Clone)]
pub struct Affinity {
    node: Option<AffinityType>,
    pod: Option<AffinityType>
}

#[derive(Debug, Default, Clone)]
pub struct AffinityType {
    required: Option<AffinityDefinition>,
    preferred: Option<AffinityDefinition>
}

#[derive(Debug, Default, Clone)]
pub struct AffinityDefinition {
    expressions: Vec<Expression>,
    weight: Option<i64>,
    topology: Option<String>
}

#[derive(Debug, Clone)]
pub struct Expression {
    key: String,
    operator: String,
    values: Vec<String>
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
            affinity_type.required = AffinityDefinition::new(value);
        }

        if let Some(value) = ast.get("preferred") {
            affinity_type.preferred = AffinityDefinition::new(value);
        }

        match kind {
            AffinityKind::Node => self.node = Some(affinity_type),
            AffinityKind::Pod => self.pod = Some(affinity_type)
        }
    }
}

impl AffinityDefinition {
    /// New
    ///
    /// # Description
    /// Create a new AffinityDefinition
    ///
    /// # Arguments
    /// * `ast` - &Value
    ///
    /// # Return
    /// Option<AffinityDefinition>
    fn new(ast: &Value) -> Option<AffinityDefinition> {
        let mut def = AffinityDefinition::default();
        if ast.get("expressions").is_none() {
            return None;
        }

        let expressions = ast.get("expressions").unwrap();
        if let Some(exp_array) = expressions.as_array() {
            def.expressions = exp_array
                .iter()
                .map(|v| Expression::convert(v))
                .collect::<Vec<Expression>>();
        }

        def.weight = get_value_for_t_lax::<i64>(ast, "weight");
        def.topology = get_value_for_t_lax::<String>(ast, "topology_key");


        Some(def)
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
                    weight = 1
                    expressions = [
                        { key = "kubernetes.io/e2e-az-name", operator = "in", values = ["foo", "bar"] }
                    ] 
        "#;

        let ast = template.parse::<Value>().unwrap();
        let affinity_ast = ast.get("affinity").unwrap();

        let affinity = super::get_affinity_from_ast(&affinity_ast);
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
                    weight = 1
                    expressions = [
                        { key = "kubernetes.io/e2e-az-name", operator = "in", values = ["foo", "bar"] }
                    ] 
        "#;

        let ast = template.parse::<Value>().unwrap();
        let affinity_ast = ast.get("affinity").unwrap();

        let affinity = super::get_affinity_from_ast(&affinity_ast).unwrap();
        let node_affinity = affinity.node.unwrap();

        assert!(node_affinity.preferred.is_some());
        assert!(node_affinity.required.is_some());

        let required = node_affinity.required.unwrap();
        assert!(required.weight.is_none());

        let first_exp = required.expressions.get(0).unwrap();
        assert_eq!(first_exp.key, "kubernetes.io/e2e-az-name");
        assert_eq!(first_exp.operator, "in");
        assert_eq!(first_exp.values, ["foo", "bar"]);

        let preferred = node_affinity.preferred.unwrap();
        assert_eq!(preferred.weight.unwrap(), 1);
    }

    #[test]
    fn expect_to_get_pod_affinity() {
        let template = r#"
        [affinity]
            [affinity.pod]
                [affinity.pod.preferred]
                    weight = 1
                    topology_key = "kubernetes.io/hostname"
                    expressions = [
                        { key = "app", operator = "in", values = ["store"] }
                    ] 
        "#;

        let ast = template.parse::<Value>().unwrap();
        let affinity_ast = ast.get("affinity").unwrap();

        let affinity = super::get_affinity_from_ast(&affinity_ast).unwrap();
        let pod_affinity = affinity.pod.unwrap();

        assert!(pod_affinity.preferred.is_some());
        assert!(pod_affinity.required.is_none());
        
        let preferred = pod_affinity.preferred.unwrap();
        assert_eq!(preferred.topology.unwrap(), "kubernetes.io/hostname");
    }
}