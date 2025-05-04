use crate::crossplane::function_runner_service_server::FunctionRunnerService;
use crate::crossplane::{ResponseMeta, RunFunctionRequest, RunFunctionResponse};
use prost_types::Duration;
use tonic::{Request, Response, Status};

pub mod crossplane {
    include!("apiextensions.r#fn.proto.v1.rs");
}

pub mod example_crd {
    // todo: mapping are missing to get the inflight request to this type...
    typify::import_types!(schema = "example/xr.jsonschema", derives = []);
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
            desired: request.desired, // MUST pass through any desired state we do not care about
            context: request.context,
            results: vec![],
            requirements: None,
            conditions: vec![],
        };
        Ok(result.into())
    }
}
