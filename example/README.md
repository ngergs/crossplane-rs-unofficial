# crossplane-rust-example

This is an
*unofficial** [crossplane](https://www.crossplane.io/) [composite function](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/)
written in Rust.

The technical requirements follow from the
official [composite functions specification](https://github.com/crossplane/crossplane/blob/main/contributing/specifications/functions.md).
Basically we set up a grpc-server that has to support some custom CLI flags as well as configuration environment
variables (primarily for crossplane to inject the mTLS-configuration).

This example defines a `Configs` custom resources which can be used to generate multiple ConfigMaps using a template (
it's just a toy example without much practical use). We just picked a Kubernetes resource as output to avoid relying on
a Cloud-dependent provider for the example.

## Most relevant Rust files

- The core logic of the composite function can be found in [src/function.rs](src/function.rs).
- The input and output mappings are in [src/lib.rs](src/lib.rs).
- The startup of the grpc server and cli flag/env-var handling is in [src/bin/server/main.rs](src/bin/server.rs).

## Compile-time dependencies

To run the protobuf codegen we need [protoc](https://protobuf.dev/installation/) at compile-time.

The git submodules needs to be pulled for protobuf codegen to work (use e.g. `git submodule update --init --recursive`).

## Example

To see this composite function in action see the [example](example)-folder.

## License

The license obviously does not apply for the git submodules.
