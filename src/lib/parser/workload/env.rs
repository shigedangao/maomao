#[derive(Debug, Default)]
pub struct EnvFrom {
    map: Vec<String>,
    secret: Vec<String>
}

#[derive(Debug, Default)]
pub struct EnvRefKey {
    kind: Option<String>,
    name: String,
    key: String
}

#[derive(Debug, Default)]
pub struct Env {
    from: Vec<EnvRefKey>,
    raw: Vec<EnvRefKey>
}
