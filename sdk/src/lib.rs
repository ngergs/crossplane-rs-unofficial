//! This is an **unofficial** Rust [composite function sdk](https://docs.crossplane.io/latest/guides/write-a-composition-function-in-go/)
//! for [crossplane](https://www.crossplane.io/).
//!
//! Crossplane composite functions are implemented as gRPC server that have to handle specific cli args to setup mtls.
//! This sdk cares about this part and has generated rust types from the published Crossplane protocol buffer schema.
//! Furthermore, it provides some helper functions to simplify function writing.
//!
//! The central functionality besides some type mapping helper traits is provided via the [`run_server`]-function
//! which will start a gRPC server that handles the composite function requests with the business logic provided by
//! the sdk-user.
//!
//! # Compile-time dependencies
//! The protocol buffer library [prost-wkt-types](https://docs.rs/prost-wkt-types/latest/prost_wkt_types) used by the sdk requires [protoc](https://protobuf.dev/installation/) at compile-time.
//!
//! # Examples
//! ## Direct composite function (synchronous)
//! ```
//! # use std::error::Error;
//! # use tonic::Status;
//! # use crossplane_fn_sdk_unofficial::{run_server, IntoResponseMeta};
//! # use crossplane_fn_sdk_unofficial::crossplane::{RunFunctionRequest, RunFunctionResponse};
//! fn composite_function(request: RunFunctionRequest) -> Result<RunFunctionResponse,Status> {
//!     // Business logic goes here
//!     Ok(RunFunctionResponse {
//!         context: request.context,
//!         meta: Some(request.meta.into_response_meta(60)),
//!         desired: request.desired,
//!         ..Default::default()
//!     })
//! }
//!
//! # tokio_test::block_on(async {
//! #    Ok::<_, Box<dyn Error>>(
//! run_server(composite_function).await?
//! #    )
//! # });
//! ```
//! ## Explicit Trait-implementation (asynchronous)
//! ```
//! # use std::error::Error;
//! # use tonic::Status;
//! # use crossplane_fn_sdk_unofficial::{run_server, CompositeFunction, IntoResponseMeta};
//! # use crossplane_fn_sdk_unofficial::crossplane::{RunFunctionRequest, RunFunctionResponse};
//! struct ExampleFunction{}
//!
//! #[tonic::async_trait]
//! impl CompositeFunction for ExampleFunction {
//!     async fn run_function(&self,request: RunFunctionRequest) -> Result<RunFunctionResponse,Status> {
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
#[allow(warnings)]
#[allow(clippy)]
#[allow(unknown_lints)]
pub mod crossplane {
    include!("generated/crossplane.rs");

    impl From<bool> for Ready {
        fn from(value: bool) -> Self {
            match value {
                true => Ready::True,
                _ => Ready::False,
            }
        }
    }
}
