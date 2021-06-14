## Templates 📃

Template need to be located in a folder. The CLI support basics Kubernetes objects such as:

- workload
- configmap & secrets
- service & ingress
- crd

All of them shared the same common information which are

```toml
kind = "workload::<type>"
name = "<name of the workload>"
namespace = "<name of the namespace> Optional"
metadata = { name = "nginx" }
# annotations are optional
annotations = { foo = "<optional>"}
```

## Basic Kubernetes objects

For basic Kubernetes object the CLI will use it's own `parser` and create a generic representation of the type of Kubernetes objects. Allowing the CLI to easily reconstruct the Kubernetes object by using the [k8s-openapi library](https://github.com/Arnavion/k8s-openapi)

The syntax resemble a bit like the YAML syntax, albeit I tried to simplify most of it. It only support main feature. Below is how a generic workload looks like

```toml
# <type> could be a deployment, daemonset
kind = "workload::<type>"
name = "<name of the workload>"
namespace = "<name of the namespace> Optional"
metadata = { name = "nginx" }
# annotations are optional
annotations = { foo = "<optional>"}

# optional
[volume_claims]
    [volume_claims.<name>]
        access_modes = ["ReadWriteOnce"]
        resource_request = [
            { key_name = "storage", value = "1Gi" }
        ]


[workload]
    replicas = "<nb of replicas>"
    # optional
    tolerations = [
        { key = 'node-role.kubernetes.io/master', effect = 'NoSchedule' }
    ]

    # containers definition
    [workload.<name>]
    image = "<image name>"
    tag = "<image version>"
    policy = "<optional>"
    # optional
    volume_mounts = [
        { name = "<name>", mount_path = "/mnt" }
    ]

        # optional
        # env_from => envFrom
        [workload.<name>.env_from]
        map = ["config"]
        secret = ["secret"]

        # optional
        # raw env value
        [workload.<name>.env]
        # from => valueFrom
        from = [
            { type = "res_field", name = "nginx-container", item = "limits.cpu" }
        ]
        # raw => 
        raw = [
            { name = "A_VALUE", item = "bar" }
        ]
```

You can find some CRD example by clicking on this [link](https://github.com/shigedangao/maomao/tree/master/examples)

## Basic network objects

For basic network objects. The CLI support service & ingress. You can find examples of definitions by clicking on this [link](https://github.com/shigedangao/maomao/tree/master/examples)

**Service**

Below is the generic syntax for a Service

```toml
kind = "network::service"
name = "nginx"
metadata = { name = "nginx", tier = "backend" }

[service]
    type = "<Type of service>"

    [service.ports]

        [service.ports.<name>]
            protocol = "<string>"
            port = "<container port>"
            target_port = "<outbound port>"
```

**Ingress**

Below is the generic syntax for an Ingress resource

```toml
kind = "network::ingress"
name = "nginx"
metadata = { name = "nginx", tier = "ingress" }

[ingress]

    [ingress.default]
        backend = { name = "<service name>", port = "<service target port>" }

    [ingress.rules]

        [ingress.rules.<name>]
            host = "<hostname>"

            [ingress.rules.<name>.paths]

                [ingress.rules.<name>.paths.0]
                    type = "<string>"
                    path = "/"
                    backend = { name = "<service name>", port = "<service target port>" }
```