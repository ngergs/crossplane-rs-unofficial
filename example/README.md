# crossplane-fn-sdk-rs-example-unofficial

This is an **unofficial** [composite function sdk](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/)
example for [crossplane](https://www.crossplane.io/) for Rust.

This example defines a `Config` custom resource which can be used to generate multiple ConfigMaps using a template 
(it's just a toy example without much practical use). We just picked a Kubernetes resource as output to avoid relying on
a Cloud-dependent provider for the example.

```yaml
apiVersion: ngergs.de/v1alpha1
kind: Config
metadata:
  name: example-configs
  namespace: test
spec:
  template: Hello {world}, it's {timeOfDay}
  valueSets:
    - name: morning
      values:
        world: world
        timeOfDay: morning
    - name: evening
      values:
        world: world
        timeOfDay: evening
```

## Most relevant Rust files

- The core logic of the composite function can be found in [src/function.rs](src/function.rs).

## Compile-time dependencies

- [protoc](https://protobuf.dev/installation/) for the protocol buffer libraries used by the sdk.
- [kopium](https://github.com/kube-rs/kopium) to generate rust types from the composite resource definition given under [schema](schema).

## Example

To see this composite function in action see the [local_test](local_test)-folder.