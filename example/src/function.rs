use crate::composite_resource::Config;
use crossplane_rs_sdk_unofficial::crossplane::function_runner_service_server::FunctionRunnerService;
use crossplane_rs_sdk_unofficial::crossplane::{Resource, RunFunctionRequest, RunFunctionResponse};
use crossplane_rs_sdk_unofficial::error::error_invalid_data;
use crossplane_rs_sdk_unofficial::tonic::{Request, Response, Status};
use crossplane_rs_sdk_unofficial::tracing::info;
use crossplane_rs_sdk_unofficial::{tonic, IntoResponseMeta, TryFromOptionResource};
use crossplane_rs_sdk_unofficial::{TryFromResource, TryIntoResource};
use k8s_openapi::api::core::v1::ConfigMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::Resource as KubeResource;
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
        let observed = request.observed.unwrap_or_default();
        let config = Config::try_from_option_resource(observed.composite)?;
        log_request(&config);
        // must pass through any desired state we do not care about
        let mut desired = request.desired.unwrap_or_default();
        let observed_configmaps = resources_into_configmaps(observed.resources)?;

        // main logic. Go through the template and value sets, template the desired configmaps
        // and compare with the observed ones to determine whether they are ready.
        let namespace = config.meta().namespace.clone().ok_or(error_invalid_data(
            "composite metadata.namespace field not set",
        ))?;
        for value_set in config.spec.value_sets {
            let mut value = config.spec.template.clone();
            for (k, v) in &value_set.values {
                value = value.replace(&format!("{{{k}}}"), v);
            }
            let configmap = ConfigMap {
                metadata: ObjectMeta {
                    name: Some(value_set.name.clone()),
                    namespace: Some(namespace.clone()),
                    ..Default::default()
                },
                data: Some(BTreeMap::from([("value".to_owned(), value)])),
                ..Default::default()
            };
            let ready = observed_configmaps
                .get(&value_set.name)
                .is_some_and(|observed_conf| observed_conf.data == configmap.data)
                .into();

            let mut desired_configmap = configmap.try_into_resource()?;
            desired_configmap.set_ready(ready);
            desired.resources.insert(value_set.name, desired_configmap);
        }

        let result = RunFunctionResponse {
            context: request.context,
            meta: Some(request.meta.into_response_meta(60)),
            desired: Some(desired),
            ..Default::default()
        };
        Ok(result.into())
    }
}

/// Logs the request metadata (`api_version,kind,name,namespace`) of the composite resource on `INFO` level.
fn log_request(config: &Config) {
    info!(
        api_version = Config::api_version(&()).into_owned(),
        kind = Config::kind(&()).into_owned(),
        name = config.meta().name,
        namespace = config.meta().namespace,
        "Received request"
    );
}

/// Gets the observed/desired configmaps from the observed/desired resources.
fn resources_into_configmaps(
    resources: HashMap<String, Resource>,
) -> Result<HashMap<String, ConfigMap>, Error> {
    resources
        .into_iter()
        .map(|(name, resource)| Ok::<_, Error>((name, ConfigMap::try_from_resource(resource)?)))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::composite_resource::{Config, ConfigSpec, ConfigValueSets};
    use crate::function::{resources_into_configmaps, ExampleFunction};
    use crossplane_rs_sdk_unofficial::crossplane::function_runner_service_server::FunctionRunnerService;
    use crossplane_rs_sdk_unofficial::crossplane::{RequestMeta, RunFunctionRequest, State};
    use crossplane_rs_sdk_unofficial::tonic::Request;
    use crossplane_rs_sdk_unofficial::{tokio, TryIntoResource};
    use k8s_openapi::api::core::v1::ConfigMap;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    use std::collections::{BTreeMap, HashMap};

    struct TestCase {
        composite: Config,
        expected_desired: HashMap<String, ConfigMap>,
    }

    #[tokio::test]
    async fn composite_function() {
        let namespace = "testspace";
        let tcs = vec![TestCase {
            composite: Config {
                metadata: namespaced_name_meta("name", namespace),
                spec: ConfigSpec {
                    template: "Hello world, it's {time of day}".to_owned(),
                    value_sets: vec![
                        config_value_set("morning", [("time of day", "morning")]),
                        config_value_set("evening", [("time of day", "evening")]),
                    ],
                },
            },
            expected_desired: HashMap::from([
                config_map_desired_entry("morning", namespace, "Hello world, it's morning"),
                config_map_desired_entry("evening", namespace, "Hello world, it's evening"),
            ]),
        }];

        for tc in tcs {
            let tag = "123";
            let req = RunFunctionRequest {
                meta: Some(RequestMeta {
                    tag: tag.to_owned(),
                }),
                observed: Some(State {
                    composite: Some(tc.composite.try_into_resource().unwrap()),
                    ..Default::default()
                }),
                ..Default::default()
            };
            let composite_fn = ExampleFunction {};
            let rsp = composite_fn
                .run_function(Request::new(req))
                .await
                .unwrap()
                .into_inner();
            let desired =
                resources_into_configmaps(rsp.desired.unwrap_or_default().resources).unwrap();
            assert_eq!(tc.expected_desired, desired);
            assert_eq!(tag, rsp.meta.unwrap_or_default().tag);
        }
    }

    /// Creates an `ObjectMeta` that just holds name and namespace information
    fn namespaced_name_meta(name: &str, namespace: &str) -> ObjectMeta {
        ObjectMeta {
            name: Some(name.to_owned()),
            namespace: Some(namespace.to_owned()),
            ..Default::default()
        }
    }

    /// Constructs a `ConfigValueSet`, i.e. a mapping of values two values used by `Config`
    fn config_value_set<const N: usize>(name: &str, kv: [(&str, &str); N]) -> ConfigValueSets {
        ConfigValueSets {
            name: name.to_owned(),
            values: kv
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
        }
    }

    /// Constructs a `ConfigMap` in the format used for this example.
    /// I.e. one that just has one key `value` with the provided value as content.
    /// It outputs it as a HashMap-Entry for the desired state.
    fn config_map_desired_entry(name: &str, namespace: &str, value: &str) -> (String, ConfigMap) {
        (
            name.to_owned(),
            ConfigMap {
                metadata: namespaced_name_meta(name, namespace),
                data: Some(BTreeMap::from([("value".to_owned(), value.to_owned())])),
                ..Default::default()
            },
        )
    }
}
