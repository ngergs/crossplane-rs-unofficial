use crate::composite_resource::XConfig;
use crate::output::{TryFromStatus, TryIntoResource};
use crossplane_rust_sdk_unofficial::crossplane::function_runner_service_server::FunctionRunnerService;
use crossplane_rust_sdk_unofficial::crossplane::{
    Ready, ResponseMeta, RunFunctionRequest, RunFunctionResponse,
};
use crossplane_rust_sdk_unofficial::prost_types::Duration;
use crossplane_rust_sdk_unofficial::tonic;
use crossplane_rust_sdk_unofficial::tonic::{Request, Response, Status};
use crossplane_rust_sdk_unofficial::tracing::info;
use k8s_openapi::api::core::v1::ConfigMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::{BTreeMap, HashMap};
use std::io::{Error, ErrorKind};

pub struct ExampleFunction {}

//  The core logic of the composite function goes here
#[tonic::async_trait]
impl FunctionRunnerService for ExampleFunction {
    async fn run_function(
        &self,
        request: Request<RunFunctionRequest>,
    ) -> Result<Response<RunFunctionResponse>, Status> {
        let request = request.into_inner();
        let observed = request.observed.ok_or(Error::new(
            ErrorKind::InvalidData,
            "composite resource field not set",
        ))?;
        let xconfig: XConfig = observed.composite.try_into()?;
        let claim_ref = xconfig.spec.claim_ref.ok_or(Error::new(
            ErrorKind::InvalidData,
            ".spec.claimRef not present",
        ))?;
        info!(
            api_version = claim_ref.api_version,
            kind = claim_ref.kind,
            name = claim_ref.name,
            namespace = claim_ref.namespace,
            "Received request"
        );
        let observed_conf = observed
            .resources
            .into_iter()
            .map(|(name, resource)| Ok::<_, Error>((name, ConfigMap::try_from_status(resource)?)))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<HashMap<_, ConfigMap>>();

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
                    namespace: Some(claim_ref.namespace.clone()),
                    ..Default::default()
                },
                data: Some(BTreeMap::from([("value".to_owned(), value)])),
                ..Default::default()
            };
            let ready = observed_conf
                .get(&value_set.name)
                .map_or(Ready::False, |observed_conf| {
                    if observed_conf.data == conf.data {
                        Ready::True
                    } else {
                        Ready::False
                    }
                });

            let mut desired_res = conf.try_into_resource()?;
            desired_res.set_ready(ready);
            desired.resources.insert(value_set.name, desired_res);
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
