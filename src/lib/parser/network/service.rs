#[derive(Debug)]
pub struct Service {
    // We don't do a check on the kind here
    // This will be done by an other module
    kind: String,
    ports: Vec<Ports>
}

#[derive(Debug)]
pub struct Ports {
    name: String,
    protocol: String,
    port: u32,
    target_port: u32
}