# crossplane-fn-sdk-rs-unofficial

This is an **unofficial** Rust [composite function sdk](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/) 
for  [crossplane](https://www.crossplane.io/).

This is a v0-version that is not yet published to crates.io. Breaking changes are common.

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
For detailed information see the  [full API documentation](https://ngergs.github.io/crossplane-fn-sdk-rs-unofficial/).

Alternatively, the [example](example)-subfolder is a good way to get started.

## Compile-time dependencies

The protocol buffer libraries used by the sdk need [protoc](https://protobuf.dev/installation/) at compile-time.

## Crates

There are **no crates published** for this sdk yet.
Please inform me if you are interested in using it and I will push it and add semantic versioning.
Till then, you can use it by defining a git dependency in Cargo.

```toml
crossplane-fn-sdk-rs-unofficial = { git = "https://github.com/ngergs/crossplane-fn-sdk-rs-unofficial.git" }
```
