## Templates 📃

Template need to be located in a folder. The CLI support basics Kubernetes objects such as:

- workload
- configmap & secrets
- service & ingress
- crd

All of them shared some basics informations which are

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

## Crd 

The CLI also support CRD. What it does is that the CLI will do a simple TOML convertion to YAML. Common annotation could still be used.

CRD use a special type of kind which always begin by the 

```toml
kind = "custom::<type of resources>"
```

Below is an example of the GKE's ManagedCertificate CRD

```toml
kind = "custom::ManagedCertificate"
version = "networking.gke.io/v1"
metadata = { name = "rusty-certificate" }

[spec]
    domains = ["foo.co.kr", "foo.co.tw", "foo.co.fr"]
```

You can find some CRD example by clicking on this [link](https://github.com/shigedangao/maomao/tree/master/examples/crd)