use std::collections::BTreeMap;
use k8s_openapi::api::core::v1::{
    Probe,
    ExecAction,
    TCPSocketAction,
    HTTPGetAction,
    HTTPHeader
};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use crate::lib::parser::workload::probes::{
    Probe as ParserProbe,
    ProbeHttpGet
};

/// Convert a collection to a TCPSocketAction
///
/// # Arguments
///
/// * `map` - BTreeMap<String, String>
fn get_socket_from_map(map: BTreeMap<String, String>) -> Option<TCPSocketAction> {
    let mut socket = TCPSocketAction::default();
    if let Some(host) = map.get("host") {
        socket.host = Some(host.to_owned());
    }

    if let Some(port) = map.get("port") {
        socket.port = IntOrString::String(port.to_owned());
    }

    Some(socket)
}

impl From<ParserProbe> for Probe {
    fn from(p: ParserProbe) -> Self {
        let mut probe = Probe {
            failure_threshold: p.failure_thresold,
            initial_delay_seconds: p.initial_delays_seconds,
            success_threshold: p.success_thresold,
            timeout_seconds: p.timeout_seconds,
            ..Default::default()
        };

        if let Some(cmd) = p.exec {
            probe.exec = Some(ExecAction { command: cmd })
        }

        if let Some(map) = p.tcp_socket {
            probe.tcp_socket = get_socket_from_map(map);
        }

        if let Some(http) = p.http_get {
            probe.http_get = Some(HTTPGetAction::from(http));
        }

        probe
    }
}

impl From<ProbeHttpGet> for HTTPGetAction {
    fn from(p: ProbeHttpGet) -> Self {
        let mut http = HTTPGetAction {
            host: p.host,
            path: p.path,
            scheme: p.scheme,
            ..Default::default()
        };

        if let Some(headers) = p.http_headers {
            http.http_headers = headers
                .into_iter()
                .map(|(name, value)| HTTPHeader{ name, value })
                .collect();
        }

        if let Some(port) = p.port {
            match port.parse::<i32>() {
                Ok(number) => http.port = IntOrString::Int(number),
                Err(_) => http.port = IntOrString::String(port)
            }
        }

        http
    }
}