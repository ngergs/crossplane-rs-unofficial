# Local test example

Run the local rust composite function in insecure mode (in the folder above)

```bash
cargo run --package crossplane-rust-example --bin server -- --tls-certs-dir . --insecure
```

and than execute the crossplane local rendering (here)

```bash
crossplane render xr.yaml composition.yaml function.yaml
```

## Jsonschema

The `xr.jsonschema` can be obtained from the `xrd.yaml` via [mikefarah yq v4](https://github.com/mikefarah/yq):

```bash
yq '.spec.versions[0].schema.openAPIV3Schema | .title="XConfig" | . *= load("claimRef.yaml")' xrd.yaml -o json > xr.jsonschema
```

## Minikube example

The [run.sh] script executes the following steps:
- Start a [minikube](https://minikube.sigs.k8s.io/) local Kubernetes cluster
- Builds the example rust composite function into a Docker image
- Packages the image into a [crossplane composition function](https://docs.crossplane.io/latest/concepts/compositions/#how-composition-functions-work)
- Bundles the composition function, the [crossplane composite resource definition](https://docs.crossplane.io/latest/concepts/composite-resource-definitions/) and a [compositon](https://docs.crossplane.io/latest/concepts/compositions/) into a [crossplane configuration](https://docs.crossplane.io/latest/concepts/packages/)
- Makes these available to the minikube instance
- Deploys the Kubernetes provider (needs to be done separately to configure the [runtime](https://github.com/crossplane/crossplane/issues/6382))
- Deploys the Configuration
- Deploys an example claim that will result in the creation of two ConfigMaps in the `test` namespace, the corresponding `Config`-[claim](https://docs.crossplane.io/latest/concepts/claims/) is also in the `test` namespace.
