use std::convert::From;
use toml::Value;
use toml::map::Map;
use crate::parser::conv::ConvertNative;
use crate::parser::util::{
    get_string_value,
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

impl Probe {
    /// New
    ///
    /// # Description
    /// Create a new Probe object
    fn new() -> Probe {
        Probe::default()
    }

    /// Set Probe Type
    ///
    /// # Description
    /// Set probe properties such as http_get, exec
    ///
    /// # Arguments
    /// * `self` - Self
    /// * `item` - &Value
    fn set_probe_type(mut self, item: &Value) -> Self {
        let probe_values = item.as_table();
        if probe_values.is_none() {
            return Probe::default();
        }

        let probe_map = probe_values.unwrap();
        let kind = probe_map.get("kind").unwrap();
        let probe = match kind.as_str().unwrap() {
            "exec" => build_probe_exec(&probe_map),
            "http" => build_probe_http_get(&probe_map),
            _ => Probe::default()
        };

        probe
    }

    /// Finish
    ///
    /// # Description
    /// * `self` - Self
    /// * `item` - &Value
    fn finish(mut self, item: &Value) -> Self {
        // Retrieve the delays
        let init_delay = get_value_for_type::<i64>(&item, "initial_delay_seconds");
        let period = get_value_for_type::<i64>(&item, "period_seconds");

        self.initial_delay_seconds = init_delay;
        self.period_seconds = period;

        self
    }
}

/// Build Probe Http Get
///
/// # Description
/// Build an http_get probe (check readiness/liveness of a probe by targeting an http endpoint)
///
/// # Arguments
/// * `item` - &Map<String, Value>
fn build_probe_http_get(item: &Map<String, Value>) -> Probe {
    let mut probe = Probe::default();

    let path_v = item.get("path");
    let port_v = item.get("port");

    if path_v.is_none() || port_v.is_none() {
        return probe;
    }

    let path = String::to(path_v.unwrap());
    let port    = i64::to(port_v.unwrap());

    if path.is_none() || port.is_none() {
        return probe;
    }

    let http_headers_array = item.get("http_headers");
    let http_get = HttpGet {
        path: path.unwrap(),
        port: port.unwrap(),
        http_headers: get_http_headers(http_headers_array)
    };

    probe.http_get = Some(http_get);
    probe
}

/// Get Http Headers
///
/// # Description
/// Retrieve HttpHeaders from a toml Array of toml Table
///
/// # Arguments
/// * `item` - Option<&Value>
fn get_http_headers(item: Option<&Value>) -> Option<Vec<HttpHeaders>> {
    if item.is_none() {
        return None;
    }

    if !item.unwrap().is_array() {
        return None;
    }

    let array = item
        .unwrap()
        .as_array()
        .unwrap();

    let res = array
        .iter()
        .map(|v| HttpHeaders::from(v.to_owned()))
        .collect::<Vec<HttpHeaders>>();

    Some(res)
}

/// Build Probe Exec
///
/// # Description
/// Build an exec probe (check readiness/liveness of a probe by using the exec command)
///
/// # Arguments
/// * `item` - &Map<String, Value>
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
            
        let liveness_table = probes_table.get("liveness").unwrap();
        let probe_values = liveness_table.as_table();
        let probe_map = probe_values.unwrap();        
        let probe = super::build_probe_http_get(probe_map);

        assert!(probe.http_get.is_some());
        let http_get = probe.http_get.unwrap();
        assert_eq!(http_get.path, "foo");
        assert_eq!(http_get.port, 8080);
    }

    #[test]
    fn get_http_headers() {
        let content = "
            [probes]        
                [probes.liveness]
                    kind = 'http'
                    path = 'foo'
                    port = 8080                
                    http_headers = [
                        { name = 'baz', value = 'wow' },
                        { name = 'yo', value = 'sabai' }
                    ]
        ";

        let table = content.parse::<Value>().unwrap();
        let probes_table = table.get("probes")
            .unwrap()
            .as_table()
            .unwrap();
            
        let liveness = probes_table.get("liveness").unwrap();
        let probe = Probe::new().set_probe_type(liveness).finish(liveness);

        let http_get = probe.http_get.unwrap();
        let headers = http_get.http_headers.unwrap();

        assert_eq!(headers.get(0).unwrap().name, "baz");
        assert_eq!(headers.get(0).unwrap().value, "wow");
 
        assert_eq!(headers.get(1).unwrap().name, "yo");
        assert_eq!(headers.get(1).unwrap().value, "sabai");
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
        let probe = Probe::new().set_probe_type(readiness).finish(readiness);

        assert!(probe.exec.is_some());
        assert!(probe.initial_delay_seconds.is_none());
        assert!(probe.period_seconds.is_none());

        let cmd = probe.exec.unwrap();
        assert_eq!(cmd.command.get(0).unwrap(), "foo");
        assert_eq!(cmd.command.get(1).unwrap(), "bar");
    }
}