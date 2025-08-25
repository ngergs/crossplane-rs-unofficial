# crossplane-rs-sdk-unofficial

This is an **unofficial** [crossplane](https://www.crossplane.io/) [composite function](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/)
sdk written in Rust.

The technical requirements follow from the official [composite functions specification](https://github.com/crossplane/crossplane/blob/main/contributing/specifications/functions.md).
Basically we set up a grpc-server that has to support some custom CLI flags as well as configuration environment
variables (primarily for crossplane to inject the mTLS-configuration).

## Docs
You can browse the [rust-docs here](https://ngergs.github.io/crossplane-rs-unofficial/).

## Most relevant Rust files

- The crossplane files are generated from the protobuf schema and included in [src/lib.rs](src/lib.rs).
- The grpc server setup is defined in [src/server.rs](src/server.rs).
- The most relevant trait implementations to map received crossplane protobuf Resources (basically `prost_wkt_types::Struct` for us) into typed Rust structs.

## Compile-time dependencies

- [protoc](https://protobuf.dev/installation/) for the protocol buffer library `prost-wkt-types` used by the sdk.

## Codegen
The `src/generated`[src/generated] file contains rust types generated from the
[official protobuf composite function schema](https://github.com/crossplane/crossplane/blob/main/proto/fn/v1/run_function.proto).

To re-generate it e.g. from the git tag `v2.0.2` use:
```bash
cargo run --features codegen codegen v2.0.2
```

## Crates

There are **no crates published** for this sdk yet.
Please inform me if you are interested in using it and I will push it and add semantic versioning.
Till then, you can use it by defining a git dependency in Cargo.

```toml
crossplane-rs-sdk-unofficial = { git = "https://github.com/ngergs/crossplane-rs-unofficial.git" }
```