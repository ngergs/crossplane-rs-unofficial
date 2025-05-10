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
minikube start
minikube addons enable registry
docker build -t crossplane-rust-config-fn ..
crossplane xpkg build --package-root=function --embed-runtime-image=crossplane-rust-config-fn --package-file=fn.xpkg
crossplane xpkg push --package-files=fn.xpkg $(minikube ip):5000/crossplane-rust-config-fn:v0.1.0
minikube image load $(minikube ip):5000/crossplane-rust-config-fn:v0.1.0
crossplane xpkg build --package-root=configuration --package-file=conf.xpkg
crossplane xpkg push --package-files=conf.xpkg $(minikube ip):5000/crossplane-rust-config:latest
minikube image load $(minikube ip):5000/crossplane-rust-config:latest
kustomize build crossplane-providers --enable-helm | kubectl apply --context minikube -f -
kustomize build minikube/crossplane --enable-helm | kubectl apply --context minikube -f -
kustomize build minikube/crossplane-providers --enable-helm | kubectl apply --context minikube -f -
kustomize build minikube --enable-helm | kubectl apply --context minikube -f -
```
