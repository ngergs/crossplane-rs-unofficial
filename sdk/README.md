# crossplane-fn-sdk-unofficial

This is an **unofficial** Rust [composite function sdk](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/)
for  [crossplane](https://www.crossplane.io/).

The technical requirements follow from the official [composite functions specification](https://github.com/crossplane/crossplane/blob/main/contributing/specifications/functions.md).
Basically we set up a grpc-server that has to support some custom CLI flags as well as configuration environment
variables (primarily for crossplane to inject the mTLS-configuration).


## Usage

The most direct way to implement a composite function would be:
```rust
fn composite_function(request: RunFunctionRequest) -> Result<RunFunctionResponse,Status> { 
    // Business logic goes here
    Ok(RunFunctionResponse {
        context: request.context,
        meta: Some(request.meta.into_response_meta(60)),
        desired: request.desired,
        ..Default::default()
    })
}

run_server(composite_function).await?
```

### Docs
For detailed information see the  [full API documentation](https://ngergs.github.io/crossplane-fn-sdk-unofficial/).

Alternatively, the [../example](../example)-subfolder is a good way to get started.

## Most relevant Rust files

- The crossplane files are generated from the protobuf schema and included in [src/lib.rs](src/lib.rs).
- The grpc server setup is defined in [src/server.rs](src/server.rs).
- The most relevant trait implementations to map received crossplane protobuf Resources (basically `prost_wkt_types::Struct` for us) into typed Rust structs.

## Compile-time dependencies

- [protoc](https://protobuf.dev/installation/) for the protocol buffer library `prost-wkt-types` used by the sdk.

## Codegen
The `src/generated`[src/generated] file contains rust types generated from the
[official protobuf composite function schema](https://github.com/crossplane/crossplane/blob/main/proto/fn/v1/run_function.proto).

To re-generate it e.g. from the git tag `v2.0.2` use from this folder:
```bash
cargo run --manifest-path=../codegen/Cargo.toml codegen v2.0.2
```

## Crates

There are **no crates published** for this sdk yet.
Please inform me if you are interested in using it and I will push it and add semantic versioning.
Till then, you can use it by defining a git dependency in Cargo.

```toml
crossplane-fn-sdk-unofficial = { git = "https://github.com/ngergs/crossplane-fn-sdk-unofficial.git" }
```