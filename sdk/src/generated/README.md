## Codegen
The `crossplane.rs` file contains rust types generated from the
[official protobuf composite function schema](https://github.com/crossplane/crossplane/blob/main/proto/fn/v1/run_function.proto).

To re-generate it e.g. from the git tag `v2.0.2` use from this folder:
```bash
cargo run --manifest-path=../../../codegen/Cargo.toml codegen v2.0.2
```
