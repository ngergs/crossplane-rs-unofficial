use crate::crossplane::function_runner_service_server::FunctionRunnerService;
use crate::crossplane::{Resource, ResponseMeta, RunFunctionRequest, RunFunctionResponse};
use crate::output::S3BucketCrossplaneApiVersion::S3AwsUpboundIoV1beta1;
use crate::output::S3BucketCrossplaneKind::Bucket;
use crate::output::{
    S3BucketCrossplane, S3BucketCrossplaneMetadata, S3BucketCrossplaneSpec,
    S3BucketCrossplaneSpecForProvider,
};
use prost_types::Duration;
use tonic::{Request, Response, Status};

pub mod crossplane {
    include!("apiextensions.r#fn.proto.v1.rs");
}

pub mod composite_resource {
    // todo: mapping are missing to get the inflight request to this type...
    typify::import_types!(schema = "example/xr.jsonschema");
}

pub mod output {
    typify::import_types!(schema = "example/bucket.jsonschema");
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

        let mut desired = request.desired.unwrap_or_default(); // MUST pass through any desired state we do not care about
        let bucket = S3BucketCrossplane {
            api_version: S3AwsUpboundIoV1beta1,
            kind: Bucket,
            metadata: S3BucketCrossplaneMetadata {
                generate_name: "test".to_string(),
                annotations: Default::default(),
                labels: Default::default(),
            },
            spec: S3BucketCrossplaneSpec {
                for_provider: S3BucketCrossplaneSpecForProvider {
                    region: "test".to_string(),
                },
            },
        };
        let val = serde_json::to_value(&bucket).map_err(|e| Status::internal(e.to_string()))?;
        let fields = serde_json::from_value(val).map_err(|e| Status::internal(e.to_string()))?;
        desired.resources.insert(
            "test-bucket".to_owned(),
            Resource {
                resource: Some(fields),
                ..Default::default()
            },
        );
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
