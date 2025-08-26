#[cfg(feature = "codegen")]
use itertools::Itertools;
#[cfg(feature = "codegen")]
use std::convert::Into;
#[cfg(feature = "codegen")]
use std::env::args;
#[cfg(feature = "codegen")]
use std::fs::OpenOptions;
#[cfg(feature = "codegen")]
use std::io::{Error, Read, Write};
#[cfg(feature = "codegen")]
use std::path::Path;
#[cfg(feature = "codegen")]
use tempfile::{NamedTempFile, TempDir};
#[cfg(feature = "codegen")]
const TARGET_PATH: &str = "./src/generated/crossplane.rs";

#[tokio::main]
/// Generates `src/generated/crossplane.rs` by fetching the Crossplane protobuf schema for the
/// provided tag argument and generates Rust types from it.
/// Errors if feature `codegen` is disabled (default).
/// Usage example: `cargo run --features codegen codegen v2.0.2`
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "codegen"))]
    return Err("`codegen` feature must be enabled".into());

    #[cfg(feature = "codegen")]
    {
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

        let target_temp_dir = TempDir::new()?;
        std::fs::create_dir_all("src/generated")?;
        tonic_prost_build::configure()
            // use prost_wkt_types to have serializable structs (used in ../example/src/lib.rs to map types)
            .extern_path(".google.protobuf.Struct", "::prost_wkt_types::Struct")
            // we only need server, client is not in scope and transport is hanlded by tonic
            .build_server(true)
            .build_transport(false)
            .build_client(false)
            .out_dir(target_temp_dir.path())
            .compile_protos(
                &[proto_file.path()],
                &[proto_file.path().parent().ok_or(
                    "temporary protobuf file is stored at a root location. Please report a bug.",
                )?],
            )?;

        prepend_to_file(
            target_temp_dir
                .path()
                .join("apiextensions.r#fn.proto.v1.rs"),
            TARGET_PATH,
            &format!(
                "// Generated from Crossplane {tag} composite function protobuf schema: https://github.com/crossplane/crossplane/tree/{tag}/proto/fn/v1\n"
            ),
        )?;
        Ok(())
    }
}

#[cfg(feature = "codegen")]
/// Copies the file from src to target (overwriting target) and prepends the provided additional data prior.
fn prepend_to_file(
    src: impl AsRef<Path>,
    target: impl AsRef<Path>,
    prepend_data: &str,
) -> Result<(), Error> {
    let mut file = OpenOptions::new().read(true).open(src)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(target)?;
    file.write_all(prepend_data.as_bytes())?;
    file.write_all(&data)?;
    Ok(())
}
