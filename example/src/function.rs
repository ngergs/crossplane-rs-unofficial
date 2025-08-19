use crate::composite_resource::Config;
use crossplane_rs_sdk_unofficial::crossplane::function_runner_service_server::FunctionRunnerService;
use crossplane_rs_sdk_unofficial::crossplane::{RunFunctionRequest, RunFunctionResponse};
use crossplane_rs_sdk_unofficial::errors::error_invalid_data;
use crossplane_rs_sdk_unofficial::tonic::{Request, Response, Status};
use crossplane_rs_sdk_unofficial::tracing::info;
use crossplane_rs_sdk_unofficial::{into_response_meta, take_from_observed_composite, tonic};
use crossplane_rs_sdk_unofficial::{TryFromResource, TryIntoResource};
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
        let mut request = request.into_inner();
        let config = take_from_observed_composite(&mut request)?;
        log_request(&config);
        let namespace = config.meta().namespace.clone().ok_or(error_invalid_data(
            "composite metadata.namespace field not set",
        ))?;

        let observed_conf = request
            .observed
            .unwrap_or_default()
            .resources
            .into_iter()
            .map(|(name, resource)| Ok::<_, Error>((name, ConfigMap::try_from_resource(resource)?)))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<HashMap<_, ConfigMap>>();

        // MUST pass through any desired state we do not care about
        let mut desired = request.desired.unwrap_or_default();
        // main logic. Go through the template and value sets, template the configmaps
        // and compare with the observed ones.
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
            context: request.context,
            meta: into_response_meta(request.meta, 60),
            desired: Some(desired),
            ..Default::default()
        };
        Ok(result.into())
    }
}

fn log_request(config: &Config) {
    info!(
        api_version = Config::api_version(&()).into_owned(),
        kind = Config::kind(&()).into_owned(),
        name = config.meta().name,
        namespace = config.meta().namespace,
        "Received request"
    );
}
