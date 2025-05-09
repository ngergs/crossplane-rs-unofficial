use crate::composite_resource::XConfig;
use crate::crossplane::function_runner_service_server::FunctionRunnerService;
use crate::crossplane::{ResponseMeta, RunFunctionRequest, RunFunctionResponse};
use k8s_openapi::api::core::v1::ConfigMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use prost_types::Duration;
use std::collections::BTreeMap;
use std::io::{Error, ErrorKind};
use tonic::{Request, Response, Status};

pub struct ExampleFunction {}

#[tonic::async_trait]
impl FunctionRunnerService for ExampleFunction {
    async fn run_function(
        &self,
        request: Request<RunFunctionRequest>,
    ) -> Result<Response<RunFunctionResponse>, Status> {
        println!("RunFunction request: {:?}", request);
        let request = request.into_inner();
        let xconfig: XConfig = request
            .observed
            .ok_or(Error::new(
                ErrorKind::InvalidData,
                "composite resource field not set",
            ))?
            .composite
            .try_into()?;
        let mut desired = request.desired.unwrap_or_default(); // MUST pass through any desired state we do not care about
        for value_set in xconfig.spec.value_sets {
            let mut value = xconfig.spec.template.clone();
            for (k, v) in &value_set.values {
                value = value.replace(&format!("{{{k}}}"), v);
            }
            let conf = ConfigMap {
                metadata: ObjectMeta {
                    // not possible? https://github.com/crossplane/crossplane/issues/1730
                    name: Some(value_set.name.clone()),
                    namespace: xconfig.spec.claim_ref.clone().map(|c| c.namespace),
                    ..Default::default()
                },
                data: Some(BTreeMap::from([("value".to_owned(), value)])),
                ..Default::default()
            };
            desired.resources.insert(value_set.name, conf.try_into()?);
        }
        let result = RunFunctionResponse {
            meta: request.meta.map(|meta| ResponseMeta {
                tag: meta.tag, // required by the spec to copy this from the request without modification
                ttl: Some(Duration {
                    seconds: 60,
                    nanos: 0,
                }), // time the result can be cached. SHOULD be set.
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
