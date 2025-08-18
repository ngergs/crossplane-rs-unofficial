use crate::composite_resource::Config;
use crossplane_rust_sdk_unofficial::crossplane::function_runner_service_server::FunctionRunnerService;
use crossplane_rust_sdk_unofficial::crossplane::{RunFunctionRequest, RunFunctionResponse};
use crossplane_rust_sdk_unofficial::errors::error_invalid_data;
use crossplane_rust_sdk_unofficial::tonic::{Request, Response, Status};
use crossplane_rust_sdk_unofficial::tracing::info;
use crossplane_rust_sdk_unofficial::{to_response_meta, tonic};
use crossplane_rust_sdk_unofficial::{TryFromResource, TryIntoResource};
use k8s_openapi::api::core::v1::ConfigMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::Resource;
use std::collections::{BTreeMap, HashMap};
use std::io::Error;

pub struct ExampleFunction {}

//  The core logic of the composite function goes here
#[tonic::async_trait]
impl FunctionRunnerService for ExampleFunction {
    async fn run_function(
        &self,
        request: Request<RunFunctionRequest>,
    ) -> Result<Response<RunFunctionResponse>, Status> {
        let request = request.into_inner();
        let observed = request
            .observed
            .ok_or(error_invalid_data("composite resource field not set"))?;
        let config = Config::try_from_resource(
            observed
                .composite
                .ok_or(error_invalid_data("resource not set"))?,
        )?;
        let namespace = config.meta().namespace.clone().ok_or(error_invalid_data(
            "composite metadata.namespace field not set",
        ))?;
        info!(
            api_version = Config::api_version(&()).into_owned(),
            kind = Config::kind(&()).into_owned(),
            name = config.meta().name,
            namespace = config.meta().namespace,
            "Received request"
        );
        let observed_conf = observed
            .resources
            .into_iter()
            .map(|(name, resource)| Ok::<_, Error>((name, ConfigMap::try_from_resource(resource)?)))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<HashMap<_, ConfigMap>>();

        let mut desired = request.desired.unwrap_or_default(); // MUST pass through any desired state we do not care about
        for value_set in config.spec.value_sets {
            let mut value = config.spec.template.clone();
            for (k, v) in &value_set.values {
                value = value.replace(&format!("{{{k}}}"), v);
            }
            let conf = ConfigMap {
                metadata: ObjectMeta {
                    name: Some(value_set.name.clone()),
                    namespace: Some(namespace.clone()),
                    ..Default::default()
                },
                data: Some(BTreeMap::from([("value".to_owned(), value)])),
                ..Default::default()
            };
            let ready = observed_conf
                .get(&value_set.name)
                .is_some_and(|observed_conf| observed_conf.data == conf.data)
                .into();

            let mut desired_res = conf.try_into_resource()?;
            desired_res.set_ready(ready);
            desired.resources.insert(value_set.name, desired_res);
        }
        let result = RunFunctionResponse {
            meta: to_response_meta(request.meta, 60),
            desired: Some(desired),
            context: request.context,
            ..Default::default()
        };

        Ok(result.into())
    }
}
