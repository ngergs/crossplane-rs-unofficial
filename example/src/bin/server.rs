use crossplane_rust_example_unofficial::function::ExampleFunction;
use crossplane_rust_sdk_unofficial::server::run_server;
use crossplane_rust_sdk_unofficial::tokio;

#[tokio::main]
/// Starts the grpc server and handles sigterm/sigint for shutdown
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_server(ExampleFunction {}).await?;
    Ok(())
}
