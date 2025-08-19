use crate::crossplane::{RequestMeta, ResponseMeta};
use prost_types::Duration;

pub trait IntoResponseMeta: Sized {
    /// Translates the `request_meta` to a `response_meta` object.
    /// `cache_seconds` is how long the response can be kept in cache.
    #[must_use]
    fn into_response_meta(self, cache_seconds: i64) -> ResponseMeta;
}

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
