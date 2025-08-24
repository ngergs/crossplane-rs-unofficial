pub use tokio;
pub use tonic;
pub use tracing;

pub use map_meta::IntoResponseMeta;
pub use map_resource::{TryFromResource, TryIntoResource};
mod error;
mod map_meta;
mod map_resource;
pub mod server;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
// Just include the crossplane types generated via tonic-build (see ../build.rs)
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
