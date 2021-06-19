## Crd - Custom Resources Definition

The CLI also support CRD. What it does is that the CLI will do a simple TOML convertion to YAML. Common template feature could still be used

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