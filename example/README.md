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

To run this example execute the following steps:

```bash
minikube up
minikube addons enable registry
kustomize build --enable-helm | kubectl apply --context minikube -f -
docker build -t crossplane-rust-config-fn ..
crossplane xpkg build --package-root=package --embed-runtime-image=crossplane-rust-config-fn --package-file=fn.xpkg
crossplane xpkg push --package-files=fn.xpkg $(minikube ip):5000/crossplane-rust-config:latest
minikube image load $(minikube ip):5000/crossplane-rust-config:latest
```
