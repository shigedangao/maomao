## Maomao

Just trying to convert TOML to Kubernetes YAML (in WiP)

## Command

```bash
cargo run generate -p <path>
```

## Custom CRD (reflexion)

After implementing some workload, translating from TOML to YAML is quite intensive, as it required the following (maybe architecture is too complex ?)
-> toml file -> Workload struct -> k8s_openapi struct -> YAML

Custom CRD does not need to use k8s_openapi. Just convert the toml::Values to Yaml (if it implement the serialize...)
- If it is, then just create a TOML which will define the rules of the toml format such that the parser or checker lib will implement a visitor checking if the TOMl respect the rules

It would be like so -> toml file -> (Rules files ?) -> YAML
It should be possible to translate directly toml to yaml the toml library implement the Deserialize trait... need to test