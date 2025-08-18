# crossplane-rust-example-unofficial

This is an **unofficial** [crossplane](https://www.crossplane.io/) [composite function](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/) example written in Rust.

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
- The input and output mappings are in [src/lib.rs](src/lib.rs).

## Compile-time dependencies

- [protoc](https://protobuf.dev/installation/) to generate rust structs from the protobuf crossplane schema (sdk).
- [kopium](https://github.com/kube-rs/kopium) to generate rust types from the composite resource definition given under [schema](schema).
- The git submodules needs to be pulled for the sdk-codegen to work (use e.g. `git submodule update --init --recursive`).

## Example

To see this composite function in action see the [local_test](local_test)-folder.