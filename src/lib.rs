use crate::crossplane::function_runner_service_server::FunctionRunnerService;
use crate::crossplane::{Resource, ResponseMeta, RunFunctionRequest, RunFunctionResponse};
use prost_types::value::Kind::{StringValue, StructValue};
use prost_types::{Duration, Struct, Value};
use std::collections::BTreeMap;
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
        desired.resources.insert(
            "test-bucket".to_owned(),
            Resource {
                resource: Some(Struct {
                    fields: BTreeMap::from([(
                        "metadata".to_owned(),
                        Value {
                            kind: Some(StructValue(Struct {
                                fields: BTreeMap::from([(
                                    "name".to_owned(),
                                    Value {
                                        kind: Some(StringValue("test".to_owned())),
                                    },
                                )]),
                            })),
                        },
                    )]),
                }),
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
