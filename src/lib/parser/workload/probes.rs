use std::collections::BTreeMap;
use toml::Value;
use crate::lib::helper::toml::get_value_for_t_lax;
use crate::lib::helper::conv::Convert;

#[derive(Debug, Default, Clone)]
pub struct Probes {
    pub liveness: Option<Probe>,
    pub readiness: Option<Probe>
}

#[derive(Debug, Default, Clone)]
pub struct Probe {
    pub exec: Option<Vec<String>>,
    pub failure_thresold: Option<i32>,
    pub http_get: Option<ProbeHttpGet>,
    pub initial_delays_seconds: Option<i32>,
    pub success_thresold: Option<i32>,
    pub tcp_socket: Option<BTreeMap<String, String>>,
    pub termination_grace_period_seconds: Option<i32>,
    pub timeout_seconds: Option<i32>
}

#[derive(Debug, Default, Clone)]
pub struct ProbeHttpGet {
    pub host: Option<String>,
    pub http_headers: Option<BTreeMap<String, String>>,
    pub path: Option<String>,
    pub port: Option<String>,
    pub scheme: Option<String>
}

impl Probes {
    /// Create a Probes struct
    ///
    /// # Arguments
    ///
    /// * `ast` - &Value
    pub fn new(ast: &Value) -> Self {
        let mut probes = Probes::default();
        if let Some(liveness) = ast.get("liveness") {
            probes.liveness = Some(Probe::new(liveness));
        }

        if let Some(readiness) = ast.get("readiness") {
            probes.readiness = Some(Probe::new(readiness));
        }

        probes
    }
}

impl Probe {
    /// Create a new probe
    ///
    /// # Arguments
    ///
    /// * `ast` - &Value
    pub fn new(ast: &Value) -> Self {
        let exec = get_value_for_t_lax::<Vec<String>>(ast, "exec");
        let http_get = get_value_for_t_lax::<ProbeHttpGet>(ast, "http_get");
        let failure = get_value_for_t_lax::<i32>(ast, "failure_thresold");
        let delay = get_value_for_t_lax::<i32>(ast, "initial_delay_seconds");
        let success = get_value_for_t_lax::<i32>(ast, "success_thresold");
        let socket = get_value_for_t_lax::<BTreeMap<String, String>>(ast, "tcp_socket");
        let termination = get_value_for_t_lax::<i32>(ast, "termination_grace_period_seconds");
        let timeout = get_value_for_t_lax::<i32>(ast, "timeout_seconds");

        Probe {
            exec,
            failure_thresold: failure,
            http_get,
            initial_delays_seconds: delay,
            success_thresold: success,
            tcp_socket: socket,
            termination_grace_period_seconds: termination,
            timeout_seconds: timeout
        }
    }
}

impl Convert for ProbeHttpGet {
    fn convert(v: &Value) -> Self {
        let host = get_value_for_t_lax::<String>(v, "host");
        let path = get_value_for_t_lax::<String>(v, "path");
        let port = get_value_for_t_lax::<String>(v, "port");
        let scheme = get_value_for_t_lax::<String>(v, "scheme");
        let headers = get_value_for_t_lax::<BTreeMap<String, String>>(v, "http_headers");

        ProbeHttpGet {
            host,
            http_headers: headers,
            path,
            port,
            scheme
        }
    }
}