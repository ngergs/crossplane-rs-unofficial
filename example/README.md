# crossplane-rust-example-unofficial

This is an **unofficial** [crossplane](https://www.crossplane.io/) [composite function](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/)
example written in Rust.

This example defines a `Configs` custom resources which can be used to generate multiple ConfigMaps using a template (
it's just a toy example without much practical use). We just picked a Kubernetes resource as output to avoid relying on
a Cloud-dependent provider for the example.

## Most relevant Rust files

- The core logic of the composite function can be found in [src/function.rs](src/function.rs).
- The input and output mappings are in [src/lib.rs](src/lib.rs).

## Compile-time dependencies

To run the protobuf codegen used by the sdk we need [protoc](https://protobuf.dev/installation/) at compile-time.

The git submodules needs to be pulled codegen to work (use e.g. `git submodule update --init --recursive`).

## Example

To see this composite function in action see the [example](example)-folder.