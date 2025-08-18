use crate::crossplane::{RequestMeta, ResponseMeta};
use prost_types::Duration;

/// Translates the `request_meta` to a `response_meta` object.
/// `cache_seconds` is how long the response can be kept in cache.
#[must_use]
pub fn to_response_meta(
    request_meta: Option<RequestMeta>,
    cache_seconds: i64,
) -> Option<ResponseMeta> {
    Some(ResponseMeta {
        tag: request_meta.map(|meta| meta.tag).unwrap_or_default(), // required by the spec to copy this from the request without modification
        ttl: Some(Duration {
            seconds: cache_seconds,
            nanos: 0,
        }),
    })
}
