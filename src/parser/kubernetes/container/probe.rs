use std::convert::From;
use toml::Value;
use crate::parser::util::{
    get_string_value,
    get_value_for_type
};

#[derive(Default, Debug)]
pub struct Probe {
    http_get: Option<HttpGet>,
    exec: Option<Exec>,
    initial_delay_seconds: i64,
    period_seconds: i64
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
    pub fn new() -> Probe {
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
    pub fn set_probe_type(mut self, item: &Value) -> Self {
        let kind = item.get("kind").unwrap();
        let probe = match kind.as_str().unwrap() {
            "exec" => build_probe_exec(&item),
            "http" => build_probe_http_get(&item),
            _ => Probe::default()
        };

        probe
    }

    /// Finish
    ///
    /// # Description
    /// * `self` - Self
    /// * `item` - &Value
    pub fn finish(mut self, item: &Value) -> Self {
        // Retrieve the delays
        let init_delay = get_value_for_type::<i64>(&item, "initial_delay_seconds");
        let period = get_value_for_type::<i64>(&item, "period_seconds");

        self.initial_delay_seconds = init_delay.unwrap_or(0);
        self.period_seconds = period.unwrap_or(0);

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
fn build_probe_http_get(item: &Value) -> Probe {
    let mut probe = Probe::default();

    let path = get_value_for_type::<String>(item, "path");
    let port = get_value_for_type::<i64>(item, "port");

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
/// * `item` - &Value
fn build_probe_exec(item: &Value) -> Probe {
    let mut probe = Probe::default();

    let command = get_value_for_type::<Vec<String>>(item, "command");
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
        let probe = super::build_probe_http_get(liveness_table);

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
                    initial_delay_seconds = 60
                    period_seconds = 20
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
        assert_eq!(probe.initial_delay_seconds, 0);
        assert_eq!(probe.period_seconds, 0);

        let cmd = probe.exec.unwrap();
        assert_eq!(cmd.command.get(0).unwrap(), "foo");
        assert_eq!(cmd.command.get(1).unwrap(), "bar");
    }
}