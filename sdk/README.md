# crossplane-rust-example

This is an
*unofficial** [crossplane](https://www.crossplane.io/) [composite function](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/)
sdk written in Rust.

The technical requirements follow from the
official [composite functions specification](https://github.com/crossplane/crossplane/blob/main/contributing/specifications/functions.md).
Basically we set up a grpc-server that has to support some custom CLI flags as well as configuration environment
variables (primarily for crossplane to inject the mTLS-configuration).

## Most relevant Rust files

- The crossplane files are generated from the protobuf schema and included in `src/lib.rs`(src/lib.rs).
- The grpc server setup is defined in `src/server.rs`.

## Compile-time dependencies

To run the protobuf codegen we need [protoc](https://protobuf.dev/installation/) at compile-time.

The git submodules needs to be pulled for protobuf codegen to work (use e.g. `git submodule update --init --recursive`).
