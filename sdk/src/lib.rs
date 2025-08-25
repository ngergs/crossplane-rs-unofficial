//! A Rust sdk for [crossplane](https://www.crossplane.io/) [composite function](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/).
//!
//! Crossplane composite functions are implemented as gRPC server that have to handle specific cli args to setup mtls.
//! This sdk cares about this part and has generated rust types from the published Crossplane protocol buffer schema.
//! Furthermore, it provides some helper functions to simplify function writing.
//!
//! # Example
//! ```
//! # use std::error::Error;
//! # use tonic::Status;
//! # use crossplane_rs_sdk_unofficial::{run_server, CompositeFunction, IntoResponseMeta};
//! # use crossplane_rs_sdk_unofficial::crossplane::{RunFunctionRequest, RunFunctionResponse};
//! struct ExampleFunction{}
//!
//! #[tonic::async_trait]
//! impl CompositeFunction for ExampleFunction {
//!     async fn run_function(&self, request: RunFunctionRequest) -> Result<RunFunctionResponse,Status> {
//!         // Business logic goes here
//!         Ok(RunFunctionResponse {
//!             context: request.context,
//!             meta: Some(request.meta.into_response_meta(60)),
//!             desired: request.desired,
//!             ..Default::default()
//!         })
//!     }
//! }
//!
//! # tokio_test::block_on(async {
//! #    Ok::<_, Box<dyn Error>>(
//! run_server(ExampleFunction{}).await?
//! #    )
//! # });
//! ```
pub use tokio;
pub use tonic;
pub use tracing;

pub use map_meta::IntoResponseMeta;
pub use map_resource::{TryFromResource, TryIntoResource};
pub use server::{run_server, CompositeFunction};
mod error;
mod map_meta;
mod map_resource;
mod server;

/// Rust types generated from the [official crossplane protobuf types](https://github.com/crossplane/crossplane/tree/main/proto/fn/v1)
/// for composite function.
pub mod crossplane {
    include!("generated/apiextensions.r#fn.proto.v1.rs");

    impl From<bool> for Ready {
        fn from(value: bool) -> Self {
            match value {
                true => Ready::True,
                false => Ready::False,
            }
        }
    }
}
