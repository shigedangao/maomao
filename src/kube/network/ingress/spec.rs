use std::convert::From;
use k8s_openapi::api::networking::v1::{
    IngressSpec,
    HTTPIngressPath,
    HTTPIngressRuleValue,
    IngressBackend,
    IngressServiceBackend,
    ServiceBackendPort,
    IngressRule,
    IngressTLS
};
use crate::lib::parser::network::ingress::{
    Ingress,
    IngressHTTPPath,
};
use crate::lib::parser::network::backend::Backend;

/// Get Ingress Spec
///
/// # Description
/// Create the IngressSpec struct from the parser value
///
/// # Arguments
/// * `ingress` - Option<Ingress>
///
/// # Return
/// Option<IngressSpec>
pub fn get_ingress_spec(ingress: Option<Ingress>) -> Option<IngressSpec> {
    if let Some(ig) = ingress {
        let rules = get_ingress_rules(&ig);
        let tls = get_tls_rules(&ig);

        return Some(IngressSpec {
            rules,
            tls,
            ..Default::default()
        });
    }


    None
}

/// Get Ingress Rules
///
/// # Description
/// Retrieve a list of rules from the parser rules
///
/// # Arguments
/// * `ingress` - &Ingress
///
/// # Return
/// Option<Vec<IngressRule>>
fn get_ingress_rules(ingress: &Ingress) -> Option<Vec<IngressRule>> {
    if let Some(rules) = ingress.rules.to_owned() {
        let r = rules
            .into_iter()
            .map(|rule| IngressRule {
                host: Some(rule.host),
                http: Some(HTTPIngressRuleValue {
                    paths: get_http_ingress_paths(rule.paths)
                })
            })
            .collect::<Vec<IngressRule>>();

        return Some(r);
    }
    
    None
}

/// Get Tls Rules
///
/// # Description
/// Retrieve the tls rules from the parser ingress struct
///
/// # Arguments
/// * `ingress` - &Ingress
///
/// # Return
/// Option<Vec<IngressTLS>>
fn get_tls_rules(ingress: &Ingress) -> Option<Vec<IngressTLS>> {
    if let Some(t) = ingress.tls.to_owned() {
        let tls = IngressTLS {
            hosts: t.hosts,
            secret_name: t.secrets
        };
        
        return Some(vec![tls]);
    }

    None
}

/// Get HTTP Ingress Paths
///
/// # Arguments
/// * `paths` - Option<Vec<IngressHTTPPath>>
///
/// # Return
/// Vec<HTTPIngressPath>
fn get_http_ingress_paths(paths: Option<Vec<IngressHTTPPath>>) -> Vec<HTTPIngressPath> {
    if let Some(paths) = paths {
        let api_paths = paths
            .into_iter()
            .map(HTTPIngressPath::from)
            .collect::<Vec<HTTPIngressPath>>();

        return api_paths;
    }

    Vec::new()
}

impl From<IngressHTTPPath> for HTTPIngressPath {
    fn from(p: IngressHTTPPath) -> Self {
        HTTPIngressPath {
            path: Some(p.path),
            path_type: Some(p.kind),
            backend: IngressBackend::from(p.backend)
        }
    }
}

impl From<Backend> for IngressBackend {
    fn from(backend: Backend) -> Self {
        let service_backend = IngressServiceBackend {
            name: backend.name,
            port: Some(ServiceBackendPort {
                name: None,
                number: Some(backend.port as i32),
            })
        };

        IngressBackend {
            service: Some(service_backend),
            resource: None
        }
    }
} 
