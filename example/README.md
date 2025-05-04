# Local test example

Run the local rust composite function in insecure mode (in the folder above)
```bash
cargo run --package crossplane-rust-example --bin server -- --tls-certs-dir . --insecure
```
and than execute the crossplane local rendering (here)
```bash
crossplane render xr.yaml composition.yaml functions.yaml
```
