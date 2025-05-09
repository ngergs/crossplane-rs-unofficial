pub mod function;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
pub mod crossplane {
    include!("apiextensions.r#fn.proto.v1.rs");
}

pub mod composite_resource {
    use crate::crossplane::Resource;
    use std::io::{Error, ErrorKind};

    typify::import_types!(schema = "example/xr.jsonschema");

    impl TryFrom<Option<Resource>> for XConfig {
        type Error = Error;

        fn try_from(value: Option<Resource>) -> Result<Self, Self::Error> {
            let resource = value
                .ok_or(Error::new(ErrorKind::InvalidData, "resource field not set"))?
                .resource
                .ok_or(Error::new(ErrorKind::InvalidData, "resource field not set"))?;
            let value = serde_json::to_value(&resource)?;
            Ok(serde_json::from_value(value)?)
        }
    }
}

pub mod output {
    use crate::crossplane::Resource;
    use k8s_openapi::api::core::v1::ConfigMap;
    use std::io::{Error, ErrorKind};

    typify::import_types!(schema = "schema/kubernetes_object.jsonschema");

    impl TryFrom<ConfigMap> for Resource {
        type Error = Error;

        fn try_from(value: ConfigMap) -> Result<Self, Self::Error> {
            // have to wrap it in a kubernetes-provider Object till crossplane v2
            // (no namespaced object support as output from compositions in v1)
            // see https://github.com/crossplane/crossplane/issues/1730
            let conf_json = serde_json::to_value(&value)?;
            let serde_json::Value::Object(conf_map) = conf_json else {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "expected structured object as config map",
                ));
            };
            let obj = Object {
                api_version: Some("kubernetes.crossplane.io/v1alpha2".to_owned()),
                kind: Some("Object".to_owned()),
                metadata: Default::default(),
                spec: ObjectSpec {
                    for_provider: ObjectSpecForProvider { manifest: conf_map },
                    provider_config_ref: Some(ObjectSpecProviderConfigRef {
                        name: "provider-kubernetes".to_string(),
                        policy: None,
                    }),
                    management_policies: vec![ObjectSpecManagementPoliciesItem::X],
                    // only empty values below
                    connection_details: vec![],
                    deletion_policy: Default::default(),
                    publish_connection_details_to: None,
                    readiness: None,
                    references: vec![],
                    watch: false,
                    write_connection_secret_to_ref: None,
                },
                status: None,
            };
            let obj_json = serde_json::to_value(&obj)?;
            let obj_struct = serde_json::from_value(obj_json)?;
            Ok(Resource {
                resource: Some(obj_struct),
                ..Default::default()
            })
        }
    }
}
