## Maomao

Just trying to convert TOML to Kubernetes YAML (in WIP)

## Command

```bash
cargo run generate -p <path> -o [OPTIONAl] <output_path>
```

## Custom CRD

Custom CRD are just converting a TOML [spec] table to YAML w/o any logics. Thus there can be some shortcomings. An example could be argo workflow which support double hyphens. This syntax is not officially supported by the YAML spec. Hence, we can't support this feature.

See examples/crd folder to see how it looks like.