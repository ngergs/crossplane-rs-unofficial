# crossplane-rs-unofficial

This is an **unofficial** [crossplane](https://www.crossplane.io/) [composite function](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/) sdk and example written in Rust.

## Docs and getting started

The [rust-docs](https://ngergs.github.io/crossplane-rs-unofficial/) are a good starting point.

See alternatively the [example](example)-subfolder to get started.

## Compile-time dependencies

The protocol buffer libraries used by the sdk need [protoc](https://protobuf.dev/installation/) at compile-time.

## Crates

There are **no crates published** for this sdk yet.
Please inform me if you are interested in using it and I will push it and add semantic versioning.
Till then, you can use it by defining a git dependency in Cargo.

```toml
crossplane-rs-sdk-unofficial = { git = "https://github.com/ngergs/crossplane-rs-unofficial.git" }
```
