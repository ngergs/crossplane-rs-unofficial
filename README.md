# crossplane-fn-sdk-rs-unofficial

This is an **unofficial** Rust [composite function sdk](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/) 
for  [crossplane](https://www.crossplane.io/).

This is a v0 version, breaking changes are common.

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

run_server(composite_function).await?
```

### Docs
For detailed information see the [full API documentation](https://docs.rs/crossplane-fn-sdk-unofficial/0.1.0/crossplane_fn_sdk_unofficial/).

The API documentation for the (potentially unpublished) main branch can be found [here](https://ngergs.github.io/crossplane-fn-sdk-rs-unofficial/).

Alternatively, the [examples](https://github.com/ngergs/crossplane-fn-sdk-rs-unofficial/tree/main/example) are a good way to get started.

## Compile-time dependencies

The protocol buffer library [prost-wkt-types](https://docs.rs/prost-wkt-types/latest/prost_wkt_types) used by the sdk requires [protoc](https://protobuf.dev/installation/) at compile-time.