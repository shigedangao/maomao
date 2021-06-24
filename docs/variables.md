## Variables 💅

Maomao allow you to use variables but in a very limited way. Indeed the variables is just a mere "replace" variable by value in the TOML template before converting to the YAML syntax. 

⚠️ Note that for an *inline-table* within an *array* there are custom code in order for the *inline-table* to render properly.

Variables support these TOML types

- number
- string
- boolean
- array
- inline table

### Create variables

Variables are declared in the same folder where your TOML template reside. The variables are declared in a file name _vars.toml

Pretty much like a Helm's value file the _vars.toml only contain key, value. Below is an example

```toml
replicas = 1
version = 1.19
metadata = { name = "node" }
```

Now it's time to use these variables in our templates. As an example for the hello-node deployment. We could update our template file by using this following syntax

```toml
kind = "workload::deployment"
name = "hello-node"
metadata = "$[metadata::typed]"

[workload]
    replicas = "$[replicas]"

    [workload.nginx]
    image = "k8s.gcr.io/echoserver"
    tag = "$[version]"
```

### Note regarding typing of the TOML template

By default the variable will be transform to a `string` representation of the variable from the _vars.toml. If you wish to use a particular type of TOML `type` i.e: float, array, inline table etc... you'll need to use this format 

```toml
foo = "$[variable::typed]"
```

The CLI will respect the `type` of the variable defined in the _vars.toml.
