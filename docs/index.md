## Maomao 🐱

Welcome to the documentation. Maomao is just a small CLI that allow you to convert TOML file to Kubernetes spec. This project has been made for fun and should not be used in production. Feel free to play with it and fork if you wish

### Quickstart guide

You can download the latest binary in the [release section](https://github.com/shigedangao/maomao/releases). The app will required you to have a valid kubernetes cluster configured as well as a kubectl with a kubeconfig setup

**Create a small template**

In this small tutorial we will create a small nodejs deployment by using the hello-node image from the kubernetes tutorial.

1. Create a folder name `template` with this command

```shell
mkdir templates
```

2. In this template folder create a file name `deployment.toml`. In this file copy, paste the following content

```toml
kind = "workload::deployment"
name = "hello-node"
metadata = { name = "node" }

[workload]
    replicas = 2

    [workload.nginx]
    image = "k8s.gcr.io/echoserver"
    tag = "1.4"
```

3. Generate a Kubernetes YAML by using this command

```yaml
maomao generate -p template -o out.yaml -m
```

4. Install the hello-node with the kubectl cmd

```shell
kubectl apply -f out.yaml
```

**Update the hello-node deployment**

In this example we want to scale up the hello-node deployment by increasing the number of *replicas*.

1. Edit the `deployment.toml` and change the *replicas* to 3

```yaml
[workload]
    replicas = 3
```

2. Before generating the change we might want to check what's going to be different between our changes and what we have in the cluster. We can do this command

```shell
maomao diff -p template
```

The command should print a `diff` in the terminal.

3. Optional: The command ship a `verify` command which will check whenever the generate YAML template is a valid YAML template with the Kubernetes cluster by using the [dry run feature](https://kubernetes.io/docs/reference/using-api/api-concepts/#dry-run)

To do that you can use the command 

```shell
maomao verify -p template
```

### Other guides

- [Using variables](variables.md)
- [Using multiple templates](templates.md)
- [CRD](crd.md)
- [List of commands](cmd.md)