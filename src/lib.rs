pub mod function;

pub mod crossplane {
    include!("apiextensions.r#fn.proto.v1.rs");
}

pub mod composite_resource {
    use crate::crossplane::Resource;
    use std::io::{Error, ErrorKind};

    typify::import_types!(schema = "example/xr.jsonschema");

    impl TryFrom<Option<Resource>> for XBuckets {
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
    use std::io::Error;

    typify::import_types!(schema = "example/bucket.jsonschema");

    impl TryFrom<S3BucketCrossplane> for Resource {
        type Error = Error;

        fn try_from(value: S3BucketCrossplane) -> Result<Self, Self::Error> {
            let value = serde_json::to_value(&value)?;
            let fields = serde_json::from_value(value)?;
            Ok(Resource {
                resource: Some(fields),
                ..Default::default()
            })
        }
    }
}
