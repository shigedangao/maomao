## Maomao 🐱

[![Rust](https://github.com/shigedangao/maomao/actions/workflows/rust.yml/badge.svg)](https://github.com/shigedangao/maomao/actions/workflows/rust.yml)

Convert TOML files to Kubernetes YAML spec ! 

## Examples

### Generate a deployment.yaml from a TOML template

- Create a folder name **templates**
- Within this folder create a file named **deployment.toml** within the **templates** folder.

Copy paste the template below:

```toml
kind = "workload::deployment"
name = "nginx"
metadata = { name = "nginx", tier = "backend" }

# container name nginx
[workload]
    replicas = 1

    [workload.nginx]
    image = "nginx"
    tag = "1.19.10"
    policy = "IfNotPresent"
```

- Generate the YAML spec by using the command below

```shell
cargo run generate -p templates -q > deployment.yaml && kubectl apply -f .
```

### Diff the templates by using the diff command

- Edit the same **deployment.toml** by changing the *replicas* field to *5*
- Check the diff by using the command below

```shell
cargo run diff -p <path>
```

### Verify the templates

If you wish to check whenever the templates are valid you can run this command

```shell
cargo run verify -p <path>
```

If the template has not been deployed then you can run this command

```shell
cargo run verify -p <path> -u
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