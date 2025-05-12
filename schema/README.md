## Provider-kubernetes schema

To extract the provider kubernetes schema use with [mikefarah yq v4](https://github.com/mikefarah/yq):

```bash
yq '.spec.versions |  map(select(.storage == true)) | .[].schema.openAPIV3Schema | .title="Object"' provider-kubernetes/package/crds/kubernetes.crossplane.io_objects.yaml -o json > kubernetes_object.jsonschema
```

## XR Jsonschema

The `xr.jsonschema` can be obtained from the `xrd.yaml` via [mikefarah yq v4](https://github.com/mikefarah/yq):

```bash
yq '.spec.versions[0].schema.openAPIV3Schema | .title="XConfig" | . *= load("claimRef.yaml")' xrd.yaml -o json > xr.jsonschema
```