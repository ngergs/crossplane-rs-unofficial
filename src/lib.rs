use crate::composite_resource::XBuckets;
use crate::crossplane::function_runner_service_server::FunctionRunnerService;
use crate::crossplane::{ResponseMeta, RunFunctionRequest, RunFunctionResponse};
use crate::output::S3BucketCrossplaneApiVersion::S3AwsUpboundIoV1beta1;
use crate::output::S3BucketCrossplaneKind::Bucket;
use crate::output::{
    S3BucketCrossplane, S3BucketCrossplaneMetadata, S3BucketCrossplaneMetadataAnnotations,
    S3BucketCrossplaneSpec, S3BucketCrossplaneSpecForProvider,
};
use prost_types::Duration;
use std::io::{Error, ErrorKind};
use tonic::{Request, Response, Status};

pub mod crossplane {
    include!("apiextensions.r#fn.proto.v1.rs");
}

pub mod composite_resource {
    use crate::crossplane::Resource;
    use std::io::{Error, ErrorKind};

    typify::import_types!(schema = "example/xr.jsonschema");

    impl TryFrom<Option<Resource>> for XBuckets {
        type Error = Error;

        fn try_from(value: Option<Resource>) -> Result<Self, Self::Error> {
            let value =
                value.ok_or(Error::new(ErrorKind::InvalidData, "resource field not set"))?;
            let value = serde_json::to_value(
                &value
                    .resource
                    .ok_or(Error::new(ErrorKind::InvalidData, "resource field not set"))?,
            )?;
            Ok(serde_json::from_value(value)?)
        }
    }
}

pub mod output {
    use crate::crossplane::Resource;
    use std::io::Error;

    typify::import_types!(schema = "example/bucket.jsonschema");

    impl TryFrom<S3BucketCrossplane> for Resource {
        type Error = Error;

        fn try_from(value: S3BucketCrossplane) -> Result<Self, Self::Error> {
            let value = serde_json::to_value(&value)?;
            let fields = serde_json::from_value(value)?;
            Ok(Resource {
                resource: Some(fields),
                ..Default::default()
            })
        }
    }
}

pub struct ExampleFunction {}

#[tonic::async_trait]
impl FunctionRunnerService for ExampleFunction {
    async fn run_function(
        &self,
        request: Request<RunFunctionRequest>,
    ) -> Result<Response<RunFunctionResponse>, Status> {
        println!("RunFunction request: {:?}", request);
        let request = request.into_inner();
        let xbucket: XBuckets = request
            .observed
            .ok_or(Error::new(
                ErrorKind::InvalidData,
                "composite resource field not set",
            ))?
            .composite
            .try_into()?;

        let mut desired = request.desired.unwrap_or_default(); // MUST pass through any desired state we do not care about
        for name in xbucket.spec.names.into_iter() {
            let bucket = S3BucketCrossplane {
                api_version: S3AwsUpboundIoV1beta1,
                kind: Bucket,
                metadata: S3BucketCrossplaneMetadata {
                    annotations: S3BucketCrossplaneMetadataAnnotations {
                        crossplane_io_external_name: Some(name.clone()),
                    },
                },
                spec: S3BucketCrossplaneSpec {
                    for_provider: S3BucketCrossplaneSpecForProvider {
                        region: xbucket.spec.region.clone(),
                    },
                },
            };
            desired.resources.insert(name, bucket.try_into()?);
        }
        let result = RunFunctionResponse {
            meta: request.meta.and_then(|meta| {
                Some(ResponseMeta {
                    tag: meta.tag, // required by the spec to copy this from the request without modification
                    ttl: Some(Duration {
                        seconds: 60,
                        nanos: 0,
                    }), // time the result can be cached. SHOULD be set.
                })
            }),
            desired: Some(desired),
            context: request.context,
            results: vec![],
            requirements: None,
            conditions: vec![],
        };
        Ok(result.into())
    }
}
