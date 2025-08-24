pub use tokio;
pub use tonic;
pub use tracing;

pub use map_meta::IntoResponseMeta;
pub use map_resource::{TryFromResource, TryIntoResource};
pub use server::run_server;
mod error;
mod map_meta;
mod map_resource;
mod server;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
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
