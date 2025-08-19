# crossplane-rs-sdk-unofficial

This is an **unofficial** [crossplane](https://www.crossplane.io/) [composite function](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/)
sdk written in Rust.

The technical requirements follow from the official [composite functions specification](https://github.com/crossplane/crossplane/blob/main/contributing/specifications/functions.md).
Basically we set up a grpc-server that has to support some custom CLI flags as well as configuration environment
variables (primarily for crossplane to inject the mTLS-configuration).

## Most relevant Rust files

- The crossplane files are generated from the protobuf schema and included in `src/lib.rs`(src/lib.rs).
- The grpc server setup is defined in `src/server.rs`.

## Compile-time dependencies

- [protoc](https://protobuf.dev/installation/) to generate rust structs from the protobuf crossplane schema (sdk).
- The git submodules needs to be pulled for the sdk-codegen to work (use e.g. `git submodule update --init --recursive`).

## Crates

There are **no crates published** for this sdk yet.
Please inform me if you are interested in using it and I will push it and add semantic versioning.
Till then, you can use it by defining a git dependency in Cargo.

```toml
crossplane-rs-sdk-unofficial = { git = "https://github.com/ngergs/crossplane-rs-unofficial.git" }
```