use std::convert::From;
use toml::Value;
use toml::map::Map;
use crate::parser::conv::ConvertNative;
use crate::parser::util::{
    get_string_value,
    get_array_for_type,
    get_value_for_type
};

#[derive(Default, Debug)]
pub struct Probe {
    http_get: Option<HttpGet>,
    exec: Option<Exec>,
    initial_delay_seconds: Option<i64>,
    period_seconds: Option<i64>
}

#[derive(Default, Debug)]
struct HttpGet {
    path: String,
    port: i64,
    http_headers: Option<Vec<HttpHeaders>>
}

#[derive(Default, Debug)]
struct HttpHeaders {
    name: String,
    value: String
}

#[derive(Default, Debug)]
struct Exec {
    command: Vec<String>
}

impl From<Value> for Probe {
    fn from(item: Value) -> Self {
        let probe_values = item.as_table();
        if probe_values.is_none() {
            return Probe::default();
        }

        let probe_map = probe_values.unwrap();
        let kind = probe_map.get("kind").unwrap();

        let mut probe = match kind.as_str().unwrap() {
            "exec" => build_probe_exec(&probe_map),
            "http" => build_probe_http_get(&probe_map),
            _ => Probe::default()
        };

        // Retrieve the delays
        let init_delay = get_value_for_type::<i64>(&item, "initial_delay_seconds");
        let period = get_value_for_type::<i64>(&item, "period_seconds");

        probe.initial_delay_seconds = init_delay;
        probe.period_seconds = period;

        probe
    }
}

impl From<Value> for HttpHeaders {
    fn from(item: Value) -> Self {
        let name = get_string_value(&item, "name");
        let value = get_string_value(&item, "value");

        if name.is_none() || value.is_none() {
            return Self::default(); 
        }

        HttpHeaders {
            name: name.unwrap(),
            value: value.unwrap()
        }
    }
}

/// Build Probe Http Get
///
/// # Description
/// Build an http_get probe (check readiness/liveness of a probe by targeting an http endpoint)
///
/// # Arguments
/// * `item` - &Value
fn build_probe_http_get(item: &Map<String, Value>) -> Probe {
    let mut probe = Probe::default();

    let path_v = item.get("path");
    let port_v = item.get("port");

    if path_v.is_none() || port_v.is_none() {
        return probe;
    }

    let path = String::to(&path_v.unwrap());
    let port    = i64::to(port_v.unwrap());

    if path.is_none() || port.is_none() {
        return probe;
    }

    let mut http_get = HttpGet {
        path: path.unwrap(),
        port: port.unwrap(),
        http_headers: None
    };

    let http_headers_map = item.get("http_headers");
    if let Some(toml_headers) = http_headers_map {
        let (_, http_headers) = get_array_for_type::<HttpHeaders>(&toml_headers, "http_headers");
        if http_headers.is_some() {
            http_get.http_headers = http_headers;
        }
    }
    
    probe.http_get = Some(http_get);
    probe
}

/// Build Probe Exec
///
/// # Description
/// Build an exec probe (check readiness/liveness of a probe by using the exec command)
///
/// # Arguments
/// * `item` - &Value
fn build_probe_exec(item: &Map<String, Value>) -> Probe {
    let mut probe = Probe::default();

    let cmd = item.get("command");
    if cmd.is_none() {
        return probe;
    }

    let command = Vec::to(cmd.unwrap());
    if command.is_none() {
        return probe;
    }

    probe.exec = Some(
        Exec {
            command: command.unwrap()
        }
    );

    probe
}

#[cfg(test)]
mod probe_test {
    use toml::Value;
    use super::Probe;

    #[test]
    fn get_http_probe() {
        let content = "
            [probes]
                [probes.readiness]
                    kind = 'exec'
                    command = [
                        'foo',
                        'bar'
                    ]
                    
                [probes.liveness]
                    kind = 'http'
                    path = 'foo'
                    port = 8080                
                    initial_delay_seconds = 60
                    period_seconds = 20
        ";

        let table = content.parse::<Value>().unwrap();
        let probes_table = table.get("probes")
            .unwrap()
            .as_table()
            .unwrap();
            
        let liveness = probes_table.get("liveness").unwrap();
        let probe = Probe::from(liveness.to_owned());

        assert!(probe.http_get.is_some());
        assert_eq!(probe.initial_delay_seconds.unwrap(), 60);
        assert_eq!(probe.period_seconds.unwrap(), 20);

        let http_get = probe.http_get.unwrap();
        assert_eq!(http_get.path, "foo");
        assert_eq!(http_get.port, 8080);
    }

    #[test]
    fn get_exec_probe() {
        let content = "
            [probes]
                [probes.readiness]
                    kind = 'exec'
                    command = [
                        'foo',
                        'bar'
                    ]
        ";

        let table = content.parse::<Value>().unwrap();
        let probe_table = table.get("probes")
            .unwrap()
            .as_table()
            .unwrap();
        
        let readiness = probe_table.get("readiness").unwrap();
        let probe = Probe::from(readiness.to_owned());

        assert!(probe.exec.is_some());
        assert!(probe.initial_delay_seconds.is_none());
        assert!(probe.period_seconds.is_none());

        let cmd = probe.exec.unwrap();
        assert_eq!(cmd.command.get(0).unwrap(), "foo");
        assert_eq!(cmd.command.get(1).unwrap(), "bar");
    }
}