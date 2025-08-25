## Local usage

Run the local rust composite function in insecure mode (in the folder above)

```bash
cargo run -- --insecure
```

and then execute the crossplane local rendering (here)

```bash
crossplane render xr.yaml composition.yaml function.yaml
```

## k3d example

The [run.sh] script executes the following steps to deploy a minimalistic crossplane setup
using this composite function into a local [k3d](https://k3d.io/) cluster.

All images are build and 'pushed' locally, no credentials for an external oci-registry are required.
For this to work `*.localhost` (in this case `k3d-crossplane.localhost`) needs to be resolved to
`127.0.0.1` ([upstream docs](https://k3d.io/v5.6.0/usage/registries/#preface-referencing-local-registries)) on your local machine.

It has retries for some steps build-in to e.g. wait for certain Kubernetes CRD to be available.
If you are done you can stop k3d via `k3d cluster delete -c k3d.yaml`:

- Builds the example rust composite function into a Docker image
- Start a [k3d](https://k3d.io/) local Kubernetes cluster
- Packages the image into a [crossplane composition function](https://docs.crossplane.io/latest/concepts/compositions/#how-composition-functions-work)
- Bundles the composition function, the [crossplane composite resource definition](https://docs.crossplane.io/latest/concepts/composite-resource-definitions/)
  and a [compositon](https://docs.crossplane.io/latest/concepts/compositions/) into a [crossplane configuration](https://docs.crossplane.io/latest/concepts/packages/)
- Deploys the Configuration
- Deploys an example claim that will result in the creation of two ConfigMaps in the `test` namespace, the corresponding
  `Config`-[claim](https://docs.crossplane.io/latest/concepts/claims/) is also in the `test` namespace.
