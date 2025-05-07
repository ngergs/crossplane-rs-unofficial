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
    use std::io::Error;

    impl TryFrom<ConfigMap> for Resource {
        type Error = Error;

        fn try_from(value: ConfigMap) -> Result<Self, Self::Error> {
            let value = serde_json::to_value(&value)?;
            let fields = serde_json::from_value(value)?;
            Ok(Resource {
                resource: Some(fields),
                ..Default::default()
            })
        }
    }
}
