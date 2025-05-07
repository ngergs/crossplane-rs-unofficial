# crossplane-rust-example

Example how to setup up a crossplane composite function using Rust.
The requirements follow from the
official [composite functions specification](https://github.com/crossplane/crossplane/blob/main/contributing/specifications/functions.md).

This example generates a simple ConfigMap to avoid dependencies on CRDs for the example results.
For CRD outputs the same schema generation technique as for the inputs can be used (see [src/lib.rs](src/lib.rs)).

## Compile-time dependencies

To run the protobuf codegen we need [protoc](https://protobuf.dev/installation/) at compile-time.
