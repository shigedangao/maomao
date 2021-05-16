## Maomao

Just trying to convert TOML to Kubernetes YAML

## Command

```bash
cargo run generate -p <path> -o [OPTIONAl] <output_path> -m [OPTIONAL]
cargo run diff -p <path>
```

## Custom CRD

Custom CRD are just converting a TOML [spec] table to YAML w/o any logics. Thus there can be some shortcomings. An example could be argo workflow which support double hyphens. This syntax is not officially supported by the YAML spec. Hence, we can't support this feature.

See examples/crd folder to see how it looks like.

## Caveats with variables

The to_string method of the toml library that I use does not render inline table properly. Thus when trying to implement inline table in an array this might create some issues with the template. Recommended way to use inline table in a TOML array is to use the following method

**In _vars.toml**

```toml
property = { key = "value" }
```

**In spec.toml**

```toml
array = [
    "$[property::typed]"
]

# This will output
array = [
    { key = "value" }
]
```