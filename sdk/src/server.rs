use crate::crossplane::function_runner_service_server::{
    FunctionRunnerService, FunctionRunnerServiceServer,
};
use clap::Parser;
use std::path::Path;
use tokio::signal::unix::{signal, SignalKind};
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use tracing::Level;

/// CLI arguments as required by the spec, <https://github.com/crossplane/crossplane/blob/main/contributing/specifications/functions.md>
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// To enable debug logging
    #[arg(long, env = "DEBUG", default_value_t = false)]
    #[clap(action)]
    debug: bool,

    /// For local debugging, skips mtls setup!
    #[arg(long, env = "INSECURE", default_value_t = false)]
    #[clap(action)]
    insecure: bool,

    /// Directory containing mTLS certs (tls.key and tls.crt) and a CA (ca.crt) for client verification
    #[arg(long, env = "TLS_SERVER_CERTS_DIR")]
    tls_certs_dir: String,
}

/// Reads a TLS certificate or key from a directory  with the given file name
fn cert_from_dir(
    cert_dir: &str,
    file_name: &str,
) -> Result<Certificate, Box<dyn std::error::Error>> {
    Ok(Certificate::from_pem(std::fs::read(
        Path::new(cert_dir).join(file_name),
    )?))
}

/// Reads the cli arguments, configures and starts the grpc server and handles sigterm/sigint for shutdown.
/// Calls the provided `FunctionRunnerService`-impl for the core business logic of the composite function.
/// The cli follows the [official composite function spec](https://github.com/crossplane/crossplane/blob/main/contributing/specifications/functions.md).
///
/// # Errors
/// - If the tcp port 9443 cannot be claimed.
/// - If cli arguments are malformed.
/// - If referenced tls certificate files are missing or have malformed content.
///
/// # Examples
///
/// ```
/// # use tonic::{Request, Response, Status};
/// # use crossplane_rs_sdk_unofficial::crossplane::function_runner_service_server::FunctionRunnerService;
/// # use crossplane_rs_sdk_unofficial::crossplane::{RunFunctionRequest, RunFunctionResponse};///
/// # use crossplane_rs_sdk_unofficial::{run_server, IntoResponseMeta};
/// struct ExampleFunction{}
///
/// #[tonic::async_trait]
/// impl FunctionRunnerService for ExampleFunction {
///    async fn run_function(
///        &self,
///       request: Request<RunFunctionRequest>,
///    ) -> Result<Response<RunFunctionResponse>, Status> {
///   let request = request.into_inner();
///   // Business logic goes here
///   Ok(RunFunctionResponse {
///             context: request.context,
///             meta: Some(request.meta.into_response_meta(60)),
///             desired: request.desired,
///             ..Default::default()
///     }.into())
///   }
/// }
///
/// // need to await in actual code
/// run_server(ExampleFunction{});
/// ```
pub async fn run_server(f: impl FunctionRunnerService) -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut log_config = tracing_subscriber::fmt().json();
    log_config = if args.debug {
        log_config.with_max_level(Level::DEBUG)
    } else {
        log_config.with_max_level(Level::INFO)
    };
    log_config.init();

    let addr = "[::]:9443".parse()?;
    let mut srv = Server::builder();
    if !args.insecure {
        let ca = cert_from_dir(args.tls_certs_dir.as_str(), "ca.crt")?;
        let cert = cert_from_dir(args.tls_certs_dir.as_str(), "tls.crt")?;
        let key = cert_from_dir(args.tls_certs_dir.as_str(), "tls.key")?;
        let tls_conf = ServerTlsConfig::new()
            .client_ca_root(ca)
            .client_auth_optional(false)
            .identity(Identity::from_pem(cert, key));
        srv = srv.tls_config(tls_conf)?;
    }
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;
    srv.add_service(FunctionRunnerServiceServer::new(f))
        .serve_with_shutdown(addr, async {
            tokio::select! {
                _ = sigterm.recv() => (),
                _ = sigint.recv() => (),
            }
        })
        .await?;
    Ok(())
}
