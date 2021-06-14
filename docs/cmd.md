## List of commands

### Generate

The generate command allow you to generate Kubernetes YAML from TOML templates. The command could be use like below:

```shell
maomao generate -p <template path> [OPTIONS]
```

**Generate YAML to a destination folder**

```shell
maomao generate -p <template path> -o <folder path>
```

**Generate YAML to a single file**

```shell
maomao generate -p <template path> -o <filename.yaml> -m
```

**Generate YAML and output in the terminal**

```shell
maomao generate -p <template path>
```

⚠️ If you wish to output the result the YAML in the terminal and pipe the output to the kubectl command then you should add the `-q` option. `-q` stand for `quiet`. Below is an example

```shell
maomao generate -p <template path> -q | kubectl apply -f -
```

### Diff

Diff command allow you to see a git style diff between your template and what's currently on the cluster. Below is how you can use

```shell
maomao diff -p <template folder>
```

### Verify

Verify command allow you to check whenever the TOML templates files are valid. Below is how you can use

**Existing kubernetes resources**

```shell
maomao verify -p <template folder>
```

**Un-released templates**

In this case, you'll need to pass the `-u` option. Below is an example

```shell
maomao verify -p <template folder> -u
```