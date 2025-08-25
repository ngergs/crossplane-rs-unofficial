#[cfg(feature = "codegen")]
use itertools::Itertools;
#[cfg(feature = "codegen")]
use std::env::args;
#[cfg(feature = "codegen")]
use std::fs::OpenOptions;
#[cfg(feature = "codegen")]
use std::io::{Error, Read, Write};
#[cfg(feature = "codegen")]
use tempfile::NamedTempFile;

#[cfg(feature = "codegen")]
const TARGET_PATH: &str = "./src/generated/apiextensions.r#fn.proto.v1.rs";

#[cfg(feature = "codegen")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tag = args().skip(2).exactly_one()?;
    let mut proto_file = NamedTempFile::new()?;
    let proto_url = &format!(
        "https://raw.githubusercontent.com/crossplane/crossplane/refs/tags/{tag}/proto/fn/v1/run_function.proto"
    );
    let proto_rsp = reqwest::get(proto_url).await?;
    if !proto_rsp.status().is_success() {
        return Err(format!(
            "Fetching crossplane protobuf file from {proto_url} failed with status code {}",
            proto_rsp.status()
        )
        .into());
    }
    proto_file.write_all(proto_rsp.bytes().await?.as_ref())?;

    std::fs::create_dir_all("src/generated")?;
    tonic_prost_build::configure()
        // use prost_wkt_types to have serializable structs (used in ../example/src/lib.rs to map types)
        .extern_path(".google.protobuf.Struct", "::prost_wkt_types::Struct")
        .build_server(true)
        .out_dir("src/generated")
        .compile_protos(
            &[proto_file.path()],
            &[proto_file.path().parent().ok_or(
                "temporary protobuf file is stored at a root location. Please report a bug.",
            )?],
        )?;

    prepend_to_file(
        TARGET_PATH,
        &format!(
            "// Generated from Crossplane {tag} composite function protobuf schema: https://github.com/crossplane/crossplane/tree/{tag}/proto/fn/v1\n"
        ),
    )?;
    Ok(())
}

#[cfg(feature = "codegen")]
/// Prepends the given test to the provided target path
fn prepend_to_file(path: &str, prepend_data: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    file.write_all(prepend_data.as_bytes())?;
    file.write_all(&data)?;
    Ok(())
}
#[cfg(not(feature = "codegen"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Err("`codegen` feature must be enabled".into())
}
