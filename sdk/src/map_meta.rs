use crate::crossplane::{RequestMeta, ResponseMeta};
use prost_types::Duration;

/// Marker trait to seal other traits so that they can't be implemented by other packages/crates.
trait Seal {}

/// Allows transforming this object into a `response_meta` object.
/// Mainly used to extend the `RequestMeta` type.
/// Sealed (cannot be implemented by users).
#[allow(private_bounds)]
pub trait IntoResponseMeta: Sized + Seal {
    /// Translates the `request_meta` to a `response_meta` object.
    /// `cache_seconds` is how long the response can be kept in cache.
    #[must_use]
    fn into_response_meta(self, cache_seconds: i64) -> ResponseMeta;
}

impl Seal for RequestMeta {}
impl IntoResponseMeta for RequestMeta {
    fn into_response_meta(self, cache_seconds: i64) -> ResponseMeta {
        ResponseMeta {
            tag: self.tag, // required by the spec to copy this from the request without modification
            ttl: Some(Duration {
                seconds: cache_seconds,
                nanos: 0,
            }),
        }
    }
}

impl Seal for Option<RequestMeta> {}
impl IntoResponseMeta for Option<RequestMeta> {
    fn into_response_meta(self, cache_seconds: i64) -> ResponseMeta {
        ResponseMeta {
            tag: self.map(|meta| meta.tag).unwrap_or_default(), // required by the spec to copy this from the request without modification
            ttl: Some(Duration {
                seconds: cache_seconds,
                nanos: 0,
            }),
        }
    }
}
