pub use tonic;
pub use tokio;
pub use tracing;
pub use prost_types;
pub mod server;

mod from_resource;
pub use from_resource::from_resource;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
// Just include the crossplane types generated via tonic-build (see ../build.rs)
pub mod crossplane {
    include!("generated/apiextensions.r#fn.proto.v1.rs");
}
