# crossplane-rust-unofficial

This is an **unofficial** [crossplane](https://www.crossplane.io/) [composite function](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/)
sdk and example written in Rust.

See the [example](example)-subfolder to get started.

## Compile-time dependencies

To run the protobuf codegen we need [protoc](https://protobuf.dev/installation/) at compile-time.

The git submodules needs to be pulled for protobuf codegen to work (use e.g. `git submodule update --init --recursive`).

## Crates

There are **no crates published** for this sdk yet.
Please inform me if you are interested in using it and I will push it and add semantic versioning.
Till then, you can use it by defining a git dependency in Cargo.

```toml
crossplane-rust-sdk-unofficial = { git = "https://github.com/ngergs/crossplane-rust-unofficial.git" }
```
## License

The license obviously does not apply for the git submodules.
