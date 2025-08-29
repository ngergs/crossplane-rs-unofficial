# crossplane-fn-sdk-unofficial-codegen

Internal crate used to (re-)generate the [Rust types for the sdk](../sdk/src/generated/crossplane.rs)
from the Crossplane [official protobuf composite function schema](https://github.com/crossplane/crossplane/blob/main/proto/fn/v1/run_function.proto).

This crate won't be published to crates.io.

## Usage
Run in [../sdk](../sdk):
```bash
cargo run --manifest-path=../codegen/Cargo.toml codegen v2.0.2
```