# crossplane-rust-example

This is an
*inofficial* [crossplane](https://www.crossplane.io/) [composite function](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/)
written in Rust.

The technical requirements follow from the
official [composite functions specification](https://github.com/crossplane/crossplane/blob/main/contributing/specifications/functions.md).
Basically we setup a grpc-server that has to support some custom flags via the CLI and env vars to configure mTLS.

This example defines a `Configs` custom resources which can be used to generate multiple ConfigMaps using a template (
it's just a toy example without much practical use). We just picked a Kubernetes resource as output to avoid relying on
a Cloud-dependent provider for the example.

## Compile-time dependencies

To run the protobuf codegen we need [protoc](https://protobuf.dev/installation/) at compile-time.

## Example

To see this composite function in action see the [example](example)-folder.