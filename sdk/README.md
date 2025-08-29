# crossplane-fn-sdk-unofficial

This is an **unofficial** Rust [composite function sdk](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/)
for  [crossplane](https://www.crossplane.io/).

The technical requirements follow from the official [composite functions specification](https://github.com/crossplane/crossplane/blob/main/contributing/specifications/functions.md).
Basically it sets up a gRPC-server that has to support CLI flags according to the specification
as well as configuration environment variables (primarily for crossplane to inject the mTLS-configuration).

## Usage

Add the sdk dependency:
```bash
cargo add crossplane-fn-sdk-unofficial
```

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

run_server(Args::parse(), composite_function).await?
```

### Docs
For detailed information see the [full API documentation](https://docs.rs/crossplane-fn-sdk-unofficial/0.1.0/crossplane_fn_sdk_unofficial/).

Alternatively, the [examples](https://github.com/ngergs/crossplane-fn-sdk-rs-unofficial/tree/main/example) are a good way to get started.

## Compile-time dependencies

- [protoc](https://protobuf.dev/installation/) for the protocol buffer library `prost-wkt-types` used by the sdk.
