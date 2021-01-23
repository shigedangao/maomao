use std::convert::From;
use k8s_openapi::api::core::v1::{
    ServiceSpec,
    ServicePort
};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use crate::lib::parser::network::service::{
    Service,
    Port
};

/// Get Service Spec
///
/// # Description
/// Retrieve the ServiceSpec struct
///
/// # Arguments
/// * `service` - Service
///
/// # Return
/// ServiceSpec
pub fn get_service_spec(service: Service) -> ServiceSpec {
    let ports = get_service_ports(&service);
    ServiceSpec {
        type_: Some(service.kind),
        ports,
        ..Default::default()
    }
}

/// Get Service Ports
///
/// # Description
/// Retrieve the Vec<ServicePort> from a parser Service struct
///
/// # Arguments
/// * `service` - &Service
///
/// # Return
/// Option<Vec<ServicePort>>
pub fn get_service_ports(service: &Service) -> Option<Vec<ServicePort>> {
    if service.ports.is_none() {
        return None;
    }

    let ports_map = service.ports.to_owned().unwrap();
    let ports = ports_map
        .into_iter()
        .map(|(name, port)| {
            let mut p = ServicePort::from(port);
            p.name = Some(name);

            p
        })
        .collect::<Vec<ServicePort>>();

    Some(ports)
}

impl From<Port> for ServicePort {
    fn from(p: Port) -> Self {
        ServicePort {
            node_port: p.node_port,
            port: p.port as i32,
            protocol: Some(p.protocol),
            target_port: Some(IntOrString::Int(p.target_port as i32)),
            ..Default::default()
        }
    }
}